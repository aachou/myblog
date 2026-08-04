use axum::http::HeaderValue;
use axum::routing::{get, post};
use axum::Router;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::RwLock;
use tera::Tera;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use tracing_subscriber::EnvFilter;

use myblog::{handlers, post, AppState};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let mut tera = Tera::new("templates/**/*.html").unwrap_or_else(|e| {
        panic!("Tera initialization failed: {}", e);
    });
    // Auto-escaping disabled globally because `post.content_html` contains pre-rendered HTML
    // that must be output raw (via `| safe` in templates). Other variables that need escaping
    // must use `escape_xml()` or Tera's `escape_xml` filter explicitly.
    tera.autoescape_on(vec![]);

    let state = Arc::new(AppState {
        tera: RwLock::new(tera),
        posts: RwLock::new(Arc::new(Vec::new())),
        about_config: RwLock::new(myblog::read_about_config()),
    });

    match post::load_posts("posts") {
        Ok(posts) => {
            *state.posts.write().unwrap_or_else(|e| e.into_inner()) = Arc::new(posts);
        }
        Err(e) => tracing::warn!("Failed to load posts: {}", e),
    }

    let watcher_state = state.clone();
    tokio::spawn(async move {
        use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};

        let (tx, mut rx) = tokio::sync::mpsc::channel(32);
        let mut watcher = match RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                if let Ok(event) = res {
                    let _ = tx.blocking_send(event);
                }
            },
            Config::default(),
        ) {
            Ok(w) => w,
            Err(e) => {
                tracing::warn!("Failed to create file watcher: {}", e);
                return;
            }
        };

        if Path::new("posts").exists() {
            let _ = watcher.watch(Path::new("posts"), RecursiveMode::NonRecursive);
        }
        if Path::new("templates").exists() {
            let _ = watcher.watch(Path::new("templates"), RecursiveMode::NonRecursive);
        }

        const DEBOUNCE: std::time::Duration = std::time::Duration::from_millis(500);
        let mut signatures: std::collections::HashMap<PathBuf, Option<u64>> =
            std::collections::HashMap::new();

        while let Some(mut event) = rx.recv().await {
            // A single save can fire many events (write + rename + temp files); coalesce
            // everything arriving within a quiet window into one reload.
            while let Ok(Some(next)) = tokio::time::timeout(DEBOUNCE, rx.recv()).await {
                event.paths.extend(next.paths);
            }

            let is_post = event.paths.iter().any(|p| {
                p.extension().is_some_and(|e| e == "md")
                    && p.parent()
                        .and_then(|d| d.file_name())
                        .is_some_and(|n| n == "posts")
            });
            let is_template = event.paths.iter().any(|p| {
                p.extension().is_some_and(|e| e == "html")
                    && p.parent()
                        .and_then(|d| d.file_name())
                        .is_some_and(|n| n == "templates")
            });

            // Skip reloads when the file content is unchanged (editors/antivirus can keep
            // touching files after a single edit). Signatures are committed only after a
            // successful reload so a failed one is retried on the next event.
            if is_post {
                let (changed, new_sigs) =
                    file_signature_changed(&signatures, &event.paths, "posts", "md");
                if changed {
                    match post::load_posts("posts") {
                        Ok(new_posts) => {
                            signatures.extend(new_sigs);
                            *watcher_state
                                .posts
                                .write()
                                .unwrap_or_else(|e| e.into_inner()) = Arc::new(new_posts);
                            tracing::info!("Posts reloaded after file change");
                        }
                        Err(e) => tracing::warn!("Failed to reload posts: {}", e),
                    }
                }
            }
            if is_template {
                let (changed, new_sigs) =
                    file_signature_changed(&signatures, &event.paths, "templates", "html");
                if changed {
                    signatures.extend(new_sigs);
                    let mut tera = watcher_state
                        .tera
                        .write()
                        .unwrap_or_else(|e| e.into_inner());
                    let _ = tera.full_reload();
                    tracing::info!("Templates reloaded after file change");
                }
            }
        }
    });

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".into());
    tracing::info!("Blog server listening on http://0.0.0.0:{}", port);

    let html_routes = Router::new()
        .route("/", get(handlers::index_handler))
        .route("/post/:slug", get(handlers::post_handler))
        .route("/tag/:name", get(handlers::tag_handler))
        .route("/about", get(handlers::about_handler))
        .route("/tags", get(handlers::tags_handler))
        .route("/search", get(handlers::search_handler))
        .route("/archive", get(handlers::archive_handler))
        .route("/feed.xml", get(handlers::feed_handler))
        .route("/sitemap.xml", get(handlers::sitemap_handler))
        .route("/api/about", post(handlers::update_about_handler))
        .route("/api/upload-avatar", post(handlers::upload_avatar_handler))
        .fallback(handlers::not_found_handler)
        .layer(SetResponseHeaderLayer::overriding(
            axum::http::header::CACHE_CONTROL,
            HeaderValue::from_static("no-cache"),
        ));

    let app = Router::new()
        .merge(html_routes)
        .nest_service("/static", ServeDir::new("static"))
        .layer(SetResponseHeaderLayer::overriding(
            axum::http::header::CONTENT_SECURITY_POLICY,
            HeaderValue::from_static(
                "default-src 'self'; script-src 'self' https://utteranc.es; style-src 'unsafe-inline' 'self'; img-src 'self' data:; frame-src https://utteranc.es; frame-ancestors 'none'",
            ),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            axum::http::header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(CompressionLayer::new())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Returns `(changed, new_signatures)` for the watched files under `dir` (with extension
/// `ext`) whose content differs from `signatures`. The caller must commit the returned
/// `new_signatures` into the map only after a reload actually succeeds; otherwise a
/// failed reload would poison the dedup and never be retried. Deleted files are tracked
/// via `None` so repeated delete events don't keep triggering reloads.
fn file_signature_changed(
    signatures: &std::collections::HashMap<PathBuf, Option<u64>>,
    paths: &[PathBuf],
    dir: &str,
    ext: &str,
) -> (bool, Vec<(PathBuf, Option<u64>)>) {
    use std::hash::{Hash, Hasher};

    let mut changed = false;
    let mut new_sigs = Vec::new();
    for p in paths {
        if !p.extension().is_some_and(|e| e == ext)
            || !p
                .parent()
                .and_then(|d| d.file_name())
                .is_some_and(|n| n == dir)
        {
            continue;
        }
        let sig = std::fs::read(p).ok().map(|bytes| {
            let mut h = std::collections::hash_map::DefaultHasher::new();
            bytes.hash(&mut h);
            h.finish()
        });
        if signatures.get(p).and_then(|v| *v) != sig {
            new_sigs.push((p.to_path_buf(), sig));
            changed = true;
        }
    }
    (changed, new_sigs)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "myblog_watcher_test_{}_{}",
            std::process::id(),
            name
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn test_file_signature_changed_detects_content_change() {
        let dir = setup_dir("content");
        let dir_name = dir.file_name().unwrap().to_str().unwrap();
        let file = dir.join("post.md");
        std::fs::write(&file, "v1").unwrap();

        let mut signatures = std::collections::HashMap::new();
        let paths = vec![file.clone()];

        let (changed, new_sigs) = file_signature_changed(&signatures, &paths, dir_name, "md");
        assert!(changed, "first sight should report changed");
        signatures.extend(new_sigs);

        let (changed, _) = file_signature_changed(&signatures, &paths, dir_name, "md");
        assert!(!changed, "identical content should be skipped");

        std::fs::write(&file, "v2").unwrap();
        let (changed, new_sigs) = file_signature_changed(&signatures, &paths, dir_name, "md");
        assert!(changed, "modified content should be reported");
        signatures.extend(new_sigs);

        let (changed, _) = file_signature_changed(&signatures, &paths, dir_name, "md");
        assert!(!changed, "rewrite with same content should be skipped");

        std::fs::remove_file(&file).unwrap();
        let (changed, new_sigs) = file_signature_changed(&signatures, &paths, dir_name, "md");
        assert!(changed, "deletion should be reported once");
        signatures.extend(new_sigs);

        let (changed, _) = file_signature_changed(&signatures, &paths, dir_name, "md");
        assert!(!changed, "repeated delete events should be skipped");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_file_signature_changed_not_committed_until_success() {
        let dir = setup_dir("retry");
        let dir_name = dir.file_name().unwrap().to_str().unwrap();
        let file = dir.join("post.md");
        std::fs::write(&file, "v1").unwrap();

        let signatures = std::collections::HashMap::new();
        let paths = vec![file.clone()];

        // Simulate a failed reload: don't commit the new signature.
        let (changed, _) = file_signature_changed(&signatures, &paths, dir_name, "md");
        assert!(changed, "first sight should report changed");

        // A repeated event must still report changed (retry), not be deduped away.
        let (changed, _) = file_signature_changed(&signatures, &paths, dir_name, "md");
        assert!(
            changed,
            "uncommitted signature must not suppress the retry event"
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_file_signature_changed_ignores_unrelated_paths() {
        let dir = setup_dir("ignore");
        let dir_name = dir.file_name().unwrap().to_str().unwrap();
        let md_file = dir.join("post.md");
        let other_file = dir.join("notes.txt");
        std::fs::write(&md_file, "x").unwrap();
        std::fs::write(&other_file, "x").unwrap();

        let signatures = std::collections::HashMap::new();
        let (changed, _) = file_signature_changed(
            &signatures,
            std::slice::from_ref(&other_file),
            dir_name,
            "md",
        );
        assert!(
            !changed,
            "unrelated file type should not trigger a change"
        );
        let (changed, _) = file_signature_changed(
            &signatures,
            std::slice::from_ref(&md_file),
            dir_name,
            "md",
        );
        assert!(changed, "md file in the dir should be reported");

        let _ = std::fs::remove_dir_all(&dir);
    }
}

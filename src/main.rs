use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;
use tera::Tera;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use tracing_subscriber::EnvFilter;
use axum::http::HeaderValue;
use axum::routing::get;
use axum::Router;

use myblog::{AppState, handlers, post};

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

        while let Some(event) = rx.recv().await {
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;

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

            if is_post {
                match post::load_posts("posts") {
                    Ok(new_posts) => {
                        *watcher_state.posts.write().unwrap_or_else(|e| e.into_inner()) = Arc::new(new_posts);
                        tracing::info!("Posts reloaded after file change");
                    }
                    Err(e) => tracing::warn!("Failed to reload posts: {}", e),
                }
            }
            if is_template {
                if let Ok(mut tera) = watcher_state.tera.write() {
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
                "default-src 'self'; style-src 'unsafe-inline' 'self'; img-src 'self' data:; frame-ancestors 'none'",
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

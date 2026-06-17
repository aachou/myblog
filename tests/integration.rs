use std::path::PathBuf;
use std::sync::Arc;
use myblog::{AppState, handlers, post};
use axum::Router;
use axum::http::HeaderValue;
use axum::routing::get;
use tower_http::compression::CompressionLayer;
use tower_http::set_header::SetResponseHeaderLayer;

fn setup_test_posts(suffix: &str) -> (Vec<post::Post>, PathBuf) {
    let dir = std::env::temp_dir().join(format!("myblog_int_{}_{}", suffix, std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    let p1 = "+++\ntitle = \"Post Three\"\ndate = \"2024-06-15\"\ntags = [\"rust\", \"web\"]\nexcerpt = \"Third post\"\n+++\n\n## Overview\n\nSome content.\n\n## Details\n\nMore text here for word count purposes.";
    let p2 = "+++\ntitle = \"Post Two\"\ndate = \"2024-03-10\"\ntags = [\"rust\", \"async\"]\n+++\n\n## Getting Started\n\nThis is a longer post with more words to test reading time calculations and other features.\n\n### Prerequisites\n\nYou need to know some basics.\n\n### Installation\n\nFollow these steps.\n\nLet me add more text here to make the word count higher so the reading time calculation can be properly validated.\n\nOne more paragraph for good measure.";
    let p3 = "+++\ntitle = \"Post One\"\ndate = \"2024-01-01\"\ntags = [\"web\", \"css\"]\n+++\n\n## Introduction\n\nShort post.";

    std::fs::write(dir.join("post-three.md"), p1).unwrap();
    std::fs::write(dir.join("post-two.md"), p2).unwrap();
    std::fs::write(dir.join("post-one.md"), p3).unwrap();

    let dir_str = dir.to_str().unwrap().to_string();
    let posts = post::load_posts(&dir_str).unwrap();
    (posts, dir)
}

#[test]
fn test_load_posts_sorted_newest_first() {
    let (posts, dir) = setup_test_posts("sorted");
    assert_eq!(posts.len(), 3);
    assert_eq!(posts[0].slug, "post-three");
    assert_eq!(posts[1].slug, "post-two");
    assert_eq!(posts[2].slug, "post-one");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_each_post_has_required_fields() {
    let (posts, dir) = setup_test_posts("fields");
    for p in &posts {
        assert!(!p.frontmatter.title.is_empty(), "Post {} has empty title", p.slug);
        assert!(!p.frontmatter.date.is_empty(), "Post {} has empty date", p.slug);
        assert!(!p.frontmatter.tags.is_empty(), "Post {} has empty tags", p.slug);
        assert!(!p.content_html.is_empty(), "Post {} has empty content", p.slug);
        assert!(p.reading_time >= 1, "Post {} has reading_time < 1", p.slug);
        assert!(p.word_count > 0, "Post {} has word_count == 0", p.slug);
        assert!(!p.excerpt.is_empty(), "Post {} has empty excerpt", p.slug);
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_post_content_is_valid_html() {
    let (posts, dir) = setup_test_posts("html");
    for p in &posts {
        assert!(
            p.content_html.starts_with('<'),
            "Post {} content_html does not start with HTML tag: {}",
            p.slug, &p.content_html[..p.content_html.len().min(50)]
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_search_text_is_lowercased() {
    let (posts, dir) = setup_test_posts("lower");
    for p in &posts {
        assert_eq!(
            p.search_text,
            p.search_text.to_lowercase(),
            "Post {} search_text is not lowercased", p.slug
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_tag_filter() {
    let (posts, dir) = setup_test_posts("tagfilt");
    let tag = "rust";
    let filtered: Vec<&post::Post> = posts.iter()
        .filter(|p| p.frontmatter.tags.iter().any(|t| t == tag))
        .collect();
    assert_eq!(filtered.len(), 2);
    for p in &filtered {
        assert!(p.frontmatter.tags.contains(&tag.to_string()));
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_render_markdown_adds_heading_ids() {
    let (posts, dir) = setup_test_posts("heading");
    let rendered = post::render_markdown(&posts[0].content_html);
    assert!(!rendered.is_empty());
    assert!(rendered.contains("id=\""));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_toc_extraction() {
    let (posts, dir) = setup_test_posts("toc");
    let posts_with_toc: Vec<&post::Post> = posts.iter()
        .filter(|p| !p.toc.is_empty())
        .collect();
    assert!(!posts_with_toc.is_empty(), "Some posts should have TOC entries");
    for p in &posts_with_toc {
        for entry in &p.toc {
            assert!(entry.level >= 2 && entry.level <= 4);
            assert!(!entry.id.is_empty());
            assert!(!entry.text.is_empty());
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_date_formats_are_valid() {
    let (posts, dir) = setup_test_posts("dates");
    let date_re = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    for p in &posts {
        assert!(
            date_re.is_match(&p.frontmatter.date),
            "Post {} has invalid date format: {}",
            p.slug, p.frontmatter.date
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_all_slugs_are_unique() {
    let (posts, dir) = setup_test_posts("slugs");
    let mut slugs: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for p in &posts {
        assert!(
            slugs.insert(&p.slug),
            "Duplicate slug found: {}", p.slug
        );
    }
    assert_eq!(slugs.len(), posts.len());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_reading_time_formula() {
    let (posts, dir) = setup_test_posts("readtime");
    for p in &posts {
        let expected = std::cmp::max(1, (p.word_count + 100) / 200);
        assert_eq!(
            p.reading_time, expected,
            "Post {} reading_time mismatch: got {}, expected {} (word_count={})",
            p.slug, p.reading_time, expected, p.word_count
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_frontmatter_excerpt_used_when_provided() {
    let (posts, dir) = setup_test_posts("excerpt1");
    let post_three = posts.iter().find(|p| p.slug == "post-three").unwrap();
    assert_eq!(post_three.excerpt, "Third post");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_frontmatter_excerpt_auto_generated_when_missing() {
    let (posts, dir) = setup_test_posts("excerpt2");
    let post_one = posts.iter().find(|p| p.slug == "post-one").unwrap();
    assert!(!post_one.excerpt.is_empty());
    assert!(post_one.excerpt.len() <= 163);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_full_post_pipeline() {
    let dir = std::env::temp_dir().join(format!("myblog_pipeline_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    let content = "+++\ntitle = \"Pipeline\"\ndate = \"2024-05-20\"\ntags = [\"test\"]\n+++\n\n## Heading\n\nBody text.";
    std::fs::write(dir.join("pipeline.md"), content).unwrap();

    let posts = post::load_posts(dir.to_str().unwrap()).unwrap();
    assert_eq!(posts.len(), 1);

    let p = &posts[0];
    assert_eq!(p.slug, "pipeline");
    assert_eq!(p.frontmatter.title, "Pipeline");
    assert!(p.content_html.contains("<h2 id=\"heading\""));
    assert_eq!(p.toc.len(), 1);
    assert_eq!(p.toc[0].text, "Heading");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_load_posts_empty_dir() {
    let dir = std::env::temp_dir().join(format!("myblog_empty_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    let posts = post::load_posts(dir.to_str().unwrap()).unwrap();
    assert!(posts.is_empty());

    let _ = std::fs::remove_dir_all(&dir);
}

fn setup_router(temp_dir: &std::path::Path) -> Router {
    let mut tera = tera::Tera::new("templates/**/*.html").unwrap();
    tera.autoescape_on(vec![]);

    let posts = post::load_posts(temp_dir.to_str().unwrap()).unwrap();
    let state = Arc::new(AppState {
        tera: std::sync::RwLock::new(tera),
        posts: std::sync::RwLock::new(Arc::new(posts)),
    });

    Router::new()
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
        ))
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
        .with_state(state)
}

#[tokio::test]
async fn test_index_handler_returns_200() {
    let dir = std::env::temp_dir().join(format!("myblog_http_index_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let post = "+++\ntitle = \"Test\"\ndate = \"2024-01-01\"\ntags = []\n+++\n\nContent";
    std::fs::write(dir.join("test.md"), post).unwrap();

    let mut app = setup_router(&dir);
    let response = tower::Service::call(
        &mut app,
        axum::http::Request::builder().uri("/").body(axum::body::Body::empty()).unwrap()
    ).await.unwrap();
    assert_eq!(response.status(), 200);

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn test_fallback_returns_404() {
    let dir = std::env::temp_dir().join(format!("myblog_http_fallback_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    let mut app = setup_router(&dir);
    let response = tower::Service::call(
        &mut app,
        axum::http::Request::builder().uri("/nonexistent").body(axum::body::Body::empty()).unwrap()
    ).await.unwrap();
    assert_eq!(response.status(), 404);

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn test_missing_post_returns_404() {
    let dir = std::env::temp_dir().join(format!("myblog_http_post404_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    let mut app = setup_router(&dir);
    let response = tower::Service::call(
        &mut app,
        axum::http::Request::builder().uri("/post/no-such-slug").body(axum::body::Body::empty()).unwrap()
    ).await.unwrap();
    assert_eq!(response.status(), 404);

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn test_feed_returns_200() {
    let dir = std::env::temp_dir().join(format!("myblog_http_feed_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let post = "+++\ntitle = \"Feed Post\"\ndate = \"2024-06-15\"\ntags = []\n+++\n\nContent";
    std::fs::write(dir.join("feed.md"), post).unwrap();

    let mut app = setup_router(&dir);
    let response = tower::Service::call(
        &mut app,
        axum::http::Request::builder().uri("/feed.xml").body(axum::body::Body::empty()).unwrap()
    ).await.unwrap();
    assert_eq!(response.status(), 200);
    assert!(response.headers().get("content-type").unwrap().to_str().unwrap().starts_with("application/rss+xml"));

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn test_search_returns_200() {
    let dir = std::env::temp_dir().join(format!("myblog_http_search_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let post = "+++\ntitle = \"Searchable\"\ndate = \"2024-01-01\"\ntags = []\n+++\n\nSearchable content";
    std::fs::write(dir.join("search-test.md"), post).unwrap();

    let mut app = setup_router(&dir);
    let response = tower::Service::call(
        &mut app,
        axum::http::Request::builder().uri("/search?q=Searchable").body(axum::body::Body::empty()).unwrap()
    ).await.unwrap();
    assert_eq!(response.status(), 200);

    let _ = std::fs::remove_dir_all(&dir);
}
#[tokio::test]
async fn test_post_handler_valid_slug_200() {
    let dir = std::env::temp_dir().join(format!("myblog_http_post200_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let post = "+++\ntitle = \"Valid Post\"\ndate = \"2024-06-15\"\ntags = []\n+++\n\nContent";
    std::fs::write(dir.join("valid-post.md"), post).unwrap();

    let mut app = setup_router(&dir);
    let response = tower::Service::call(
        &mut app,
        axum::http::Request::builder().uri("/post/valid-post").body(axum::body::Body::empty()).unwrap()
    ).await.unwrap();
    assert_eq!(response.status(), 200);

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn test_post_page_has_utterances_script() {
    use http_body_util::BodyExt;

    let dir = std::env::temp_dir().join(format!("myblog_utt_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let post = "+++\ntitle = \"Utterances Test\"\ndate = \"2024-06-15\"\ntags = []\n+++\n\nContent";
    std::fs::write(dir.join("utt-test.md"), post).unwrap();

    let mut app = setup_router(&dir);
    let response = tower::Service::call(
        &mut app,
        axum::http::Request::builder().uri("/post/utt-test").body(axum::body::Body::empty()).unwrap()
    ).await.unwrap();
    assert_eq!(response.status(), 200);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let html = String::from_utf8(body.to_vec()).unwrap();
    assert!(html.contains("utteranc.es/client.js"), "Post page should include utterances script");
    assert!(html.contains("repo=\"aachou/myblog\""), "Post page should include repo config");
    assert!(html.contains("issue-term=\"pathname\""), "Post page should use pathname issue term");
    assert!(html.contains("theme=\"preferred-color-scheme\""), "Post page should use preferred-color-scheme");

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn test_tag_handler_returns_200() {
    let dir = std::env::temp_dir().join(format!("myblog_http_tag_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let post = "+++\ntitle = \"Tagged\"\ndate = \"2024-01-01\"\ntags = [\"mytag\"]\n+++\n\nContent";
    std::fs::write(dir.join("tagged.md"), post).unwrap();

    let mut app = setup_router(&dir);
    let response = tower::Service::call(
        &mut app,
        axum::http::Request::builder().uri("/tag/mytag").body(axum::body::Body::empty()).unwrap()
    ).await.unwrap();
    assert_eq!(response.status(), 200);

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn test_tag_handler_no_results_200() {
    let dir = std::env::temp_dir().join(format!("myblog_http_tagno_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let post = "+++\ntitle = \"Tagged\"\ndate = \"2024-01-01\"\ntags = [\"a\"]\n+++\n\nContent";
    std::fs::write(dir.join("tagged.md"), post).unwrap();

    let mut app = setup_router(&dir);
    let response = tower::Service::call(
        &mut app,
        axum::http::Request::builder().uri("/tag/nonexistent").body(axum::body::Body::empty()).unwrap()
    ).await.unwrap();
    assert_eq!(response.status(), 200);

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn test_about_handler_returns_200() {
    let dir = std::env::temp_dir().join(format!("myblog_http_about_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    let mut app = setup_router(&dir);
    let response = tower::Service::call(
        &mut app,
        axum::http::Request::builder().uri("/about").body(axum::body::Body::empty()).unwrap()
    ).await.unwrap();
    assert_eq!(response.status(), 200);

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn test_tags_handler_returns_200() {
    let dir = std::env::temp_dir().join(format!("myblog_http_tags_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let post = "+++\ntitle = \"Tagged\"\ndate = \"2024-01-01\"\ntags = [\"demo\"]\n+++\n\nContent";
    std::fs::write(dir.join("tagged.md"), post).unwrap();

    let mut app = setup_router(&dir);
    let response = tower::Service::call(
        &mut app,
        axum::http::Request::builder().uri("/tags").body(axum::body::Body::empty()).unwrap()
    ).await.unwrap();
    assert_eq!(response.status(), 200);

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn test_archive_handler_returns_200() {
    let dir = std::env::temp_dir().join(format!("myblog_http_archive_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let post = "+++\ntitle = \"Archive Post\"\ndate = \"2024-06-15\"\ntags = []\n+++\n\nContent";
    std::fs::write(dir.join("archive-post.md"), post).unwrap();

    let mut app = setup_router(&dir);
    let response = tower::Service::call(
        &mut app,
        axum::http::Request::builder().uri("/archive").body(axum::body::Body::empty()).unwrap()
    ).await.unwrap();
    assert_eq!(response.status(), 200);

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn test_sitemap_handler_returns_200() {
    let dir = std::env::temp_dir().join(format!("myblog_http_sitemap_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let post = "+++\ntitle = \"Sitemap\"\ndate = \"2024-06-15\"\ntags = []\n+++\n\nContent";
    std::fs::write(dir.join("sitemap.md"), post).unwrap();

    let mut app = setup_router(&dir);
    let response = tower::Service::call(
        &mut app,
        axum::http::Request::builder().uri("/sitemap.xml").body(axum::body::Body::empty()).unwrap()
    ).await.unwrap();
    assert_eq!(response.status(), 200);
    let content_type = response.headers().get("content-type").unwrap().to_str().unwrap().to_string();
    assert!(content_type.starts_with("application/xml"), "expected application/xml, got: {}", content_type);

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn test_search_without_query_200() {
    let dir = std::env::temp_dir().join(format!("myblog_http_searchnoq_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let post = "+++\ntitle = \"Searchable\"\ndate = \"2024-01-01\"\ntags = []\n+++\n\nContent";
    std::fs::write(dir.join("search.md"), post).unwrap();

    let mut app = setup_router(&dir);
    let response = tower::Service::call(
        &mut app,
        axum::http::Request::builder().uri("/search").body(axum::body::Body::empty()).unwrap()
    ).await.unwrap();
    assert_eq!(response.status(), 200);

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn test_search_empty_query_200() {
    let dir = std::env::temp_dir().join(format!("myblog_http_searchemp_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let post = "+++\ntitle = \"Searchable\"\ndate = \"2024-01-01\"\ntags = []\n+++\n\nContent";
    std::fs::write(dir.join("search.md"), post).unwrap();

    let mut app = setup_router(&dir);
    let response = tower::Service::call(
        &mut app,
        axum::http::Request::builder().uri("/search?q=").body(axum::body::Body::empty()).unwrap()
    ).await.unwrap();
    assert_eq!(response.status(), 200);

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn test_search_no_results_200() {
    let dir = std::env::temp_dir().join(format!("myblog_http_nores_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let post = "+++\ntitle = \"Real\"\ndate = \"2024-01-01\"\ntags = []\n+++\n\nContent";
    std::fs::write(dir.join("real.md"), post).unwrap();

    let mut app = setup_router(&dir);
    let response = tower::Service::call(
        &mut app,
        axum::http::Request::builder().uri("/search?q=zzzznonexistent").body(axum::body::Body::empty()).unwrap()
    ).await.unwrap();
    assert_eq!(response.status(), 200);

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn test_response_headers() {
    let dir = std::env::temp_dir().join(format!("myblog_http_headers_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let post = "+++\ntitle = \"Headers\"\ndate = \"2024-01-01\"\ntags = []\n+++\n\nContent";
    std::fs::write(dir.join("headers.md"), post).unwrap();

    let mut app = setup_router(&dir);
    let response = tower::Service::call(
        &mut app,
        axum::http::Request::builder().uri("/").body(axum::body::Body::empty()).unwrap()
    ).await.unwrap();

    let csp = response.headers().get("content-security-policy")
        .expect("CSP header should be present")
        .to_str().unwrap();
    assert!(csp.contains("default-src 'self'"), "CSP should contain default-src 'self': {}", csp);
    assert!(csp.contains("https://utteranc.es"), "CSP should allow utteranc.es: {}", csp);
    assert!(csp.contains("script-src 'self' https://utteranc.es"), "CSP should allow utteranc.es scripts: {}", csp);
    assert!(csp.contains("frame-src https://utteranc.es"), "CSP should allow utteranc.es frames: {}", csp);

    let no_sniff = response.headers().get("x-content-type-options")
        .expect("X-Content-Type-Options header should be present")
        .to_str().unwrap();
    assert_eq!(no_sniff, "nosniff");

    let cache = response.headers().get("cache-control")
        .expect("Cache-Control header should be present")
        .to_str().unwrap();
    assert_eq!(cache, "no-cache");

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn test_feed_handler_empty_posts_200() {
    let dir = std::env::temp_dir().join(format!("myblog_http_feedempty_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();

    let mut app = setup_router(&dir);
    let response = tower::Service::call(
        &mut app,
        axum::http::Request::builder().uri("/feed.xml").body(axum::body::Body::empty()).unwrap()
    ).await.unwrap();
    assert_eq!(response.status(), 200);
    let content_type = response.headers().get("content-type").unwrap().to_str().unwrap().to_string();
    assert!(content_type.starts_with("application/rss+xml"), "expected application/rss+xml, got: {}", content_type);

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn test_index_pagination_page_2_200() {
    let dir = std::env::temp_dir().join(format!("myblog_http_page2_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    // Create 3 posts to test pagination with posts_per_page=5 (default)
    // With 3 posts and page=2, it should still return 200 with empty page
    for i in 0..3u8 {
        let content = format!(
            "+++\ntitle = \"Post {}\"\ndate = \"2024-06-{:02}\"\ntags = []\n+++\n\nContent",
            15 - i, 15 - i
        );
        std::fs::write(dir.join(format!("post-{}.md", i)), content).unwrap();
    }

    let mut app = setup_router(&dir);
    let response = tower::Service::call(
        &mut app,
        axum::http::Request::builder().uri("/?page=2").body(axum::body::Body::empty()).unwrap()
    ).await.unwrap();
    assert_eq!(response.status(), 200);

    let _ = std::fs::remove_dir_all(&dir);
}

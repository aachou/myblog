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

use myblog::{AppState, PostCache, handlers};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let mut tera = Tera::new("templates/**/*.html").unwrap_or_else(|e| {
        panic!("Tera initialization failed: {}", e);
    });
    tera.autoescape_on(vec![]);

    tracing::info!("Blog server listening on http://127.0.0.1:3000");

    let state = Arc::new(AppState {
        tera,
        post_cache: RwLock::new(PostCache {
            posts: Arc::new(Vec::new()),
            last_mtime: None,
        }),
    });

    let app = Router::new()
        .route("/", get(handlers::index_handler))
        .route("/post/:slug", get(handlers::post_handler))
        .route("/tag/:name", get(handlers::tag_handler))
        .route("/about", get(handlers::about_handler))
        .route("/tags", get(handlers::tags_handler))
        .route("/search", get(handlers::search_handler))
        .route("/archive", get(handlers::archive_handler))
        .route("/feed.xml", get(handlers::feed_handler))
        .route("/sitemap.xml", get(handlers::sitemap_handler))
        .nest_service("/static", ServeDir::new("static"))
        .layer(CompressionLayer::new())
        .layer(SetResponseHeaderLayer::overriding(
            axum::http::header::CACHE_CONTROL,
            HeaderValue::from_static("no-cache"),
        ))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

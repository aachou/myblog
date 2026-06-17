pub mod post;
pub mod handlers;

use std::sync::OnceLock;
use std::sync::{Arc, RwLock};
use tera::Tera;

pub fn posts_per_page() -> usize {
    static V: OnceLock<usize> = OnceLock::new();
    *V.get_or_init(|| {
        std::env::var("POSTS_PER_PAGE").ok().and_then(|s| s.parse().ok()).unwrap_or(5)
    })
}

pub fn site_url() -> &'static str {
    static V: OnceLock<String> = OnceLock::new();
    V.get_or_init(|| {
        std::env::var("SITE_URL").unwrap_or_else(|_| "http://127.0.0.1:3000".into())
    })
}

pub fn site_title() -> &'static str {
    static V: OnceLock<String> = OnceLock::new();
    V.get_or_init(|| {
        std::env::var("SITE_TITLE").unwrap_or_else(|_| "MyBlog".into())
    })
}

pub fn site_desc() -> &'static str {
    static V: OnceLock<String> = OnceLock::new();
    V.get_or_init(|| {
        std::env::var("SITE_DESC").unwrap_or_else(|_| "A personal blog built with Rust and Axum".into())
    })
}

pub struct AppState {
    pub tera: RwLock<Tera>,
    pub posts: RwLock<Arc<Vec<post::Post>>>,
}

pub fn get_cached_posts(state: &AppState) -> Arc<Vec<post::Post>> {
    state.posts.read().unwrap_or_else(|e| e.into_inner()).clone()
}

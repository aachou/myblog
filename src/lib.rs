pub mod post;
pub mod handlers;

use std::sync::{Arc, RwLock};
use std::time::SystemTime;
use tera::Tera;

pub const POSTS_PER_PAGE: usize = 5;
pub const SITE_URL: &str = "http://127.0.0.1:3000";
pub const SITE_TITLE: &str = "MyBlog";
pub const SITE_DESC: &str = "A personal blog built with Rust and Axum";

#[allow(dead_code)]
pub struct PostCache {
    pub posts: Arc<Vec<post::Post>>,
    pub last_mtime: Option<SystemTime>,
}

#[allow(dead_code)]
pub struct AppState {
    pub tera: Tera,
    pub post_cache: RwLock<PostCache>,
}

fn posts_mtime() -> Option<SystemTime> {
    let dir = std::fs::read_dir("posts").ok()?;
    dir.filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        .filter_map(|e| e.metadata().ok())
        .filter_map(|m| m.modified().ok())
        .max()
}

fn recover<T>(e: std::sync::PoisonError<T>) -> T {
    e.into_inner()
}

#[allow(dead_code)]
pub fn get_cached_posts(state: &AppState) -> Arc<Vec<post::Post>> {
    let current_mtime = posts_mtime();

    {
        let cache = state.post_cache.read().unwrap_or_else(recover);
        if let (Some(cached), Some(current)) = (cache.last_mtime, current_mtime) {
            if current <= cached && !cache.posts.is_empty() {
                return Arc::clone(&cache.posts);
            }
        }
    }

    let mut cache = state.post_cache.write().unwrap_or_else(recover);
    if let (Some(cached), Some(current)) = (cache.last_mtime, current_mtime) {
        if current <= cached && !cache.posts.is_empty() {
            return Arc::clone(&cache.posts);
        }
    }

    match post::load_posts("posts") {
        Ok(posts) => {
            let posts = Arc::new(posts);
            cache.posts = Arc::clone(&posts);
            cache.last_mtime = current_mtime;
            posts
        }
        Err(e) => {
            tracing::warn!("Failed to reload posts: {}", e);
            Arc::clone(&cache.posts)
        }
    }
}

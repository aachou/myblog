pub mod post;
pub mod handlers;

use std::sync::OnceLock;
use std::sync::{Arc, RwLock};
use tera::Tera;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SiteConfig {
    #[serde(default = "default_title")]
    pub title: String,
    #[serde(default = "default_desc")]
    pub description: String,
    #[serde(default = "default_url")]
    pub url: String,
    #[serde(default = "default_ppp")]
    pub posts_per_page: usize,
}

fn default_title() -> String { "MyBlog".into() }
fn default_desc() -> String { "A personal blog built with Rust and Axum".into() }
fn default_url() -> String { "http://127.0.0.1:3000".into() }
fn default_ppp() -> usize { 5 }

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            title: default_title(),
            description: default_desc(),
            url: default_url(),
            posts_per_page: default_ppp(),
        }
    }
}

pub fn read_site_config() -> &'static SiteConfig {
    static V: OnceLock<SiteConfig> = OnceLock::new();
    V.get_or_init(|| {
        let mut config: SiteConfig = std::fs::read_to_string("config/site.json")
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default();

        if let Ok(val) = std::env::var("SITE_TITLE") { config.title = val; }
        if let Ok(val) = std::env::var("SITE_DESC") { config.description = val; }
        if let Ok(val) = std::env::var("SITE_URL") { config.url = val; }
        if let Ok(val) = std::env::var("POSTS_PER_PAGE") {
            if let Ok(n) = val.parse() { config.posts_per_page = n; }
        }

        config
    })
}

pub fn posts_per_page() -> usize {
    read_site_config().posts_per_page
}

pub fn site_url() -> &'static str {
    &read_site_config().url
}

pub fn site_title() -> &'static str {
    &read_site_config().title
}

pub fn site_desc() -> &'static str {
    &read_site_config().description
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AboutConfig {
    pub author_name: String,
    pub avatar_path: String,
}

impl Default for AboutConfig {
    fn default() -> Self {
        Self {
            author_name: "阿愁".into(),
            avatar_path: "/static/images/avatar.jpg".into(),
        }
    }
}

pub fn read_about_config() -> AboutConfig {
    let path = "config/about.json";
    match std::fs::read_to_string(path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => AboutConfig::default(),
    }
}

pub fn save_about_config(config: &AboutConfig) {
    let path = "config/about.json";
    if let Ok(content) = serde_json::to_string_pretty(config) {
        let _ = std::fs::write(path, content);
    }
}

pub struct AppState {
    pub tera: RwLock<Tera>,
    pub posts: RwLock<Arc<Vec<post::Post>>>,
    pub about_config: RwLock<AboutConfig>,
}

pub fn get_cached_posts(state: &AppState) -> Arc<Vec<post::Post>> {
    state.posts.read().unwrap_or_else(|e| e.into_inner()).clone()
}

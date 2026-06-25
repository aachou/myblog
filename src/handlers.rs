use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use axum::extract::{Multipart, Path, Query, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::post::escape_xml;
use crate::{get_cached_posts, posts_per_page, save_about_config, site_desc, site_title, site_url, AppState};

#[derive(Deserialize)]
pub struct PageQuery {
    page: Option<usize>,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    q: Option<String>,
    page: Option<usize>,
}

#[derive(Serialize)]
pub struct PageInfo {
    current: usize,
    total: usize,
    has_prev: bool,
    has_next: bool,
    prev: usize,
    next: usize,
    pages: Vec<usize>,
    show_start_ellipsis: bool,
    show_end_ellipsis: bool,
}

fn compute_page_info(current: usize, total: usize) -> PageInfo {
    let has_prev = current > 1;
    let has_next = current < total;
    let prev = if has_prev { current - 1 } else { 1 };
    let next = if has_next { current + 1 } else { total };

    let mut pages = Vec::new();
    let show_start_ellipsis;
    let show_end_ellipsis;
    if total <= 5 {
        for p in 1..=total { pages.push(p); }
        show_start_ellipsis = false;
        show_end_ellipsis = false;
    } else {
        let win_start = (if current <= 3 { 2 } else { current - 1 }).max(2).min(total - 1);
        let win_end = (if current >= total - 2 { total - 1 } else { current + 1 }).max(2).min(total - 1);
        for p in win_start..=win_end { pages.push(p); }
        show_start_ellipsis = win_start > 2;
        show_end_ellipsis = win_end < total - 1;
    }
    PageInfo { current, total, has_prev, has_next, prev, next, pages, show_start_ellipsis, show_end_ellipsis }
}

fn render(state: &AppState, template: &str, ctx: &tera::Context) -> Html<String> {
    Html(state.tera.read().unwrap_or_else(|e| e.into_inner()).render(template, ctx).unwrap_or_else(|e| {
        format!("<h1>Template error</h1><p>{}</p>", e)
    }))
}

fn date_to_rfc2822(date: &str) -> String {
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() < 3 { return date.to_string(); }
    let y = parts[0];
    let m = match parts[1] {
        "01" => "Jan", "02" => "Feb", "03" => "Mar", "04" => "Apr",
        "05" => "May", "06" => "Jun", "07" => "Jul", "08" => "Aug",
        "09" => "Sep", "10" => "Oct", "11" => "Nov", "12" => "Dec",
        _ => parts[1],
    };
    let d = parts[2];
    format!("{}, {} {} {} 00:00:00 GMT", weekday(y, m, d), d, m, y)
}

fn weekday(y: &str, m: &str, d: &str) -> &'static str {
    let y: i32 = y.parse().unwrap_or(2024);
    let m: i32 = ["", "Jan", "Feb", "Mar", "Apr", "May", "Jun",
                    "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]
        .iter().position(|&s| s == m).unwrap_or(1) as i32;
    let d: i32 = d.parse().unwrap_or(1);
    let (y, m) = if m < 3 { (y - 1, m + 12) } else { (y, m) };
    let c = y / 100;
    let y_ = y % 100;
    let w = (d + (13 * (m + 1)) / 5 + y_ + y_ / 4 + c / 4 + 5 * c).rem_euclid(7);
    ["Sat", "Sun", "Mon", "Tue", "Wed", "Thu", "Fri"][w as usize]
}

pub async fn index_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PageQuery>,
) -> Html<String> {
    let posts = get_cached_posts(&state);
    let page = query.page.unwrap_or(1).max(1);
    let total_posts = posts.len();
    let total_pages = if total_posts == 0 { 1 } else { total_posts.div_ceil(posts_per_page()) };
    let page = page.min(total_pages);

    let start = (page - 1) * posts_per_page();
    let end = std::cmp::min(start + posts_per_page(), total_posts);
    let page_posts = &posts[start..end];

    let page_info = compute_page_info(page, total_pages);

    let mut ctx = tera::Context::new();
    ctx.insert("posts", &page_posts);
    ctx.insert("page_info", &page_info);
    ctx.insert("og_title", &site_title());
    ctx.insert("og_description", &site_desc());
    ctx.insert("og_url", &site_url());
    render(&state, "index.html", &ctx)
}

pub async fn post_handler(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Response {
    let posts = get_cached_posts(&state);
    if let Some((idx, post)) = posts.iter().enumerate().find(|(_, p)| p.slug == slug) {
        let prev = if idx + 1 < posts.len() { Some(&posts[idx + 1]) } else { None };
        let next = if idx > 0 { Some(&posts[idx - 1]) } else { None };
        let prev_slug = prev.map(|p| p.slug.as_str());
        let prev_title = prev.map(|p| p.frontmatter.title.as_str());
        let next_slug = next.map(|p| p.slug.as_str());
        let next_title = next.map(|p| p.frontmatter.title.as_str());

        let related: Vec<&crate::post::Post> = posts.iter()
            .filter(|p| p.slug != slug)
            .filter(|p| p.frontmatter.tags.iter().any(|t| post.frontmatter.tags.contains(t)))
            .take(3)
            .collect();

        let mut ctx = tera::Context::new();
        ctx.insert("post", post);
        ctx.insert("related", &related);
        ctx.insert("prev_slug", &prev_slug);
        ctx.insert("prev_title", &prev_title);
        ctx.insert("next_slug", &next_slug);
        ctx.insert("next_title", &next_title);
        ctx.insert("og_title", &post.frontmatter.title);
        ctx.insert("og_description", &post.excerpt);
        ctx.insert("og_url", &format!("{}/post/{}", site_url(), slug));
        render(&state, "post.html", &ctx).into_response()
    } else {
        let mut ctx = tera::Context::new();
        ctx.insert("slug", &escape_xml(&slug));
        (StatusCode::NOT_FOUND, render(&state, "404.html", &ctx)).into_response()
    }
}

pub async fn tag_handler(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
    Query(query): Query<PageQuery>,
) -> Html<String> {
    let posts = get_cached_posts(&state);
    let page = query.page.unwrap_or(1).max(1);

    let filtered: Vec<&crate::post::Post> = posts.iter()
        .filter(|p| p.frontmatter.tags.iter().any(|t| t == &name))
        .collect();

    let total_filtered = filtered.len();
    let total_pages = if total_filtered == 0 { 1 } else { total_filtered.div_ceil(posts_per_page()) };
    let page = page.min(total_pages);

    let start = (page - 1) * posts_per_page();
    let end = std::cmp::min(start + posts_per_page(), total_filtered);
    let page_posts = &filtered[start..end];

    let page_info = compute_page_info(page, total_pages);

    let tag_display = escape_xml(&name);
    let tag_url = url::form_urlencoded::byte_serialize(name.as_bytes()).collect::<String>();

    let mut ctx = tera::Context::new();
    ctx.insert("posts", &page_posts);
    ctx.insert("page_info", &page_info);
    ctx.insert("tag", &tag_display);
    ctx.insert("tag_raw", &name);
    ctx.insert("total_filtered", &total_filtered);
    ctx.insert("og_title", &format!("#{} - {}", tag_display, site_title()));
    ctx.insert("og_description", &format!("Posts tagged with #{}", tag_display));
    ctx.insert("og_url", &format!("{}/tag/{}", site_url(), tag_url));
    render(&state, "tag.html", &ctx)
}

fn read_about_md(path: &str) -> String {
    std::fs::read_to_string(path)
        .map(|content| crate::post::render_markdown(&content))
        .unwrap_or_else(|e| format!("<p>Failed to load about page: {}</p>", e))
}

pub async fn about_handler(State(state): State<Arc<AppState>>) -> Html<String> {

    let posts = get_cached_posts(&state);
    let tag_count: usize = posts.iter()
        .flat_map(|p| p.frontmatter.tags.iter())
        .collect::<HashSet<&String>>()
        .len();

    let total_words: usize = posts.iter().map(|p| p.word_count).sum();

    let about_html = read_about_md("pages/about.md");

    let config = state.about_config.read().unwrap_or_else(|e| e.into_inner());
    let author_name = config.author_name.clone();
    let raw_path = config.avatar_path.clone();
    drop(config);

    let avatar_path = if raw_path.starts_with("/static/") {
        let file_path = format!(".{}", raw_path);
        let mtime = std::fs::metadata(&file_path)
            .ok()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);
        format!("{}?v={}", raw_path, mtime)
    } else {
        raw_path.clone()
    };

    let mut ctx = tera::Context::new();
    ctx.insert("about_content", &about_html);
    ctx.insert("posts", &*posts);
    ctx.insert("tag_count", &tag_count);
    ctx.insert("word_count", &total_words);
    ctx.insert("author_name", &author_name);
    ctx.insert("avatar_path", &avatar_path);
    ctx.insert("og_title", &format!("About - {}", site_title()));
    ctx.insert("og_description", &site_desc());
    ctx.insert("og_url", &format!("{}/about", site_url()));
    render(&state, "about.html", &ctx)
}

pub async fn tags_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let posts = get_cached_posts(&state);

    let mut tag_freq: HashMap<&str, usize> = HashMap::new();
    for p in &*posts {
        for t in &p.frontmatter.tags {
            *tag_freq.entry(t).or_insert(0) += 1;
        }
    }
    let max_freq = tag_freq.values().copied().max().unwrap_or(1);
    let tag_cloud: Vec<TagCloud> = tag_freq.iter()
        .map(|(name, count)| TagCloud {
            name: name.to_string(),
            count: *count,
            weight: *count as f64 / max_freq as f64,
        })
        .collect();

    let mut ctx = tera::Context::new();
    ctx.insert("tag_cloud", &tag_cloud);
    ctx.insert("total_tags", &tag_cloud.len());
    ctx.insert("og_title", &format!("Tags - {}", site_title()));
    ctx.insert("og_description", "Browse all tags");
    ctx.insert("og_url", &format!("{}/tags", site_url()));
    render(&state, "tags.html", &ctx)
}

pub async fn search_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> Html<String> {
    let posts = get_cached_posts(&state);
    let q = query.q.as_deref().unwrap_or("").trim().to_lowercase();

    let all_results: Vec<&crate::post::Post> = if q.is_empty() {
        Vec::new()
    } else {
        let tokens: Vec<&str> = q.split_whitespace().collect();
        posts.iter()
            .filter(|p| {
                tokens.iter().all(|token| {
                    p.frontmatter.title.to_lowercase().contains(token)
                        || p.frontmatter.tags.iter().any(|t| t.to_lowercase().contains(token))
                        || p.search_text.contains(token)
                })
            })
            .collect()
    };

    let page = query.page.unwrap_or(1).max(1);
    let total_posts = all_results.len();
    let total_pages = if total_posts == 0 { 1 } else { total_posts.div_ceil(posts_per_page()) };
    let page = page.min(total_pages);
    let start = (page - 1) * posts_per_page();
    let end = std::cmp::min(start + posts_per_page(), total_posts);
    let results = &all_results[start..end];
    let page_info = compute_page_info(page, total_pages);

    let query_display = escape_xml(&q);
    let query_url = url::form_urlencoded::byte_serialize(q.as_bytes()).collect::<String>();

    let mut ctx = tera::Context::new();
    ctx.insert("query", &query_display);
    ctx.insert("query_url", &query_url);
    ctx.insert("results", &results);
    ctx.insert("result_count", &total_posts);
    ctx.insert("page_info", &page_info);
    ctx.insert("og_title", &format!("Search - {}", site_title()));
    ctx.insert("og_description", &format!("Search results for \"{}\"", query_display));
    ctx.insert("og_url", &format!("{}/search?q={}", site_url(), query_url));
    render(&state, "search.html", &ctx)
}

#[derive(Serialize)]
struct TagCloud {
    name: String,
    count: usize,
    weight: f64,
}

#[derive(Serialize)]
struct ArchivePost {
    slug: String,
    title: String,
    date: String,
    reading_time: usize,
}

#[derive(Serialize)]
struct ArchiveGroup {
    year: String,
    months: Vec<ArchiveMonth>,
}

#[derive(Serialize)]
struct ArchiveMonth {
    month: String,
    posts: Vec<ArchivePost>,
}

fn month_index(name: &str) -> u8 {
    match name {
        "January" => 1, "February" => 2, "March" => 3, "April" => 4,
        "May" => 5, "June" => 6, "July" => 7, "August" => 8,
        "September" => 9, "October" => 10, "November" => 11, "December" => 12,
        _ => 0,
    }
}

fn group_by_year_month(posts: &[crate::post::Post]) -> Vec<ArchiveGroup> {
    let mut groups: Vec<ArchiveGroup> = Vec::new();
    for post in posts {
        let parts: Vec<&str> = post.frontmatter.date.split('-').collect();
        if parts.len() < 2 { continue; }
        let year = parts[0].to_string();
        let month_num = parts[1].to_string();
        let month_name = match month_num.as_str() {
            "01" => "January", "02" => "February", "03" => "March",
            "04" => "April", "05" => "May", "06" => "June",
            "07" => "July", "08" => "August", "09" => "September",
            "10" => "October", "11" => "November", "12" => "December",
            _ => &month_num,
        }.to_string();

        let ap = ArchivePost {
            slug: post.slug.clone(),
            title: post.frontmatter.title.clone(),
            date: post.frontmatter.date.clone(),
            reading_time: post.reading_time,
        };

        let year_idx = groups.iter().position(|g| g.year == year);
        if let Some(idx) = year_idx {
            let g = &mut groups[idx];
            if let Some(existing) = g.months.iter_mut().find(|m| m.month == month_name) {
                existing.posts.push(ap);
            } else {
                g.months.push(ArchiveMonth { month: month_name, posts: vec![ap] });
            }
        } else {
            groups.push(ArchiveGroup {
                year,
                months: vec![ArchiveMonth { month: month_name, posts: vec![ap] }],
            });
        }
    }

    for g in &mut groups {
        g.months.sort_by_key(|b| std::cmp::Reverse(month_index(&b.month)));
    }
    groups
}

pub async fn archive_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PageQuery>,
) -> Html<String> {
    let posts = get_cached_posts(&state);

    let groups = group_by_year_month(&posts);

    let total = groups.len();
    let page = query.page.unwrap_or(1).max(1).min(total.max(1));
    let idx = page - 1;
    let current = if groups.is_empty() { None } else { Some(&groups[idx]) };
    let has_prev = page > 1;
    let has_next = page < total;
    let prev_year = if has_prev { Some(&groups[idx - 1].year) } else { None };
    let next_year = if has_next { Some(&groups[idx + 1].year) } else { None };

    let mut ctx = tera::Context::new();
    ctx.insert("current_year_group", &current);
    ctx.insert("total_years", &total);
    ctx.insert("current_page", &page);
    ctx.insert("prev_page", &(page - 1));
    ctx.insert("next_page", &(page + 1));
    ctx.insert("has_prev", &has_prev);
    ctx.insert("has_next", &has_next);
    ctx.insert("prev_year", &prev_year);
    ctx.insert("next_year", &next_year);
    ctx.insert("total_posts", &posts.len());
    ctx.insert("og_title", &format!("Archive - {}", site_title()));
    ctx.insert("og_description", "Browse all posts by date");
    ctx.insert("og_url", &format!("{}/archive", site_url()));
    render(&state, "archive.html", &ctx)
}

pub async fn feed_handler(State(state): State<Arc<AppState>>) -> Response {
    let posts = get_cached_posts(&state);

    let mut items = String::new();
    for post in &*posts {
        let url = format!("{}/post/{}", site_url(), post.slug);
        let date = date_to_rfc2822(&post.frontmatter.date);
        let content = format!("<![CDATA[{}]]>", post.content_html);
        let title = crate::post::escape_xml(&post.frontmatter.title);
        items.push_str(&format!(
            r#"  <item>
    <title>{}</title>
    <link>{}</link>
    <guid>{}</guid>
    <pubDate>{}</pubDate>
    <description>{}</description>
  </item>
"#, title, url, url, date, content));
    }

    let body = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title>{}</title>
    <link>{}</link>
    <description>{}</description>
    <language>zh-CN</language>
    <atom:link href="{}/feed.xml" rel="self" type="application/rss+xml"/>
    <lastBuildDate>{}</lastBuildDate>
{}
  </channel>
</rss>"#,
        site_title(), site_url(), site_desc(), site_url(),
        date_to_rfc2822(posts.first().map(|p| &p.frontmatter.date).unwrap_or(&String::new())),
        items);

    Response::builder()
        .header("Content-Type", "application/rss+xml; charset=utf-8")
        .body(axum::body::Body::from(body))
        .expect("feed response builder should succeed")
}

fn popular_tags(posts: &[crate::post::Post]) -> Vec<&str> {
    let mut tag_freq: HashMap<&str, usize> = HashMap::new();
    for p in posts {
        for t in &p.frontmatter.tags {
            *tag_freq.entry(t).or_insert(0) += 1;
        }
    }
    let mut pop_tags: Vec<(&str, usize)> = tag_freq.into_iter().collect();
    pop_tags.sort_by_key(|b| std::cmp::Reverse(b.1));
    pop_tags.into_iter().take(10).map(|(t, _)| t).collect()
}

pub async fn not_found_handler(
    State(state): State<Arc<AppState>>,
    uri: axum::http::Uri,
) -> Response {
    let posts = get_cached_posts(&state);

    let recent_posts: Vec<&crate::post::Post> = posts.iter().take(5).collect();

    let pop_tags = popular_tags(&posts);

    let mut ctx = tera::Context::new();
    ctx.insert("requested_path", &escape_xml(uri.path()));
    ctx.insert("recent_posts", &recent_posts);
    ctx.insert("pop_tags", &pop_tags);
    (StatusCode::NOT_FOUND, render(&state, "404.html", &ctx)).into_response()
}

fn generate_sitemap_urls(posts: &[crate::post::Post]) -> String {
    let mut urls = format!(
        r#"  <url>
    <loc>{0}/</loc>
    <priority>1.0</priority>
  </url>
  <url>
    <loc>{0}/about</loc>
    <priority>0.8</priority>
  </url>
  <url>
    <loc>{0}/archive</loc>
    <priority>0.7</priority>
  </url>
"#, site_url());

    for post in posts {
        urls.push_str(&format!(
            r#"  <url>
    <loc>{}/post/{}</loc>
    <lastmod>{}T00:00:00Z</lastmod>
    <priority>0.9</priority>
  </url>
"#, site_url(), post.slug, post.frontmatter.date));
    }
    urls
}

pub async fn sitemap_handler(State(state): State<Arc<AppState>>) -> Response {
    let posts = get_cached_posts(&state);
    let urls = generate_sitemap_urls(&posts);

    let body = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
{}
</urlset>"#, urls);

    Response::builder()
        .header("Content-Type", "application/xml; charset=utf-8")
        .body(axum::body::Body::from(body))
        .expect("sitemap response builder should succeed")
}

#[derive(Deserialize)]
pub struct UpdateAboutPayload {
    pub author_name: Option<String>,
    pub avatar_path: Option<String>,
}

pub async fn update_about_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateAboutPayload>,
) -> Response {
    let mut config = state.about_config.write().unwrap_or_else(|e| e.into_inner());
    if let Some(name) = payload.author_name {
        if !name.is_empty() {
            config.author_name = name;
        }
    }
    if let Some(path) = payload.avatar_path {
        if !path.is_empty() {
            config.avatar_path = path;
        }
    }
    let config_clone = config.clone();
    drop(config);
    save_about_config(&config_clone);
    Json(serde_json::json!({"ok": true})).into_response()
}

pub async fn upload_avatar_handler(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Response {
    use std::io::Write;

    let field = match multipart.next_field().await {
        Ok(Some(f)) => f,
        _ => return (StatusCode::BAD_REQUEST, "No file uploaded").into_response(),
    };

    let content_type = field.content_type().unwrap_or("").to_string();
    let allowed = ["image/jpeg", "image/png", "image/webp"];
    if !allowed.contains(&content_type.as_str()) {
        return (StatusCode::BAD_REQUEST, "Only JPG, PNG, WebP allowed").into_response();
    }

    let ext = match content_type.as_str() {
        "image/png" => "png",
        "image/webp" => "webp",
        _ => "jpg",
    };

    let data = match field.bytes().await {
        Ok(d) => d,
        _ => return (StatusCode::BAD_REQUEST, "Failed to read file").into_response(),
    };

    if data.len() > 5 * 1024 * 1024 {
        return (StatusCode::BAD_REQUEST, "File too large (max 5MB)").into_response();
    }

    let filepath = format!("static/images/avatar.{}", ext);
    match std::fs::File::create(&filepath) {
        Ok(mut f) => {
            if f.write_all(&data).is_err() {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to save file").into_response();
            }
        }
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create file").into_response(),
    }

    let avatar_path = format!("/static/images/avatar.{}", ext);
    let mut config = state.about_config.write().unwrap_or_else(|e| e.into_inner());
    config.avatar_path = avatar_path.clone();
    let config_clone = config.clone();
    drop(config);
    save_about_config(&config_clone);

    Json(serde_json::json!({"avatar_path": avatar_path})).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_page_info_total_less_or_equal_5() {
        let info = compute_page_info(1, 3);
        assert_eq!(info.current, 1);
        assert_eq!(info.total, 3);
        assert!(!info.has_prev);
        assert!(info.has_next);
        assert_eq!(info.pages, vec![1, 2, 3]);
        assert!(!info.show_start_ellipsis);
        assert!(!info.show_end_ellipsis);
    }

    #[test]
    fn test_compute_page_info_total_5() {
        let info = compute_page_info(3, 5);
        assert_eq!(info.pages, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_compute_page_info_large_total_current_start() {
        let info = compute_page_info(1, 20);
        assert_eq!(info.current, 1);
        assert!(!info.has_prev);
        assert!(info.has_next);
        assert_eq!(info.pages, vec![2]);
        assert!(!info.show_start_ellipsis);
        assert!(info.show_end_ellipsis);
    }

    #[test]
    fn test_compute_page_info_large_total_current_middle() {
        let info = compute_page_info(10, 20);
        assert_eq!(info.pages, vec![9, 10, 11]);
        assert!(info.show_start_ellipsis);
        assert!(info.show_end_ellipsis);
    }

    #[test]
    fn test_compute_page_info_large_total_current_end() {
        let info = compute_page_info(20, 20);
        assert_eq!(info.pages, vec![19]);
        assert!(info.show_start_ellipsis);
        assert!(!info.show_end_ellipsis);
        assert!(info.has_prev);
        assert!(!info.has_next);
    }

    #[test]
    fn test_compute_page_info_single_page() {
        let info = compute_page_info(1, 1);
        assert!(!info.has_prev);
        assert!(!info.has_next);
        assert_eq!(info.pages, vec![1]);
    }

    #[test]
    fn test_compute_page_info_current_2_of_large() {
        let info = compute_page_info(2, 20);
        assert_eq!(info.pages, vec![2, 3]);
        assert!(!info.show_start_ellipsis);
        assert!(info.show_end_ellipsis);
    }

    #[test]
    fn test_compute_page_info_current_19_of_large() {
        let info = compute_page_info(19, 20);
        assert_eq!(info.pages, vec![18, 19]);
        assert!(info.show_start_ellipsis);
        assert!(!info.show_end_ellipsis);
    }

    #[test]
    fn test_date_to_rfc2822_valid() {
        let result = date_to_rfc2822("2024-06-15");
        assert_eq!(result, "Sat, 15 Jun 2024 00:00:00 GMT");
    }

    #[test]
    fn test_date_to_rfc2822_short() {
        assert_eq!(date_to_rfc2822("2024"), "2024");
    }

    #[test]
    fn test_weekday_known_date() {
        assert_eq!(weekday("2024", "Jun", "15"), "Sat");
        assert_eq!(weekday("2024", "Jan", "1"), "Mon");
        assert_eq!(weekday("2024", "Dec", "25"), "Wed");
    }

    #[test]
    fn test_weekday_leap_year() {
        assert_eq!(weekday("2024", "Feb", "29"), "Thu");
    }

    #[test]
    fn test_weekday_all_months() {
        assert_eq!(weekday("2024", "Jan", "1"), "Mon");
        assert_eq!(weekday("2024", "Feb", "1"), "Thu");
        assert_eq!(weekday("2024", "Mar", "1"), "Fri");
        assert_eq!(weekday("2024", "Apr", "1"), "Mon");
        assert_eq!(weekday("2024", "May", "1"), "Wed");
        assert_eq!(weekday("2024", "Jun", "1"), "Sat");
        assert_eq!(weekday("2024", "Jul", "1"), "Mon");
        assert_eq!(weekday("2024", "Aug", "1"), "Thu");
        assert_eq!(weekday("2024", "Sep", "1"), "Sun");
        assert_eq!(weekday("2024", "Oct", "1"), "Tue");
        assert_eq!(weekday("2024", "Nov", "1"), "Fri");
        assert_eq!(weekday("2024", "Dec", "1"), "Sun");
    }

    #[test]
    fn test_tag_handler_empty_tag() {
        let mut tag_freq: HashMap<&str, usize> = HashMap::new();
        tag_freq.insert("rust", 5);
        tag_freq.insert("web", 3);
        let max_freq = tag_freq.values().copied().max().unwrap_or(1);
        let cloud: Vec<TagCloud> = tag_freq.iter()
            .map(|(name, count)| TagCloud {
                name: name.to_string(),
                count: *count,
                weight: *count as f64 / max_freq as f64,
            })
            .collect();
        assert_eq!(cloud.len(), 2);
        let rust = cloud.iter().find(|t| t.name == "rust").unwrap();
        assert!((rust.weight - 1.0).abs() < f64::EPSILON);
        let web = cloud.iter().find(|t| t.name == "web").unwrap();
        assert!((web.weight - 0.6).abs() < f64::EPSILON);
    }

    #[test]
    fn test_search_filter_logic() {
        let tokens: Vec<&str> = "rust memory".split_whitespace().collect();
        let titles = vec!["Rust Memory Model", "Rust Async", "Web Design"];
        let search_texts = vec!["memory model in rust", "async programming in rust", "css layout tips"];

        let matches: Vec<&str> = titles.iter().enumerate()
            .filter(|(i, title)| {
                tokens.iter().all(|token| {
                    title.to_lowercase().contains(token)
                        || search_texts[*i].contains(token)
                })
            })
            .map(|(_, title)| *title)
            .collect();
        assert_eq!(matches, vec!["Rust Memory Model"], "both 'rust' and 'memory' must match");
    }

    #[test]
    fn test_search_filter_logic_single_token() {
        let tokens: Vec<&str> = "rust".split_whitespace().collect();
        let titles = vec!["Rust Memory", "Web Design", "Async Rust"];

        let matches: Vec<&str> = titles.iter()
            .filter(|title| {
                tokens.iter().all(|token| {
                    title.to_lowercase().contains(token)
                })
            })
            .map(|title| *title)
            .collect();
        assert_eq!(matches, vec!["Rust Memory", "Async Rust"]);
    }

    #[test]
    fn test_search_filter_logic_no_match() {
        let tokens: Vec<&str> = "python".split_whitespace().collect();
        let titles = vec!["Rust Memory", "Web Design"];

        let matches: Vec<&str> = titles.iter()
            .filter(|title| {
                tokens.iter().all(|token| {
                    title.to_lowercase().contains(token)
                })
            })
            .map(|title| *title)
            .collect();
        assert!(matches.is_empty());
    }

    #[test]
    fn test_generate_sitemap_urls_with_posts() {
        let posts = vec![
            crate::post::Post {
                frontmatter: crate::post::Frontmatter {
                    title: "P1".into(), date: "2024-06-15".into(),
                    tags: vec!["a".into()], excerpt: None,
                },
                slug: "p1".into(), content_html: "<p>P1</p>".into(),
                excerpt: "P1".into(), reading_time: 1, toc: vec![], word_count: 1, search_text: "p1".into(),
            },
        ];
        let urls = generate_sitemap_urls(&posts);
        assert!(urls.contains("/post/p1"), "should contain post slug: {}", urls);
        assert!(urls.contains("2024-06-15"), "should contain post date");
        assert!(urls.contains("/about"), "should contain about page");
        assert!(urls.contains("/archive"), "should contain archive page");
        assert!(urls.contains("</url>"), "should have closing url tags");
        assert!(urls.contains("priority>1.0<"), "should have homepage priority");
    }

    #[test]
    fn test_generate_sitemap_urls_empty() {
        let posts: Vec<crate::post::Post> = vec![];
        let urls = generate_sitemap_urls(&posts);
        assert!(urls.contains("/about"), "static pages always present");
        assert!(urls.contains("/archive"), "static pages always present");
        assert!(!urls.contains("/post/"), "no post entries");
    }

    #[test]
    fn test_group_by_year_month_basic() {
        let posts = vec![
            crate::post::Post {
                frontmatter: crate::post::Frontmatter {
                    title: "P1".into(), date: "2024-06-15".into(),
                    tags: vec![], excerpt: None,
                },
                slug: "p1".into(), content_html: "".into(),
                excerpt: "".into(), reading_time: 1, toc: vec![], word_count: 1, search_text: "".into(),
            },
            crate::post::Post {
                frontmatter: crate::post::Frontmatter {
                    title: "P2".into(), date: "2024-03-10".into(),
                    tags: vec![], excerpt: None,
                },
                slug: "p2".into(), content_html: "".into(),
                excerpt: "".into(), reading_time: 1, toc: vec![], word_count: 1, search_text: "".into(),
            },
            crate::post::Post {
                frontmatter: crate::post::Frontmatter {
                    title: "P3".into(), date: "2023-12-01".into(),
                    tags: vec![], excerpt: None,
                },
                slug: "p3".into(), content_html: "".into(),
                excerpt: "".into(), reading_time: 1, toc: vec![], word_count: 1, search_text: "".into(),
            },
        ];
        let groups = group_by_year_month(&posts);
        assert_eq!(groups.len(), 2, "two distinct years");
        assert_eq!(groups[0].year, "2024");
        assert_eq!(groups[0].months.len(), 2, "two months in 2024");
        assert_eq!(groups[0].months[0].month, "June", "months sorted descending");
        assert_eq!(groups[0].months[1].month, "March");
        assert_eq!(groups[1].year, "2023");
        assert_eq!(groups[1].months.len(), 1);
        assert_eq!(groups[1].months[0].month, "December");
    }

    #[test]
    fn test_group_by_year_month_empty() {
        let posts: Vec<crate::post::Post> = vec![];
        let groups = group_by_year_month(&posts);
        assert!(groups.is_empty());
    }

    #[test]
    fn test_popular_tags_top_10() {
        let posts: Vec<crate::post::Post> = (0..15).map(|i| {
            let tag = format!("tag-{}", i);
            crate::post::Post {
                frontmatter: crate::post::Frontmatter {
                    title: format!("P{}", i), date: "2024-06-15".into(),
                    tags: vec![tag],
                    excerpt: None,
                },
                slug: format!("p{}", i), content_html: "".into(),
                excerpt: "".into(), reading_time: 1, toc: vec![], word_count: 1,
                search_text: "".into(),
            }
        }).collect();
        let tags = popular_tags(&posts);
        assert_eq!(tags.len(), 10, "at most 10 popular tags returned, got {}", tags.len());
    }

    #[test]
    fn test_popular_tags_less_than_10() {
        let posts = vec![
            crate::post::Post {
                frontmatter: crate::post::Frontmatter {
                    title: "P1".into(), date: "2024-06-15".into(),
                    tags: vec!["rust".into()], excerpt: None,
                },
                slug: "p1".into(), content_html: "".into(),
                excerpt: "".into(), reading_time: 1, toc: vec![], word_count: 1, search_text: "".into(),
            },
        ];
        let tags = popular_tags(&posts);
        assert_eq!(tags, vec!["rust"]);
    }

    #[test]
    fn test_read_about_md_missing_file() {
        let result = read_about_md("/nonexistent/path/about.md");
        assert!(result.starts_with("<p>Failed to load about page:"), "got: {}", result);
        assert!(result.ends_with("</p>"));
    }
}

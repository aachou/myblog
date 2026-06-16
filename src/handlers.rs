#![allow(dead_code)]
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::response::{Html, IntoResponse, Response};
use serde::{Deserialize, Serialize};

use crate::post::escape_xml;
use crate::{get_cached_posts, AppState, POSTS_PER_PAGE, SITE_DESC, SITE_TITLE, SITE_URL};

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
    Html(state.tera.render(template, ctx).unwrap_or_else(|e| {
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
    let w = (d + (13 * (m + 1)) / 5 + y_ + y_ / 4 + c / 4 + 5 * c) % 7;
    ["Sat", "Sun", "Mon", "Tue", "Wed", "Thu", "Fri"][w as usize]
}

pub async fn index_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PageQuery>,
) -> Html<String> {
    let posts = get_cached_posts(&state);
    let page = query.page.unwrap_or(1).max(1);
    let total_posts = posts.len();
    let total_pages = if total_posts == 0 { 1 } else { (total_posts + POSTS_PER_PAGE - 1) / POSTS_PER_PAGE };
    let page = page.min(total_pages);

    let start = (page - 1) * POSTS_PER_PAGE;
    let end = std::cmp::min(start + POSTS_PER_PAGE, total_posts);
    let page_posts = &posts[start..end];

    let page_info = compute_page_info(page, total_pages);

    let mut ctx = tera::Context::new();
    ctx.insert("posts", &page_posts);
    ctx.insert("page_info", &page_info);
    ctx.insert("og_title", SITE_TITLE);
    ctx.insert("og_description", SITE_DESC);
    ctx.insert("og_url", SITE_URL);
    render(&state, "index.html", &ctx)
}

pub async fn post_handler(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> Response {
    let posts = get_cached_posts(&state);
    if let Some(post) = posts.iter().find(|p| p.slug == slug) {
        let idx = posts.iter().position(|p| p.slug == slug).unwrap();
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
        ctx.insert("og_url", &format!("{}/post/{}", SITE_URL, slug));
        render(&state, "post.html", &ctx).into_response()
    } else {
        let mut ctx = tera::Context::new();
        ctx.insert("slug", &escape_xml(&slug));
        render(&state, "404.html", &ctx).into_response()
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

    let total_posts = filtered.len();
    let total_pages = if total_posts == 0 { 1 } else { (total_posts + POSTS_PER_PAGE - 1) / POSTS_PER_PAGE };
    let page = page.min(total_pages);

    let start = (page - 1) * POSTS_PER_PAGE;
    let end = std::cmp::min(start + POSTS_PER_PAGE, total_posts);
    let page_posts = &filtered[start..end];

    let page_info = compute_page_info(page, total_pages);

    let mut ctx = tera::Context::new();
    ctx.insert("posts", &page_posts);
    ctx.insert("page_info", &page_info);
    ctx.insert("tag", &escape_xml(&name));
    ctx.insert("og_title", &format!("#{} - {}", escape_xml(&name), SITE_TITLE));
    ctx.insert("og_description", &format!("Posts tagged with #{}", escape_xml(&name)));
    ctx.insert("og_url", &format!("{}/tag/{}", SITE_URL, escape_xml(&name)));
    render(&state, "tag.html", &ctx)
}

pub async fn about_handler(State(state): State<Arc<AppState>>) -> Html<String> {

    let posts = get_cached_posts(&state);
    let tag_count: usize = posts.iter()
        .flat_map(|p| p.frontmatter.tags.iter())
        .collect::<HashSet<&String>>()
        .len();

    let total_words: usize = posts.iter().map(|p| p.word_count).sum();

    let about_html = std::fs::read_to_string("pages/about.md")
        .map(|content| crate::post::render_markdown(&content))
        .unwrap_or_else(|e| format!("<p>Failed to load about page: {}</p>", e));

    let mut ctx = tera::Context::new();
    ctx.insert("about_content", &about_html);
    ctx.insert("posts", &*posts);
    ctx.insert("tag_count", &tag_count);
    ctx.insert("word_count", &total_words);
    ctx.insert("og_title", &format!("About - {}", SITE_TITLE));
    ctx.insert("og_description", SITE_DESC);
    ctx.insert("og_url", &format!("{}/about", SITE_URL));
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
    ctx.insert("og_title", &format!("Tags - {}", SITE_TITLE));
    ctx.insert("og_description", "Browse all tags");
    ctx.insert("og_url", &format!("{}/tags", SITE_URL));
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
        posts.iter()
            .filter(|p| {
                p.frontmatter.title.to_lowercase().contains(&q)
                    || p.frontmatter.tags.iter().any(|t| t.to_lowercase().contains(&q))
                    || p.search_text.contains(&q)
            })
            .collect()
    };

    let page = query.page.unwrap_or(1).max(1);
    let total_posts = all_results.len();
    let total_pages = if total_posts == 0 { 1 } else { (total_posts + POSTS_PER_PAGE - 1) / POSTS_PER_PAGE };
    let page = page.min(total_pages);
    let start = (page - 1) * POSTS_PER_PAGE;
    let end = std::cmp::min(start + POSTS_PER_PAGE, total_posts);
    let results = &all_results[start..end];
    let page_info = compute_page_info(page, total_pages);

    let mut ctx = tera::Context::new();
    ctx.insert("query", &escape_xml(&q));
    ctx.insert("results", &results);
    ctx.insert("result_count", &total_posts);
    ctx.insert("page_info", &page_info);
    ctx.insert("og_title", &format!("Search - {}", SITE_TITLE));
    ctx.insert("og_description", &format!("Search results for \"{}\"", escape_xml(&q)));
    ctx.insert("og_url", &format!("{}/search?q={}", SITE_URL, escape_xml(&q)));
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

pub async fn archive_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PageQuery>,
) -> Html<String> {
    let posts = get_cached_posts(&state);

    let mut groups: Vec<ArchiveGroup> = Vec::new();
    'outer: for post in &*posts {
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

        for g in &mut groups {
            if g.year == year {
                if let Some(last) = g.months.last_mut() {
                    if last.month == month_name {
                        last.posts.push(ap);
                        continue 'outer;
                    }
                }
                g.months.push(ArchiveMonth { month: month_name, posts: vec![ap] });
                continue 'outer;
            }
        }
        groups.push(ArchiveGroup {
            year,
            months: vec![ArchiveMonth { month: month_name, posts: vec![ap] }],
        });
    }

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
    ctx.insert("og_title", &format!("Archive - {}", SITE_TITLE));
    ctx.insert("og_description", "Browse all posts by date");
    ctx.insert("og_url", &format!("{}/archive", SITE_URL));
    render(&state, "archive.html", &ctx)
}

pub async fn feed_handler(State(state): State<Arc<AppState>>) -> Response {
    let posts = get_cached_posts(&state);

    let mut items = String::new();
    for post in &*posts {
        let url = format!("{}/post/{}", SITE_URL, post.slug);
        let date = date_to_rfc2822(&post.frontmatter.date);
        let excerpt = crate::post::escape_xml(&post.excerpt);
        let title = crate::post::escape_xml(&post.frontmatter.title);
        items.push_str(&format!(
            r#"  <item>
    <title>{}</title>
    <link>{}</link>
    <guid>{}</guid>
    <pubDate>{}</pubDate>
    <description>{}</description>
  </item>
"#, title, url, url, date, excerpt));
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
        SITE_TITLE, SITE_URL, SITE_DESC, SITE_URL,
        date_to_rfc2822(&posts.first().map(|p| &p.frontmatter.date).unwrap_or(&String::new())),
        items);

    Response::builder()
        .header("Content-Type", "application/rss+xml; charset=utf-8")
        .body(axum::body::Body::from(body))
        .unwrap()
}

pub async fn sitemap_handler(State(state): State<Arc<AppState>>) -> Response {
    let posts = get_cached_posts(&state);

    let mut urls = format!(
        r#"  <url>
    <loc>{}/</loc>
    <priority>1.0</priority>
  </url>
  <url>
    <loc>{}/about</loc>
    <priority>0.8</priority>
  </url>
  <url>
    <loc>{}/archive</loc>
    <priority>0.7</priority>
  </url>
"#, SITE_URL, SITE_URL, SITE_URL);

    for post in &*posts {
        urls.push_str(&format!(
            r#"  <url>
    <loc>{}/post/{}</loc>
    <lastmod>{}T00:00:00Z</lastmod>
    <priority>0.9</priority>
  </url>
"#, SITE_URL, post.slug, post.frontmatter.date));
    }

    let body = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
{}
</urlset>"#, urls);

    Response::builder()
        .header("Content-Type", "application/xml; charset=utf-8")
        .body(axum::body::Body::from(body))
        .unwrap()
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
        let query = "rust".to_lowercase();
        let keywords = vec!["rust", "web", "async"];

        let matches: Vec<&str> = keywords.iter()
            .filter(|t| t.to_lowercase().contains(&query))
            .copied()
            .collect();
        assert_eq!(matches, vec!["rust"]);
    }
}

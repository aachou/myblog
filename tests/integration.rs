use std::path::PathBuf;
use myblog::post;

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

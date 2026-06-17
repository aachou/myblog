use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::OnceLock;
use syntect::highlighting::ThemeSet;
use syntect::html::styled_line_to_highlighted_html;
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::easy::HighlightLines;
use syntect::util::LinesWithEndings;

static DATE_RE: OnceLock<Regex> = OnceLock::new();
fn is_valid_date(date: &str) -> bool {
    DATE_RE.get_or_init(|| Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()).is_match(date)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Frontmatter {
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub excerpt: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TocEntry {
    pub level: usize,
    pub id: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Post {
    pub frontmatter: Frontmatter,
    pub slug: String,
    pub content_html: String,
    pub excerpt: String,
    pub reading_time: usize,
    pub toc: Vec<TocEntry>,
    pub word_count: usize,
    pub search_text: String,
}

fn get_syntax_set() -> &'static SyntaxSet {
    static SS: OnceLock<SyntaxSet> = OnceLock::new();
    SS.get_or_init(SyntaxSet::load_defaults_newlines)
}

fn get_theme_set() -> &'static ThemeSet {
    static TS: OnceLock<ThemeSet> = OnceLock::new();
    TS.get_or_init(ThemeSet::load_defaults)
}

fn parse_frontmatter(content: &str) -> Option<(Frontmatter, String)> {
    let content = content.trim();
    if let Some(remaining) = content.strip_prefix("+++") {
        let end = remaining.find("+++")?;
        let frontmatter_str = &remaining[..end];
        let markdown = remaining[end + 3..].trim();
        let frontmatter: Frontmatter = toml::from_str(frontmatter_str).ok()?;
        Some((frontmatter, markdown.to_string()))
    } else {
        None
    }
}

fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == ' ' { c } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-")
}

fn unescape_html(text: &str) -> String {
    text.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&#x27;", "'")
}

fn strip_html_tags(html: &str) -> String {
    #[derive(PartialEq)]
    enum State { Normal, Tag, Script, Style, Comment }
    let mut result = String::new();
    let mut state = State::Normal;
    let mut buf = String::new();
    let lower = html.to_lowercase();

    for (i, c) in html.char_indices() {
        match state {
            State::Normal => {
                if c == '<' {
                    let rest = &lower[i..];
                    if rest.starts_with("<!--") {
                        state = State::Comment;
                    } else if rest.starts_with("<script") {
                        state = State::Script;
                        buf.clear();
                    } else if rest.starts_with("<style") {
                        state = State::Style;
                        buf.clear();
                    } else {
                        state = State::Tag;
                    }
                } else {
                    result.push(c);
                }
            }
            State::Tag => {
                if c == '>' {
                    state = State::Normal;
                }
            }
            State::Script => {
                if c == '<' {
                    buf.clear();
                }
                buf.push(c);
                if buf.to_lowercase().ends_with("</script>") {
                    state = State::Normal;
                }
            }
            State::Style => {
                if c == '<' {
                    buf.clear();
                }
                buf.push(c);
                if buf.to_lowercase().ends_with("</style>") {
                    state = State::Normal;
                }
            }
            State::Comment => {
                if c == '>' && html[..i].ends_with("-->") {
                    state = State::Normal;
                }
            }
        }
    }
    result.trim().to_string()
}

pub fn extract_toc(html: &str) -> Vec<TocEntry> {
    let mut entries = Vec::new();
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r#"<h([2-4])\s+id="([^"]*)"[^>]*>"#).unwrap());
    for cap in re.captures_iter(html) {
        let level: usize = cap[1].parse().unwrap_or(2);
        let id = cap[2].to_string();
        let start = cap.get(0).unwrap().end();
        let close = format!("</h{}>", level);
        if let Some(end) = html[start..].find(&close) {
            let text = strip_html_tags(&html[start..start + end]);
            entries.push(TocEntry { level, id, text });
        }
    }
    entries
}

pub fn word_count(html: &str) -> usize {
    strip_html_tags(html).split_whitespace().count()
}

pub fn escape_xml(text: &str) -> String {
    text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
}

pub fn reading_time(html: &str) -> usize {
    std::cmp::max(1, (word_count(html) + 100) / 200)
}

pub fn render_markdown(content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let html_with_ids = add_heading_ids(&html_output);
    highlight_code_blocks(&html_with_ids)
}

fn add_heading_ids(html: &str) -> String {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r#"<h([1-6])([^>]*)>(.*?)</h[1-6]>"#).unwrap());
    re.replace_all(html, |caps: &regex::Captures| {
        let level = &caps[1];
        let attrs = &caps[2];
        let text = &caps[3];
        if attrs.contains("id=") {
            caps[0].to_string()
        } else if attrs.trim().is_empty() {
            let id = slugify(text);
            format!("<h{} id=\"{}\">{}</h{}>", level, id, text, level)
        } else {
            let id = slugify(text);
            format!("<h{} id=\"{}\" {}>{}</h{}>", level, id, attrs.trim_start(), text, level)
        }
    }).to_string()
}

fn highlight_code_blocks(html: &str) -> String {
    let ss = get_syntax_set();
    let ts = get_theme_set();
    let theme = &ts.themes["InspiredGitHub"];

    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(
        r#"(?s)<pre><code(?: class="language-([\w.#+\-]+)")?>(.*?)</code></pre>"#
    ).unwrap());

    re.replace_all(html, |caps: &regex::Captures| {
        let lang = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let code_escaped = caps.get(2).unwrap().as_str();
        let code = unescape_html(code_escaped.trim_end_matches('\n'));
        format_code_block(&code, lang, ss, theme)
    }).to_string()
}

fn resolve_syntax<'a>(lang: &'a str, ss: &'a SyntaxSet) -> Option<&'a SyntaxReference> {
    if let Some(syntax) = ss.find_syntax_by_token(lang) {
        return Some(syntax);
    }
    let mapped = match lang {
        "jsx" | "tsx" => "js",
        "typescript" => "js",
        "kotlin" => "java",
        "dart" => "js",
        "vue" | "svelte" => "html",
        "elixir" => "rb",
        _ => return None,
    };
    ss.find_syntax_by_extension(mapped)
        .or_else(|| ss.find_syntax_by_name(mapped))
}

fn format_code_block(code: &str, lang: &str, ss: &SyntaxSet, theme: &syntect::highlighting::Theme) -> String {
    let lang_attr = if lang.is_empty() { String::new() } else { format!(" data-lang=\"{}\"", lang) };

    let syntax = resolve_syntax(lang, ss);
    if syntax.is_none() {
        let lines: Vec<String> = code.split('\n').map(|s| s.trim_end_matches('\r').to_string()).collect();
        let mut out = format!("<div class=\"code-block\"{}>", lang_attr);
        out.push_str("<table class=\"code-table\"><tbody>");
        for (i, line) in lines.iter().enumerate() {
            let num = i + 1;
            out.push_str(&format!("<tr><td class=\"ln\">{}</td><td class=\"lc\">{}</td></tr>\n", num, escape_xml(line)));
        }
        out.push_str("</tbody></table></div>");
        return out;
    }
    let syntax = syntax.unwrap();

    let mut highlighter = HighlightLines::new(syntax, theme);
    let mut out = format!("<div class=\"code-block\"{}>", lang_attr);
    out.push_str("<table class=\"code-table\"><tbody>");
    for (i, line) in LinesWithEndings::from(code).enumerate() {
        let num = i + 1;
        let ranges = highlighter.highlight_line(line, ss).unwrap_or_default();
        let line_html = styled_line_to_highlighted_html(&ranges, syntect::html::IncludeBackground::No)
            .unwrap_or_else(|_| escape_xml(line.trim_end()));
        out.push_str(&format!("<tr><td class=\"ln\">{}</td><td class=\"lc\">{}</td></tr>\n", num, line_html.trim_end()));
    }
    out.push_str("</tbody></table></div>");
    out
}

pub fn load_posts(dir: &str) -> Result<Vec<Post>, Box<dyn std::error::Error>> {
    let dir_path = Path::new(dir);
    if !dir_path.exists() {
        fs::create_dir_all(dir_path)?;
        return Ok(Vec::new());
    }

    let mut posts = Vec::new();
    let entries = fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "md") {
            let content = fs::read_to_string(&path)?;
            match parse_frontmatter(&content) {
                Some((frontmatter, markdown)) => {
                    let slug = path.file_stem()
                        .and_then(|s| s.to_str())
                        .map(|s| s.to_string())
                        .unwrap_or_default();
                    let content_html = render_markdown(&markdown);

                    let excerpt = frontmatter.excerpt.clone().unwrap_or_else(|| {
                        let text = strip_html_tags(&content_html);
                        let truncated: String = text.chars().take(160).collect();
                        if text.chars().count() > 160 { format!("{}...", truncated) } else { truncated }
                    });

                    let toc = extract_toc(&content_html);
                    let wc = word_count(&content_html);
                    let rt = reading_time(&content_html);
                    let search_text = strip_html_tags(&content_html).to_lowercase();

                    if !is_valid_date(&frontmatter.date) {
                        tracing::warn!("Skipping {}: invalid date format '{}' (expected YYYY-MM-DD)", path.display(), frontmatter.date);
                        continue;
                    }

                    posts.push(Post {
                        frontmatter, slug, content_html, excerpt,
                        reading_time: rt, toc, word_count: wc, search_text,
                    });
                }
                None => {
                    tracing::warn!("Skipping {}: invalid frontmatter", path.display());
                }
            }
        }
    }

    posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));
    Ok(posts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter_valid() {
        let input = "+++\ntitle = \"Test Post\"\ndate = \"2024-01-15\"\ntags = [\"rust\", \"web\"]\n+++\n\n# Hello\n\nWorld";
        let result = parse_frontmatter(input);
        assert!(result.is_some());
        let (fm, md) = result.unwrap();
        assert_eq!(fm.title, "Test Post");
        assert_eq!(fm.date, "2024-01-15");
        assert_eq!(fm.tags, vec!["rust", "web"]);
        assert!(fm.excerpt.is_none());
        assert!(md.contains("# Hello"));
    }

    #[test]
    fn test_parse_frontmatter_no_delimiters() {
        let input = "# Just a heading\n\nNo frontmatter here.";
        assert!(parse_frontmatter(input).is_none());
    }

    #[test]
    fn test_parse_frontmatter_missing_end() {
        let input = "+++\ntitle = \"Broken\"\ndate = \"2024-01-15\"\ntags = []\n";
        assert!(parse_frontmatter(input).is_none());
    }

    #[test]
    fn test_parse_frontmatter_invalid_toml() {
        let input = "+++\ntitle =\n+++\n\nContent";
        assert!(parse_frontmatter(input).is_none());
    }

    #[test]
    fn test_parse_frontmatter_empty() {
        assert!(parse_frontmatter("").is_none());
    }

    #[test]
    fn test_slugify_basic() {
        assert_eq!(slugify("Hello World"), "hello-world");
    }

    #[test]
    fn test_slugify_special_chars() {
        assert_eq!(slugify("Rust & Web Dev!"), "rust-web-dev");
    }

    #[test]
    fn test_slugify_multi_spaces() {
        assert_eq!(slugify("foo    bar   baz"), "foo-bar-baz");
    }

    #[test]
    fn test_slugify_leading_trailing_spaces() {
        assert_eq!(slugify("  spaced  "), "spaced");
    }

    #[test]
    fn test_slugify_alphanumeric_only() {
        assert_eq!(slugify("abc123"), "abc123");
    }

    #[test]
    fn test_slugify_unicode() {
        assert_eq!(slugify("Hello 世界"), "hello-世界");
    }

    #[test]
    fn test_strip_html_tags_basic() {
        assert_eq!(strip_html_tags("<p>Hello</p>"), "Hello");
    }

    #[test]
    fn test_strip_html_tags_with_text_around() {
        assert_eq!(strip_html_tags("before <b>bold</b> after"), "before bold after");
    }

    #[test]
    fn test_strip_html_tags_self_closing() {
        assert_eq!(strip_html_tags("Line1<br>Line2<hr>"), "Line1Line2");
    }

    #[test]
    fn test_strip_html_tags_no_tags() {
        assert_eq!(strip_html_tags("plain text"), "plain text");
    }

    #[test]
    fn test_strip_html_tags_empty() {
        assert_eq!(strip_html_tags(""), "");
    }

    #[test]
    fn test_extract_toc_heading_levels() {
        let html = r#"<h2 id="sec1">Section 1</h2><p>text</p><h3 id="sec1-1">Sub 1.1</h3><p>more</p><h4 id="sec1-1-1">Sub sub</h4>"#;
        let toc = extract_toc(html);
        assert_eq!(toc.len(), 3);
        assert_eq!(toc[0].level, 2);
        assert_eq!(toc[0].id, "sec1");
        assert_eq!(toc[0].text, "Section 1");
        assert_eq!(toc[1].level, 3);
        assert_eq!(toc[1].text, "Sub 1.1");
        assert_eq!(toc[2].level, 4);
    }

    #[test]
    fn test_extract_toc_ignores_h1_h5_h6() {
        let html = r#"<h1 id="top">Top</h1><h5 id="small">Small</h5><h6 id="tiny">Tiny</h6>"#;
        let toc = extract_toc(html);
        assert!(toc.is_empty());
    }

    #[test]
    fn test_extract_toc_no_headings() {
        assert!(extract_toc("<p>no headings</p>").is_empty());
    }

    #[test]
    fn test_extract_toc_multiple_same_level() {
        let html = r#"<h2 id="a">A</h2><h2 id="b">B</h2><h2 id="c">C</h2>"#;
        assert_eq!(extract_toc(html).len(), 3);
    }

    #[test]
    fn test_word_count_basic() {
        assert_eq!(word_count("<p>one two three four</p>"), 4);
    }

    #[test]
    fn test_word_count_empty() {
        assert_eq!(word_count(""), 0);
    }

    #[test]
    fn test_word_count_with_html() {
        assert_eq!(word_count("<div><p>hello world</p><span>foo bar</span></div>"), 3);
    }

    #[test]
    fn test_reading_time_minimum() {
        assert_eq!(reading_time("<p>short</p>"), 1);
    }

    #[test]
    fn test_reading_time_200_words() {
        let words = "word ".repeat(200);
        assert_eq!(reading_time(&format!("<p>{}</p>", words)), 1);
    }

    #[test]
    fn test_reading_time_400_words() {
        let words = "word ".repeat(400);
        assert_eq!(reading_time(&format!("<p>{}</p>", words)), 2);
    }

    #[test]
    fn test_escape_xml_all() {
        assert_eq!(escape_xml("a&b<c>d\"e'f"), "a&amp;b&lt;c&gt;d&quot;e&apos;f");
    }

    #[test]
    fn test_escape_xml_no_special() {
        assert_eq!(escape_xml("hello world"), "hello world");
    }

    #[test]
    fn test_escape_xml_empty() {
        assert_eq!(escape_xml(""), "");
    }

    #[test]
    fn test_unescape_html_all() {
        assert_eq!(unescape_html("&amp; &lt; &gt; &quot; &#39; &#x27;"), "& < > \" ' '");
    }

    #[test]
    fn test_unescape_html_plain() {
        assert_eq!(unescape_html("no entities"), "no entities");
    }

    #[test]
    fn test_add_heading_ids_without_id() {
        let html = "<h2>Hello World</h2>";
        let result = add_heading_ids(html);
        assert!(result.contains(r#"id="hello-world""#));
    }

    #[test]
    fn test_add_heading_ids_preserves_existing_id() {
        let html = r#"<h2 id="custom-id">Hello</h2>"#;
        let result = add_heading_ids(html);
        assert!(result.contains(r#"id="custom-id""#));
    }

    #[test]
    fn test_add_heading_ids_multiple() {
        let html = "<h2>First</h2><h3>Second</h3><h4>Third</h4>";
        let result = add_heading_ids(html);
        assert!(result.contains(r#"id="first""#));
        assert!(result.contains(r#"id="second""#));
        assert!(result.contains(r#"id="third""#));
    }

    #[test]
    fn test_add_heading_ids_skips_h1() {
        let html = "<h1>Title</h1>";
        let result = add_heading_ids(html);
        assert!(result.contains(r#"id="title""#));
    }

    #[test]
    fn test_render_markdown_basic() {
        let result = render_markdown("# Hello\n\nThis is **bold**.");
        assert!(result.contains("<h1"));
        assert!(result.contains("id="));
        assert!(result.contains("<strong>"));
    }

    #[test]
    fn test_render_markdown_table() {
        let md = "| A | B |\n|---|---|\n| 1 | 2 |";
        let result = render_markdown(md);
        assert!(result.contains("<table>"));
    }

    #[test]
    fn test_render_markdown_code_block() {
        let md = "```rust\nfn main() {}\n```";
        let result = render_markdown(md);
        assert!(result.contains("code-block"));
        assert!(result.contains("data-lang=\"rust\""));
        assert!(result.contains("main"));
    }

    #[test]
    fn test_render_markdown_plain_code_block() {
        let md = "```\nplain code\n```";
        let result = render_markdown(md);
        assert!(result.contains("code-block"));
        assert!(result.contains("ln\">1<"));
    }

    #[test]
    fn test_render_markdown_task_list() {
        let md = "- [x] done\n- [ ] todo";
        let result = render_markdown(md);
        assert!(result.contains("checked"));
    }

    #[test]
    fn test_render_markdown_footnote() {
        let md = "text[^1]\n\n[^1]: note";
        let result = render_markdown(md);
        assert!(result.contains("footnote"));
    }

    #[test]
    fn test_load_posts_from_directory() {
        let dir = std::env::temp_dir().join(format!("myblog_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let post1 = "+++\ntitle = \"Post One\"\ndate = \"2024-06-15\"\ntags = [\"test\"]\n+++\n\n# Post One\n\nContent here.";
        let post2 = "+++\ntitle = \"Post Two\"\ndate = \"2024-01-10\"\ntags = [\"test\", \"rust\"]\n+++\n\n# Post Two\n\nMore content.";
        std::fs::write(dir.join("post-one.md"), post1).unwrap();
        std::fs::write(dir.join("post-two.md"), post2).unwrap();

        let result = load_posts(dir.to_str().unwrap()).unwrap();
        // Sorted newest first
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].slug, "post-one");
        assert_eq!(result[1].slug, "post-two");
        assert_eq!(result[0].reading_time, 1);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_load_posts_nonexistent_dir() {
        let dir = std::env::temp_dir().join(format!("myblog_test_nonexistent_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let result = load_posts(dir.to_str().unwrap()).unwrap();
        assert!(result.is_empty());
        // Cleanup: load_posts creates the directory if it doesn't exist
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_load_posts_with_excerpt() {
        let dir = std::env::temp_dir().join(format!("myblog_test_excerpt_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let content = "+++\ntitle = \"Excerpt Post\"\ndate = \"2024-03-01\"\ntags = []\nexcerpt = \"Custom summary\"\n+++\n\nBody";
        std::fs::write(dir.join("excerpt.md"), content).unwrap();
        let posts = load_posts(dir.to_str().unwrap()).unwrap();
        assert_eq!(posts[0].excerpt, "Custom summary");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_load_posts_generates_excerpt() {
        let dir = std::env::temp_dir().join(format!("myblog_test_autoexcerpt_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let long_body = "A".repeat(300);
        let content = format!(
            "+++\ntitle = \"No Excerpt\"\ndate = \"2024-04-01\"\ntags = []\n+++\n\n{}",
            long_body
        );
        std::fs::write(dir.join("no-excerpt.md"), content).unwrap();
        let posts = load_posts(dir.to_str().unwrap()).unwrap();
        assert_eq!(posts[0].excerpt.len(), 163); // 160 chars + "..."

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_full_post_pipeline() {
        let md = "+++\ntitle = \"Pipeline Test\"\ndate = \"2025-01-01\"\ntags = [\"pipeline\", \"test\"]\nexcerpt = \"Pipeline excerpt\"\n+++\n\n## Heading\n\nSome **bold** text. More text here for word count.\n\n```python\nprint(\"hello\")\n```";
        let (fm, body) = parse_frontmatter(md).unwrap();
        assert_eq!(fm.title, "Pipeline Test");

        let html = render_markdown(&body);
        assert!(html.contains("<h2 id=\"heading\""));

        let toc = extract_toc(&html);
        assert!(!toc.is_empty());
        assert_eq!(toc[0].text, "Heading");

        let wc = word_count(&html);
        assert!(wc > 0);

        let rt = reading_time(&html);
        assert_eq!(rt, 1);

        let stripped = strip_html_tags(&html).to_lowercase();
        assert!(stripped.contains("heading"));
        assert!(stripped.contains("print"));
    }

    #[test]
    fn test_highlight_code_blocks_with_lang() {
        let html = "<pre><code class=\"language-rust\">fn main() {}</code></pre>";
        let result = highlight_code_blocks(html);
        assert!(result.contains("code-block"), "should have code-block wrapper");
        assert!(result.contains("data-lang=\"rust\""), "should preserve language attribute");
        assert!(result.contains("<table"), "should include table markup");
    }

    #[test]
    fn test_highlight_code_blocks_plain() {
        let html = "<pre><code>plain text</code></pre>";
        let result = highlight_code_blocks(html);
        assert!(result.contains("code-block"), "should have code-block wrapper");
        assert!(!result.contains("data-lang"), "should NOT have language attribute for plain blocks");
        assert!(result.contains("<table"), "should include table markup");
    }

    #[test]
    fn test_resolve_syntax_jsx() {
        let ss = get_syntax_set();
        let syntax = resolve_syntax("jsx", ss);
        assert!(syntax.is_some(), "jsx should resolve to JavaScript");
        assert_eq!(syntax.unwrap().name, "JavaScript");
    }

    #[test]
    fn test_resolve_syntax_typescript() {
        let ss = get_syntax_set();
        let syntax = resolve_syntax("typescript", ss);
        assert!(syntax.is_some(), "typescript should resolve to JavaScript");
        assert_eq!(syntax.unwrap().name, "JavaScript");
    }

    #[test]
    fn test_resolve_syntax_kotlin() {
        let ss = get_syntax_set();
        let syntax = resolve_syntax("kotlin", ss);
        assert!(syntax.is_some(), "kotlin should resolve to Java");
        assert_eq!(syntax.unwrap().name, "Java");
    }

    #[test]
    fn test_resolve_syntax_dart() {
        let ss = get_syntax_set();
        let syntax = resolve_syntax("dart", ss);
        assert!(syntax.is_some(), "dart should resolve to JavaScript");
        assert_eq!(syntax.unwrap().name, "JavaScript");
    }

    #[test]
    fn test_resolve_syntax_vue() {
        let ss = get_syntax_set();
        let syntax = resolve_syntax("vue", ss);
        assert!(syntax.is_some(), "vue should resolve to HTML");
        assert_eq!(syntax.unwrap().name, "HTML");
    }

    #[test]
    fn test_resolve_syntax_svelte() {
        let ss = get_syntax_set();
        let syntax = resolve_syntax("svelte", ss);
        assert!(syntax.is_some(), "svelte should resolve to HTML");
        assert_eq!(syntax.unwrap().name, "HTML");
    }

    #[test]
    fn test_resolve_syntax_elixir() {
        let ss = get_syntax_set();
        let syntax = resolve_syntax("elixir", ss);
        assert!(syntax.is_some(), "elixir should resolve to Ruby");
        assert_eq!(syntax.unwrap().name, "Ruby");
    }

    #[test]
    fn test_resolve_syntax_unknown() {
        let ss = get_syntax_set();
        assert!(resolve_syntax("mermaid", ss).is_none(), "mermaid should not be resolved");
        assert!(resolve_syntax("hcl", ss).is_none(), "hcl should not be resolved");
        assert!(resolve_syntax("unknown_language", ss).is_none(), "unknown language should not be resolved");
    }

    #[test]
    fn test_highlight_code_blocks_jsx() {
        let html = r#"<pre><code class="language-jsx">const x = <div /></code></pre>"#;
        let result = highlight_code_blocks(html);
        assert!(result.contains("code-block"), "should have code-block wrapper");
        assert!(result.contains(r#"data-lang="jsx""#), "should preserve original jsx language attribute");
    }

    #[test]
    fn test_highlight_code_blocks_typescript() {
        let html = r#"<pre><code class="language-typescript">const x: number = 1</code></pre>"#;
        let result = highlight_code_blocks(html);
        assert!(result.contains("code-block"), "should have code-block wrapper");
        assert!(result.contains(r#"data-lang="typescript""#), "should preserve original language attribute");
    }

    #[test]
    fn test_highlight_code_blocks_unmapped() {
        let html = r#"<pre><code class="language-mermaid">graph TD; A-->B;</code></pre>"#;
        let result = highlight_code_blocks(html);
        assert!(result.contains("code-block"), "should have code-block wrapper");
        assert!(result.contains(r#"data-lang="mermaid""#), "should preserve original language attribute");
        assert!(!result.contains("<span style"), "should NOT have syntax-highlighted spans");
    }

    #[test]
    fn test_strip_html_tags_case_insensitive() {
        assert_eq!(strip_html_tags("<BR>"), "");
        assert_eq!(strip_html_tags("<Script>alert(1)</Script>"), "");
        assert_eq!(strip_html_tags("<IMG SRC=\"x\">"), "");
        assert_eq!(strip_html_tags("<P>Hello</P>"), "Hello");
    }

    #[test]
    fn test_add_heading_ids_duplicates() {
        let html = "<h2>Intro</h2><h2>Intro</h2>";
        let result = add_heading_ids(html);
        assert!(result.contains(r#"id="intro""#), "both occurrences should have id=intro: {}", result);
        assert_eq!(result.matches(r#"id="intro""#).count(), 2, "both headings get same id");
    }

    #[test]
    fn test_load_posts_empty_tags() {
        let dir = std::env::temp_dir().join(format!("myblog_test_empty_tags_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let content = "+++\ntitle = \"No Tags\"\ndate = \"2024-05-20\"\ntags = []\n+++\n\nBody";
        std::fs::write(dir.join("no-tags.md"), content).unwrap();
        let posts = load_posts(dir.to_str().unwrap()).unwrap();
        assert_eq!(posts.len(), 1);
        assert!(posts[0].frontmatter.tags.is_empty(), "tags should be empty, got: {:?}", posts[0].frontmatter.tags);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_load_posts_short_body_excerpt() {
        let dir = std::env::temp_dir().join(format!("myblog_test_short_excerpt_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let body = "Short body here"; // 15 chars, well under 160
        let content = format!(
            "+++\ntitle = \"Short\"\ndate = \"2024-05-20\"\ntags = []\n+++\n\n{}",
            body
        );
        std::fs::write(dir.join("short.md"), content).unwrap();
        let posts = load_posts(dir.to_str().unwrap()).unwrap();
        assert_eq!(posts.len(), 1);
        // Excerpt should not end with "..." since body is shorter than 160 chars
        assert!(!posts[0].excerpt.ends_with("..."), "short body should not have ellipsis: '{}'", posts[0].excerpt);
        assert_eq!(posts[0].excerpt, "Short body here");

        let _ = std::fs::remove_dir_all(&dir);
    }
}

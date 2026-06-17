# AGENTS.md — myblog

Personal blog built with Rust + Axum 0.7. Flat-file, no database.

## Commands

```
cargo build
cargo run            # http://127.0.0.1:3000
cargo test           # all 106 tests (74 unit + 32 integration), no external services
cargo clippy         # zero warnings expected
cargo test <name>    # focused test
```

## Routes (10 handlers in `src/handlers.rs`)

All in one inner router with `Cache-Control: no-cache`. Outer router adds static serving + CSP + `X-Content-Type-Options: nosniff` + `CompressionLayer`.

| Route | Returns | Notes |
|---|---|---|
| `/` | `Html` | paginated index |
| `/post/:slug` | `Response` | 200 or 404 |
| `/tag/:name` | `Html` | filtered + paginated |
| `/about` | `Html` | renders `pages/about.md` |
| `/tags` | `Html` | tag cloud with weight |
| `/search?q=` | `Html` | full-text via `search_text` field |
| `/archive` | `Html` | grouped by year/month |
| `/feed.xml` | `Response` | RSS 2.0, `application/rss+xml` |
| `/sitemap.xml` | `Response` | XML, `application/xml` |
| fallback | `Response` | 404 with recent posts + popular tags |

## Architecture

- **`src/main.rs`**: Two-pass router — inner (all 10 routes + Cache-Control) merged into outer (ServeDir + CSP + nosniff + CompressionLayer + state). `notify` watcher hot-reloads on `posts/*.md` and `templates/*.html`.
- **`src/lib.rs`**: `AppState { tera: RwLock<Tera>, posts: RwLock<Arc<Vec<Post>>> }`. 4 config values (`SITE_URL`, `SITE_TITLE`, `SITE_DESC`, `POSTS_PER_PAGE`) read from env via `OnceLock` (cached after first access, never re-read).
- **`src/post.rs`**: Parses `+++`-delimited TOML frontmatter from `posts/*.md`. Markdown via pulldown-cmark (tables, footnotes, strikethrough, tasklists, heading attributes enabled). **Slug = filename stem**, not frontmatter. Code blocks highlighted via syntect theme `"InspiredGitHub"` using `SyntaxSet::load_defaults_newlines` + `default-fancy` features.
- **Tera**: `autoescape_on(vec![])` — auto-escaping disabled globally. `post.content_html` is pre-rendered HTML (use `| safe` in templates). All other variables must use `escape_xml` filter or `escape_xml()` helper explicitly.
- **Regex**: all hot-path regex cached via `OnceLock` (heading IDs, TOC extraction, code block highlighting, date validation).
- **RwLock poisoning recovery**: `lock().unwrap_or_else(|e| e.into_inner())` on all 3 locks.
- **JS** (`static/script.js`): `/` focuses search input; `ArrowLeft`/`ArrowRight` navigate prev/next post.

## Post format (`posts/*.md`)

```
+++
title = "Post Title"
date = "2024-06-15"
tags = ["rust", "web"]
excerpt = "Optional"   # auto-generated (160 chars + "...") if missing
+++
```

Sorted newest-first by date. Slug = filename stem.

## Tests

- **Unit**: `src/handlers.rs` + `src/post.rs` under `#[cfg(test)]`
- **Integration**: `tests/integration.rs` — uses `#[tokio::test]` + `tower::Service::call` for HTTP testing. `tower` crate is a dev-dependency.
- **Setup pattern**: `setup_test_posts(suffix)` creates temp dir with 3 posts, returns `(Vec<Post>, PathBuf)`. Temp dirs are cleaned up after each test.
- No external services, no fixtures. `cargo test` is sufficient.

## Deployment

- **Dockerfile**: multi-stage (`rust:slim-bookworm` build → `debian:bookworm-slim` runtime). Copies `templates/`, `static/`, `pages/`, `posts/` into runtime.
- **Config env vars**: `SITE_URL` (default `http://127.0.0.1:3000`), `SITE_TITLE`, `SITE_DESC`, `POSTS_PER_PAGE` (default `5`), `PORT` (default `3000`).
- `notify` watcher works in dev; on Railway the app restarts on each deploy.

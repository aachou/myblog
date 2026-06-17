# AGENTS.md — myblog

## Project

Personal blog built with Rust + Axum 0.7. Flat-file, no database.

## Commands

```
cargo build          # compile
cargo run            # start server at http://127.0.0.1:3000
cargo test           # runs all 106 tests (74 unit + 32 integration)
cargo clippy         # zero warnings expected
```

## Architecture

- **Entrypoint**: `src/main.rs` — binds `127.0.0.1:3000`, sets up Tera templates from `templates/**/*.html`, spawns `notify` file watcher. Applies CSP (`default-src 'self'; style-src 'unsafe-inline' 'self'; img-src 'self' data:; frame-ancestors 'none'`), `X-Content-Type-Options: nosniff`, and `CompressionLayer`. Split router: outer (static, feed, sitemap, fallback) + inner `Cache-Control: no-cache` (HTML routes).
- **App state**: `src/lib.rs` — `AppState` holds `tera: RwLock<Tera>` + `posts: RwLock<Arc<Vec<Post>>>`. All 4 config values (`site_url`, `site_title`, `site_desc`, `posts_per_page`) cached in `OnceLock` functions returning `&'static str`/`usize`.
- **Posts**: `src/post.rs` — parses `+++`-delimited TOML frontmatter from `posts/*.md`, renders Markdown via pulldown-cmark, highlights code blocks via syntect (single-pass regex for both lang and plain blocks). Regex caching via `OnceLock` for `add_heading_ids`, `extract_toc`, `highlight_code_blocks`, `is_valid_date`.
- **Handlers**: `src/handlers.rs` — 10 route handlers + `not_found_handler` (shows recent posts + popular tags + search). Pagination with `div_ceil`. `Response`-returning for 404/feed/sitemap; `Html`-returning for index/tag/etc. Extracted helpers: `group_by_year_month`, `popular_tags`, `generate_sitemap_urls`.
- **Templates**: `templates/` — Tera HTML templates, `escape_xml` and `urlencode` filters used in templates for XSS safety. `urlencode` for href attributes, `escape_xml` for display text. Skip-to-content link in `base.html`. `aria-label` on search inputs and archive nav.
- **Static**: `static/` — served under `/static`. CSS classes used for all styling (no inline styles except dynamic tag-cloud font-size).
- **About page**: `pages/about.md`

## Post format

Posts live in `posts/*.md`. Frontmatter uses TOML with `+++` delimiters:

```toml
+++
title = "Post Title"
date = "2024-06-15"
tags = ["rust", "web"]
excerpt = "Optional custom excerpt"
+++
```

Slug is derived from the filename (not frontmatter). Posts sorted newest-first by date.

## Tests

- Unit tests: inline in `src/handlers.rs` and `src/post.rs` under `#[cfg(test)]`
- Integration tests: `tests/integration.rs` — create temp directories with test posts, clean up after
- No external services needed. `cargo test` is sufficient.

## Style notes

- `tera.autoescape_on(vec![])` — auto-escaping disabled globally
- `syntect` theme: `"InspiredGitHub"` with `default-fancy` features
- All `unwrap()` replaced with `expect()` or `unwrap_or_else(|e| e.into_inner())` (for poisoning recovery)
- All regex in hot paths cached via `OnceLock`
- Handlers that return `Response` vs `Html<String>`: feed/sitemap/fallback use `Response` for explicit status codes; all others use `Html<String>`
- `RwLock` poisoning recovery pattern: `lock().unwrap_or_else(|e| e.into_inner())` on all 3 locks (tera, posts, watcher_state.posts)

## Deployment

- **Dockerfile**: multi-stage build, runtime image is `debian:bookworm-slim` (~20 MB)
- **Config env vars** (set in platform dashboard):
  - `SITE_URL` — public URL (e.g. `https://myblog.up.railway.app`)
  - `SITE_TITLE` — default `MyBlog`
  - `SITE_DESC` — default `A personal blog built with Rust and Axum`
  - `POSTS_PER_PAGE` — default `5`
- **Recommended platform**: Railway (auto-detect Dockerfile, free quota for personal blog)
- **Note**: `notify` file watcher works in development; on Railway the app restarts on each deploy
- **Local dev**: `cargo run` starts at `http://127.0.0.1:3000`

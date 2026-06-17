# MyBlog

Personal blog built with **Rust + Axum 0.7**. Flat-file, no database. Markdown posts with TOML frontmatter.

## Features

- Markdown rendering with syntax highlighting (syntect)
- RSS feed & XML sitemap
- Tag cloud, archive by year/month, full-text search
- Related posts, pagination, reading time
- CSP headers, XSS protection, accessibility (skip-link, aria-labels)
- 106 tests (74 unit + 32 integration), zero clippy warnings

## Quick Start

```bash
cargo run
# → http://127.0.0.1:3000
```

## Writing Posts

Create `posts/my-post.md`:

```toml
+++
title = "My Post"
date = "2024-06-15"
tags = ["rust", "web"]
excerpt = "Optional excerpt"
+++

## Content here
```

Slug is derived from the filename (`my-post`). No database, no admin panel — just push `.md` files.

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `SITE_URL` | `http://127.0.0.1:3000` | Public URL of the blog |
| `SITE_TITLE` | `MyBlog` | Site title |
| `SITE_DESC` | `A personal blog built with Rust and Axum` | Site description |
| `POSTS_PER_PAGE` | `5` | Posts per page |

## Deployment

Recommended: **Railway** (auto-detects Dockerfile, free quota).

1. Push to GitHub
2. Railway → New Project → Deploy from GitHub repo
3. Set environment variables in dashboard
4. Done — subsequent pushes auto-deploy

Or deploy anywhere that supports Docker.

## License

MIT

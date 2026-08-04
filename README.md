# MyBlog

Personal blog built with **Rust + Axum 0.7**. Flat-file, no database. Markdown posts with TOML frontmatter.

![README](./README.png)

## Features

- Markdown rendering with syntax highlighting (syntect, alias mapping for language names)
- KaTeX math: inline `$..$` and block `$$..$$` (client-side rendering)
- Superscript `^text^` and subscript `~text~`
- Footnotes with correct numbering
- Task lists, tables, strikethrough, heading attributes
- Table of contents extracted from headings
- RSS feed & XML sitemap
- Tag cloud (weighted), archive grouped by year/month
- Full-text search (lowercased at build time)
- Pagination, reading time, auto-generated excerpts
- About page — author name & avatar editable inline (with cropping)
- Comments via [utterances](https://utteranc.es) (GitHub Issues-based, client-side)
- CSP security headers, XSS protection, accessibility (skip-link, aria-labels)
- Hot-reload via `notify` (dev), Docker multi-stage build (prod)
- 138 tests (105 unit + 33 integration), zero clippy warnings

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
excerpt = "Optional — auto-generated from first 160 chars if omitted"
+++

## Content here

Inline math: $E = mc^2$

Block math: $$\sum_{i=1}^n i = \frac{n(n+1)}{2}$$

Superscript: E = mc^2^  Subscript: H~2~O

Footnotes: Here is a fact.[^1]

[^1]: The footnote text.
```

Slug is derived from the filename (`my-post`). No database, no admin panel — just push `.md` files.

## Configuration

### Site Config (`config/site.json`)

```json
{
  "title": "MyBlog",
  "description": "A personal blog built with Rust and Axum",
  "url": "http://127.0.0.1:3000",
  "posts_per_page": 5
}
```

Can be overridden by environment variables:

| Variable | Description |
|----------|-------------|
| `SITE_TITLE` | Overrides `title` in config |
| `SITE_DESC` | Overrides `description` in config |
| `SITE_URL` | Overrides `url` in config |
| `POSTS_PER_PAGE` | Overrides `posts_per_page` in config |

Priority: **env var > config file > code default**.

### About Config (`config/about.json`)

```json
{
  "author_name": "阿愁",
  "avatar_path": "/static/images/avatar.jpg"
}
```

Editable inline on `/about` page — click the name to edit, click the avatar to upload a new one (with cropping, max 5 MB, JPG/PNG/WebP).

## Deployment

Recommended: **Railway** (auto-detects Dockerfile, free quota).

1. Push to GitHub
2. Railway → New Project → Deploy from GitHub repo
3. Set environment variables in dashboard
4. Done — subsequent pushes auto-deploy

Or deploy anywhere that supports Docker.

## License

MIT
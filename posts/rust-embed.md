+++
title = "Embedding Files in Rust with rust-embed"
date = "2025-06-18"
tags = ["rust", "tooling", "performance"]
excerpt = "Learn how to embed static assets directly into Rust binaries using the rust-embed crate for zero-cost distribution."
+++

Shipping a Rust binary that depends on external files is inconvenient. The `rust-embed` crate solves this by compiling files directly into your executable.

## Why Embed Files?

- Zero deployment dependencies: everything is in one binary
- Atomic deployment: no missing file errors
- Versioned by default: files match the exact build
- Faster startup: no disk I/O for assets

## Getting Started

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
rust-embed = "8"
```

## Basic Usage

```rust
use rust_embed::Embed;
use std::path::Path;

#[derive(Embed)]
#[folder = "static/"]
struct Asset;

fn main() {
    let file = Asset::get("index.html").unwrap();
    let content = std::str::from_utf8(file.data.as_ref()).unwrap();
    println!("{}", content);
}
```

## Generated Methods

The derive macro generates the following methods:

| Method | Description |
|--------|-------------|
| `get(path)` | Retrieve a single file |
| `iter()` | List all embedded files |
| `assets()` | Return metadata for all files |

## Working with Binary Files

Images and other binary data work seamlessly:

```rust
let logo = Asset::get("logo.png").unwrap();
let image_data = logo.data.as_ref();
```

## Compression

Enable optional compression for smaller binaries:

```toml
[dependencies]
rust-embed = { version = "8", features = ["compression"] }
```

Compressed files are decompressed at runtime. This trades binary size for a small CPU overhead.

## Integration with Web Servers

Embedding an SPA into an Axum server:

```rust
use axum::{Router, response::IntoResponse};

fn serve_static(path: &str) -> impl IntoResponse {
    match Asset::get(path.trim_start_matches('/')) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            ([(axum::http::header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => (StatusCode::NOT_FOUND, "Not found").into_response(),
    }
}
```

## Filtering Files

Include only specific patterns:

```rust
#[derive(Embed)]
#[folder = "assets/"]
#[include = "*.html"]
#[include = "*.css"]
#[exclude = "*.map"]
struct Asset;
```

## Performance

Embedded files are stored as `&'static [u8]` slices. Access is O(1) via a perfect hash function. There is no runtime overhead compared to reading from disk.

## Limitations

- Embedded files increase binary size
- Build time increases proportionally to asset size
- Files over a few hundred MB should be loaded externally
- Hot-reloading during development requires separate handling

## Development vs Production

Feature-gate your embedding strategy:

```rust
#[cfg(debug_assertions)]
fn load_file(path: &str) -> Vec<u8> {
    std::fs::read(format!("dev-assets/{}", path)).unwrap()
}

#[cfg(not(debug_assertions))]
fn load_file(path: &str) -> Vec<u8> {
    Asset::get(path).unwrap().data.to_vec()
}
```

For small to medium projects, `rust-embed` simplifies distribution enormously.

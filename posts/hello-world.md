+++
title = "Hello, World! 鈥?Building a Blog with Rust"
date = "2024-06-16"
tags = ["rust", "web", "axum"]
excerpt = "My first blog post! Exploring how to build a modern, fast, and beautiful blog using the Rust programming language and the Axum web framework."
+++

Welcome to the very first post of **MyBlog**! This blog is built entirely with Rust 鈥?a language known for its performance, safety, and expressiveness.

## Why Rust?

Rust offers a unique combination of:

- **Zero-cost abstractions**: High-level ergonomics without runtime overhead
- **Memory safety**: The borrow checker ensures no null pointer dereferences or data races
- **Excellent tooling**: Cargo, rustfmt, clippy, and a vibrant ecosystem

## The Tech Stack

Here's what powers this blog:

| Component    | Technology                           |
|-------------|--------------------------------------|
| Framework    | [Axum](https://github.com/tokio-rs/axum) |
| Templates    | [Tera](https://tera.netlify.app/)    |
| Markdown     | [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) |
| Highlighting | [Syntect](https://github.com/trishume/syntect) |
| Styling      | Custom CSS with dark mode support    |

## A Code Sample

Here's a quick example of Rust's pattern matching:

```rust
#[derive(Debug)]
enum Status {
    Active,
    Inactive,
    Pending { since: chrono::NaiveDateTime },
}

fn describe_status(status: &Status) -> &str {
    match status {
        Status::Active => "User is active",
        Status::Inactive => "User is inactive",
        Status::Pending { since } if since > chrono::Utc::now().naive_utc() => {
            "Pending approval"
        }
        _ => "Unknown status",
    }
}
```

## What's Next?

This blog will cover topics like:

1. Systems programming with Rust
2. Web development patterns
3. Database design and optimization
4. Personal projects and experiments

Stay tuned for more content!

> *"The only way to learn a new programming language is by writing programs in it." 鈥?Dennis Ritchie*

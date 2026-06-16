+++
title = "Building CLI Tools with Rust: A Practical Guide"
date = "2024-06-08"
tags = ["rust", "cli", "tutorial"]
excerpt = "Learn how to build command-line tools in Rust using clap, handle input/output, and distribute your binaries."
+++

Rust is an excellent language for building CLI tools. It's fast, safe, and compiles to a single binary.

## Project Setup

```bash
cargo new my-cli
cd my-cli
```

Add dependencies:

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
anyhow = "1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

## Argument Parsing with Clap

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "greet")]
#[command(about = "A simple greeting tool")]
struct Cli {
    name: String,

    #[arg(short, long, default_value = "1")]
    count: u8,

    #[arg(short, long)]
    formal: bool,
}

fn main() {
    let cli = Cli::parse();

    for _ in 0..cli.count {
        if cli.formal {
            println!("Good day, {}!", cli.name);
        } else {
            println!("Hey {}!", cli.name);
        }
    }
}
```

## Error Handling

Use `anyhow` for simple error handling:

```rust
use anyhow::{Context, Result};
use std::fs;

fn read_config(path: &str) -> Result<String> {
    fs::read_to_string(path)
        .with_context(|| format!("Failed to read config from {}", path))
}

fn main() -> Result<()> {
    let config = read_config("config.toml")?;
    println!("Config: {}", config);
    Ok(())
}
```

## Cross-Platform Distribution

```bash
# Build for current platform
cargo build --release

# Cross-compile (requires cross-installation)
cargo install cross
cross build --release --target x86_64-pc-windows-gnu
cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target aarch64-apple-darwin
```

Rust's CLI ecosystem makes it easy to build professional tools that users love.

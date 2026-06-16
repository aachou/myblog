+++
title = "Getting Started with Rust: A Beginner's Guide"
date = "2024-06-14"
tags = ["rust", "tutorial", "beginners"]
excerpt = "A comprehensive guide to getting started with Rust programming. Learn about installation, basic syntax, ownership, and best practices."
+++

Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. Let's dive in!

## Installation

The recommended way to install Rust is through `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

On Windows, download the installer from [rustup.rs](https://rustup.rs/) or use:

```powershell
winget install Rustlang.Rustup
```

## Your First Program

Create a new project:

```bash
cargo new hello-rust
cd hello-rust
```

Edit `src/main.rs`:

```rust
fn main() {
    println!("Hello, Rust!");
    
    let name = "World";
    let greeting = format!("Hello, {}!", name);
    println!("{}", greeting);
}
```

Run it:

```bash
cargo run
```

## Understanding Ownership

Ownership is Rust's most unique feature. It enables memory safety without a garbage collector.

```rust
fn main() {
    // s comes into scope
    let s = String::from("hello");
    
    // s is moved to takes_ownership
    takes_ownership(s);
    
    // s is no longer valid here 鈥?compile error!
    // println!("{}", s);
    
    let x = 5;
    // x is copied (i32 implements Copy)
    makes_copy(x);
    // x is still valid
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string dropped here

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
```

## Common Data Structures

### Vectors

```rust
let mut numbers: Vec<i32> = Vec::new();
numbers.push(1);
numbers.push(2);
numbers.push(3);

// Using the vec! macro
let more_numbers = vec![4, 5, 6];

// Iterating
for n in &numbers {
    println!("{n}");
}
```

### HashMaps

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Alice"), 100);
scores.insert(String::from("Bob"), 85);

let alice_score = scores.get("Alice").copied().unwrap_or(0);
```

## Error Handling

Rust uses `Result<T, E>` and `Option<T>` for error handling:

```rust
use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?; // ? propagates the error
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Usage
match read_file("config.toml") {
    Ok(config) => println!("Config: {}", config),
    Err(e) => eprintln!("Error reading config: {}", e),
}
```

## Traits and Generics

```rust
trait Speak {
    fn speak(&self) -> String;
}

struct Dog { name: String }
struct Cat { name: String }

impl Speak for Dog {
    fn speak(&self) -> String {
        format!("{} says: Woof!", self.name)
    }
}

impl Speak for Cat {
    fn speak(&self) -> String {
        format!("{} says: Meow!", self.name)
    }
}

fn make_speak<T: Speak>(animal: &T) {
    println!("{}", animal.speak());
}

fn main() {
    make_speak(&Dog { name: "Buddy".into() });
    make_speak(&Cat { name: "Whiskers".into() });
}
```

## Tips for Beginners

1. **Read the Book**: [The Rust Programming Language](https://doc.rust-lang.org/book/) is excellent
2. **Use Clippy**: Run `cargo clippy` for linting
3. **Format Your Code**: `cargo fmt` keeps your code clean
4. **Start Small**: Build CLI tools to get comfortable
5. **Embrace the Compiler**: Rust's error messages are incredibly helpful

> *"Rust is the future of systems programming." 鈥?The Rust Community*

+++
title = "Design Patterns in Rust: Idiomatic Approaches"
date = "2026-09-14"
tags = ["rust", "design-patterns", "programming"]
excerpt = "Many classic Gang of Four patterns translate poorly to Rust because ownership and borrowing change the game. Here are the patterns that shine in idiomatic Rust."
++++

Design patterns are solutions to recurring problems in a given context. The Gang of Four book was written for object-oriented languages with shared mutable state. Rust's ownership model, traits, and lack of inheritance call for a fresh look at these patterns.

## Builder Pattern

The Builder pattern is ubiquitous in Rust and is used in the standard library itself.

```rust
#[derive(Debug)]
struct Request {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

struct RequestBuilder {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl RequestBuilder {
    fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            method: "GET".to_string(),
            headers: vec![],
            body: None,
        }
    }

    fn method(mut self, method: &str) -> Self {
        self.method = method.to_string();
        self
    }

    fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }

    fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }

    fn build(self) -> Result<Request, &'static str> {
        if self.url.is_empty() {
            return Err("URL must not be empty");
        }
        Ok(Request {
            url: self.url,
            method: self.method,
            headers: self.headers,
            body: self.body,
        })
    }
}
```

Using `self` (not `&self`) in builder methods allows chaining and enforces a linear flow.

## Newtype Pattern

The newtype pattern wraps a single type in a tuple struct to add type safety and custom behavior.

```rust
struct Email(String);

impl Email {
    fn new(value: &str) -> Result<Self, &'static str> {
        if value.contains('@') {
            Ok(Self(value.to_string()))
        } else {
            Err("Invalid email")
        }
    }
}

// Prevent accidental mixing with raw strings
fn send_notification(email: &Email) {
    println!("Sending to: {}", email.0);
}
```

This prevents passing a raw string where an email is expected and provides a natural place for validation.

## Strategy Pattern via Traits

The Strategy pattern maps directly to Rust traits.

```rust
trait CompressionStrategy {
    fn compress(&self, data: &[u8]) -> Vec<u8>;
}

struct GzipCompressor;
impl CompressionStrategy for GzipCompressor {
    fn compress(&self, data: &[u8]) -> Vec<u8> {
        // Gzip implementation
        data.to_vec()
    }
}

struct BrotliCompressor;
impl CompressionStrategy for BrotliCompressor {
    fn compress(&self, data: &[u8]) -> Vec<u8> {
        // Brotli implementation
        data.to_vec()
    }
}

struct Compressor {
    strategy: Box<dyn CompressionStrategy>,
}
```

Use `Box<dyn Trait>` for runtime polymorphism or generics for static dispatch.

## State Pattern with Enums

The State pattern in OOP involves multiple classes implementing the same interface. In Rust, enums with data are more natural.

```rust
enum ConnectionState {
    Disconnected,
    Connecting { retries: u32 },
    Connected { session_id: u64 },
    Reconnecting { backoff: Duration },
}

impl ConnectionState {
    fn transition(self, event: ConnectionEvent) -> Self {
        match (self, event) {
            (ConnectionState::Disconnected, ConnectionEvent::Connect) => {
                ConnectionState::Connecting { retries: 0 }
            }
            (ConnectionState::Connecting { retries }, ConnectionEvent::Success(session_id)) => {
                ConnectionState::Connected { session_id }
            }
            (ConnectionState::Connected { .. }, ConnectionEvent::Disconnect) => {
                ConnectionState::Disconnected
            }
            _ => self,
        }
    }
}
```

Exhaustive matching ensures all state transitions are handled.

## RAII Pattern

Rust uses RAII (Resource Acquisition Is Initialization) natively. The `Drop` trait guarantees cleanup when a value goes out of scope.

```rust
struct DatabaseConnection {
    conn: sqlite::Connection,
}

impl Drop for DatabaseConnection {
    fn drop(&mut self) {
        println!("Closing database connection");
        // Connection is automatically closed
    }
}
```

## Option and Result Instead of Null

The Null Object pattern is built into the language with `Option<T>`.

```rust
fn find_user(id: u64) -> Option<User> {
    if id == 0 {
        return None;
    }
    Some(User { id, name: "Alice".to_string() })
}

// No null checks 鈥?pattern matching
match find_user(42) {
    Some(user) => println!("Found: {}", user.name),
    None => println!("User not found"),
}
```

## Iterator Pattern

Rust's iterator combinators provide a functional approach that replaces many traditional patterns.

```rust
let total: u64 = orders
    .iter()
    .filter(|order| order.paid)
    .map(|order| order.amount)
    .sum();
```

## Conclusion

Rust's ownership system makes some classic patterns unnecessary and others more elegant. Embrace enums instead of inheritance hierarchies, traits instead of interfaces, and the type system instead of runtime checks. The result is code that is both safe and expressive.

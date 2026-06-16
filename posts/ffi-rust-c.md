+++
title = "FFI Between Rust and C: A Practical Guide"
date = "2026-11-25"
tags = ["rust", "c", "ffi", "systems-programming"]
excerpt = "Calling C from Rust and exporting Rust functions for C consumption is a common requirement. This guide covers the ABI details, safety considerations, and tooling that makes FFI manageable."
+++

Foreign Function Interface (FFI) allows Rust code to interoperate with C libraries and vice versa. This is essential for using existing C libraries from Rust and for embedding Rust in C or C++ projects.

## Calling C from Rust

The `extern "C"` block declares functions from a C library.

```rust
use std::ffi::{CStr, CString};
use std::os::raw::c_int;

extern "C" {
    fn puts(s: *const std::os::raw::c_char) -> c_int;
    fn strlen(s: *const std::os::raw::c_char) -> usize;
}

fn main() {
    let message = CString::new("Hello from Rust!").unwrap();
    unsafe {
        puts(message.as_ptr());
        let len = strlen(message.as_ptr());
        println!("Length: {}", len);
    }
}
```

Note the `unsafe` block 鈥?FFI calls are inherently unsafe because the compiler cannot verify the C function's behavior.

## Linking Libraries

Tell Rust how to link against C libraries in `build.rs`:

```rust
// build.rs
fn main() {
    println!("cargo:rustc-link-lib=ssl");
    println!("cargo:rustc-link-search=/usr/local/lib");
}
```

Or declare link attributes directly:

```rust
#[link(name = "ssl")]
extern "C" {
    fn SSL_new(ctx: *mut SSL_CTX) -> *mut SSL;
}
```

## Safety Wrappers

Raw FFI is unsafe and error-prone. Wrap it in safe Rust abstractions.

```rust
struct SslContext(*mut SSL_CTX);

impl SslContext {
    fn new() -> Result<Self, &'static str> {
        unsafe {
            let ctx = SSL_CTX_new(SSLv23_method());
            if ctx.is_null() {
                Err("Failed to create SSL context")
            } else {
                Ok(SslContext(ctx))
            }
        }
    }
}

impl Drop for SslContext {
    fn drop(&mut self) {
        unsafe { SSL_CTX_free(self.0); }
    }
}
```

This ensures resources are freed even if the caller forgets.

## Memory Ownership

Rust and C have different ownership models. When passing data across the boundary, be explicit about who is responsible.

| Scenario | Pattern |
|---|---|
| Rust allocates, C uses | Leak memory or pass a destructor callback |
| C allocates, Rust uses | Wrapper with Drop that calls C free function |
| Callback from C to Rust | Use `extern "C"` closure trampoline |

```rust
extern "C" fn callback(data: *mut std::os::raw::c_void) {
    // Cast back to Rust type
    let state: &mut MyState = unsafe { &mut *(data as *mut MyState) };
    state.handle_event();
}
```

## Exporting Rust to C

Mark a function with `#[no_mangle]` and `extern "C"` to make it callable from C.

```rust
#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn rust_hello(name: *const std::os::raw::c_char) -> *mut std::os::raw::c_char {
    let name_str = unsafe { CStr::from_ptr(name) };
    let greeting = format!("Hello, {}!", name_str.to_str().unwrap());
    CString::new(greeting).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn rust_free_string(s: *mut std::os::raw::c_char) {
    unsafe {
        if !s.is_null() {
            drop(CString::from_raw(s));
        }
    }
}
```

The corresponding C header:

```c
int32_t rust_add(int32_t a, int32_t b);
char* rust_hello(const char* name);
void rust_free_string(char* s);
```

## Using `cbindgen` for Headers

The `cbindgen` tool automatically generates C headers from your Rust code.

```bash
cbindgen --lang c --output mylib.h
```

```rust
// src/lib.rs
#[no_mangle]
pub extern "C" fn process_data(input: *const u8, len: usize) -> i32 {
    // implementation
}
```

This eliminates the error-prone manual header maintenance.

## Error Handling

Rust's `Result` cannot cross the FFI boundary directly. Convert errors to error codes or errno-style patterns.

```rust
#[no_mangle]
pub extern "C" fn open_file(path: *const c_char) -> i32 {
    let path_str = match unsafe { CStr::from_ptr(path) }.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };
    match File::open(path_str) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}
```

## Data Structures

Passing complex data across FFI requires `#[repr(C)]` to ensure Rust structs match C layout.

```rust
#[repr(C)]
struct Point {
    x: f64,
    y: f64,
}

extern "C" {
    fn distance(p1: *const Point, p2: *const Point) -> f64;
}
```

## Conclusion

FFI between Rust and C is well-supported and battle-tested. The key is wrapping raw `unsafe` calls in safe abstractions that manage memory and enforce invariants. With proper wrappers and tools like `cbindgen`, FFI can be both safe and ergonomic.

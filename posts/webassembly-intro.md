+++
title = "WebAssembly Intro"
date = "2022-05-18"
tags = ["webassembly", "wasm", "performance"]
excerpt = "An overview of WebAssembly 鈥?a binary instruction format that runs near-native speed in the browser. See how to compile C/Rust to WASM."
+++

WebAssembly (WASM) is a low-level binary instruction format that runs in modern browsers at near-native speed. It is designed as a portable compilation target for languages like C, C++, and Rust.

## Why WebAssembly?

JavaScript is interpreted, which imposes overhead. WASM provides a compact binary format that can be decoded and compiled much faster. Use cases include:

- Video editing and image processing
- Game engines
- Scientific simulations
- Cryptographic computations

## Compiling from C

Using Emscripten, you can compile C code directly to WASM:

```c
#include <emscripten/emscripten.h>

EMSCRIPTEN_KEEPALIVE
int add(int a, int b) {
  return a + b;
}
```

```bash
emcc add.c -o add.js -s EXPORTED_FUNCTIONS="['_add']"
```

## Compiling from Rust

Rust has first-class WASM support through `wasm-pack`:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
```

```bash
wasm-pack build --target web
```

## Calling WASM from JavaScript

```javascript
import init, { add, fibonacci } from "./pkg/wasm_example.js";

async function main() {
  await init();
  console.log(add(2, 3));       // 5
  console.log(fibonacci(10));   // 55
}
main();
```

## WASM vs JavaScript Performance

| Task | JS | WASM | Speedup |
|------|----|------|---------|
| Fibonacci(40) | 1.2s | 0.3s | 4x |
| Mandelbrot | 850ms | 210ms | 4x |
| Image blur | 320ms | 45ms | 7x |

## Limitations

WASM cannot directly access the DOM. It must call JavaScript functions to manipulate the page. The WASM-GC proposal aims to fix this, but for now you need a bridge layer.

## The Future

With WASI (WebAssembly System Interface), WASM is expanding beyond the browser into server-side runtimes and edge computing. The component model will enable composable WASM modules.

WebAssembly is not a replacement for JavaScript 鈥?it is a complement that handles performance-critical workloads.

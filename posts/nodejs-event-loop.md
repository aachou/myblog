+++
title = "The Node.js Event Loop in Practice"
date = "2023-10-14"
tags = ["nodejs", "event-loop", "async"]
excerpt = "Understand the phases of the Node.js event loop, how microtasks and macrotasks interact, and practical patterns to avoid blocking the loop."
+++

The event loop is the heart of Node.js. It enables non-blocking I/O by offloading operations to the system kernel whenever possible. Understanding its phases helps you write performant, predictable asynchronous code.

## The Six Phases

The event loop operates in a cycle of phases, each with its own FIFO callback queue:

```text
   ┌───────────────────────────┐
┌─>│           timers          │
│  └─────────────┬─────────────┘
│  ┌─────────────┴─────────────┐
│  │     pending callbacks     │
│  └─────────────┬─────────────┘
│  ┌─────────────┴─────────────┐
│  │       idle, prepare       │
│  └─────────────┬─────────────┘
│  ┌─────────────┴─────────────┐
│  │           poll            │
│  └─────────────┬─────────────┘
│  ┌─────────────┴─────────────┐
│  │           check           │
│  └─────────────┬─────────────┘
│  ┌─────────────┴─────────────┐
│  │      close callbacks      │
│  └───────────────────────────┘
└───────────────────────────────┘
```

| Phase | What runs here |
|-------|----------------|
| **timers** | `setTimeout()`, `setInterval()` callbacks |
| **pending** | I/O callbacks deferred to next iteration |
| **idle/prepare** | Internal use |
| **poll** | Retrieve new I/O events, execute I/O callbacks |
| **check** | `setImmediate()` callbacks |
| **close** | Close event callbacks (e.g., `socket.on("close")`) |

## Microtasks vs Macrotasks

Microtasks (Promise callbacks, `queueMicrotask`) run between each phase, not within a single phase:

```javascript
console.log("1: sync");

setTimeout(() => console.log("2: macrotask"), 0);

Promise.resolve().then(() => console.log("3: microtask"));

queueMicrotask(() => console.log("4: microtask"));

setImmediate(() => console.log("5: check phase"));

console.log("6: sync");

// Output: 1, 6, 3, 4, 2, 5
```

## Blocking the Event Loop

CPU-intensive operations block all phases:

```javascript
// BAD — blocks the event loop for 5 seconds
const start = Date.now();
while (Date.now() - start < 5000) {
    // busy wait
}
console.log("done");
```

Use `setImmediate()` to yield control back to the loop, or offload work to worker threads:

```javascript
// GOOD — yields to the event loop between chunks
function processLargeArray(arr) {
    if (arr.length === 0) return;

    const chunk = arr.splice(0, 50);
    chunk.forEach(item => heavyOperation(item));

    setImmediate(() => processLargeArray(arr));
}
```

## Worker Threads for CPU Work

For truly parallel computation, use worker threads:

```javascript
const { Worker } = require("worker_threads");

function runWorker(filename, data) {
    return new Promise((resolve, reject) => {
        const worker = new Worker(filename, { workerData: data });
        worker.on("message", resolve);
        worker.on("error", reject);
    });
}

// main.js
runWorker("./heavy-compute.js", largeDataset)
    .then(result => console.log(result));
```

## Common Anti-Patterns

```javascript
// Anti-pattern: Nested promises (promise chaining is fine)
doSomething().then(() => {
    doSomethingElse().then(() => {
        // too deep
    });
});

// Better: async/await
async function handle() {
    await doSomething();
    await doSomethingElse();
}
```

## Measuring Event Loop Lag

Monitor loop health:

```javascript
function monitorLag(threshold = 50) {
    const start = Date.now();
    setImmediate(() => {
        const lag = Date.now() - start;
        if (lag > threshold) {
            console.warn(`Event loop lag: ${lag}ms`);
        }
        monitorLag(threshold);
    });
}
```

The event loop is not a mysterious black box. By understanding its phases and respecting the microtask/macrotask distinction, you can write Node.js applications that remain responsive under load and behave predictably in production.

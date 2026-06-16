+++
title = "Python 异步编程入门：从回调到协程"
date = "2024-06-29"
tags = ["python", "async", "chinese"]
excerpt = "介绍 Python 异步编程的发展历程和核心概念，从回调函数到 async/await 语法。"
+++

Python 的异步编程经历了几次重要的演进，从最初的回调函数到现在的 async/await 语法。

## 协程基础

```python
import asyncio

async def fetch_data(url):
    print(f"Fetching {url}")
    await asyncio.sleep(1)
    return {"data": "example"}

async def main():
    result = await fetch_data("https://api.example.com")
    print(result)

asyncio.run(main())
```

## 实际应用

异步编程在 Web 开发、爬虫和微服务中广泛应用，可以显著提高 I/O 密集型应用的性能。

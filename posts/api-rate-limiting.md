+++
title = "API Rate Limiting 实现方案"
date = "2024-07-20"
tags = ["api", "security", "backend", "redis", "chinese"]
excerpt = "Implementing rate limiting for APIs using token bucket, sliding window, and Redis-based distributed limiting."
+++

API 限流是保障系统稳定性的重要手段。本文对比令牌桶、滑动窗口和漏桶三种经典算法，并演示如何基于 Redis 构建分布式限流方案，有效抵御流量突增与恶意请求。

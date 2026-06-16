+++
title = "System Design Interview: A Structured Approach"
date = "2025-11-07"
tags = ["system-design", "interviews", "architecture"]
excerpt = "Ace the system design interview with a repeatable framework that covers requirements, estimation, data model, and high-level design."
+++

System design interviews evaluate your ability to architect large-scale distributed systems. The key is a structured, repeatable approach.

## The Framework

1. **Clarify requirements** (5 min)
2. **Estimate scale** (5 min)
3. **Data model** (5 min)
4. **High-level design** (10 min)
5. **Deep dive** (15 min)
6. **Wrap-up** (5 min)

## Step 1: Clarify Requirements

Ask questions to scope the problem:

- What are the core features?
- What is the read/write ratio?
- What are the latency requirements?
- How many users? What growth rate?
- What consistency model is needed?

For a URL shortener:

```
Functional: generate short URLs, redirect
Non-functional: 99.9% uptime, <100ms redirect
DAU: 100 million
Write: 100 new URLs/second
Read: 10,000 redirects/second
```

## Step 2: Scale Estimation

```python
# Traffic estimates
writes_per_second = 100  # new URLs
reads_per_second = 10000  # redirects
storage_per_year = writes_per_second * 86400 * 365 * 500  # bytes

# Cache calculation
cache_size = reads_per_second * 3600 * 500  # 1 hour of 500B entries
```

## Step 3: Data Model

```
URLs
  id: bigint (primary key)
  short_code: varchar(7) (unique index)
  original_url: text
  created_at: timestamp
  user_id: bigint (foreign key)

Users
  id: bigint (primary key)
  email: varchar(255)
  created_at: timestamp
```

## Step 4: High-Level Design

```mermaid
Client -> Load Balancer -> Web Servers -> Cache -> Database
                                    -> Queue -> Analytics
```

Key components:

- **Load balancer**: distribute traffic
- **Web servers**: stateless, horizontally scalable
- **Cache**: Redis for hot URLs
- **Database**: primary + replicas
- **Queue**: async analytics processing

## Step 5: Deep Dive Topics

| Topic | Consideration |
|-------|---------------|
| Database sharding | Hash-based by short_code |
| Caching strategy | Write-through, LRU eviction |
| Rate limiting | Token bucket per user/IP |
| ID generation | Snowflake-style unique IDs |
| Analytics | Stream processing with Kafka |

## Database Sharding

```python
def get_shard(short_code: str) -> int:
    return hash(short_code) % NUM_SHARDS
```

Use consistent hashing to minimize rebalancing.

## Caching Strategy

Cache popular URLs in Redis. Use a write-through cache so misses are rare. Set TTL based on access patterns:

```python
cache.set(short_code, original_url, ttl=3600)  # 1 hour
```

## Rate Limiting

```python
class TokenBucket:
    def __init__(self, rate, capacity):
        self.rate = rate
        self.capacity = capacity
        self.tokens = capacity

    def allow_request(self):
        now = time.time()
        self.tokens += (now - self.last_refill) * self.rate
        self.tokens = min(self.tokens, self.capacity)
        if self.tokens >= 1:
            self.tokens -= 1
            return True
        return False
```

## Step 6: Wrap Up

Summarize the design and mention trade-offs. Discuss potential improvements like CDN integration, multi-region deployment, and monitoring.

Practice the framework until it becomes automatic. A clear structure is more important than a perfect design.

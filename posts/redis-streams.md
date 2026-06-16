+++
title = "Redis Streams: Building Event-Driven Architectures"
date = "2025-03-10"
tags = ["redis", "streaming", "event-driven"]
excerpt = "Redis Streams offer a powerful log-based data structure for event sourcing, message queues, and real-time data processing."
+++

Redis Streams, introduced in Redis 5.0, provide an append-only log structure that is ideal for event-driven applications. Unlike pub/sub, streams persist messages and support consumer groups.

## The Stream Data Structure

A stream is a sequence of entries, each with a unique ID and a set of field-value pairs:

```bash
XADD mystream * sensor-id 1234 temperature 19.8
XADD mystream * sensor-id 1235 temperature 21.3
```

The `*` tells Redis to auto-generate an ID based on the current timestamp.

## Reading from Streams

You can read ranges, follow new entries, or consume from a specific point:

```bash
XRANGE mystream - + COUNT 10
XREAD COUNT 2 STREAMS mystream 0
```

## Consumer Groups

Consumer groups enable distributed processing with automatic load balancing:

```bash
XGROUP CREATE mystream mygroup $
XREADGROUP GROUP mygroup consumer1 COUNT 1 STREAMS mystream >
```

The `>` symbol tells Redis to deliver only messages not yet sent to any consumer.

## Acknowledging Messages

After processing, consumers acknowledge messages to prevent redelivery:

```python
import redis

r = redis.Redis()
r.xack('mystream', 'mygroup', message_id)
```

## The Pending Entries List (PEL)

Messages delivered but not yet acknowledged remain in the PEL. This enables:

- Failure recovery: reprocess messages from crashed consumers
- Monitoring: detect slow or stuck consumers
- Exactly-once semantics: track delivery state

```bash
XPENDING mystream mygroup
```

## Key Differences from Pub/Sub

| Feature | Pub/Sub | Streams |
|---------|---------|---------|
| Message persistence | No | Yes |
| Consumer groups | No | Yes |
| Message replay | No | Yes |
| Backpressure | No | Via consumer groups |
| At-least-once delivery | No | Yes |

## Practical Use Case: Activity Feed

```python
# Producer
r.xadd('user:feed', {'user_id': 42, 'action': 'liked_post', 'post_id': 99})

# Consumer with pagination
entries = r.xrevrange('user:feed', count=20)
for entry_id, fields in entries:
    print(fields)
```

## Stream Trimming

Prevent unbounded memory growth with capped streams:

```bash
XADD mystream MAXLEN ~ 10000 * field value
```

The `~` allows Redis to trim efficiently rather than exactly.

## Performance Characteristics

Streams are backed by radix trees, providing O(log N) insertion and lookup. With consumer groups, Redis Streams can handle millions of messages per second on commodity hardware.

## When to Choose Streams Over Kafka

Redis Streams excel when you need a lightweight, embedded stream processor with minimal operational overhead. They are ideal for small to medium workloads where Kafka would be overkill.

Redis Streams bridge the gap between simple pub/sub and full-fledged event streaming platforms.

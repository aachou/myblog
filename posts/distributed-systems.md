+++
title = "Distributed Systems"
date = "2022-08-08"
tags = ["distributed-systems", "architecture", "scalability"]
excerpt = "Core concepts in distributed systems: CAP theorem, consensus algorithms, replication, and failure modes every engineer should know."
+++

Distributed systems are collections of independent computers that appear to the user as a single coherent system. Building them is hard because parts fail independently and messages can be lost.

## CAP Theorem

The CAP theorem states that a distributed data store can only provide two of three guarantees:

- **Consistency** 鈥?every read returns the most recent write
- **Availability** 鈥?every request receives a response
- **Partition tolerance** 鈥?the system continues operating despite network failures

```text
        Consistency
            |
      CP ---+--- AP
            |
         Partition
        Tolerance
```

In practice, partitions are inevitable, so you choose between CP and AP.

## Consensus Algorithms

Consensus is the problem of getting multiple nodes to agree on a value. Paxos and Raft are the most well-known protocols:

```python
# Simplified Raft leader election
class RaftNode:
    def __init__(self, node_id):
        self.id = node_id
        self.state = "follower"
        self.current_term = 0
        self.voted_for = None

    def start_election(self):
        self.state = "candidate"
        self.current_term += 1
        self.voted_for = self.id
        votes = 1
        # request votes from other nodes...
        if votes > len(cluster) // 2:
            self.state = "leader"
```

## Replication Strategies

| Strategy | Durability | Latency | Use Case |
|----------|------------|---------|----------|
| Synchronous | High | High | Financial systems |
| Asynchronous | Low | Low | Analytics |
| Quorum-based | Medium | Medium | General purpose |

## Failure Modes

Systems fail in interesting ways:

- **Crash-stop** 鈥?node stops responding permanently
- **Crash-recover** 鈥?node comes back after a failure
- **Byzantine** 鈥?node behaves arbitrarily (malicious)

## Time and Ordering

There is no global clock in a distributed system. Logical clocks (Lamport, vector clocks) provide a way to order events:

```python
class LamportClock:
    def __init__(self):
        self.time = 0

    def tick(self):
        self.time += 1
        return self.time

    def update(self, other_time):
        self.time = max(self.time, other_time) + 1
```

## Practical Advice

Start with a monolith. Only split into services when you understand the boundaries. Use idempotent operations, retry with exponential backoff, and design for failure.

Distributed systems are inherently complex 鈥?embrace that complexity with careful design and thorough testing.

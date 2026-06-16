+++
title = "TCP vs UDP: Choosing the Right Transport Protocol"
date = "2025-12-01"
tags = ["networking", "protocols", "fundamentals"]
excerpt = "TCP and UDP are the foundational transport layer protocols. Understand their differences to make informed architectural decisions."
+++

TCP and UDP are the two primary transport protocols in the Internet protocol suite. Each serves fundamentally different use cases.

## TCP: Transmission Control Protocol

TCP is connection-oriented and guarantees reliable, ordered delivery:

- Three-way handshake establishes a connection
- Sequence numbers track packet order
- Acknowledgments confirm receipt
- Retransmission handles lost packets
- Flow control prevents overwhelming receivers
- Congestion control manages network load

```python
import socket

# TCP server
server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
server.bind(('localhost', 8080))
server.listen(5)

while True:
    client, addr = server.accept()
    data = client.recv(1024)
    client.send(b"HTTP/1.1 200 OK\r\n\r\nHello")
    client.close()
```

## UDP: User Datagram Protocol

UDP is connectionless and provides best-effort delivery:

- No handshake required
- No guaranteed delivery
- No ordering guarantees
- No congestion control
- Lower overhead and latency

```python
import socket

# UDP server
server = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
server.bind(('localhost', 8080))

while True:
    data, addr = server.recvfrom(1024)
    server.sendto(b"Hello", addr)
```

## Protocol Comparison

| Feature | TCP | UDP |
|---------|-----|-----|
| Connection | Connection-oriented | Connectionless |
| Reliability | Guaranteed delivery | Best-effort |
| Ordering | Ordered | Unordered |
| Headers | 20-60 bytes | 8 bytes |
| Speed | Slower (overhead) | Faster |
| Use cases | Web, email, file transfer | Streaming, gaming, DNS |

## The Three-Way Handshake

```
Client -> SYN        -> Server
Client <- SYN-ACK    <- Server
Client -> ACK        -> Server
```

This adds one round trip of latency before data transfer begins.

## When to Use TCP

- File transfers: HTTP, FTP, SMTP
- Database connections
- Message queues
- Any application where data integrity is critical

## When to Use UDP

- Real-time streaming: video calls, voice
- Online gaming (fast-paced)
- DNS queries
- IoT sensor data
- Network discovery protocols

## QUIC: The Modern Compromise

Google's QUIC protocol (HTTP/3) combines TCP-like reliability with UDP-like low latency:

```
QUIC over UDP
鈹溾攢鈹€ Built-in TLS 1.3
鈹溾攢鈹€ 0-RTT connection establishment
鈹溾攢鈹€ Multiplexed streams (no head-of-line blocking)
鈹斺攢鈹€ Connection migration
```

## Real-World Example: Video Streaming

```python
# Live streaming: prefer UDP
# Packet loss = brief glitch, not full buffering

# On-demand streaming: can use TCP
# Buffering compensates for retransmission delays
```

## Head-of-Line Blocking

TCP guarantees order, but a lost packet blocks delivery of subsequent packets until retransmission. This is fatal for real-time apps but acceptable for file transfers.

## Connection Overhead

TCP maintains connection state in the kernel: send/receive buffers, congestion window, sequence numbers. A server with 1 million TCP connections uses significant memory. UDP is stateless and handles more connections per server.

The choice between TCP and UDP ultimately depends on your tolerance for data loss versus your tolerance for latency.

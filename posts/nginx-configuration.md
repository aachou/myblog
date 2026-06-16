+++
title = "Production-Ready Nginx Configuration Patterns"
date = "2023-09-01"
tags = ["nginx", "web-server", "devops"]
excerpt = "Learn battle-tested Nginx configuration patterns including reverse proxying, caching, rate limiting, SSL termination, and load balancing for production deployments."
+++

Nginx powers over 30% of all websites. Its event-driven architecture handles thousands of concurrent connections with minimal resource usage. This guide covers configuration patterns you will actually use in production.

## Basic Server Block

Every Nginx config starts with a server block that defines what to do with incoming requests:

```nginx
server {
    listen 80;
    server_name example.com www.example.com;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name example.com;

    ssl_certificate /etc/ssl/certs/example.com.pem;
    ssl_certificate_key /etc/ssl/private/example.com.key;

    root /var/www/example.com/public;
    index index.html;

    location / {
        try_files $uri $uri/ /index.html;
    }
}
```

## Reverse Proxy

Forward requests to backend services:

```nginx
location /api/ {
    proxy_pass http://backend:8080;
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;

    proxy_read_timeout 60s;
    proxy_connect_timeout 10s;
}
```

## Caching Static Assets

Nginx excels at serving static files with aggressive caching:

```nginx
location ~* \.(jpg|jpeg|png|gif|ico|css|js|woff2)$ {
    expires 365d;
    add_header Cache-Control "public, immutable";
    access_log off;

    # Open file cache
    open_file_cache max=1000 inactive=20s;
    open_file_cache_valid 30s;
    open_file_cache_min_uses 2;
    open_file_cache_errors on;
}
```

## Rate Limiting

Protect your backend from abuse:

```nginx
# Define a rate limit zone
limit_req_zone $binary_remote_addr zone=api:10m rate=10r/s;

server {
    location /api/ {
        limit_req zone=api burst=20 nodelay;
        proxy_pass http://backend:8080;
    }

    # Stricter for auth endpoints
    location /api/auth/ {
        limit_req zone=auth burst=5 nodelay;
        proxy_pass http://backend:8080;
    }
}
```

| Directive | Purpose |
|-----------|---------|
| `limit_req_zone` | Define shared memory zone and rate |
| `limit_req` | Apply rate limiting to a location |
| `burst` | Allow temporary bursts above the rate |
| `nodelay` | Process burst immediately (no queuing delay) |

## Load Balancing

Distribute traffic across multiple upstream servers:

```nginx
upstream backend_servers {
    least_conn;
    server app1:8080 weight=3;
    server app2:8080 weight=2;
    server app3:8080 backup;

    keepalive 32;
}

server {
    location / {
        proxy_pass http://backend_servers;
        proxy_http_version 1.1;
        proxy_set_header Connection "";
    }
}
```

## Security Headers

Hardening Nginx with security-related headers:

```nginx
server {
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Strict-Transport-Security "max-age=63072000" always;
    add_header Referrer-Policy "strict-origin-when-cross-origin" always;
    add_header Permissions-Policy "camera=(), microphone=(), geolocation=()" always;
}
```

## Logging Format

Customize log output for analysis:

```nginx
log_format json escape=json
    '{'
        '"time":"$time_local",'
        '"remote_addr":"$remote_addr",'
        '"request":"$request",'
        '"status":$status,'
        '"body_bytes":$body_bytes_sent,'
        '"request_time":$request_time,'
        '"upstream_addr":"$upstream_addr",'
        '"upstream_time":"$upstream_response_time"'
    '}';

access_log /var/log/nginx/access.log json;
```

## Gzip Compression

Compress text-based responses:

```nginx
gzip on;
gzip_vary on;
gzip_proxied any;
gzip_comp_level 6;
gzip_types text/plain text/css application/json application/javascript text/xml application/xml text/javascript;
gzip_min_length 256;
```

Nginx performance tuning is an iterative process. Measure baseline metrics, apply one change at a time, and validate with tools like `ab` or `wrk`. Properly configured Nginx can easily handle tens of thousands of concurrent connections.

+++
title = "Web Performance Optimization: Practical Tips"
date = "2024-06-12"
tags = ["web", "performance", "optimization"]
excerpt = "Practical techniques for improving web application performance including caching strategies, asset optimization, and rendering best practices."
+++

Performance is not just a feature 鈥?it's a fundamental aspect of user experience. Here are practical tips to make your web apps faster.

## 1. Optimize Images

Images are often the biggest assets on a page:

- Use **WebP** format instead of JPEG/PNG (30% smaller on average)
- Implement **lazy loading** with `loading="lazy"`
- Serve responsive images using `srcset`

```html
<img
  src="photo-800w.webp"
  srcset="photo-400w.webp 400w, photo-800w.webp 800w, photo-1200w.webp 1200w"
  sizes="(max-width: 600px) 400px, (max-width: 1200px) 800px, 1200px"
  loading="lazy"
  alt="Description"
>
```

## 2. Leverage Caching

### Browser Caching

```nginx
# Nginx configuration
location /static/ {
    expires 1y;
    add_header Cache-Control "public, immutable";
}
```

### HTTP Caching Headers

| Header            | Purpose                              |
|-------------------|--------------------------------------|
| `Cache-Control`   | Directives for caching behavior      |
| `ETag`            | Version identifier for resources     |
| `Last-Modified`   | Timestamp of last modification       |
| `Expires`         | Absolute expiration time             |

## 3. Minimize JavaScript

```javascript
// 鉂?Bad: Blocking script
<script src="large-library.js"></script>

// 鉁?Good: Async loading
<script async src="analytics.js"></script>

// 鉁?Better: Defer non-critical scripts
<script defer src="app.js"></script>
```

## 4. Critical CSS

Inline critical CSS in the `<head>` to eliminate render-blocking:

```html
<head>
  <style>
    /* Critical above-the-fold styles */
    body { font-family: system-ui, sans-serif; margin: 0; }
    header { padding: 1rem; background: #f8f9fa; }
    .hero { min-height: 60vh; display: flex; align-items: center; }
  </style>
  <link rel="stylesheet" href="/styles/full.css" media="print" onload="this.media='all'">
</head>
```

## 5. Use CDN and Compression

```bash
# Enable Brotli compression (Nginx)
brotli on;
brotli_types text/html text/css application/javascript image/svg+xml;

# Enable gzip as fallback
gzip on;
gzip_types text/html text/css application/javascript;
```

## 6. Database Query Optimization

```sql
-- 鉂?N+1 query problem
SELECT * FROM posts; -- 1 query
-- Then for each post:
SELECT * FROM comments WHERE post_id = ?; -- N queries

-- 鉁?Use JOIN
SELECT p.*, c.*
FROM posts p
LEFT JOIN comments c ON c.post_id = p.id;

-- 鉁?Or batch with IN
SELECT * FROM comments WHERE post_id IN (1, 2, 3, ...);
```

## 7. Measure What Matters

Tools to use:

- **Lighthouse** 鈥?Overall performance audit
- **WebPageTest** 鈥?Detailed waterfall analysis
- **BundlePhobia** 鈥?Check npm package costs

> *"Performance is the鐢ㄦ埛浣撻獙 (user experience) that is most often overlooked, but it's the one that matters the most."* 鈥?Adapted from various sources

### Key Metrics

| Metric           | Good      | Needs Work | Poor      |
|-----------------|-----------|------------|-----------|
| First Contentful Paint | < 1.8s | 1.8s鈥?.0s | > 3.0s |
| Largest Contentful Paint | < 2.5s | 2.5s鈥?.0s | > 4.0s |
| First Input Delay | < 100ms | 100ms鈥?00ms | > 300ms |
| Cumulative Layout Shift | < 0.1 | 0.1鈥?.25 | > 0.25 |

Start measuring today and iterate!

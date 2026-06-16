+++
title = "Database Design Patterns for Modern Applications"
date = "2024-06-06"
tags = ["database", "architecture", "backend"]
excerpt = "Essential database design patterns including indexing strategies, normalization vs denormalization, and query optimization techniques."
+++

Good database design is crucial for application performance and maintainability.

## Normalization vs Denormalization

**Normalization** reduces data redundancy:

```sql
-- 3NF: Separate authors table
CREATE TABLE authors (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    author_id INTEGER REFERENCES authors(id),
    title TEXT NOT NULL,
    body TEXT
);
```

**Denormalization** improves read performance:

```sql
-- Denormalized: Embed author name
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    author_name TEXT NOT NULL,
    title TEXT NOT NULL,
    body TEXT
);
```

## Indexing Strategies

```sql
-- B-tree index for general purpose
CREATE INDEX idx_posts_created_at ON posts(created_at);

-- Composite index for common queries
CREATE INDEX idx_posts_author_status ON posts(author_id, status);

-- Partial index for filtered queries
CREATE INDEX idx_active_posts ON posts(created_at) WHERE status = 'active';

-- Covering index (all needed columns)
CREATE INDEX idx_post_list ON posts(author_id, created_at DESC, title);
```

## Connection Pooling

```rust
use sqlx::postgres::PgPoolOptions;

let pool = PgPoolOptions::new()
    .max_connections(20)
    .connect("postgres://user:pass@localhost/db")
    .await?;
```

## Migrations

```rust
// Always use migrations for schema changes
sqlx::migrate!("./migrations").run(&pool).await?;
```

## Query Optimization

- Use `EXPLAIN ANALYZE` to understand query plans
- Avoid `SELECT *` in production
- Use pagination with keyset pagination (WHERE id > ?) instead of OFFSET
- Batch inserts with `INSERT INTO ... VALUES (...), (...), ...`

> *"A well-designed database is worth more than all the caching layers in the world."*

+++
title = "REST API Versioning Strategies: A Practical Comparison"
date = "2025-05-12"
tags = ["api", "rest", "backend"]
excerpt = "Compare URI, header, and query parameter versioning approaches for REST APIs and learn which strategy fits your use case."
+++

API versioning is inevitable. Once your API is in production, consumers depend on its contract. Breaking changes require a versioning strategy.

## URI Versioning

The most common approach embeds the version in the URL path:

```
GET /api/v1/users
GET /api/v2/users
```

**Pros**: Explicit, cacheable, easy to route, simple to implement.

**Cons**: Clutters URLs, violates REST purity (resource identity differs by version).

```python
@router.get("/api/v1/users")
def get_users_v1():
    return [{"id": 1, "name": "Alice"}]

@router.get("/api/v2/users")
def get_users_v2():
    return [{"id": 1, "name": "Alice", "email": "alice@example.com"}]
```

## Header Versioning

Version information lives in the `Accept` header:

```
GET /api/users
Accept: application/vnd.myapp.v1+json
```

**Pros**: Clean URLs, follows REST best practices, separates concerns.

**Cons**: Harder to test in browsers, requires client cooperation, less visible.

```python
@router.get("/api/users")
def get_users(version: str = Header("v1")):
    if version == "v1":
        return users_v1()
    return users_v2()
```

## Query Parameter Versioning

The version is a query parameter:

```
GET /api/users?version=1
GET /api/users?version=2
```

**Pros**: Simple to implement and test, cacheable.

**Cons**: Pollutes query string, easy to forget, ambiguous with other params.

## Strategy Comparison

| Strategy | Visibility | Difficulty | Cacheability | REST Purity |
|----------|------------|------------|--------------|-------------|
| URI path | High | Low | Excellent | Low |
| Header | Low | Medium | Good | High |
| Query param | Medium | Low | Excellent | Low |
| Content type | Low | High | Good | High |

## Handling Breaking Changes

Not every change needs a new version. Additive changes (new fields, new endpoints) are backward-compatible. Breaking changes include:

- Removing fields or endpoints
- Changing field types
- Changing validation rules
- Altering response structure

## Sunset Policies

Always communicate deprecation clearly:

```json
{
    "Sunset": "Sat, 31 Dec 2025 23:59:59 GMT",
    "Deprecation": "true"
}

```
Include a link to migration docs in response headers.

## API Gateway Routing

With microservices, API gateways handle version routing:

```yaml
routes:
  - path: /api/v1/users
    service: user-service-v1
  - path: /api/v2/users
    service: user-service-v2
```

This lets you deploy old and new versions side by side.

## When to Cut a Version

- When removing a field that clients depend on
- When changing the semantics of an existing endpoint
- When the request or response format fundamentally changes

Internal refactors and additive changes should not trigger a version bump. Use OpenAPI specs to document changes and communicate clearly with consumers.

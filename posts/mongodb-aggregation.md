+++
title = "Advanced MongoDB Aggregation Pipeline Patterns"
date = "2023-07-12"
tags = ["mongodb", "database", "data-engineering"]
excerpt = "The MongoDB aggregation pipeline is a powerful data processing framework. Master stages like $lookup, $group, $unwind, and $facet to build complex analytics queries."
+++

MongoDB's aggregation pipeline processes documents through a series of stages, each transforming the data before passing it to the next. This composable design lets you build everything from simple filters to multi-dimensional analytics.

## Pipeline Basics

A pipeline is an array of stages executed in sequence:

```javascript
db.orders.aggregate([
    { $match: { status: "shipped" } },
    { $group: {
        _id: "$customerId",
        totalSpent: { $sum: "$amount" },
        orderCount: { $sum: 1 }
    }},
    { $sort: { totalSpent: -1 } },
    { $limit: 10 }
])
```

| Stage | Purpose |
|-------|---------|
| `$match` | Filter documents (like WHERE) |
| `$project` | Reshape documents (like SELECT) |
| `$group` | Aggregate by a key |
| `$sort` | Order results |
| `$limit` | Cap result count |

## Joining Collections with $lookup

The `$lookup` stage performs a left outer join with another collection:

```javascript
db.orders.aggregate([
    { $match: { date: { $gte: ISODate("2023-01-01") } } },
    { $lookup: {
        from: "customers",
        localField: "customerId",
        foreignField: "_id",
        as: "customer"
    }},
    { $unwind: "$customer" },
    { $project: {
        orderId: 1,
        total: 1,
        "customer.name": 1,
        "customer.email": 1
    }}
])
```

## Unwinding Arrays

`$unwind` deconstructs an array field into multiple documents:

```javascript
// Input: { _id: 1, items: ["a", "b", "c"] }
db.collection.aggregate([
    { $unwind: "$items" }
])
// Output: { _id: 1, items: "a" }
//         { _id: 1, items: "b" }
//         { _id: 1, items: "c" }
```

## Multi-Faceted Analytics with $facet

`$facet` processes multiple pipelines within a single stage:

```javascript
db.products.aggregate([
    { $facet: {
        categories: [
            { $group: { _id: "$category", count: { $sum: 1 } } }
        ],
        priceRanges: [
            { $bucket: {
                groupBy: "$price",
                boundaries: [0, 10, 50, 100, 500, 1000],
                default: "Other",
                output: { count: { $sum: 1 } }
            }}
        ],
        stats: [
            { $group: {
                _id: null,
                avgPrice: { $avg: "$price" },
                maxPrice: { $max: "$price" },
                totalProducts: { $sum: 1 }
            }}
        ]
    }}
])
```

## Window Functions with $setWindowFields

Introduced in MongoDB 5.0, `$setWindowFields` enables rank, moving averages, and cumulative sums:

```javascript
db.sales.aggregate([
    { $setWindowFields: {
        partitionBy: "$productId",
        sortBy: { date: 1 },
        output: {
            runningTotal: {
                $sum: "$amount",
                window: { documents: ["unbounded", "current"] }
            }
        }
    }}
])
```

## Performance Tips

- Place `$match` and `$limit` early to reduce documents as soon as possible.
- Use indexes on fields referenced in `$match` and `$sort`.
- Avoid `$unwind` on large arrays unless necessary — it multiplies documents.
- Use `$lookup` with `let` + `pipeline` for correlated subqueries instead of foreignField.

```javascript
// Optimized $lookup with pipeline
db.orders.aggregate([
    { $lookup: {
        from: "reviews",
        let: { productId: "$productId" },
        pipeline: [
            { $match: { $expr: { $eq: ["$productId", "$$productId"] } } },
            { $limit: 5 },
            { $sort: { createdAt: -1 } }
        ],
        as: "recentReviews"
    }}
])
```

The aggregation pipeline is MongoDB's superpower. It moves computation to the database, reducing network round-trips and enabling real-time analytics without external tools.

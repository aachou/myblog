+++
title = "SQL Joins Explained: From INNER to FULL OUTER"
date = "2025-09-14"
tags = ["sql", "database", "beginners"]
excerpt = "Master SQL joins with clear explanations and examples of INNER, LEFT, RIGHT, FULL OUTER, CROSS, and SELF joins."
+++

SQL joins combine rows from two or more tables based on related columns. Understanding joins is essential for working with relational databases.

## Setting Up Sample Data

```sql
CREATE TABLE customers (
    id INT PRIMARY KEY,
    name VARCHAR(50)
);

CREATE TABLE orders (
    id INT PRIMARY KEY,
    customer_id INT,
    total DECIMAL(10,2)
);

INSERT INTO customers VALUES
(1, 'Alice'), (2, 'Bob'), (3, 'Charlie');

INSERT INTO orders VALUES
(101, 1, 50.00), (102, 1, 75.00), (103, 2, 30.00);
```

## INNER JOIN

Returns only rows with matching values in both tables:

```sql
SELECT c.name, o.total
FROM customers c
INNER JOIN orders o ON c.id = o.customer_id;
```

| name | total |
|------|-------|
| Alice | 50.00 |
| Alice | 75.00 |
| Bob | 30.00 |

## LEFT JOIN (LEFT OUTER JOIN)

Returns all rows from the left table and matched rows from the right:

```sql
SELECT c.name, o.total
FROM customers c
LEFT JOIN orders o ON c.id = o.customer_id;
```

Charlie appears with NULL because he has no orders.

## RIGHT JOIN (RIGHT OUTER JOIN)

The mirror of LEFT JOIN. Returns all rows from the right table:

```sql
SELECT c.name, o.total
FROM customers c
RIGHT JOIN orders o ON c.id = o.customer_id;
```

This is less common because you can usually rewrite it as a LEFT JOIN.

## FULL OUTER JOIN

Returns all rows when there is a match in either table:

```sql
SELECT c.name, o.total
FROM customers c
FULL OUTER JOIN orders o ON c.id = o.customer_id;
```

## Join Type Comparison

| Join Type | Left Table | Right Table |
|-----------|------------|-------------|
| INNER | Only matches | Only matches |
| LEFT | All rows | Only matches |
| RIGHT | Only matches | All rows |
| FULL OUTER | All rows | All rows |
| CROSS | Every row | Every row |

## CROSS JOIN

Produces a Cartesian product of both tables:

```sql
SELECT c.name, p.product_name
FROM customers c
CROSS JOIN products p;
```

Useful for generating combinations but dangerous on large tables.

## SELF JOIN

Joining a table to itself, useful for hierarchical data:

```sql
SELECT e.name AS employee, m.name AS manager
FROM employees e
LEFT JOIN employees m ON e.manager_id = m.id;
```

## JOIN with Multiple Conditions

```sql
SELECT *
FROM orders o
JOIN shipments s
    ON o.id = s.order_id
    AND s.status = 'delivered';
```

## Performance Considerations

Always put indexes on join columns. Use `EXPLAIN` to verify query plans:

```sql
EXPLAIN SELECT * FROM orders o
JOIN customers c ON o.customer_id = c.id;
```

Nested loop joins work well for small datasets. Hash joins are better for large, unsorted data. Merge joins excel when both inputs are sorted.

Joins are the superpower of relational databases. Master them and SQL becomes intuitive.

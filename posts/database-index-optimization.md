+++
title = "数据库索引优化：提升查询性能的实用技巧"
date = "2024-07-11"
tags = ["database", "performance", "chinese"]
excerpt = "深入讲解数据库索引的工作原理和优化策略，帮助开发者写出高效的查询语句。"
+++

索引是数据库性能调优中最有效的工具之一。合理使用索引可以大幅提升查询速度。

## 索引类型

- B-Tree 索引：最常用，适合范围查询
- Hash 索引：适合等值查询
- 全文索引：适合文本搜索
- 复合索引：多列组合查询

## 优化原则

```sql
-- 为经常查询的列创建索引
CREATE INDEX idx_user_email ON users(email);

-- 复合索引要注意列顺序
CREATE INDEX idx_order_date_user ON orders(user_id, created_at);
```

## 注意事项

索引虽然能加速查询，但也会降低写入性能，需要权衡使用。

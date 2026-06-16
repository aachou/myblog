+++
title = "Docker Compose 实战：多服务部署指南"
date = "2024-06-23"
tags = ["docker", "devops", "chinese"]
excerpt = "学习如何使用 Docker Compose 编排多个容器服务，实现高效的本地开发和部署流程。"
+++

Docker Compose 是定义和运行多容器 Docker 应用的工具。通过一个 YAML 文件，你可以配置应用所需的所有服务。

## 基本配置

```yaml
version: '3.8'
services:
  web:
    build: .
    ports:
      - "3000:3000"
  db:
    image: postgres:15
    environment:
      POSTGRES_DB: myapp
```

## 网络与卷

Compose 会自动创建一个网络，让所有服务可以互相通信。卷用于持久化数据。

+++
title = "Linux 服务器初始化配置最佳实践"
date = "2024-07-02"
tags = ["linux", "devops", "chinese"]
excerpt = "新服务器到手后的必要配置步骤，包括安全设置、用户管理和常用工具安装。"
+++

初始化一台新 Linux 服务器时，正确的配置可以避免后续的安全隐患。

## 基本安全设置

1. 使用 SSH 密钥登录，禁用密码登录
2. 配置防火墙，只开放必要端口
3. 启用自动安全更新
4. 设置 fail2ban 防止暴力破解

## 用户管理

```bash
# 创建新用户
adduser deploy
# 赋予 sudo 权限
usermod -aG sudo deploy
```

## 系统监控

安装 htop、iotop、netdata 等监控工具，随时掌握系统状态。

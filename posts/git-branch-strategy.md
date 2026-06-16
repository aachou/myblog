+++
title = "Git 分支策略：选择适合团队的工作流"
date = "2024-06-26"
tags = ["git", "devops", "chinese"]
excerpt = "比较主流的 Git 分支策略，包括 Git Flow、GitHub Flow 和 Trunk-Based Development，帮助团队选择合适的工作流。"
+++

选择正确的 Git 分支策略对团队的开发效率至关重要。以下是几种主流策略的对比。

## Git Flow

Git Flow 使用 master 和 develop 两个主分支，配合 feature、release 和 hotfix 分支。适合有固定发布周期的项目。

## GitHub Flow

GitHub Flow 更加简化：从 main 分支创建 feature 分支，完成后通过 Pull Request 合并。适合持续部署的项目。

## Trunk-Based Development

开发者直接向主分支提交小批量更改，配合特性开关控制功能发布。适合需要快速迭代的团队。

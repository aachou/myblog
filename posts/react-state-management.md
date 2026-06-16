+++
title = "React 状态管理方案对比：从 useState 到 Jotai"
date = "2024-07-08"
tags = ["react", "frontend", "chinese"]
excerpt = "对比 React 生态中各种状态管理方案，帮助开发者在不同场景下选择最合适的工具。"
+++

React 的状态管理生态非常丰富，从内置的 useState 到第三方库，各有适用场景。

## 内置方案

useState 适合组件局部状态，useReducer 适合复杂状态逻辑，Context 适合跨组件共享。

## 外部库

Redux 适合大型应用，Zustand 轻量灵活，Jotai 基于原子模型，Recoil 类似 Jotai 但由 Meta 维护。

## 选择建议

小型项目用 useState + Context 就足够了。随着应用增长，可以考虑 Zustand 或 Jotai 这类轻量方案。

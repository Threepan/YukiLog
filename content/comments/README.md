# 评论 (Comments)

镜像自各文章页的公开评论接口 `GET /api/public/posts/<slug>/comments`。

全站 23 篇文章逐一请求，共 **4** 条已通过审核（`approved`）的评论，分布在 **4** 篇文章上；其余 19 篇暂无评论。评论 id 为连续的 1–4，可确认无遗漏。

原始聚合数据见 [`../_api/comments.json`](../_api/comments.json)（含每篇文章的完整响应与空列表清单）。

## 全部评论

| id | 时间 (CST) | 昵称 | 内容 | 所属文章 |
| --- | --- | --- | --- | --- |
| 1 | 2026-03-22 16:24:13 | tomoko | 我学习 | [SSH 隧道 - 如何在本地访问远程内网服务?](../posts/ssh-01.md) |
| 2 | 2026-04-06 14:22:37 | [Huhu](https://huhu.tomstudio.site) | 你的博客做的好好，羡慕 | [YukiLog - 1 - 新生](../posts/yukilog-1-blog.md) |
| 3 | 2026-04-17 17:22:32 | Ariko | 封面图的画师是谁啊。 | [YukiLog - 11 - Tailwind 的故事，以及那个圆角问题](../posts/yukilog-11-tailwind.md) |
| 4 | 2026-06-25 11:07:01 | alittleshark | 喵 | [CNN-01: 神经网络是如何识别图形的?](../posts/cnn-01-how-neural-networks-recognize-shapes.md) |

## 明细

### #1 — tomoko

> 我学习

- 文章：[SSH 隧道 - 如何在本地访问远程内网服务?](../posts/ssh-01.md)（`post_id=22`）
- 时间：2026-03-22 16:24:13
- 状态：`approved`｜层级：顶层评论
- 访客环境：Desktop Edge 146.0.0.0 · Windows 10 NT 10.0

### #2 — Huhu

> 你的博客做的好好，羡慕

- 文章：[YukiLog - 1 - 新生](../posts/yukilog-1-blog.md)（`post_id=4`）
- 时间：2026-04-06 14:22:37
- 站点：<https://huhu.tomstudio.site>
- 状态：`approved`｜层级：顶层评论
- 访客环境：Desktop Edge 146.0.0.0 · Windows 10 NT 10.0

### #3 — Ariko

> 封面图的画师是谁啊。

- 文章：[YukiLog - 11 - Tailwind 的故事，以及那个圆角问题](../posts/yukilog-11-tailwind.md)（`post_id=15`）
- 时间：2026-04-17 17:22:32
- 状态：`approved`｜层级：顶层评论
- 访客环境：Mobile Edge 140.0.0.0 · Android 10

### #4 — alittleshark

> 喵

- 文章：[CNN-01: 神经网络是如何识别图形的?](../posts/cnn-01-how-neural-networks-recognize-shapes.md)（`post_id=23`）
- 时间：2026-06-25 11:07:01
- 状态：`approved`｜层级：顶层评论
- 访客环境：Desktop Edge 146.0.0.0 · Windows 10 NT 10.0

> 说明：接口同时返回了访客邮箱与 IP 等字段，已保留在 `_api/comments.json` 中以维持镜像完整性，但未在本索引里展开。

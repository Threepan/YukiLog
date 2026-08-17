---
id: 9
title: "YukiLog - 5 - Service 层"
slug: "yukilog-5-service"
summary: "从增删改查到真正的业务逻辑——这篇记录了我意识到\"系统\"需要更多东西的那个时刻"
cover_image: "https://list.yeastar.xin/d/%E6%81%8B/YukiLog/yukilog-5-service.jpg"
status: "published"
is_featured: false
theme: "博客搭建 (bkdj)"
tags: ["blog", "rust", "教程"]
view_count: 265
created_at: "2026-02-20 04:40:10"
updated_at: "2026-08-17 15:51:44"
source: "https://blog.yeastar.xin/posts/yukilog-5-service"
---

## 引言

我最早写的程序是学生管理系统。那时候对"系统"的理解很简单：增删改查，加上持久化存储，就完事了。逻辑是线性的，一个操作对应一个数据库动作，没有什么需要协调的

但 YukiLog 不一样。它需要上线到公网，需要和前端交互，需要面对各种各样的请求。当我开始设计"创建文章"这个功能的时候，我意识到它根本不是一个简单的 `INSERT`：

* 文章有标签，标签可能还不存在，需要先创建
* 文章属于某个主题，需要通过 slug 查找主题 ID
* 如果文章直接发布，主题和标签的 `post_count` 都要 +1
* 这些操作要么全部成功，要么全部回滚，不能出现中间状态
* slug 格式需要校验，不能包含非法字符

这五件事分散在四张表里，没有任何一个 Repository 函数能单独完成。我需要一个地方来放这些"跨表的协调逻辑"

这就是 Service 层自然长出来的原因

---

## 细腰的形成

在我的设计文档里，我把这个架构叫做**细腰模型**：

```
┌──────────────────────────────┐
│   现实世界 / 用户行为 / 输入
└──────────────┬───────────────┘
               │
               ▼
┌──────────────────────────────┐
│        前端（UI / 状态）
└──────────────┬───────────────┘
               │  JSON / HTTP
               ▼
┌──────────────────────────────┐
│     Handler 层（接口边界）
└──────────────┬───────────────┘
               │
               ▼
┌──────────────────────────────┐
│    Service 层（✨细腰核心✨）
│    · 业务规则
│    · 用例编排
│    · 与 HTTP / DB 完全无关
└──────────────┬───────────────┘
               │
               ▼
┌──────────────────────────────┐
│    Repo 层（数据访问）
└──────────────┬───────────────┘
               │
               ▼
┌──────────────────────────────┐
│    PostgreSQL
└──────────────────────────────┘
```

细腰模型不是设计出来的，而是复杂系统在演化中自然长出来的形态。上端是用户世界，复杂而不可控；下端是数据库，复杂且冷酷；中间的 Service 层，是我用抽象和约定把混沌压缩成秩序的地方

Service 层有一条铁律：**它不认识任何基础设施**。不知道有 PostgreSQL，不知道有 HTTP，不知道有 Redis。它只接受 Rust 类型，只返回 Rust 类型，只抛 Rust 错误

---

## 数据在边界上的变形

从 Repository 拿到数据之后，我发现一个烦人的问题：`PostDto` 里到处是 `Option`

```rust
pub struct PostDto {
    pub status: Option,  // 数据库可能没有值
    pub view_count: Option,     // 数据库可能没有值
    pub created_at: Option>,
}
```

这些 `Option` 忠实反映了数据库的现实——字段允许为 null，所以 DTO 用 `Option` 表达。但在业务逻辑里，一篇文章必然有状态，必然有浏览量。我不想在每个业务函数里都写 `.unwrap_or(PostStatus::Draft)`

所以 Service 层重新定义了自己的 `Post`，在边界上把这些 `Option` 消除掉：

```rust
pub struct Post {
    pub status: PostStatus,              // 一定有值，默认 Draft
    pub view_count: i64,                 // 一定有值，默认 0
    pub created_at: DateTime,
}

impl From for Post {
    fn from(dto: PostDto) -> Self {
        Self {
            status: dto.status.unwrap_or(PostStatus::Draft),
            view_count: dto.view_count.unwrap_or(0),
            // ...
        }
    }
}
```

从这一层往上，业务代码只会看到确定的类型，不再需要处理数据库层面的不确定性

---

## 编排：一个动作，多个操作

回到"创建文章"这个例子。Service 层把五步操作编排在一起，用事务保证原子性：

```rust
pub async fn create_post(db: &DatabaseConnection, input: CreatePostInput) -> ServiceResult {
    if !is_valid_slug(&input.slug) {
        return Err(ServiceError::InvalidInput("slug 格式不合法".to_string()));
    }

    let txn = db.begin().await?;  // 开启事务

    // 1. 获取/创建标签
    // 2. 查找主题 ID
    // 3. 插入文章记录
    // 4. 绑定标签关联
    // 5. 如果已发布，同步计数

    txn.commit().await?;  // 全部成功才提交
    Ok(post_dto.into())
}
```

事务的意义是：这五步操作要么全部成功，要么全部回滚。不会出现"文章创建了但标签没绑上"的中间状态。Repository 层的函数接受 `ConnectionTrait`，对事务无感知——它们不知道自己是在事务里还是在普通连接里运行，这个细节由 Service 层管理

---

## 计数器的维护

第二篇设计数据库时，`themes` 和 `tags` 表都有 `post_count` 字段，当时没有解释谁来维护它

`post_count` 是主题和标签**自身的元数据**。"这个主题下有多少篇已发布文章"是描述主题本身的信息，把它存在表里，是为了让每一行记录都能自描述——不需要跨表聚合，仅凭自身字段就能推断出有意义的状态

维护这个计数的逻辑分散在三个地方：创建文章、更新文章、删除文章。其中更新最复杂，因为需要感知状态的变化：

```rust
let delta = match (&old_status, &new_status) {
    (Draft, Published) => 1,   // 草稿变发布，+1
    (Published, Draft) => -1,  // 发布变草稿，-1
    _ => 0,
};
```

这种"感知前后状态差异"的逻辑，只有在 Service 层才能正确实现。Repository 层的单个函数只看到当前操作，没有上下文

---

## 防刷：意识到外界的恶意

开发到这里，我遇到了一个之前从没想过的问题

`view_count` 记录浏览量，每次文章被访问就 +1。但如果不加限制，任何人都可以写一个脚本疯狂请求，把浏览量刷爆——更严重的是，这会给数据库带来大量无意义的写操作

> 我发现，只要是涉及到"和外界交互"，就免不了去讨论安全问题

解决方案是 Redis 限流：同一个 IP 在一段时间内只能触发一次计数。Service 层在处理浏览量增加之前，先检查 Redis：

```rust
pub async fn increment_view_count(
    db: &DatabaseConnection,
    redis: &redis::Client,
    post_id: i64,
    ip: &str,
) -> ServiceResult<()> {
    let cache_key = format!("view:post:{}:{}", post_id, ip);
    let allowed = check_rate_limit(redis, &cache_key, 3600).await?;

    if allowed {
        repo::posts::increment_view_count(db, post_id).await?;
    }
    Ok(())
}
```

Redis 在这里作为参数传入，不是全局依赖。Service 层和 Redis 之间的数据交换极其简单：传入一个 key 和 TTL，返回一个 bool。这个接口足够轻薄，Redis 是可插拔的

---

## 错误的归宿

Service 层定义了自己的错误类型，它是整条错误传播链的中间节点：

```
RepoError  →  ServiceError  →  HTTP 状态码
```

```rust
pub enum ServiceError {
    Repo(#[from] RepoError),      // 数据库错误自动转换
    InvalidInput(String),          // 业务校验失败
    NotFound,                      // 资源不存在
}
```

`#[from]` 让 `RepoError` 可以用 `?` 自动转换成 `ServiceError`，不需要手写 `map_err`。Service 层的函数只需要关心业务逻辑，错误会自动沿着链路向上传播，最终在 Handler 层被转成 HTTP 响应

---

## 小结

Service 层是细腰最窄的地方，也是整个系统最值得仔细阅读的地方

它做的事情可以用一句话概括：**把复杂的业务需求压缩成干净的函数接口**

下一篇，这些函数接口需要暴露给外界。前端不认识 Rust 函数，它只认识 HTTP——于是"接口"这个概念，终于要正面登场了

# Repository 层 API 文档

> 本文档描述 Repository 层的完整接口规范和使用说明。
>
> 最后更新：2026-01-24

---

## 📋 概述

Repository 层位于 `src/infra/repository/`，提供数据访问抽象，封装所有数据库操作。遵循单一职责原则，不包含业务逻辑。

### 设计原则

- ✅ **只负责数据访问**，业务逻辑在 Service 层
- ✅ **返回原始 Entity Model**，不返回 DTO
- ✅ **统一错误处理**，使用 `Result<T, DbErr>`
- ✅ **分页统一格式**，返回 `(Vec<Model>, u64)` 元组
- ✅ **支持复杂查询**，使用 SeaORM 的 QueryBuilder

---

## 🗂️ Repository 清单

| Repository               | 文件            | 实体         | 说明     |
| ------------------------ | --------------- | ------------ | -------- |
| **UsersRepository**      | `users.rs`      | `users`      | 用户管理 |
| **PostsRepository**      | `posts.rs`      | `posts`      | 文章管理 |
| **CategoriesRepository** | `categories.rs` | `categories` | 分类管理 |
| **TagsRepository**       | `tags.rs`       | `tags`       | 标签管理 |
| **CommentsRepository**   | `comments.rs`   | `comments`   | 评论管理 |
| **LinksRepository**      | `links.rs`      | `links`      | 友链管理 |

---

## 📊 枚举值规范

### 1. **用户角色** (`users.role`)

| 值        | 说明     | 默认值 |
| --------- | -------- | ------ |
| `"user"`  | 普通用户 | ✅      |
| `"admin"` | 管理员   | -      |

**Repository 中的使用：**
```rust
// 查询管理员用户
repo.find_by_role("admin").await?;

// 更新用户角色
repo.update_role(user_id, "admin").await?;
```

---

### 2. **文章状态** (`posts.status`)

| 值            | 说明   | 默认值 |
| ------------- | ------ | ------ |
| `"draft"`     | 草稿   | ✅      |
| `"published"` | 已发布 | -      |
| `"archived"`  | 已归档 | -      |

**Repository 中的使用：**
```rust
// 查询已发布文章
repo.find_published_paginated(page, size).await?;

// 查询草稿
repo.find_by_status("draft").await?;

// 更新状态
repo.update_status(post_id, "published").await?;
```

---

### 3. **友链状态** (`links.link_status`)

| 值          | 说明   | 默认值 |
| ----------- | ------ | ------ |
| `"broken"`  | 失效   | ✅      |
| `"pending"` | 待审核 | -      |
| `"active"`  | 已激活 | -      |

**Repository 中的使用：**
```rust
// 查询已激活友链
repo.find_active().await?;

// 查询待审核友链
repo.find_pending().await?;

// 更新状态
repo.update_status(link_id, "active").await?;
```

---

### 4. **评论审核状态** (`comments.is_reviewed`)

| 值      | 说明   | 默认值         |
| ------- | ------ | -------------- |
| `true`  | 已审核 | ✅ (Admin 评论) |
| `false` | 待审核 | ✅ (游客评论)   |

**Repository 中的使用：**
```rust
// 查询已审核评论
repo.find_approved_by_post_id(post_id).await?;

// 查询待审核评论
repo.find_pending_review().await?;

// 审核评论
repo.review(comment_id, true).await?;
```

---

## 🔍 UsersRepository

### 构造函数
```rust
pub fn new(db: DatabaseConnection) -> Self
```

### 基础查询

| 方法                             | 返回类型            | 说明                     |
| -------------------------------- | ------------------- | ------------------------ |
| `find_by_id(id)`                 | `Option<Model>`     | 根据 ID 查询             |
| `find_by_username(username)`     | `Option<Model>`     | 根据用户名查询（登录用） |
| `find_by_email(email)`           | `Option<Model>`     | 根据邮箱查询             |
| `find_by_role(role)`             | `Vec<Model>`        | 根据角色查询             |
| `find_all_paginated(page, size)` | `(Vec<Model>, u64)` | 分页获取所有用户         |

### 验证方法

| 方法                           | 返回类型 | 说明               |
| ------------------------------ | -------- | ------------------ |
| `exists_by_username(username)` | `bool`   | 检查用户名是否存在 |
| `exists_by_email(email)`       | `bool`   | 检查邮箱是否存在   |

### 写操作

| 方法                    | 返回类型       | 说明               |
| ----------------------- | -------------- | ------------------ |
| `create(user)`          | `Model`        | 创建用户           |
| `update(user)`          | `Model`        | 更新用户           |
| `delete(id)`            | `DeleteResult` | 删除用户（硬删除） |
| `update_role(id, role)` | `Model`        | 更新用户角色       |

### 统计方法

| 方法          | 返回类型 | 说明         |
| ------------- | -------- | ------------ |
| `count_all()` | `u64`    | 获取用户总数 |

---

## 📝 PostsRepository

### 构造函数
```rust
pub fn new(db: DatabaseConnection) -> Self
```

### 基础查询

| 方法                                             | 返回类型            | 说明                   |
| ------------------------------------------------ | ------------------- | ---------------------- |
| `find_by_id(id)`                                 | `Option<Model>`     | 根据 ID 查询           |
| `find_by_slug(slug)`                             | `Option<Model>`     | 根据 slug 查询         |
| `find_published_paginated(page, size)`           | `(Vec<Model>, u64)` | 分页获取已发布文章     |
| `find_published_by_category(cat_id, page, size)` | `(Vec<Model>, u64)` | 根据分类查询已发布文章 |
| `find_published_by_tag(tag_id, page, size)`      | `(Vec<Model>, u64)` | 根据标签查询已发布文章 |
| `find_all_paginated(page, size)`                 | `(Vec<Model>, u64)` | 获取所有文章（含草稿） |
| `find_by_status(status)`                         | `Vec<Model>`        | 根据状态查询           |
| `find_for_archives()`                            | `Vec<Model>`        | 获取归档数据           |
| `find_recent(limit)`                             | `Vec<Model>`        | 获取最近更新的文章     |

### 写操作

| 方法                           | 返回类型       | 说明         |
| ------------------------------ | -------------- | ------------ |
| `create(post)`                 | `Model`        | 创建文章     |
| `update(post)`                 | `Model`        | 更新文章     |
| `delete(id)`                   | `DeleteResult` | 删除文章     |
| `update_status(id, status)`    | `Model`        | 更新文章状态 |
| `update_pinned(id, is_pinned)` | `Model`        | 切换置顶状态 |
| `increment_view_count(id)`     | `()`           | 增加浏览量   |

### 标签关联

| 方法                          | 返回类型   | 说明                     |
| ----------------------------- | ---------- | ------------------------ |
| `sync_tags(post_id, tag_ids)` | `()`       | 同步文章标签（先删后增） |
| `get_tag_ids(post_id)`        | `Vec<i64>` | 获取文章的标签 ID 列表   |

### 统计方法

| 方法                | 返回类型 | 说明             |
| ------------------- | -------- | ---------------- |
| `count_published()` | `u64`    | 获取已发布文章数 |
| `count_drafts()`    | `u64`    | 获取草稿数       |

---

## 🏷️ CategoriesRepository

### 构造函数
```rust
pub fn new(db: DatabaseConnection) -> Self
```

### 基础查询

| 方法                         | 返回类型            | 说明             |
| ---------------------------- | ------------------- | ---------------- |
| `find_by_id(id)`             | `Option<Model>`     | 根据 ID 查询     |
| `find_by_slug(slug)`         | `Option<Model>`     | 根据 slug 查询   |
| `find_all()`                 | `Vec<Model>`        | 获取所有分类     |
| `find_all_with_post_count()` | `Vec<(Model, u64)>` | 获取分类及文章数 |

### 验证方法

| 方法                   | 返回类型 | 说明               |
| ---------------------- | -------- | ------------------ |
| `exists_by_name(name)` | `bool`   | 检查名称是否存在   |
| `exists_by_slug(slug)` | `bool`   | 检查 slug 是否存在 |

### 写操作

| 方法               | 返回类型       | 说明     |
| ------------------ | -------------- | -------- |
| `create(category)` | `Model`        | 创建分类 |
| `update(category)` | `Model`        | 更新分类 |
| `delete(id)`       | `DeleteResult` | 删除分类 |

### 统计方法

| 方法                       | 返回类型 | 说明             |
| -------------------------- | -------- | ---------------- |
| `count_posts(category_id)` | `u64`    | 获取分类下文章数 |
| `count_all()`              | `u64`    | 获取分类总数     |

---

## 🔖 TagsRepository

### 构造函数
```rust
pub fn new(db: DatabaseConnection) -> Self
```

### 基础查询

| 方法                    | 返回类型            | 说明                   |
| ----------------------- | ------------------- | ---------------------- |
| `find_by_id(id)`        | `Option<Model>`     | 根据 ID 查询           |
| `find_by_slug(slug)`    | `Option<Model>`     | 根据 slug 查询         |
| `find_by_name(name)`    | `Option<Model>`     | 根据名称查询           |
| `find_all()`            | `Vec<Model>`        | 获取所有标签           |
| `find_all_with_count()` | `Vec<(Model, u64)>` | 获取标签云（含引用数） |
| `find_by_ids(ids)`      | `Vec<Model>`        | 批量查询标签           |

### 验证方法

| 方法                   | 返回类型 | 说明               |
| ---------------------- | -------- | ------------------ |
| `exists_by_name(name)` | `bool`   | 检查名称是否存在   |
| `exists_by_slug(slug)` | `bool`   | 检查 slug 是否存在 |

### 写操作

| 方法                         | 返回类型       | 说明           |
| ---------------------------- | -------------- | -------------- |
| `create(tag)`                | `Model`        | 创建标签       |
| `update(tag)`                | `Model`        | 更新标签       |
| `delete(id)`                 | `DeleteResult` | 删除标签       |
| `find_or_create(name, slug)` | `Model`        | 查找或创建标签 |

### 统计方法

| 方法                  | 返回类型 | 说明             |
| --------------------- | -------- | ---------------- |
| `count_posts(tag_id)` | `u64`    | 获取标签引用次数 |
| `count_all()`         | `u64`    | 获取标签总数     |

---

## 💬 CommentsRepository

### 构造函数
```rust
pub fn new(db: DatabaseConnection) -> Self
```

### 基础查询

| 方法                                          | 返回类型            | 说明                         |
| --------------------------------------------- | ------------------- | ---------------------------- |
| `find_by_id(id)`                              | `Option<Model>`     | 根据 ID 查询                 |
| `find_approved_by_post_id(post_id)`           | `Vec<Model>`        | 获取文章的已审核评论         |
| `find_all_by_post_id(post_id)`                | `Vec<Model>`        | 获取文章所有评论（含未审核） |
| `find_pending_review()`                       | `Vec<Model>`        | 获取待审核评论               |
| `find_all_paginated(page, size, is_reviewed)` | `(Vec<Model>, u64)` | 分页获取评论                 |
| `find_by_user_id(user_id)`                    | `Vec<Model>`        | 获取用户的所有评论           |
| `find_recent(limit)`                          | `Vec<Model>`        | 获取最近评论                 |
| `find_by_ip(ip)`                              | `Vec<Model>`        | 根据 IP 查询评论             |

### 写操作

| 方法                             | 返回类型       | 说明         |
| -------------------------------- | -------------- | ------------ |
| `create(comment)`                | `Model`        | 创建评论     |
| `update(comment)`                | `Model`        | 更新评论     |
| `delete(id)`                     | `DeleteResult` | 删除评论     |
| `review(id, is_approved)`        | `Model`        | 审核评论     |
| `batch_review(ids, is_approved)` | `u64`          | 批量审核评论 |

### 验证方法

| 方法         | 返回类型 | 说明             |
| ------------ | -------- | ---------------- |
| `exists(id)` | `bool`   | 检查评论是否存在 |

### 统计方法

| 方法                                       | 返回类型 | 说明             |
| ------------------------------------------ | -------- | ---------------- |
| `count_by_post_id(post_id, only_approved)` | `u64`    | 获取文章评论数   |
| `count_pending_review()`                   | `u64`    | 获取待审核评论数 |
| `count_all()`                              | `u64`    | 获取评论总数     |

---

## 🔗 LinksRepository

### 构造函数
```rust
pub fn new(db: DatabaseConnection) -> Self
```

### 基础查询

| 方法                                 | 返回类型            | 说明               |
| ------------------------------------ | ------------------- | ------------------ |
| `find_by_id(id)`                     | `Option<Model>`     | 根据 ID 查询       |
| `find_active()`                      | `Vec<Model>`        | 获取已激活友链     |
| `find_all()`                         | `Vec<Model>`        | 获取所有友链       |
| `find_by_status(status)`             | `Vec<Model>`        | 根据状态查询       |
| `find_paginated(page, size, status)` | `(Vec<Model>, u64)` | 分页获取友链       |
| `find_by_url(url)`                   | `Option<Model>`     | 根据 URL 查询      |
| `find_pending()`                     | `Vec<Model>`        | 获取待审核友链     |
| `find_recent(limit)`                 | `Vec<Model>`        | 获取最近申请的友链 |

### 验证方法

| 方法                 | 返回类型 | 说明              |
| -------------------- | -------- | ----------------- |
| `exists_by_url(url)` | `bool`   | 检查 URL 是否存在 |

### 写操作

| 方法                               | 返回类型       | 说明         |
| ---------------------------------- | -------------- | ------------ |
| `create(link)`                     | `Model`        | 创建友链     |
| `update(link)`                     | `Model`        | 更新友链     |
| `delete(id)`                       | `DeleteResult` | 删除友链     |
| `update_status(id, status)`        | `Model`        | 更新友链状态 |
| `batch_update_status(ids, status)` | `u64`          | 批量更新状态 |

### 统计方法

| 方法              | 返回类型 | 说明             |
| ----------------- | -------- | ---------------- |
| `count_pending()` | `u64`    | 获取待审核友链数 |
| `count_active()`  | `u64`    | 获取已激活友链数 |
| `count_all()`     | `u64`    | 获取友链总数     |

---

## 🔧 使用示例

### 1. 基础 CRUD

```rust
use crate::infra::repository::UsersRepository;
use crate::entities::users;
use sea_orm::Set;

// 创建 Repository
let repo = UsersRepository::new(db.clone());

// 创建用户
let new_user = users::ActiveModel {
    username: Set("alice".to_string()),
    password_hash: Set("hashed_password".to_string()),
    email: Set(Some("alice@example.com".to_string())),
    role: Set(Some("user".to_string())),
    ..Default::default()
};
let user = repo.create(new_user).await?;

// 查询用户
let found = repo.find_by_username("alice").await?;

// 更新用户
let mut user: users::ActiveModel = found.unwrap().into();
user.nickname = Set(Some("Alice".to_string()));
repo.update(user).await?;

// 删除用户
repo.delete(user_id).await?;
```

### 2. 分页查询

```rust
use crate::infra::repository::PostsRepository;

let repo = PostsRepository::new(db.clone());

// 获取第 2 页，每页 10 条
let (posts, total) = repo.find_published_paginated(2, 10).await?;

println!("共 {} 篇文章，当前页 {} 篇", total, posts.len());
```

### 3. 复杂过滤

```rust
use crate::infra::repository::CommentsRepository;

let repo = CommentsRepository::new(db.clone());

// 获取待审核评论
let pending = repo.find_pending_review().await?;

// 分页获取已审核评论
let (comments, total) = repo.find_all_paginated(1, 20, Some(true)).await?;
```

### 4. 关联操作

```rust
use crate::infra::repository::PostsRepository;

let repo = PostsRepository::new(db.clone());

// 同步文章标签
repo.sync_tags(post_id, vec![1, 2, 3]).await?;

// 获取文章标签
let tag_ids = repo.get_tag_ids(post_id).await?;
```

### 5. 批量操作

```rust
use crate::infra::repository::CommentsRepository;

let repo = CommentsRepository::new(db.clone());

// 批量审核评论
let affected = repo.batch_review(vec![1, 2, 3], true).await?;
println!("已审核 {} 条评论", affected);
```

---

## ⚠️ 注意事项

### 1. 事务处理

Repository 方法默认**不包含事务**，需要在 Service 层手动控制：

```rust
// ❌ 错误：每个操作独立提交
repo.create_post(post).await?;
repo.sync_tags(post_id, tag_ids).await?;

// ✅ 正确：使用事务
let txn = db.begin().await?;
let post = repo.create_post_in_txn(&txn, post).await?;
repo.sync_tags_in_txn(&txn, post.id, tag_ids).await?;
txn.commit().await?;
```

### 2. N+1 查询问题

避免在循环中查询：

```rust
// ❌ 错误：N+1 查询
let posts = repo.find_all().await?;
for post in posts {
    let tags = tag_repo.find_by_ids(post.tag_ids).await?; // N 次查询
}

// ✅ 正确：使用 LoaderTrait 或批量查询
use sea_orm::LoaderTrait;
let posts = Posts::find().all(&db).await?;
let tags = posts.load_many(Tags, &db).await?;
```

### 3. 枚举值校验

Repository 不校验枚举值，应在 Service 层校验：

```rust
// Service 层
pub async fn update_post_status(&self, id: i64, status: &str) -> Result<Post, AppError> {
    // ✅ 校验枚举值
    if !["draft", "published", "archived"].contains(&status) {
        return Err(AppError::Validation("Invalid status".into()));
    }
    
    self.repo.update_status(id, status).await
        .map_err(|e| AppError::Database(e))
}
```

### 4. 软删除 vs 硬删除

当前所有 `delete` 方法都是**硬删除**，如需软删除需修改：

```rust
// 硬删除
repo.delete(id).await?;

// 软删除（需要添加 deleted_at 字段）
repo.soft_delete(id).await?; // 设置 deleted_at = NOW()
```

### 5. 分页参数校验

Repository 不校验分页参数，应在 Service/Handler 层校验：

```rust
// Handler/Service 层
let page = query.page.max(1); // 最小为 1
let size = query.size.min(100).max(1); // 1-100 之间
```

---

## 📚 相关文档

- [Architecture.md](./Architecture.md) - 架构设计
- [Contract.md](./Contract.md) - API 契约
- [SeaORM 文档](https://www.sea-ql.org/SeaORM/)

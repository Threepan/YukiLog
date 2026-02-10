<div align="center">

# YukiLog Handler 层文档

这个文档啊~我们来讲公共接口, 也就是前台接口

</div>

## 技术架构

---

#### Redis 限流系统

使用 Redis 实现 IP 限流和访问控制, 防止恶意刷访问量和评论灌水

源码: [yukilog-backend/src/handler/utils.rs]

#### IP 提取

```rust
pub fn get_client_ip(headers: &HeaderMap, addr: SocketAddr) -> String
```

**优先级:**
1. `X-Forwarded-For` header (Nginx/Cloudflare 反向代理)
2. `X-Real-IP` header (Nginx)
3. 连接 IP (直连)

#### 限流检查

```rust
pub async fn check_rate_limit(
    redis: &redis::Client,
    cache_key: &str,
    ttl: u64,
) -> Result<bool, redis::RedisError>
```

**返回值:**
- `Ok(true)` - 允许访问
- `Ok(false)` - 限流中

---

#### Gravatar 生成

```rust
pub fn generate_gravatar_url(email: &str) -> String
```

**特性:**
- MD5 哈希计算
- 大小写不敏感
- 默认头像: `d=identicon` (几何图案)
- 尺寸: 80x80 像素

**其他默认头像选项:**
- `monsterid` - 小怪物
- `wavatar` - 卡通脸
- `retro` - 8位像素
- `robohash` - 机器人

---

## 前台接口

顾名思义, 就是前端渲染博客页面和处理公共逻辑使用的接口

源码: 

* [yukilog-backend/src/handler/public/themes.rs]
* [yukilog-backend/src/handler/public/tags.rs]
* [yukilog-backend/src/handler/public/posts.rs]
* [yukilog-backend/src/handler/public/comments.rs]
* [yukilog-backend/src/handler/public/links.rs]

---

#### Themes 主题 - 3 个接口

```bash
GET     /api/public/themes              - 主题列表
GET     /api/public/themes/:slug        - 主题详情
POST    /api/public/themes/:slug/view   - 浏览记数 (IP限流 10分钟)
```

#### Tags 标签 - 3 个接口

```bash
GET    /api/public/tags             - 标签列表/云
GET    /api/public/tags/:slug       - 标签详情  
POST   /api/public/tags/:slug/view  - 浏览计数 (IP限流 10分钟)
```

#### Posts 文章 - 3 个接口

```bash
GET    /api/public/posts            - 文章列表 (分页+过滤)
GET    /api/public/posts/:slug      - 文章详情
POST   /api/public/posts/:slug/view - 浏览计数 (IP限流 10分钟)
```

#### Comments 评论 - 3个接口

```bash
GET    /api/public/posts/:slug/comments        - 评论树
GET    /api/public/comments/:id/replies        - 懒加载回复
POST   /api/public/posts/:slug/comments        - 发表评论 (频率限制 10秒 + Gravatar)
```

#### Links 友链 - 2 个接口

```bash
GET    /api/public/links            - 友链列表 (仅 active)
POST   /api/public/links            - 申请友链
```

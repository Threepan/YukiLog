---
id: 6
title: "YukiLog - 2 - 数据库设计"
slug: "yukilog-2-databasedesign"
summary: "一个博客系统需要存什么、怎么存？本篇设计了支持无限嵌套评论、标签分类、友链管理的数据库结构，以及完整的部署流程"
cover_image: "https://list.yeastar.xin/d/%E6%81%8B/YukiLog/yukilog-2-databasedesign.jpg"
status: "published"
is_featured: false
theme: "博客搭建 (bkdj)"
tags: ["blog", "psql", "教程"]
view_count: 243
created_at: "2026-02-19 02:38:44"
updated_at: "2026-08-17 09:22:39"
source: "https://blog.yeastar.xin/posts/yukilog-2-databasedesign"
---

## 📚 引言

这篇博客用于记录 `YukiLog` 的后端主数据库设计

对于一个 **CMS系统** 来说, 利用数据库管理内容是非常重要的

本篇是 `YukiLog` 系列的第二篇开发日志, 主要聊聊我为它设计的表结构:

* 支持无线嵌套的评论系统
* 便于快速检索的索引设计
* 一键部署脚本

同时在文章最后也会给出 `psql` 的部署教程

---

## 🌳 数据库表结构

> 以下部分将会详细解析每一个表的设计, 你也可以去 Github 查看原文件 [yukilog.sql](https://github.com/Yueosa/YukiLog/blob/main/yukilog-database/db/yukilog.sql)
> 一键建表脚本 [deploy_db.sh](https://github.com/Yueosa/YukiLog/blob/main/yukilog-database/db/deploy_db.sh)

```sql
-- 1. 主题表 themes
CREATE TABLE IF NOT EXISTS themes (
    id BIGSERIAL PRIMARY KEY,               -- ID 号
    name VARCHAR(50) NOT NULL UNIQUE,       -- 名称
    slug VARCHAR(50) NOT NULL UNIQUE,       -- slug
    description TEXT,                       -- 描述
    post_count INT DEFAULT 0,               -- 文章数
    view_count BIGINT DEFAULT 0,            -- 浏览量
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

* 主题其实就是传统博客系统中的"分类"
* `slug` 允许将 **URL** 从 `/posts?id=18273&ref=home` 变成 `posts/database-design`
  * 这样的格式更加清晰, 没有参数污染, 天生就更加利好 **SEO** 搜索

```sql
-- 2. 标签表 tags
CREATE TABLE IF NOT EXISTS tags (
    id BIGSERIAL PRIMARY KEY,               -- ID 号
    name VARCHAR(50) NOT NULL UNIQUE,       -- 名称
    slug VARCHAR(50) NOT NULL UNIQUE,       -- slug
    post_count INT DEFAULT 0,               -- 文章数
    view_count BIGINT DEFAULT 0,            -- 浏览量
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

* 一篇博客文章可能拥有多个标签, 例如 `rust` `psql` 都是一个小标签

```sql
-- 3. 文章表 posts
CREATE TABLE IF NOT EXISTS posts (
    id BIGSERIAL PRIMARY KEY,               -- ID 号
    title VARCHAR(255) NOT NULL,            -- 标题
    slug VARCHAR(255) NOT NULL UNIQUE,      -- slug
    summary TEXT,                           -- 摘要
    content TEXT NOT NULL,                  -- 内容
    cover_image VARCHAR(255),               -- 封面 (file:// 或 https://)
    status VARCHAR(20) DEFAULT 'draft',     -- 状态 (draft 或 published)

    theme_id BIGINT REFERENCES themes(id) ON DELETE SET NULL,

    view_count BIGINT DEFAULT 0,            -- 浏览量
  
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

* 文章表需要记录的是文章的元数据
* `theme_id` 在这里作为外键插入, 因为主题和文章是 **一对多关系**
* 这里还额外设计了一个 `updated_at` 字段, 是为了追踪更新时间

```sql
-- 4. 文章-标签关联表 post_tags
CREATE TABLE IF NOT EXISTS post_tags (
    post_id BIGINT REFERENCES posts(id) ON DELETE CASCADE,
    tag_id BIGINT REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (post_id, tag_id)
);
```

* 一个标签会被多篇文章需要, 一篇文章也有多个标签, 所以这里使用关联表来实现, 双向的一对多关系, 就是多对多关系
* 所以这个表的任务就是, 将文章和标签绑定到一起

```sql
-- 5. 评论表 comments
CREATE TABLE IF NOT EXISTS comments (
    id BIGSERIAL PRIMARY KEY,               -- ID 号
    post_id BIGINT REFERENCES posts(id) ON DELETE CASCADE,
  
    content TEXT NOT NULL,                  -- 内容
  
    guest_nick VARCHAR(50) NOT NULL,        -- 昵称
    guest_email VARCHAR(100),               -- 邮箱
    guest_website VARCHAR(200),             -- 个人网站
  
    parent_id BIGINT REFERENCES comments(id) ON DELETE CASCADE,
    root_id BIGINT REFERENCES comments(id) ON DELETE CASCADE,
  
    status VARCHAR(20) DEFAULT 'pending',   -- approved, pending, spam
  
    ip VARCHAR(45),                         -- IPv4/IPv6
    ua VARCHAR(255),                        -- User-Agent
  
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

* 在这个系统中评论依赖于文章, 所以 `post_id` 被作为外键引入 (我其实考虑过在 "关于" "友链" 等页面做评论区, 但后来觉得是没必要的)
* 实现评论楼中楼的核心抽象是 `parent_id` 和 `root_id`
  * `parent_id` 是父评论, 也就是被回复的那一条评论
  * `root_id` 是当前评论树的根评论, 只要自动从父评论那里继承下来就好
* 为了防止网站被恶意刷屏/攻击, 所以还需要 `ip` `ua` 字段
  * `ip` 字段主要是用来做 "同一个ip短时间内不能多次刷屏" 的检测
  * `ua` 则包含了很多关键请求信息/设备信息, 对于过滤简单脚本/机器人来说非常好用

```sql
-- 6. 友链表 links
CREATE TABLE IF NOT EXISTS links (
    id BIGSERIAL PRIMARY KEY,               -- ID 号
    title VARCHAR(100) NOT NULL,            -- 站点标题
    url VARCHAR(255) NOT NULL UNIQUE,       -- 站点 URL
    avatar VARCHAR(255),                    -- 站点头像
    description TEXT,                       -- 站点描述
  
    status VARCHAR(20) DEFAULT 'pending',   -- active, pending, broken
  
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

* 我觉得友链也是博客最重要的部分, 所以单独做了一个表出来

```sql
-- themes
CREATE INDEX IF NOT EXISTS idx_themes_cteated_at ON themes (created_at DESC);
-- tags
CREATE INDEX IF NOT EXISTS idx_tags_created_at ON tags (created_at DESC);
-- posts
CREATE INDEX IF NOT EXISTS idx_posts_status_created_at ON posts (status, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_posts_theme_id ON posts (theme_id);
CREATE INDEX IF NOT EXISTS idx_posts_updated_at ON posts (updated_at DESC);
CREATE INDEX IF NOT EXISTS idx_posts_view_count ON posts (view_count DESC);
-- post_tags
CREATE INDEX IF NOT EXISTS idx_post_tags_tag_id ON post_tags (tag_id);
CREATE INDEX IF NOT EXISTS idx_post_tags_post_id ON post_tags (post_id);
-- comments
CREATE INDEX IF NOT EXISTS idx_comments_post_id_created_at ON comments (post_id, created_at ASC);
CREATE INDEX IF NOT EXISTS idx_comments_status ON comments (status);
CREATE INDEX IF NOT EXISTS idx_comments_parent_id ON comments (parent_id);
CREATE INDEX IF NOT EXISTS idx_comments_root_id ON comments (root_id);
CREATE INDEX IF NOT EXISTS idx_comments_ip ON comments (ip);
-- links
CREATE INDEX IF NOT EXISTS idx_links_status ON links (status);
CREATE INDEX IF NOT EXISTS idx_links_created_at ON links (created_at DESC);
```

* 给某些表的某些字段添加索引, 这就相当于写了一页目录
* 他的优势是查询速度从 O(n) 进化为 O(log n)
* 缺点是会占用空间, 以及写入速度会变慢, 因为 insert/update 操作要维护索引

```sql
CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_posts_updated_at
    BEFORE UPDATE ON posts
    FOR EACH ROW
    EXECUTE FUNCTION update_modified_column();
```

* 这是一个给 **TRIGGER** 用的函数, 他的意思是当 `post` 表的数据被更新时, 自动将 `updated_at` 修改为当前时间

---

## 部署数据库

> 虽然已经提供了一键建表脚本, 但还是教学一下如何手动安装配置

#### **我的生产环境是 `Ubuntu`, 使用的软件源是清华源**

```shell
# 添加清华源到 apt 的软件源列表
echo "deb https://mirrors.tuna.tsinghua.edu.cn/postgresql/repos/apt/ noble-pgdg main" | sudo tee /etc/apt/sources.list.d/pgdg.list

# 导入 GPG 密钥
wget -qO https://mirrors.tuna.tsinghua.edu.cn/postgresql/repos/apt/ACCC4CF8.asc | sudo apt-key add -
```

#### **下载 PostgreSQL**

```shell
sudo apt update && sudo apt install postgresql-16 postgresql-client-16

sudo systemctl enable --now postgresql 
```

#### 数据库用户和权限配置

```shell
# 切换到postgres系统用户
sudo -i -u postgres

# 进入PostgreSQL交互终端
psql
```

#### 创建数据库和超级用户

```sql
-- 1. 创建超级用户 lian (请将 '你的密码' 替换为你真实的密码)
CREATE ROLE lian WITH LOGIN SUPERUSER PASSWORD '你的密码';

-- 2. 创建数据库 yukilog
CREATE DATABASE yukilog;

-- 3. 将数据库 yukilog 的所有权赋予 lian
ALTER DATABASE yukilog OWNER TO lian;

-- 4. 退出 psql
\q
```

#### 配置认证方式

```shell
# 编辑客户端认证配置文件
sudo vim /etc/postgresql/16/main/pg_hba.conf
```

```text
# 修改前：
local   all             all                                     peer
host    all             all             127.0.0.1/32            ident
host    all             all             ::1/128                 ident

# 修改后：
local   all             all                                     md5
host    all             all             127.0.0.1/32            md5
host    all             all             ::1/128                 md5
```

重启服务

```shell
sudo systemctl restart postgresql
```

**具体说明见 [数据库安全与网路配置详解](#数据库安全与网络配置详解)**

#### 导入数据库

```shell
psql -U lian -d yukilog -f init_db.sql
```

```shell
psql -U lian -d yukilog
```

```sql
\dt
\df
\d posts
```

#### 配置远程访问

```shell
# 编辑PostgreSQL主配置文件
sudo vim /etc/postgresql/16/main/postgresql.conf
```

```text
# 修改前：
#listen_addresses = 'localhost'

# 修改后：
listen_addresses = '*'
```

```shell
# 编辑认证配置文件，添加远程访问规则
sudo vim /etc/postgresql/16/main/pg_hba.conf
```

```text
host    all             all             0.0.0.0/0               md5
```

**具体说明见 [数据库安全与网路配置详解](#数据库安全与网络配置详解)**

#### 重启服务

```shell
# 重启PostgreSQL服务
sudo systemctl restart postgresql
```

---

## 数据库安全与网络配置详解

在安装完 `PostgreSQL` 后，默认的配置通常非常保守（**仅允许本地连接且使用系统用户认证**）

为了让我们的 `YukiLog` 后端程序能够顺利连接，并支持必要的远程管理，我们需要对认证策略和监听地址进行调整

1. **配置验证方式 (`pg_hba.conf`)**

**为什么要从 `peer/ident` 修改为 `md5`？**

`PostgreSQL` 默认使用 `peer` 或 `ident` 认证：

- **Peer/Ident:** 要求你的操作系统用户名必须与数据库用户名一致, 这在自动化部署或多应用环境下非常受限
- **MD5/Password:** 允许我们通过用户名 + 密码的方式进行校验

通过将认证方式改为 `md5`，我们确保了后端程序（如 `Rust` 或 `Go` 编写的服务）可以通过配置文件中的凭据安全地访问数据库，而不必依赖特定的系统用户身份

2. **开启网络监听 (`postgresql.conf`)**

**为什么要修改 `listen_addresses`？**

默认情况下，`PostgreSQL` 仅监听 `localhost`（即 `127.0.0.1`）, 这意味着数据库只接受来自服务器自身的连接请求

将 `listen_addresses` 设置为 `'*'`，意味着数据库将监听服务器上所有网卡的请求

这是实现**远程连接**或**容器间通信**（如果你将数据库和应用部署在不同机器/容器中）的前提

- **⚠️ 安全警告与最小权限原则:**
  PostgreSQL 默认只监听本地，这是为了遵循 **攻击面最小化 (Minimize Attack Surface)** 原则。
  将 `listen_addresses` 设为 `*` 并放行 `0.0.0.0/0` 等同于拆掉了数据库的第一道防线。在生产环境中，请严格遵循以下操作：
  - **网络层防火墙 (Security Group):** 在云服务商后台，仅放行你个人的 IP 或应用服务器的内网 IP。
  - **强密码策略:** 配合 `md5`/`scram-sha-256` 认证。
  - **最佳实践:** 如果应用和数据库在同一台服务器，**不要开启远程访问**，保持默认的 `localhost` 监听是最安全的。

3. **放行远程访问规则 (`pg_hba.conf`)**

**为什么还要加一行 `0.0.0.0/0`**

仅仅开启监听是不够的，`PostgreSQL` 还有一层“防火墙”机制（HBA, Host-Based Authentication）

添加 `host all all 0.0.0.0/0 md5` 意味着**允许任何 IP 地址**尝试连接

**安全提示:** 虽然我们设置了全局允许，但因为配合了 `md5` 强密码校验，且在实际生产中通常还会有云服务器的安全组（Security Group）拦截非授权端口，所以这是兼顾灵活性与安全性的常见做法

---

-- 1. 用户表 (Users)
-- 用于登录后台管理, 以及未来可能的注册用户评论
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,             -- 自增ID
    username VARCHAR(50) NOT NULL UNIQUE, -- 用户名 (用于登录)
    password_hash VARCHAR(255) NOT NULL,  -- 密码 (Argon2 哈希后的密码)
    email VARCHAR(255) UNIQUE,            -- 邮箱 (用于发送通知)
    nickname VARCHAR(50),                 -- 显示名称
    avatar_url VARCHAR(255),              -- 头像链接
    role VARCHAR(20) DEFAULT 'user',      -- 角色: admin, user
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 2. 分类表 (Categories)
-- 文章的分类, 比如 "技术", "生活", "随笔"
CREATE TABLE categories (
    id BIGSERIAL PRIMARY KEY,             -- 自增ID
    name VARCHAR(50) NOT NULL,            -- 分类名
    slug VARCHAR(50) NOT NULL UNIQUE,     -- URL 友好名字
    description TEXT,                     -- 分类描述
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 3. 标签表 (Tags)
-- 比分类更灵活, 比如 "Rust", "Axum", "Vue"
CREATE TABLE tags (
    id BIGSERIAL PRIMARY KEY,             -- 自增ID
    name VARCHAR(50) NOT NULL,            -- 标签名
    slug VARCHAR(50) NOT NULL UNIQUE,     -- URL 友好名字
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 4. 文章表 (POsts)
-- 博客的核心
CREATE TABLE posts (
    id BIGSERIAL PRIMARY KEY,             -- 自增ID
    title VARCHAR(255) NOT NULL,          -- 标题
    sub_title VARCHAR(255),               -- 副标题
    slug VARCHAR(255) NOT NULL UNIQUE,    -- 文章路径
    summary TEXT,                         -- 摘要 (用于列表展示)
    content TEXT NOT NULL,                -- 文章正文 (Markdown 源码)
    cover_image VARCHAR(255),             -- 封面图 URL
    status VARCHAR(20) DEFAULT 'draft',   -- 状态: draft, published, archived

    category_id BIGINT REFERENCES categories(id) ON DELETE SET NULL, -- 外键关联分类
    user_id BIGINT REFERENCES users(id) ON DELETE CASCADE,           -- 作者

    view_count BIGINT DEFAULT 0,          -- 阅读量
    is_pinned BOOLEAN DEFAULT FALSE,      -- 是否置顶

    published_at TIMESTAMP WITH TIME ZONE, -- 真正发布的时间（可能与创建时间不同）
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 5. 文章-标签关联表 (Post_Tags)
-- 多对多关系: 一篇文章可以有多个标签
CREATE TABLE post_tags (
    post_id BIGINT REFERENCES posts(id) ON DELETE CASCADE,  -- 文章ID
    tag_id BIGINT REFERENCES tags(id) ON DELETE CASCADE,    -- 标签ID
    PRIMARY KEY (post_id, tag_id)
);

-- 6. 评论表 (Comments)
-- 支持无限层级嵌套评论 (楼中楼)
CREATE TABLE comments (
    id BIGSERIAL PRIMARY KEY,
    post_id BIGINT REFERENCES posts(id) ON DELETE CASCADE, -- 关联哪篇文章

-- 评论内容相关
    content TEXT NOT NULL,                -- 评论内容
    files JSONB DEFAULT '[]',             -- 评论附件

-- 如果是登录用户，关联 user_id；如果是游客，存 nickname/email
    user_id BIGINT REFERENCES users(id) ON DELETE SET NULL, 
    guest_nickname VARCHAR(50),
    guest_email VARCHAR(100),
    guest_website VARCHAR(200),

-- 核心：父评论 ID 如果为空，说明是顶层评论；如果不为空，说明是回复某人的
    parent_id BIGINT REFERENCES comments(id) ON DELETE CASCADE,

    is_reviewed BOOLEAN DEFAULT TRUE,     -- 是否已审核 (如果是 Admin 发的默认 True，游客发的可能要 False)

    ua VARCHAR(255),                      -- 记录 UserAgent (设备信息)
    ip VARCHAR(45),                       -- 记录 IP (防灌水)

    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 7. 友链表 (Links)
-- 用于存放友链信息
CREATE TABLE links (
    id BIGSERIAL PRIMARY KEY,                           -- 自增ID
    link_title VARCHAR(50) NOT NULL,                    -- 站点标题
    link_url VARCHAR(255) NOT NULL UNIQUE,              -- 站点url
    link_avatar VARCHAR(255),                           -- 站点头像
    link_desc TEXT,                                     -- 站点描述
    link_status VARCHAR(20) NOT NULL DEFAULT 'broken',  -- 站点状态 active, , broken
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 索引优化 (Rust 代码跑起来前，先把这些加上，查询飞快)
CREATE INDEX idx_posts_slug ON posts(slug);
CREATE INDEX idx_posts_status ON posts(status);
CREATE INDEX idx_comments_post_id ON comments(post_id);

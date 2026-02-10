-- ============================== 
-- YukiLog 数据库核心表设计
-- ==============================

-- 1. 主题表 themes
CREATE TABLE themes (
    id BIGSERIAL PRIMARY KEY,           -- ID 号
    name VACHAR(50) NOT NULL UNIQUE,    -- 名称
    slug VACHAR(50) NOT NULL UNIQUE,    -- slug
    desc TEXT,                          -- 描述
    post_count INT DEFAULT 0,           -- 文章数
    view_count INT DEFAULT 0,           -- 浏览量
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
);

-- 2. 标签表 tags
CREATE TABLE tags (
    id BIGSERIAL PRIMARY KEY,           -- ID 号
    name VACHAR(50) NOT NULL UNIQUE,    -- 名称
    slug VACHAR(50) NOT NULL UNIQUE,    -- slug
    post_count INT DEFAULT 0,           -- 文章数
    view_count INT DEFAULT 0,           -- 浏览量
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
);

-- 3. 文章表 posts
CREATE TABLE posts (
    id BIGSERIAL PRIMARY KEY,           -- ID 号
    title VACHAR(255) NOT NULL,         -- 标题
    slug VACHAR(255) NOT NULL UNIQUE,   -- slug
    summary TEXT,                       -- 摘要
    content TEXT NOT NULL,              -- 内容
    cover_image VACHAR(255),            -- 封面 (file:// 或 https://)
    status VACHAR(20) DEFAULT 'draft',  -- 状态 (draft 或 published)
    
);

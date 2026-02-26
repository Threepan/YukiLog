-- v2: 添加 is_featured 字段
-- 允许将文章标记为精选，精选文章在首页展示
ALTER TABLE posts ADD COLUMN IF NOT EXISTS is_featured BOOLEAN NOT NULL DEFAULT FALSE;
CREATE INDEX IF NOT EXISTS idx_posts_is_featured ON posts (is_featured);

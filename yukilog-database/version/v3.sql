-- v3: 随记表
-- 轻量内容形态，支持 Markdown 短内容 + 心情标记，时间流排列
CREATE TABLE IF NOT EXISTS notes (
    id          BIGSERIAL PRIMARY KEY,               -- ID 号
    content     TEXT NOT NULL,                        -- Markdown 内容
    mood        VARCHAR(20),                          -- 心情标记（可选）
    status      VARCHAR(20) DEFAULT 'published',      -- published / draft / private
    created_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_notes_status_created_at ON notes (status, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_notes_created_at ON notes (created_at DESC);

CREATE TRIGGER update_notes_updated_at
    BEFORE UPDATE ON notes
    FOR EACH ROW
    EXECUTE FUNCTION update_modified_column();

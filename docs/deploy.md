# YukiLog 部署指南

本文档详细说明 YukiLog 一键部署脚本的使用方法、工作原理、维护命令和故障排查。

---

## 📋 目录

- [快速开始](#快速开始)
- [部署脚本工作流程](#部署脚本工作流程)
- [运行模式](#运行模式)
- [环境要求](#环境要求)
- [部署后的系统结构](#部署后的系统结构)
- [日常维护命令](#日常维护命令)
- [故障排查](#故障排查)
- [手动部署指南](#手动部署指南)

---

## 🚀 快速开始

### 首次部署

```bash
# 1. 克隆项目并进入目录
git clone https://github.com/yourusername/YukiLog.git
cd YukiLog

# 2. 赋予脚本执行权限
chmod +x deploy.sh

# 3. 运行部署脚本（需要 sudo）
sudo ./deploy.sh
```

### 交互式配置

部署脚本会依次询问以下信息：

| 配置项 | 说明 | 默认值 |
|--------|------|--------|
| **域名** | 网站域名（需提前解析到服务器） | `blog.yeastar.xin` |
| **数据库主机** | PostgreSQL 地址 | `localhost` |
| **数据库端口** | PostgreSQL 端口 | `5432` |
| **数据库名称** | 数据库名 | `yukilog` |
| **数据库用户** | 数据库用户名 | `lian` |
| **数据库密码** | 数据库密码 | `defaultpassword` |
| **Redis URL** | Redis 连接地址 | `redis://localhost:6379` |
| **管理员账号** | 后台管理员用户名 | `admin` |
| **管理员密码** | 后台管理员密码 | *必填，无默认值* |

> 💡 **提示**：所有配置项都有默认值（除密码外），直接回车即可使用默认配置。

---

## 🔄 部署脚本工作流程

### 完整 9 步流程

#### 1️⃣ 收集部署信息
- 交互式输入域名、数据库、管理员账号等配置
- 自动生成 JWT_SECRET（使用 `openssl rand -base64 32`）
- 智能端口检测：从默认端口开始递增扫描，避免占用冲突

#### 2️⃣ 准备环境
- **自动换源**：检测网络状态，优先切换到中科大镜像源（USTC），不可达时自动切换阿里云镜像
- **清理失效源**：移除旧的 PostgreSQL APT 源配置（避免 `Release file` 错误）
- 备份原有 `sources.list`（带时间戳）

#### 3️⃣ 检查系统依赖
安装以下工具（已安装则跳过）：

| 工具 | 用途 |
|------|------|
| `curl` / `git` | 基础工具 |
| `nginx` | 反向代理服务器 |
| `certbot` + `python3-certbot-nginx` | SSL 证书申请 |
| `postgresql` + `postgresql-contrib` | 数据库 |
| `redis-server` | 缓存和限流 |
| `rustup` | Rust 工具链 |
| `nodejs` (v20) | Node.js 运行时 |
| `pnpm` | 包管理器 |

#### 4️⃣ PostgreSQL 初始化
- 创建数据库用户（`CREATE USER`，幂等）
- 创建数据库（`CREATE DATABASE`，幂等）
- 导入表结构 `yukilog.sql`（检测到已有表则跳过）

#### 5️⃣ Redis 配置
- 安装 Redis
- 配置监听地址 `127.0.0.1`（仅本地访问）
- 启用 systemd 管理
- 验证连接（`redis-cli ping`）

#### 6️⃣ 生成环境配置文件
**后端 `.env` (yukilog-backend/.env)**：
```bash
DATABASE_URL=postgresql://user:pass@host:port/dbname
REDIS_URL=redis://localhost:6379
SERVER_HOST=127.0.0.1
SERVER_PORT=<自动检测的可用端口>
JWT_SECRET=<自动生成的随机密钥>
JWT_EXPIRES_IN=604800
ADMIN_USERNAME=<你输入的用户名>
ADMIN_PASSWORD_HASH=<Argon2 哈希>
CORS_ALLOWED_ORIGINS=https://your-domain.com
RUST_LOG=info
```

**前端 `.env` (yukilog-hanakoi/.env)**：
```bash
PUBLIC_API_URL=https://your-domain.com/api
```

> ⚠️ **重要**：`.env` 文件只在首次部署时生成，已存在则跳过（不会覆盖你的配置）

#### 7️⃣ 构建后端
- 安装 Rust（如未安装）
- 编译 `cargo build --release`
- 生成 `target/release/yukilog-backend` 二进制
- 使用 `hash_password` 工具生成密码哈希并更新 `.env`

#### 8️⃣ 构建前端
- 安装 Node.js + pnpm（如未安装）
- 运行 `pnpm install` 安装依赖
- 运行 `pnpm build` 构建生产版本
- 生成 `dist/` 目录（包含 SSR 服务器）

#### 9️⃣ 注册系统服务 + Nginx + SSL

**创建 systemd 服务**：
- `yukilog-backend.service` — 后端 Rust 服务
- `yukilog-hanakoi.service` — 前端 Astro/Node 服务

**生成 Nginx 配置**：
- `/api/*` → 反向代理到后端 `127.0.0.1:<后端端口>`
- 其他请求 → 反向代理到前端 `127.0.0.1:<前端端口>`
- 静态资源缓存优化 (`/_astro/*`)
- 安全头配置

**SSL 证书申请**：
- Let's Encrypt 自动申请（通过 `certbot --nginx`）
- 自动配置 HTTPS 重定向
- 启用 certbot.timer 自动续期

---

## 🔀 运行模式

### 模式 1：首次部署
脚本检测到**没有**构建产物时，执行完整安装流程：
```
✓ 安装所有依赖
✓ 初始化数据库
✓ 生成 .env 配置
✓ 编译后端 + 构建前端
✓ 注册服务 + 配置 Nginx + 申请 SSL
```

### 模式 2：更新模式（代码更新）
脚本检测到**已有**构建产物时，会询问：

```
[!] 检测到已有构建产物：
  • 后端二进制 (target/release/yukilog-backend)
  • 前端构建 (dist/)

如果你更新了代码，需要重新构建；如果是首次部署，可跳过重建。
是否重新构建项目（更新模式）? [Y/n]:
```

**选择 Y（或回车）**：
```
✓ 清理旧的构建产物
✓ 重新编译后端
✓ 重新构建前端
✓ 重启服务
✓ 保留 .env 配置（不覆盖）
```

**选择 n**：
```
✓ 保留现有构建
✓ 仅重启服务
✓ 适用于配置变更（如修改 .env）
```

### 模式 3：配置变更
如果只修改了 `.env` 配置，无需重新运行脚本：

```bash
# 编辑配置
vim yukilog-backend/.env

# 重启服务使配置生效
sudo systemctl restart yukilog-backend
sudo systemctl restart yukilog-hanakoi
```

---

## 💻 环境要求

### 系统要求
- **操作系统**：Ubuntu 20.04+ / Debian 11+（使用 apt 包管理器）
- **内存**：至少 2GB（编译 Rust 需要）
- **磁盘**：至少 5GB 可用空间
- **权限**：需要 `sudo` 权限

### 网络要求
- **外网访问**：用于下载依赖包、Rust 工具链、Node.js
- **域名解析**：域名需提前解析到服务器公网 IP（SSL 证书申请需要）
- **端口开放**：
  - `80` (HTTP，Let's Encrypt 验证)
  - `443` (HTTPS，生产访问)

---

## 📂 部署后的系统结构

### 目录布局
```
YukiLog/
├── deploy.sh                       # 部署脚本
├── yukilog-backend/
│   ├── .env                        # 后端配置（自动生成）
│   ├── target/release/
│   │   └── yukilog-backend         # 后端二进制
│   └── ...
├── yukilog-hanakoi/
│   ├── .env                        # 前端配置（自动生成）
│   ├── dist/                       # 前端构建产物
│   │   └── server/entry.mjs        # SSR 入口
│   └── ...
└── yukilog-database/
    └── db/yukilog.sql              # 数据库表结构
```

### Systemd 服务
```bash
# 服务文件位置
/etc/systemd/system/yukilog-backend.service
/etc/systemd/system/yukilog-hanakoi.service
```

**后端服务配置**：
```ini
[Unit]
Description=YukiLog Backend (Rust/Axum)
After=network.target postgresql.service redis-server.service

[Service]
Type=simple
User=<当前用户>
WorkingDirectory=<项目路径>/yukilog-backend
EnvironmentFile=<项目路径>/yukilog-backend/.env
ExecStart=<项目路径>/yukilog-backend/target/release/yukilog-backend
Restart=on-failure
```

**前端服务配置**：
```ini
[Unit]
Description=YukiLog Frontend (Astro/Node)
After=network.target

[Service]
Type=simple
User=<当前用户>
WorkingDirectory=<项目路径>/yukilog-hanakoi
EnvironmentFile=<项目路径>/yukilog-hanakoi/.env
Environment=HOST=127.0.0.1
Environment=PORT=<自动检测的端口>
ExecStart=/usr/bin/node <项目路径>/yukilog-hanakoi/dist/server/entry.mjs
Restart=on-failure
```

### Nginx 配置
```bash
# 配置文件位置
/etc/nginx/sites-available/yukilog
/etc/nginx/sites-enabled/yukilog  # 软链接
```

**反向代理规则**：
```nginx
# API 请求 → 后端
location /api/ {
    proxy_pass http://127.0.0.1:<后端端口>/api/;
}

# 其他请求 → 前端 (Astro SSR)
location / {
    proxy_pass http://127.0.0.1:<前端端口>;
}

# 静态资源缓存
location /_astro/ {
    proxy_pass http://127.0.0.1:<前端端口>/_astro/;
    expires 30d;
    add_header Cache-Control "public, immutable";
}
```

### 数据库表结构
```
yukilog (PostgreSQL 数据库)
├── themes          # 主题表
├── tags            # 标签表
├── posts           # 文章表
├── post_tags       # 文章-标签关联表
├── comments        # 评论表（支持无限嵌套）
└── links           # 友链表
```

---

## 🛠️ 日常维护命令

### 服务管理

```bash
# 查看服务状态
sudo systemctl status yukilog-backend
sudo systemctl status yukilog-hanakoi

# 启动服务
sudo systemctl start yukilog-backend
sudo systemctl start yukilog-hanakoi

# 停止服务
sudo systemctl stop yukilog-backend
sudo systemctl stop yukilog-hanakoi

# 重启服务（配置变更后）
sudo systemctl restart yukilog-backend
sudo systemctl restart yukilog-hanakoi

# 查看服务是否开机自启
sudo systemctl is-enabled yukilog-backend
sudo systemctl is-enabled yukilog-hanakoi

# 禁用开机自启（如需）
sudo systemctl disable yukilog-backend
sudo systemctl disable yukilog-hanakoi
```

### 日志查看

```bash
# 实时查看后端日志
journalctl -u yukilog-backend -f

# 查看最近 50 条后端日志
journalctl -u yukilog-backend -n 50

# 查看前端日志
journalctl -u yukilog-hanakoi -f

# 查看 Nginx 访问日志
tail -f /var/log/nginx/yukilog_access.log

# 查看 Nginx 错误日志
tail -f /var/log/nginx/yukilog_error.log

# 按时间范围查看日志
journalctl -u yukilog-backend --since "2026-02-13 10:00:00" --until "2026-02-13 11:00:00"
```

### Nginx 管理

```bash
# 测试配置文件语法
sudo nginx -t

# 重载配置（不中断服务）
sudo systemctl reload nginx

# 重启 Nginx
sudo systemctl restart nginx

# 查看 Nginx 状态
sudo systemctl status nginx
```

### 数据库维护

```bash
# 登录数据库
psql -h localhost -U lian -d yukilog

# 备份数据库
pg_dump -h localhost -U lian yukilog > yukilog_backup_$(date +%Y%m%d).sql

# 恢复数据库
psql -h localhost -U lian -d yukilog < yukilog_backup_20260213.sql

# 查看数据库连接数
psql -h localhost -U lian -d yukilog -c "SELECT count(*) FROM pg_stat_activity;"
```

### SSL 证书管理

```bash
# 查看证书有效期
sudo certbot certificates

# 手动续期证书
sudo certbot renew

# 模拟续期（测试）
sudo certbot renew --dry-run

# 查看自动续期状态
sudo systemctl status certbot.timer
```

---

## 🔧 故障排查

### 问题 1：密码哈希生成失败

**现象**：
```
[!] hash_password 编译失败，使用占位哈希（请稍后手动替换）
```

**原因**：步骤 5 生成 `.env` 时 Rust 还未安装，无法编译 `hash_password` 工具。

**解决方案 1（推荐）**：删除 `.env` 重新生成
```bash
rm yukilog-backend/.env yukilog-hanakoi/.env
sudo ./deploy.sh
# 遇到"是否重新构建项目？"时选 n
```

**解决方案 2**：手动生成哈希
```bash
cd yukilog-backend
NEW_HASH=$(cargo run --bin hash_password --release -- <你的密码> 2>/dev/null | tail -1)
sed -i "s|TEMP_PLACEHOLDER_WILL_BE_UPDATED_AFTER_RUST_BUILD|${NEW_HASH}|" .env
sudo systemctl restart yukilog-backend
```

### 问题 2：端口已被占用

**现象**：
```
Error: Address already in use (os error 98)
```

**排查**：
```bash
# 查看端口占用
sudo ss -lntp | grep 3639  # 替换为实际端口

# 如果是其他进程占用，修改 .env 中的端口
vim yukilog-backend/.env
# 修改 SERVER_PORT=<新端口>

# 重启服务
sudo systemctl restart yukilog-backend
```

### 问题 3：Nginx 无法启动

**排查**：
```bash
# 测试配置语法
sudo nginx -t

# 查看详细错误
sudo systemctl status nginx
journalctl -u nginx -n 50

# 常见问题：端口 80 被占用
sudo ss -lntp | grep :80

# 检查是否有其他站点配置冲突
ls -la /etc/nginx/sites-enabled/
```

### 问题 4：SSL 证书申请失败

**排查**：
```bash
# 1. 确认域名解析正确
ping blog.yeastar.xin

# 2. 确认 80 端口可访问（Let's Encrypt 验证需要）
curl http://blog.yeastar.xin

# 3. 手动重试申请
sudo certbot --nginx -d blog.yeastar.xin

# 4. 查看 certbot 日志
sudo journalctl -u certbot -n 50
```

### 问题 5：前端页面 500 错误

**排查**：
```bash
# 查看前端日志
journalctl -u yukilog-hanakoi -n 100

# 检查环境变量
cat yukilog-hanakoi/.env

# 确认 API 地址正确
curl https://your-domain.com/api/public/site-info

# 重新构建前端
cd yukilog-hanakoi
rm -rf dist/
pnpm build
sudo systemctl restart yukilog-hanakoi
```

### 问题 6：后端无法连接数据库

**排查**：
```bash
# 测试数据库连接
psql -h localhost -U lian -d yukilog -c "SELECT 1;"

# 检查 PostgreSQL 服务状态
sudo systemctl status postgresql

# 查看后端日志中的连接错误
journalctl -u yukilog-backend -n 50 | grep -i "database\|postgres"

# 确认 DATABASE_URL 配置正确
cat yukilog-backend/.env | grep DATABASE_URL
```

---

## 📝 手动部署指南

如果不使用自动化脚本，也可以手动部署（用于学习或自定义需求）。

### 1. 安装依赖

```bash
# PostgreSQL
sudo apt install postgresql postgresql-contrib

# Redis
sudo apt install redis-server

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Node.js 20
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs

# pnpm
npm install -g pnpm

# Nginx
sudo apt install nginx

# Certbot
sudo apt install certbot python3-certbot-nginx
```

### 2. 配置数据库

```bash
# 创建用户和数据库
sudo -u postgres psql << EOF
CREATE USER lian WITH PASSWORD 'your_password';
CREATE DATABASE yukilog OWNER lian;
\q
EOF

# 导入表结构
psql -h localhost -U lian -d yukilog -f yukilog-database/db/yukilog.sql
```

### 3. 生成配置文件

**后端 `.env`**：
```bash
cd yukilog-backend
cp .env.example .env
vim .env  # 编辑配置

# 生成 JWT_SECRET
openssl rand -base64 32

# 生成密码哈希
cargo run --bin hash_password -- your_admin_password
# 复制输出的哈希到 ADMIN_PASSWORD_HASH
```

**前端 `.env`**：
```bash
cd yukilog-hanakoi
cp .env.example .env
vim .env  # 设置 PUBLIC_API_URL
```

### 4. 构建项目

```bash
# 后端
cd yukilog-backend
cargo build --release

# 前端
cd yukilog-hanakoi
pnpm install
pnpm build
```

### 5. 配置 systemd 服务

参考自动生成的服务文件（见上文 "部署后的系统结构" 章节），手动创建：
```bash
sudo vim /etc/systemd/system/yukilog-backend.service
sudo vim /etc/systemd/system/yukilog-hanakoi.service
sudo systemctl daemon-reload
sudo systemctl enable --now yukilog-backend yukilog-hanakoi
```

### 6. 配置 Nginx

参考自动生成的配置（见上文），手动创建：
```bash
sudo vim /etc/nginx/sites-available/yukilog
sudo ln -s /etc/nginx/sites-available/yukilog /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

### 7. 申请 SSL 证书

```bash
sudo certbot --nginx -d your-domain.com
```

---

## 🔐 安全建议

1. **定期更新系统**：
   ```bash
   sudo apt update && sudo apt upgrade
   ```

2. **修改默认密码**：
   - 数据库密码
   - 管理员密码
   - SSH 密码

3. **配置防火墙**：
   ```bash
   sudo ufw allow 80/tcp
   sudo ufw allow 443/tcp
   sudo ufw allow 22/tcp  # SSH
   sudo ufw enable
   ```

4. **定期备份数据库**：
   ```bash
   # 创建定时任务
   crontab -e
   # 添加每日备份
   0 2 * * * pg_dump -h localhost -U lian yukilog > /backup/yukilog_$(date +\%Y\%m\%d).sql
   ```

5. **监控日志**：定期检查 `journalctl` 和 Nginx 日志中的异常访问

---

## 📚 相关文档

- [YukiLog 前端架构文档](../yukilog-hanakoi/docs/architecture.md)
- [YukiLog 后端 API 文档](../yukilog-backend/docs/api.md)
- [YukiLog 功能规划路线图](../yukilog-hanakoi/docs/roadmap.md)

---

**最后更新**：2026-02-13  
**脚本版本**：v1.0  
**维护者**：YukiLog Team

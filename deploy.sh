#!/usr/bin/env bash
# ============================================================
#  YukiLog 一键部署脚本
#  ✦ 生成 .env → 部署 PostgreSQL / Redis → 构建前后端
#    → 注册 systemd 服务 → 配置 nginx + SSL
#  ✦ 所有操作幂等：已存在的资源不会被覆写
# ============================================================
set -euo pipefail

# ── 颜色定义 ──────────────────────────────────────────────
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
CYAN='\033[0;36m'; BOLD='\033[1m'; NC='\033[0m'

info()  { echo -e "${GREEN}[✓]${NC} $*"; }
warn()  { echo -e "${YELLOW}[!]${NC} $*"; }
err()   { echo -e "${RED}[✗]${NC} $*" >&2; }
step()  { echo -e "\n${CYAN}${BOLD}══ $* ══${NC}"; }

# ── 目录定位 ──────────────────────────────────────────────
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
BACKEND_DIR="$SCRIPT_DIR/yukilog-backend"
FRONTEND_DIR="$SCRIPT_DIR/yukilog-hanakoi"
DATABASE_DIR="$SCRIPT_DIR/yukilog-database"

# ── 默认值 ────────────────────────────────────────────────
DEFAULT_DOMAIN="blog.yeastar.xin"
DEFAULT_DB_HOST="localhost"
DEFAULT_DB_PORT="5432"
DEFAULT_DB_NAME="yukilog"
DEFAULT_DB_USER="lian"
DEFAULT_DB_PASS="defaultpassword"
DEFAULT_REDIS_URL="redis://localhost:6379"
DEFAULT_BACKEND_PORT=3639
DEFAULT_FRONTEND_PORT=4132

# ── 运行时状态变量 ────────────────────────────────────────
NEED_HASH_UPDATE=false  # 是否需要在编译后更新密码哈希
FORCE_REBUILD=false     # 是否强制重新构建（用于代码更新）

# ============================================================
#  工具函数
# ============================================================

# 查找可用端口：从 $1 开始递增，直到找到未被占用的端口
find_free_port() {
    local port="$1"
    local max_attempts=50
    for ((i = 0; i < max_attempts; i++)); do
        if ! ss -lntp 2>/dev/null | grep -q ":${port} " && \
           ! ss -lnup 2>/dev/null | grep -q ":${port} "; then
            echo "$port"
            return 0
        fi
        ((port++))
    done
    err "无法在 $1 - $((port)) 范围内找到可用端口"
    return 1
}

# 检查命令是否存在
has_cmd() { command -v "$1" &>/dev/null; }

# 确认提示 (默认 Y)
confirm() {
    local msg="$1"
    read -rp "$(echo -e "${YELLOW}$msg [Y/n]: ${NC}")" ans
    [[ -z "$ans" || "$ans" =~ ^[Yy] ]]
}

# ============================================================
#  第 1 步：收集部署信息
# ============================================================
step "1/9  收集部署信息"

read -rp "$(echo -e "${CYAN}域名${NC} (默认 ${DEFAULT_DOMAIN}): ")" DOMAIN
DOMAIN="${DOMAIN:-$DEFAULT_DOMAIN}"

echo ""
echo -e "${BOLD}数据库配置${NC} (直接回车使用默认值)"
read -rp "  DB 主机 (${DEFAULT_DB_HOST}): " DB_HOST; DB_HOST="${DB_HOST:-$DEFAULT_DB_HOST}"
read -rp "  DB 端口 (${DEFAULT_DB_PORT}): " DB_PORT; DB_PORT="${DB_PORT:-$DEFAULT_DB_PORT}"
read -rp "  DB 名称 (${DEFAULT_DB_NAME}): " DB_NAME; DB_NAME="${DB_NAME:-$DEFAULT_DB_NAME}"
read -rp "  DB 用户 (${DEFAULT_DB_USER}): " DB_USER; DB_USER="${DB_USER:-$DEFAULT_DB_USER}"
read -rsp "  DB 密码 (默认隐藏): " DB_PASS; echo
DB_PASS="${DB_PASS:-$DEFAULT_DB_PASS}"

DATABASE_URL="postgresql://${DB_USER}:${DB_PASS}@${DB_HOST}:${DB_PORT}/${DB_NAME}"

echo ""
read -rp "$(echo -e "${CYAN}Redis URL${NC} (默认 ${DEFAULT_REDIS_URL}): ")" REDIS_URL
REDIS_URL="${REDIS_URL:-$DEFAULT_REDIS_URL}"

echo ""
echo -e "${BOLD}管理员账号${NC}"
read -rp "  用户名 (默认 admin): " ADMIN_USERNAME; ADMIN_USERNAME="${ADMIN_USERNAME:-admin}"
read -rsp "  密码: " ADMIN_PASSWORD; echo
while [[ -z "$ADMIN_PASSWORD" ]]; do
    warn "密码不能为空"
    read -rsp "  密码: " ADMIN_PASSWORD; echo
done

# ── 端口自动检测 ──────────────────────────────────────────
step "1.5/9  检测可用端口"

BACKEND_PORT=$(find_free_port "$DEFAULT_BACKEND_PORT")
info "后端端口: $BACKEND_PORT"

# 前端起始端口，确保不与后端冲突
FRONTEND_START="$DEFAULT_FRONTEND_PORT"
if [[ "$FRONTEND_START" -eq "$BACKEND_PORT" ]]; then
    ((FRONTEND_START++))
fi
FRONTEND_PORT=$(find_free_port "$FRONTEND_START")
info "前端端口: $FRONTEND_PORT"

# ── 生成 JWT_SECRET ───────────────────────────────────────
JWT_SECRET=$(openssl rand -base64 32)
info "JWT Secret 已自动生成"

# ── 打印摘要 ──────────────────────────────────────────────
echo ""
echo -e "${BOLD}部署摘要${NC}"
echo "  域名:        $DOMAIN"
echo "  数据库:      postgresql://${DB_USER}:****@${DB_HOST}:${DB_PORT}/${DB_NAME}"
echo "  Redis:       $REDIS_URL"
echo "  后端端口:    $BACKEND_PORT"
echo "  前端端口:    $FRONTEND_PORT"
echo "  管理员:      $ADMIN_USERNAME"
echo ""

if ! confirm "确认以上信息开始部署?"; then
    echo "部署已取消。"
    exit 0
fi

# ── 检测更新模式 ──────────────────────────────────────────
BACKEND_BIN_CHECK="$BACKEND_DIR/target/release/yukilog-backend"
FRONTEND_DIST_CHECK="$FRONTEND_DIR/dist"

if [[ -f "$BACKEND_BIN_CHECK" ]] || [[ -d "$FRONTEND_DIST_CHECK" ]]; then
    echo ""
    warn "检测到已有构建产物："
    [[ -f "$BACKEND_BIN_CHECK" ]] && echo "  • 后端二进制 (target/release/yukilog-backend)"
    [[ -d "$FRONTEND_DIST_CHECK" ]] && echo "  • 前端构建 (dist/)"
    echo ""
    echo -e "${CYAN}如果你更新了代码，需要重新构建；如果是首次部署，可跳过重建。${NC}"
    if confirm "是否重新构建项目（更新模式）?"; then
        FORCE_REBUILD=true
        warn "将强制重新构建后端和前端"
        # 清理构建产物
        [[ -f "$BACKEND_BIN_CHECK" ]] && rm -f "$BACKEND_BIN_CHECK" && info "已清理后端二进制"
        [[ -d "$FRONTEND_DIST_CHECK" ]] && rm -rf "$FRONTEND_DIST_CHECK" && info "已清理前端构建"
    else
        info "保留现有构建，跳过编译步骤"
    fi
fi

# ── 清理可能存在的问题源 + 自动换源 ───────────────────────
step "准备环境"

# 移除可能失效的 PostgreSQL APT 源
if [[ -f /etc/apt/sources.list.d/pgdg.list ]]; then
    warn "检测到可能失效的 PostgreSQL APT 源，正在清理..."
    sudo rm -f /etc/apt/sources.list.d/pgdg.list
    info "已清理旧的 PostgreSQL 源配置"
fi

# 自动切换国内镜像源（中科大 USTC / 阿里云）
SOURCES_LIST="/etc/apt/sources.list"
SOURCES_BACKUP="/etc/apt/sources.list.backup-$(date +%Y%m%d-%H%M%S)"

if [[ -f "$SOURCES_LIST" ]] && ! grep -q "mirrors.ustc.edu.cn\|mirrors.aliyun.com" "$SOURCES_LIST"; then
    warn "检测到官方源，正在切换为国内镜像..."
    
    # 获取 Ubuntu 版本代号
    UBUNTU_CODENAME=$(lsb_release -sc 2>/dev/null || grep VERSION_CODENAME /etc/os-release | cut -d= -f2)
    
    if [[ -z "$UBUNTU_CODENAME" ]]; then
        warn "无法检测 Ubuntu 版本，跳过换源"
    else
        # 测试镜像源可达性（超时 2 秒）
        MIRROR=""
        if curl -m 2 -s http://mirrors.ustc.edu.cn/ > /dev/null 2>&1; then
            MIRROR="mirrors.ustc.edu.cn"
            info "选择中科大镜像源 (USTC)"
        elif curl -m 2 -s http://mirrors.aliyun.com/ > /dev/null 2>&1; then
            MIRROR="mirrors.aliyun.com"
            info "选择阿里云镜像源 (Aliyun)"
        fi
        
        if [[ -n "$MIRROR" ]]; then
            # 备份原文件
            sudo cp "$SOURCES_LIST" "$SOURCES_BACKUP"
            info "已备份原文件到 $SOURCES_BACKUP"
            
            # 生成新的 sources.list
            sudo tee "$SOURCES_LIST" > /dev/null <<SOURCEEOF
# YukiLog 部署脚本自动生成 - 国内镜像源 (${MIRROR})
deb http://${MIRROR}/ubuntu/ ${UBUNTU_CODENAME} main restricted universe multiverse
deb http://${MIRROR}/ubuntu/ ${UBUNTU_CODENAME}-updates main restricted universe multiverse
deb http://${MIRROR}/ubuntu/ ${UBUNTU_CODENAME}-backports main restricted universe multiverse
deb http://${MIRROR}/ubuntu/ ${UBUNTU_CODENAME}-security main restricted universe multiverse
SOURCEEOF
            info "已切换为 ${MIRROR} 镜像源"
            
            # 更新索引
            sudo apt-get update -qq
            info "镜像源索引更新完成"
        else
            warn "国内镜像源不可达，保持原有配置"
        fi
    fi
elif grep -q "mirrors.ustc.edu.cn\|mirrors.aliyun.com" "$SOURCES_LIST"; then
    info "已使用国内镜像源，跳过换源"
fi

# ============================================================
#  第 2 步：安装系统依赖 (幂等)
# ============================================================
step "2/9  检查系统依赖"

install_if_missing() {
    local cmd="$1" pkg="${2:-$1}"
    if ! has_cmd "$cmd"; then
        warn "$cmd 未安装，正在安装 $pkg ..."
        sudo apt-get update -qq
        sudo apt-get install -y -qq "$pkg"
        info "$pkg 已安装"
    else
        info "$cmd 已就绪"
    fi
}

# 基础工具
install_if_missing "curl" "curl"
install_if_missing "git" "git"
install_if_missing "openssl" "openssl"
install_if_missing "ss" "iproute2"

# Nginx
install_if_missing "nginx" "nginx"

# Certbot
if ! has_cmd certbot; then
    warn "certbot 未安装，正在安装..."
    sudo apt-get update -qq
    sudo apt-get install -y -qq certbot python3-certbot-nginx
    info "certbot 已安装"
else
    info "certbot 已就绪"
fi

# ============================================================
#  第 3 步：PostgreSQL
# ============================================================
step "3/9  PostgreSQL"

if ! has_cmd psql; then
    warn "PostgreSQL 未安装，正在安装..."
    sudo apt-get update -qq
    sudo apt-get install -y -qq postgresql postgresql-contrib
    sudo systemctl enable --now postgresql
    info "PostgreSQL 已安装并启动"
else
    info "PostgreSQL 客户端已就绪"
fi

# 确保 PostgreSQL 服务运行
if systemctl is-active --quiet postgresql 2>/dev/null; then
    info "PostgreSQL 服务正在运行"
else
    warn "正在启动 PostgreSQL 服务..."
    sudo systemctl start postgresql
    info "PostgreSQL 服务已启动"
fi

# 创建数据库用户（幂等）
if sudo -u postgres psql -tAc "SELECT 1 FROM pg_roles WHERE rolname='${DB_USER}'" | grep -q 1; then
    info "数据库用户 '${DB_USER}' 已存在"
else
    warn "正在创建数据库用户 '${DB_USER}'..."
    sudo -u postgres psql -c "CREATE USER ${DB_USER} WITH PASSWORD '${DB_PASS}';"
    info "数据库用户 '${DB_USER}' 已创建"
fi

# 创建数据库（幂等）
if sudo -u postgres psql -tAc "SELECT 1 FROM pg_database WHERE datname='${DB_NAME}'" | grep -q 1; then
    info "数据库 '${DB_NAME}' 已存在"
else
    warn "正在创建数据库 '${DB_NAME}'..."
    sudo -u postgres psql -c "CREATE DATABASE ${DB_NAME} OWNER ${DB_USER};"
    info "数据库 '${DB_NAME}' 已创建"
fi

# 授权
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE ${DB_NAME} TO ${DB_USER};" 2>/dev/null || true

# 导入表结构（幂等：SQL 内部有 DROP IF EXISTS）
SQL_FILE="$DATABASE_DIR/db/yukilog.sql"
if [[ -f "$SQL_FILE" ]]; then
    # 检查是否已有表
    TABLE_COUNT=$(PGPASSWORD="$DB_PASS" psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -tAc \
        "SELECT count(*) FROM information_schema.tables WHERE table_schema='public';" 2>/dev/null || echo "0")
    if [[ "$TABLE_COUNT" -gt 0 ]]; then
        info "数据库已有 ${TABLE_COUNT} 张表，跳过导入 (如需重建请手动运行 yukilog.sql)"
    else
        warn "正在导入数据库表结构..."
        PGPASSWORD="$DB_PASS" psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -f "$SQL_FILE"
        info "数据库表结构导入完成"
    fi
else
    warn "找不到 $SQL_FILE，请稍后手动导入"
fi

# ============================================================
#  第 4 步：Redis
# ============================================================
step "4/9  Redis"

if has_cmd redis-cli && redis-cli ping &>/dev/null; then
    info "Redis 已就绪 (PONG)"
else
    if ! has_cmd redis-server; then
        warn "Redis 未安装，正在安装..."
        sudo apt-get update -qq
        sudo apt-get install -y -qq redis-server
    fi
    # 配置
    REDIS_CONF="/etc/redis/redis.conf"
    if [[ -f "$REDIS_CONF" ]]; then
        sudo sed -i 's/^bind .*/bind 127.0.0.1 ::1/' "$REDIS_CONF"
        sudo sed -i 's/^supervised .*/supervised systemd/' "$REDIS_CONF"
    fi
    sudo systemctl enable --now redis-server
    # 验证
    sleep 1
    if redis-cli ping &>/dev/null; then
        info "Redis 已安装并就绪 (PONG)"
    else
        warn "Redis 已安装但 ping 失败，请手动检查"
    fi
fi

# ============================================================
#  第 5 步：生成 .env 文件 (幂等)
# ============================================================
step "5/9  生成环境配置文件"

# ── 后端 .env ─────────────────────────────────────────────
BACKEND_ENV="$BACKEND_DIR/.env"
if [[ -f "$BACKEND_ENV" ]]; then
    info "后端 .env 已存在，跳过生成"
    NEED_HASH_UPDATE=false
else
    warn "正在生成后端 .env (密码哈希将在 Rust 编译后生成) ..."
    ADMIN_HASH="TEMP_PLACEHOLDER_WILL_BE_UPDATED_AFTER_RUST_BUILD"
    NEED_HASH_UPDATE=true

    cat > "$BACKEND_ENV" <<ENVEOF
# ====================================
# YukiLog Backend 配置 (自动生成)
# ====================================

# 数据库
DATABASE_URL=${DATABASE_URL}

# Redis
REDIS_URL=${REDIS_URL}

# 服务器
SERVER_HOST=127.0.0.1
SERVER_PORT=${BACKEND_PORT}

# JWT
JWT_SECRET=${JWT_SECRET}
JWT_EXPIRES_IN=604800

# 管理员
ADMIN_USERNAME=${ADMIN_USERNAME}
ADMIN_PASSWORD_HASH=${ADMIN_HASH}

# CORS
CORS_ALLOWED_ORIGINS=https://${DOMAIN}

# 日志
RUST_LOG=info
ENVEOF
    info "后端 .env 已生成"
fi

# ── 前端 .env ─────────────────────────────────────────────
FRONTEND_ENV="$FRONTEND_DIR/.env"
if [[ -f "$FRONTEND_ENV" ]]; then
    info "前端 .env 已存在，跳过生成"
else
    warn "正在生成前端 .env ..."
    cat > "$FRONTEND_ENV" <<ENVEOF
# YukiLog 前端配置 (自动生成)
PUBLIC_API_URL=https://${DOMAIN}/api
ENVEOF
    info "前端 .env 已生成"
fi

# ============================================================
#  第 6 步：构建后端
# ============================================================
step "6/9  构建后端 (Rust)"

# 检查 Rust 工具链
if ! has_cmd cargo; then
    warn "Rust 未安装，正在通过 rustup 安装..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    # 明确设置 Rust 路径（支持 sudo 环境）
    export PATH="$HOME/.cargo/bin:$PATH"
    source "$HOME/.cargo/env" 2>/dev/null || true
    info "Rust 已安装"
else
    info "Rust 工具链已就绪 ($(rustc --version))"
fi

# 确保 cargo 可用（防止 sudo 环境下找不到）
if ! has_cmd cargo; then
    export PATH="$HOME/.cargo/bin:/root/.cargo/bin:$PATH"
fi

pushd "$BACKEND_DIR" > /dev/null
BACKEND_BIN="$BACKEND_DIR/target/release/yukilog-backend"
if [[ -f "$BACKEND_BIN" ]] && [[ "$FORCE_REBUILD" != "true" ]]; then
    info "后端二进制已存在，跳过编译 (如需重新编译请删除 target/release/yukilog-backend)"
else
    warn "正在编译后端 (release 模式，可能需要几分钟)..."
    cargo build --release
    info "后端编译完成"
fi

# 生成密码哈希并更新 .env（如果需要）
if [[ "$NEED_HASH_UPDATE" == "true" ]]; then
    warn "正在生成管理员密码哈希 (Argon2) ..."
    
    # 确保 cargo 在 PATH 中
    export PATH="$HOME/.cargo/bin:/root/.cargo/bin:$PATH"
    
    # 生成哈希，增加错误捕获
    ADMIN_HASH=$(cargo run --bin hash_password --release -- "$ADMIN_PASSWORD" 2>&1 | tail -1)
    
    # 验证哈希格式（Argon2 哈希以 $argon2 开头）
    if [[ -n "$ADMIN_HASH" ]] && [[ "$ADMIN_HASH" =~ ^\$argon2 ]]; then
        info "密码哈希生成成功"
    else
        err "密码哈希生成失败: $ADMIN_HASH"
        ADMIN_HASH="FAILED_TO_GENERATE_PLEASE_REGENERATE_MANUALLY"
    fi
    
    # 更新 .env 中的占位符
    if [[ -f "$BACKEND_ENV" ]]; then
        sed -i "s|TEMP_PLACEHOLDER_WILL_BE_UPDATED_AFTER_RUST_BUILD|${ADMIN_HASH}|" "$BACKEND_ENV"
        info "后端 .env 密码哈希已更新"
    fi
fi

popd > /dev/null

# ============================================================
#  第 7 步：构建前端
# ============================================================
step "7/9  构建前端 (Astro)"

# 检查 Node.js
if ! has_cmd node; then
    warn "Node.js 未安装，正在安装..."
    curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
    sudo apt-get install -y -qq nodejs
    info "Node.js 已安装 ($(node --version))"
else
    info "Node.js 已就绪 ($(node --version))"
fi

# 检查 pnpm
if ! has_cmd pnpm; then
    warn "pnpm 未安装，正在安装..."
    npm install -g pnpm
    info "pnpm 已安装"
else
    info "pnpm 已就绪"
fi

pushd "$FRONTEND_DIR" > /dev/null
if [[ -d "dist" ]] && [[ "$FORCE_REBUILD" != "true" ]]; then
    info "前端已构建，跳过 (如需重新构建请删除 dist/ 目录)"
else
    warn "正在安装前端依赖..."
    pnpm install --frozen-lockfile 2>/dev/null || pnpm install
    warn "正在构建前端..."
    pnpm build
    info "前端构建完成"
fi
popd > /dev/null

# ============================================================
#  第 8 步：Systemd 服务 + Nginx + 启动
# ============================================================
step "8/9  配置系统服务与 Nginx"

CURRENT_USER="$(whoami)"
CURRENT_GROUP="$(id -gn)"

# ── systemd: 后端 ────────────────────────────────────────
BACKEND_SERVICE="/etc/systemd/system/yukilog-backend.service"
if [[ -f "$BACKEND_SERVICE" ]]; then
    info "后端 systemd 服务已存在，跳过创建"
else
    warn "正在创建后端 systemd 服务..."
    sudo tee "$BACKEND_SERVICE" > /dev/null <<SVCEOF
[Unit]
Description=YukiLog Backend (Rust/Axum)
After=network.target postgresql.service redis-server.service
Wants=postgresql.service redis-server.service

[Service]
Type=simple
User=${CURRENT_USER}
Group=${CURRENT_GROUP}
WorkingDirectory=${BACKEND_DIR}
EnvironmentFile=${BACKEND_DIR}/.env
ExecStart=${BACKEND_DIR}/target/release/yukilog-backend
Restart=on-failure
RestartSec=5
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
SVCEOF
    info "后端 systemd 服务已创建"
fi

# ── systemd: 前端 ────────────────────────────────────────
FRONTEND_SERVICE="/etc/systemd/system/yukilog-hanakoi.service"
if [[ -f "$FRONTEND_SERVICE" ]]; then
    info "前端 systemd 服务已存在，跳过创建"
else
    warn "正在创建前端 systemd 服务..."
    sudo tee "$FRONTEND_SERVICE" > /dev/null <<SVCEOF
[Unit]
Description=YukiLog Frontend (Astro/Node)
After=network.target

[Service]
Type=simple
User=${CURRENT_USER}
Group=${CURRENT_GROUP}
WorkingDirectory=${FRONTEND_DIR}
EnvironmentFile=${FRONTEND_DIR}/.env
Environment=HOST=127.0.0.1
Environment=PORT=${FRONTEND_PORT}
ExecStart=$(which node) ${FRONTEND_DIR}/dist/server/entry.mjs
Restart=on-failure
RestartSec=5
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
SVCEOF
    info "前端 systemd 服务已创建"
fi

# 重载 daemon
sudo systemctl daemon-reload

# 启用并启动服务
for svc in yukilog-backend yukilog-hanakoi; do
    sudo systemctl enable "$svc"
    if systemctl is-active --quiet "$svc"; then
        info "$svc 已在运行，正在重启..."
        sudo systemctl restart "$svc"
    else
        warn "正在启动 $svc ..."
        sudo systemctl start "$svc"
    fi
    # 等待一下确认启动
    sleep 2
    if systemctl is-active --quiet "$svc"; then
        info "$svc 启动成功 ✓"
    else
        err "$svc 启动失败，请检查: journalctl -u $svc -n 30"
    fi
done

# ── Nginx 配置 ───────────────────────────────────────────
NGINX_CONF="/etc/nginx/sites-available/yukilog"
NGINX_LINK="/etc/nginx/sites-enabled/yukilog"

if [[ -f "$NGINX_CONF" ]]; then
    info "Nginx 配置已存在，跳过创建"
else
    warn "正在生成 Nginx 配置..."
    sudo tee "$NGINX_CONF" > /dev/null <<NGINXEOF
# YukiLog — Nginx 反向代理配置 (自动生成)
# 后端: 127.0.0.1:${BACKEND_PORT}  前端: 127.0.0.1:${FRONTEND_PORT}

upstream yukilog_backend {
    server 127.0.0.1:${BACKEND_PORT};
    keepalive 16;
}

upstream yukilog_frontend {
    server 127.0.0.1:${FRONTEND_PORT};
    keepalive 16;
}

server {
    listen 80;
    listen [::]:80;
    server_name ${DOMAIN};

    # ── API 反向代理 → 后端 ──
    location /api/ {
        proxy_pass http://yukilog_backend/api/;
        proxy_http_version 1.1;
        proxy_set_header Host              \$host;
        proxy_set_header X-Real-IP         \$remote_addr;
        proxy_set_header X-Forwarded-For   \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_set_header Connection        "";

        # 超时
        proxy_connect_timeout 30s;
        proxy_read_timeout    60s;
        proxy_send_timeout    60s;
    }

    # ── 其余请求 → 前端 (Astro SSR) ──
    location / {
        proxy_pass http://yukilog_frontend;
        proxy_http_version 1.1;
        proxy_set_header Host              \$host;
        proxy_set_header X-Real-IP         \$remote_addr;
        proxy_set_header X-Forwarded-For   \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_set_header Connection        "";

        # 超时
        proxy_connect_timeout 30s;
        proxy_read_timeout    60s;
        proxy_send_timeout    60s;
    }

    # ── 静态资源缓存 ──
    location /_astro/ {
        proxy_pass http://yukilog_frontend/_astro/;
        proxy_http_version 1.1;
        proxy_set_header Connection "";
        expires 30d;
        add_header Cache-Control "public, immutable";
    }

    # ── 安全头 ──
    add_header X-Frame-Options       "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff"    always;
    add_header X-XSS-Protection      "1; mode=block" always;

    # ── 日志 ──
    access_log /var/log/nginx/yukilog_access.log;
    error_log  /var/log/nginx/yukilog_error.log;
}
NGINXEOF
    info "Nginx 配置已生成"
fi

# 软链接
if [[ -L "$NGINX_LINK" ]]; then
    info "Nginx 软链已存在"
else
    warn "正在创建 Nginx 软链..."
    sudo ln -s "$NGINX_CONF" "$NGINX_LINK"
    info "Nginx 软链已创建"
fi

# 移除默认站点（如存在）
if [[ -L "/etc/nginx/sites-enabled/default" ]]; then
    warn "正在移除 Nginx 默认站点..."
    sudo rm -f /etc/nginx/sites-enabled/default
fi

# 测试并重载
if sudo nginx -t 2>&1; then
    sudo systemctl reload nginx
    info "Nginx 配置已重载"
else
    err "Nginx 配置测试失败，请手动检查"
fi

# ============================================================
#  第 9 步：SSL 证书 (Let's Encrypt)
# ============================================================
step "9/9  SSL 证书"

CERT_PATH="/etc/letsencrypt/live/${DOMAIN}"
if [[ -d "$CERT_PATH" ]]; then
    info "SSL 证书已存在 ($CERT_PATH)，跳过申请"
else
    warn "正在申请 Let's Encrypt SSL 证书..."
    if sudo certbot --nginx -d "$DOMAIN" --non-interactive --agree-tos \
         --register-unsafely-without-email --redirect; then
        info "SSL 证书申请成功，HTTPS 已启用"
    else
        err "SSL 证书申请失败"
        echo "  可能原因: 域名未解析到本机 / 80 端口不可达"
        echo "  稍后手动重试: sudo certbot --nginx -d ${DOMAIN}"
    fi
fi

# ── 设置 certbot 自动续期 ────────────────────────────────
if systemctl is-enabled --quiet certbot.timer 2>/dev/null; then
    info "certbot 自动续期已启用"
else
    sudo systemctl enable --now certbot.timer 2>/dev/null || \
        warn "certbot.timer 不可用，请手动配置续期 cron"
fi

# ============================================================
#  部署完成
# ============================================================
echo ""
echo -e "${GREEN}${BOLD}════════════════════════════════════════${NC}"
echo -e "${GREEN}${BOLD}  ✨ YukiLog 部署完成！${NC}"
echo -e "${GREEN}${BOLD}════════════════════════════════════════${NC}"
echo ""
echo -e "  网站地址:       ${CYAN}https://${DOMAIN}${NC}"
echo -e "  后端 API:       ${CYAN}https://${DOMAIN}/api${NC}"
echo -e "  管理后台:       ${CYAN}https://${DOMAIN}/admin/login${NC}"
echo -e "  管理员账号:     ${BOLD}${ADMIN_USERNAME}${NC}"
echo ""
echo -e "  ${BOLD}服务管理:${NC}"
echo "    sudo systemctl status  yukilog-backend"
echo "    sudo systemctl status  yukilog-hanakoi"
echo "    sudo systemctl restart yukilog-backend"
echo "    sudo systemctl restart yukilog-hanakoi"
echo ""
echo -e "  ${BOLD}查看日志:${NC}"
echo "    journalctl -u yukilog-backend -f"
echo "    journalctl -u yukilog-hanakoi -f"
echo "    tail -f /var/log/nginx/yukilog_access.log"
echo ""
echo -e "  ${BOLD}端口信息:${NC}"
echo "    后端: 127.0.0.1:${BACKEND_PORT}"
echo "    前端: 127.0.0.1:${FRONTEND_PORT}"
echo ""

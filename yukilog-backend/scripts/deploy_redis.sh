#!/usr/bin/env bash
set -e

echo "🍓 更新软件源..."
sudo apt update

echo "🍓 安装 redis-server..."
sudo apt install -y redis-server

REDIS_CONF="/etc/redis/redis.conf"

echo "🍓 确认 Redis 监听地址和端口..."

# 监听本地
sudo sed -i 's/^bind .*/bind 127.0.0.1 ::1/' "$REDIS_CONF"

# 端口 6379（默认就是这个，但我们还是显式保证一下）
sudo sed -i 's/^#\?port .*/port 6379/' "$REDIS_CONF"

# 作为 systemd 服务运行
sudo sed -i 's/^supervised .*/supervised systemd/' "$REDIS_CONF"

echo "🍓 重载并启动 Redis..."
sudo systemctl daemon-reexec
sudo systemctl enable redis-server
sudo systemctl restart redis-server

echo "🍓 Redis 状态："
sudo systemctl status redis-server --no-pager

echo "🍓 当前监听端口："
ss -lntp | grep 6379 || echo "⚠️ 没检测到 6379 端口"

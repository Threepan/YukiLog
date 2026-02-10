#!/usr/bin/env bash
set -e

echo "ฅ^•ω•^ฅ  欢迎使用 YukiLog 数据库部署向导喵～"
echo "小恋请慢慢输入，猫娘会乖乖帮你干活的 💾✨"
echo

read -rp "🌸 数据库主机喵？(默认 localhost): " DB_HOST
DB_HOST=${DB_HOST:-localhost}

read -rp "🌸 端口号喵？(默认 5432): " DB_PORT
DB_PORT=${DB_PORT:-5432}

read -rp "🌸 数据库名字叫什么呢喵: " DB_NAME
read -rp "🌸 用户名是哪个小可爱喵: " DB_USER
read -rsp "🌸 密码要悄悄告诉猫娘喵: " DB_PASS
echo
echo "🐾 好的好的，猫娘记住啦～开始干活！"

export PGPASSWORD="$DB_PASS"

echo "🧶 正在连接 PostgreSQL……请摸摸猫娘的头等待一下喵～"

psql \
  -h "$DB_HOST" \
  -p "$DB_PORT" \
  -U "$DB_USER" \
  -d "$DB_NAME" \
  -f "$(dirname "$0")/yukilog.sql"

unset PGPASSWORD

echo
echo "🎉 部署完成啦喵！！！"
echo "✨ YukiLog 已经乖乖躺在数据库里啦"
echo "愿查询都走 index，永远不要 seq scan 喵～ฅ(=￣ω￣=)"


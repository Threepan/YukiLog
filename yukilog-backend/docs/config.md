<div align="center">

# YukiLog Config 配置文档

这一层是具体的网络接口封装, 但是这个文档我只想说明 config 运行时配置部分

</div>

## .env 文件

这个文件是用来配置基本信息的, 因为我数据库没做用户表, 所以把鉴权相关项也放到这里了

接下来我简单讲解 `.env` 文件的配置项

完整注释请查看示例文件 [.env.example](../.env.example)

---

#### (1) `DATABASE_URL`

我的数据库使用了 `postgresql`, 所以这一项的格式就是

```ini
# 翻译: postgresql://用户名:密码@服务器地址:数据库服务端口/数据库
DATABASE_URL=postgresql://username:password@localhost:5432/yukilog
```

---

#### (2) `REDIS_URL`

`Redis` 用于 IP 限流和缓存, 防止恶意刷访问量和评论灌水

```ini
# 默认无密码
REDIS_URL=redis://localhost:6379

# 如果 Redis 设置了密码
# REDIS_URL=redis://:password@localhost:6379/0
```

**功能说明:**
* **浏览计数防刷**: 10 分钟内同一 IP 只计数一次
* **评论频率限制**: 10 秒内只能发一条评论
* **使用 TTL 自动过期**: 不需要手动清理缓存

---

#### (3) `SERVER_HOST` <-> `SERVER_PORT`

这一项是配置你的后端服务监听哪一块网卡, 哪一个端口

```ini
# 127.0.0.1 就是只监听本机的意思
SERVER_HOST=127.0.0.1
SERVER_PORT=3000
```

---

#### (4) `JWT_SECRET` <-> `JWT_EXPIRES_IN`

`JWT` 就是在登录成功后给你发一个凭证, 只要持有这个凭证就不用再次登录

```ini
JWT_SECRET=your-secret-key-change-this-in-production-use-openssl-rand
JWT_EXPIRES_IN=604800
```

生成 `JWT_SECRET`, 这个值是为了让后端前后校验, 确保 `token` 没有被篡改

```bash
openssl rand -base64 32 # 请至少生成 32 字节
```

---

#### (5) `ADMIN_USERNAME` <-> `ADMIN_PASSWORD_HASH`

管理员凭据, 因为没有做用户表, 所以直接存在配置文件里

```ini
ADMIN_USERNAME=admin
ADMIN_PASSWORD_HASH=$argon2id$v=19$m=19456,t=2,p=1$...
```

**生成密码哈希:**

```bash
# 使用我们的密码哈希工具
cargo run --bin hash_password -- your_password

# 输出会包含 ADMIN_PASSWORD_HASH=... 
# 复制到 .env 文件即可
```

**安全建议:**
* 密码至少 16 字符
* 包含大小写字母、数字、特殊字符
* 不要使用常见密码

---

#### (6) `CORS_ALLOWED_ORIGINS`

这一项是允许哪些前端域名访问我们的后端, 是用在浏览器环境的

```ini
CORS_ALLOWED_ORIGINS=http://localhost:5173,https://yourdomain.com
```

---

#### (7) `RUST_LOG`

这一项配置日志级别, 其实你不太可能来看日志(

```ini
RUST_LOG=info
```

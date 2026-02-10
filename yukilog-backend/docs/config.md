<div align="center">

# YukiLog Handler 层文档

这一层是具体的网络接口封装, 但是这个文档我只想说明 config 运行时配置部分

</div>

## .env 文件

这个文件是用来配置基本信息的, 因为我数据库没做用户表, 所以把鉴权相关项也放到这里了

接下来我简单讲解 `.env` 文件的配置项

完整注释请查看示例文件 [.env.example](../env.example)

---

#### (1) `DATABASE_URL`

我的数据库使用了 `postgresql`, 所以这一项的格式就是

```ini
# 翻译: postgresql://用户名:密码@服务器地址:数据库服务端口/数据库
DATABASE_URL=postgresql://username:password@localhost:5432/yukilog
```

---

#### (2) `SERVER_HOST` <-> `SERVER_PORT`

这一项是配置你的后端服务监听哪一块网卡, 哪一个端口

```ini
# 127.0.0.1 就是只监听本机的意思
SERVER_HOST=127.0.0.1
SERVER_PORT=3000
```

---

#### (3) `JWT_SECRET` <-> `JWT_EXPIRES_IN`

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

#### (4) `CORS_ALLOWED_ORIGINS`

这一项是允许哪些前端域名访问我们的后端, 是用在浏览器环境的

```ini
CORS_ALLOWED_ORIGINS=http://localhost:5173,https://yourdomain.com
```

---

#### (5) `RUST_LOG`

这一项配置日志级别, 其实你不太可能来看日志(

```ini
RUST_LOG=info
```

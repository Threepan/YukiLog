<div align="center">

# YukiLog APP 层文档

这个文档讲的就是我们的应用如何部署了

</div>

<div align="center">

## 运行状态

源码: [yukilog-backend/src/handler/state.rs](../src/handler/state.rs)

```rust
pub struct AppState {
    /// SeaORM 数据库连接
    pub db: DatabaseConnection,
    /// Redis 客户端（用于限流和缓存）
    pub redis: redis::Client,
}
```

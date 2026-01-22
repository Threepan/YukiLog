use axum::{routing::get, Router};
use sea_orm::{Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Serialize, Deserialize, Debug)]
struct SmokeTest {
    status: String,
    message: String,
    version: String,
}

#[tokio::main]
async fn main() {
    // 1. 初始化日志
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Setting default subscriber failed");

    info!("🚀 Yukilog-Backend 编译测试开始...");

    // 2. 测试 Serde
    let test_json = SmokeTest {
        status: "OK".to_string(),
        message: "Serde works!".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    info!("✅ 序列化测试: {:?}", test_json);

    // 3. 测试数据库连接 (尝试连接，但不闪退)
    let db_url = "postgres://lian:password@localhost:5432/yukilog";
    // 注意：Database::connect 是异步的
    let db_conn: Result<DatabaseConnection, _> = Database::connect(db_url).await;
    match db_conn {
        Ok(_) => info!("✅ 数据库连接成功！"),
        Err(e) => info!("⚠️ 数据库未就绪 (正常现象): {}", e),
    }

    // 4. 测试 Axum 路由
    let _app: Router = Router::new().route("/health", get(|| async { "Backend is Alive!" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("🌐 编译成功！Web 服务配置在: http://{}", addr);

    // 如果你想让程序运行而不立刻退出，可以取消下面三行的注释
    // info!("按 Ctrl+C 停止服务...");
    // let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}

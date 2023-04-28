use dotenv::dotenv;
use sea_orm::Database;
use std::sync::Arc;

use axum;
mod config;
pub mod entity;
mod err;
mod handler;
mod router;
mod state;
pub mod tool;

mod param;

pub use err::{AppError, AppErrorType};
pub type AppResult<T> = Result<T, AppError>;

#[tokio::main]
async fn main() {
    // 初始化日志
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "hiss=debug,sea_orm=debug");
    }

    tracing_subscriber::fmt::init();

    // 解析 .env 文件
    dotenv().ok();

    let cfg = config::Config::from_env().expect("初始化配置失败");

    let conn = Database::connect(&cfg.database_url).await.unwrap();

    tracing::info!("🚀 Server started successfully: {}", &cfg.web.addr);

    let app = router::init().layer(axum::Extension(Arc::new(state::AppState { conn })));

    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

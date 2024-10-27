use tokio;
use tracing::{error, info};

mod config;
pub mod crypto_tools;
mod data;
mod error;
mod exchange;
mod risk;
mod strategy;
mod trading_system;
mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();
    info!("Application started");

    // 加载配置
    let config = config::TradingConfig::load_trading_config("config.json").await?;
    info!("Configuration loaded successfully");

    // 初始化交易系统
    let trading_system = trading_system::TradingSystem::new(config).await?;

    // 启动交易循环
    trading_system.run().await?;

    Ok(())
}

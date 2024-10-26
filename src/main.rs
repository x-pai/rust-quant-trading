use tokio;
use tracing::{info, error};

mod config;
mod types;
mod data;
mod strategy;
mod risk;
mod exchange;
mod trading_system;
mod error;
pub mod crypto_tools;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    // 加载配置
    let config = config::TradingConfig::load_trading_config("").await?;
    info!("Configuration loaded successfully");
    
    // 初始化交易系统
    let trading_system = trading_system::TradingSystem::new(config).await?;
    
    // 启动交易循环
    trading_system.run().await?;
    
    Ok(())
}

use anyhow::Result;
use config::{Config, File};
use rust_decimal::Decimal;
use serde::Deserialize;
use tokio::fs;

#[derive(Debug, Deserialize)]
pub struct TradingConfig {
    pub api_key: String,
    pub api_secret: String,
    pub exchange: String,
    pub symbols: Vec<String>,
    pub risk_limits: RiskLimits,
    pub strategy_params: StrategyParams,
    pub notification: NotificationConfig,
}

impl TradingConfig {
    pub async fn load_trading_config(file_path: &str) -> Result<Self> {
        let config_data = fs::read_to_string(file_path).await?;
        let config: TradingConfig = serde_json::from_str(&config_data)?;
        Ok(config)
    }
}

#[derive(Debug, Deserialize)]
pub struct RiskLimits {
    pub max_position_size: Decimal,
    pub max_drawdown: Decimal,
    pub stop_loss_rate: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct StrategyParams {
    pub short_window: usize,
    pub long_window: usize,
}

#[derive(Deserialize, Debug)]
pub struct NotificationConfig {
    pub dingtalk: DingTalkConfig,
}

#[derive(Deserialize, Debug)]
pub struct DingTalkConfig {
    pub webhook: String,
    pub secret: String,
}

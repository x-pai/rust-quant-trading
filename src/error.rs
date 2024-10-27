use thiserror::Error;

#[derive(Error, Debug)]
pub enum TradingError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Strategy error: {0}")]
    StrategyError(String),

    #[error("Risk check failed: {0}")]
    RiskError(String),

    #[error("Parse failed: {0}")]
    ParseError(String),
}

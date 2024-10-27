use std::fmt;
use thiserror::Error;

// pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Notification error: {0}")]
    NotificationError(String),

    #[error("Strategy error: {0}")]
    StrategyError(String),

    #[error("Risk check failed: {0}")]
    RiskError(String),

    #[error("Parse failed: {0}")]
    ParseError(String),

    #[error("Time error: {0}")]
    TimeError(#[from] std::time::SystemTimeError),

    #[error("Unknown error: {0}")]
    UnknownError(String),
}

impl Error {
    pub fn notification(msg: impl Into<String>) -> Self {
        Error::NotificationError(msg.into())
    }

    pub fn strategy(msg: impl Into<String>) -> Self {
        Error::StrategyError(msg.into())
    }

    pub fn config(msg: impl Into<String>) -> Self {
        Error::ConfigError(msg.into())
    }
}

// 实现从字符串转换为Error的能力
impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::UnknownError(err.to_string())
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::UnknownError(err)
    }
}

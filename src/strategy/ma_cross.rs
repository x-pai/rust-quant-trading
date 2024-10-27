use super::Strategy;

use async_trait::async_trait;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use std::sync::Arc;

use crate::config::TradingConfig;
use crate::error::Error;
use crate::notification::{dingtalk::DingTalkBot, NotificationMessage};
use crate::types::{Kline, Signal};

// 为策略信号定义消息格式
#[derive(Debug)]
pub struct MaCrossSignal {
    pub symbol: String,
    pub direction: String,
    pub price: f64,
    pub timestamp: i64,
}

impl NotificationMessage for MaCrossSignal {
    fn to_notification_message(&self) -> String {
        format!(
            "MA交叉信号提醒\n币对: {}\n方向: {}\n价格: {}\n时间: {}",
            self.symbol, self.direction, self.price, self.timestamp
        )
    }
}

#[derive(Debug)]
pub struct MACrossStrategy {
    config: Arc<TradingConfig>,
    dingtalk: DingTalkBot,
}

impl MACrossStrategy {
    pub fn new(config: Arc<TradingConfig>) -> Self {
        Self {
            dingtalk: DingTalkBot::new(
                config.notification.dingtalk.webhook.clone(),
                config.notification.dingtalk.secret.clone(),
            ),
            config,
            // ... 其他字段初始化 ...
        }
    }

    /// 计算给定窗口大小的移动平均值
    pub fn calculate_ma(&self, data: &[Kline], window_size: usize) -> Option<f64> {
        if data.len() < window_size {
            return None;
        }

        // 计算收盘价的总和
        let sum: Decimal = data[data.len() - window_size..]
            .iter()
            .map(|kline| kline.close)
            .sum();

        // 将 Decimal 转换为 f64，并计算平均值
        sum.to_f64().map(|avg| avg / window_size as f64)
    }

    async fn on_signal(&self, signal: MaCrossSignal) -> Result<(), Error> {
        // 发送信号到钉钉
        self.dingtalk
            .send_text(signal.to_notification_message())
            .await?;

        // 处理其他信号逻辑...
        Ok(())
    }
}

#[async_trait]
impl Strategy for MACrossStrategy {
    async fn generate_signal(&self, data: &[Kline]) -> Result<Signal, Error> {
        if data.len() < self.config.strategy_params.long_window as usize {
            return Err(Error::StrategyError("Insufficient data".to_string()));
        }

        let short_ma = self.calculate_ma(data, self.config.strategy_params.short_window);
        let long_ma = self.calculate_ma(data, self.config.strategy_params.long_window);

        match (short_ma, long_ma) {
            (Some(short), Some(long)) => {
                if short > long {
                    Ok(Signal::Buy)
                } else if short < long {
                    Ok(Signal::Sell)
                } else {
                    Ok(Signal::Hold)
                }
            }
            _ => Err(Error::StrategyError("MA calculation failed".to_string())),
        }
    }

    fn calculate_position_size(&self, price: Decimal) -> Decimal {
        // 实现仓位计算逻辑
        self.config.risk_limits.max_position_size / price
    }
}

use super::signal::{Direction, MACrossSignal};
use async_trait::async_trait;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use std::sync::Arc;

use crate::config::TradingConfig;
use crate::error::Error;
use crate::notification::{dingtalk::DingTalkBot, NotificationMessage};
use crate::strategy::Strategy;
use crate::types::{Kline, Signal};

pub struct MACrossStrategy {
    // symbol: String,
    // fast_period: usize,
    // slow_period: usize,
    dingtalk: DingTalkBot,
    // last_cross: Option<Direction>,
    config: Arc<TradingConfig>,
}

impl MACrossStrategy {
    pub fn new(dingtalk: DingTalkBot, config: Arc<TradingConfig>) -> Self {
        Self { dingtalk, config }
    }

    pub async fn on_kline(&mut self, kline: &Kline) -> Result<Option<MACrossSignal>, Error> {
        // 策略逻辑实现...
        Ok(None)
    }

    async fn send_signal(&self, signal: MACrossSignal) -> Result<(), Error> {
        self.dingtalk
            .send_text(signal.to_notification_message())
            .await?;
        Ok(())
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

    async fn on_signal(&self, signal: MACrossSignal) -> Result<(), Error> {
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


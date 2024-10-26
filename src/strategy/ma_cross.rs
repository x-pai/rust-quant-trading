use super::Strategy;

use async_trait::async_trait;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use std::sync::Arc;

use crate::config::TradingConfig;
use crate::error::TradingError;
use crate::types::{Kline, Signal};

#[derive(Debug)]
pub struct MACrossStrategy {
    config: Arc<TradingConfig>,
}

impl MACrossStrategy {
    pub fn new(config: Arc<TradingConfig>) -> Self {
        MACrossStrategy { config }
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
}

#[async_trait]
impl Strategy for MACrossStrategy {
    async fn generate_signal(&self, data: &[Kline]) -> Result<Signal, TradingError> {
        if data.len() < self.config.strategy_params.long_window as usize {
            return Err(TradingError::StrategyError("Insufficient data".to_string()));
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
            _ => Err(TradingError::StrategyError(
                "MA calculation failed".to_string(),
            )),
        }
    }

    fn calculate_position_size(&self, price: Decimal) -> Decimal {
        // 实现仓位计算逻辑
        self.config.risk_limits.max_position_size / price
    }
}

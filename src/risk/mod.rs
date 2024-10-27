use crate::config::TradingConfig;
use crate::error::TradingError;
use crate::types::Position;

use rust_decimal::Decimal;
use std::collections::HashMap;
use std::sync::Arc;

pub struct RiskManager {
    config: Arc<TradingConfig>,
    positions: HashMap<String, Position>,
}

impl RiskManager {
    pub fn new(config: Arc<TradingConfig>) -> Self {
        Self {
            config,
            positions: HashMap::new(),
        }
    }

    pub fn check_risk(
        &self,
        symbol: &str,
        size: Decimal,
        price: Decimal,
    ) -> Result<bool, TradingError> {
        // 检查持仓限制
        let total_exposure: Decimal = self
            .positions
            .values()
            .map(|p| p.size * p.current_price)
            .sum();

        if total_exposure + (size * price) > self.config.risk_limits.max_position_size {
            return Ok(false);
        }

        // 检查回撤限制
        let total_pnl: Decimal = self.positions.values().map(|p| p.unrealized_pnl).sum();

        if total_pnl < -self.config.risk_limits.max_drawdown {
            return Ok(false);
        }

        Ok(true)
    }

    pub fn calculate_stop_loss(&self, entry_price: Decimal) -> Decimal {
        entry_price * (Decimal::ONE - self.config.risk_limits.stop_loss_rate)
    }
}

mod ma_cross;
pub use ma_cross::MACrossStrategy;

use rust_decimal::Decimal;
use crate::error::TradingError;

use async_trait::async_trait;
use crate::types::{Kline, Signal};

#[async_trait]
pub trait Strategy {
    async fn generate_signal(&self, data: &[Kline]) -> Result<Signal, TradingError>;
    fn calculate_position_size(&self, price: Decimal) -> Decimal;
}
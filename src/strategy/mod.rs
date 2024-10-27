mod ma_cross;
pub use ma_cross::MACrossStrategy;

use crate::error::TradingError;
use rust_decimal::Decimal;

use crate::types::{Kline, Signal};
use async_trait::async_trait;

#[async_trait]
pub trait Strategy {
    async fn generate_signal(&self, data: &[Kline]) -> Result<Signal, TradingError>;
    fn calculate_position_size(&self, price: Decimal) -> Decimal;
}

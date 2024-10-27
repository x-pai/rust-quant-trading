mod binance;
pub use binance::BinanceExchange;

mod tushare;
pub use tushare::TushareExchange;

use async_trait::async_trait;
use rust_decimal::Decimal;
use std::collections::HashMap;

use crate::error::Error;
use crate::types::{Order, OrderType, Position};

#[async_trait]
pub trait Exchange: Send + Sync {
    async fn place_order(
        &self,
        symbol: &str,
        order_type: OrderType,
        size: Decimal,
        price: Option<Decimal>,
    ) -> Result<Order, Error>;

    async fn cancel_order(&self, order_id: &str) -> Result<(), Error>;

    async fn get_positions(&self) -> Result<Vec<Position>, Error>;

    async fn get_balance(&self) -> Result<HashMap<String, Decimal>, Error>;
}

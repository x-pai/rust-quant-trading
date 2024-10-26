mod binance;

pub use binance::BinanceExchange;
use async_trait::async_trait;
use rust_decimal::Decimal;
use std::collections::HashMap;

use crate::types::{Order, OrderType, Position};
use crate::error::TradingError;

#[async_trait]
pub trait Exchange: Send + Sync {
    async fn place_order(
        &self, 
        symbol: &str, 
        order_type: OrderType, 
        size: Decimal, 
        price: Option<Decimal>
    ) -> Result<Order, TradingError>;
    
    async fn cancel_order(&self, order_id: &str) -> Result<(), TradingError>;
    
    async fn get_positions(&self) -> Result<Vec<Position>, TradingError>;
    
    async fn get_balance(&self) -> Result<HashMap<String, Decimal>, TradingError>;
}
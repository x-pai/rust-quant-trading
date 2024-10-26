use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt; // 引入 fmt 模块

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kline {
    pub timestamp: DateTime<Utc>,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub symbol: String,
    pub size: Decimal,
    pub entry_price: Decimal,
    pub current_price: Decimal,
    pub unrealized_pnl: Decimal,
}

#[derive(Debug, Clone)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}

#[derive(Debug, Clone)]
pub enum OrderType {
    Buy,
    Sell,
    Hold,
    StopLoss,
}

// 实现 Display trait
impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderType::Buy => write!(f, "buy"),
            OrderType::Sell => write!(f, "sell"),
            OrderType::Hold => write!(f, "hold"),
            OrderType::StopLoss => write!(f, "stop_loss"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Order {
    pub order_id: String,
    pub symbol: String,
    pub order_type: OrderType, // Assuming OrderType is an enum you defined
    pub size: Decimal,
    pub price: Option<Decimal>,
    pub status: String, // Adjust type as necessary based on binance_order.status type
}

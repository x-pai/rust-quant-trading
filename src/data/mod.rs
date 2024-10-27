mod binance;
pub use binance::BinanceDataFetcher;

mod tushare;
pub use tushare::TushareDataFetcher;

use crate::error::TradingError;
use crate::types::Kline;
use async_trait::async_trait;
use rust_decimal::Decimal;

#[async_trait]
pub trait DataFetcher {
    async fn fetch(&self) -> Result<Vec<Kline>, TradingError>;
    async fn fetch_klines(
        &self,
        symbol: &str,
        interval: &str,
        limit: u32,
    ) -> Result<Vec<Kline>, TradingError>;
    async fn fetch_ticker(&self, symbol: &str) -> Result<Decimal, TradingError>;
}

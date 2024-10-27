// src/data_fetchers/binance.rs
use super::DataFetcher;

use async_trait::async_trait;
use rust_decimal::Decimal;
use std::sync::Arc;

use crate::config::TradingConfig;
use crate::error::Error;
use crate::types::Kline;

#[derive(Debug)]
pub struct BinanceDataFetcher {
    config: Arc<TradingConfig>,
}

impl BinanceDataFetcher {
    pub fn new(config: Arc<TradingConfig>) -> Self {
        BinanceDataFetcher { config }
    }

    pub async fn fetch_data(&self) -> Result<Vec<Kline>, Error> {
        // 在这里实现从 Binance 获取数据的逻辑
        Ok(vec![]) // 返回模拟数据或实际 API 数据
    }
}

#[async_trait]
impl DataFetcher for BinanceDataFetcher {
    async fn fetch(&self) -> Result<Vec<Kline>, Error> {
        self.fetch_data().await
    }

    async fn fetch_klines(
        &self,
        symbol: &str,
        interval: &str,
        limit: u32,
    ) -> Result<Vec<Kline>, Error> {
        // let url = format!("{}/api/v3/klines", self.base_url);
        // let response = self.client
        //     .get(&url)
        //     .query(&[
        //         ("symbol", symbol),
        //         ("interval", interval),
        //         ("limit", &limit.to_string()),
        //     ])
        //     .send()
        //     .await
        //     .map_err(|e| Error::NetworkError(e.to_string()))?;

        // let klines: Vec<BinanceKlineResponse> = response
        //     .json()
        //     .await
        //     .map_err(|e| Error::ParseError(e.to_string()))?;

        // // 将 Binance API 返回的数据转换为 Vec<Kline>
        // let result = klines
        //     .into_iter()
        //     .map(|k| Kline {
        //         timestamp: chrono::Utc.timestamp_millis(k.0),
        //         open: k.1.parse().unwrap_or(Decimal::ZERO),
        //         high: k.2.parse().unwrap_or(Decimal::ZERO),
        //         low: k.3.parse().unwrap_or(Decimal::ZERO),
        //         close: k.4.parse().unwrap_or(Decimal::ZERO),
        //         volume: k.5.parse().unwrap_or(Decimal::ZERO),
        //     })
        //     .collect();

        // Ok(result)
        Ok(vec![])
    }

    async fn fetch_ticker(&self, symbol: &str) -> Result<Decimal, Error> {
        // let url = format!("{}/api/v3/ticker/price", self.base_url);
        // let response = self.client
        //     .get(&url)
        //     .query(&[("symbol", symbol)])
        //     .send()
        //     .await
        //     .map_err(|e| Error::NetworkError(e.to_string()))?;

        // // 从响应中提取价格
        // #[derive(Deserialize)]
        // struct PriceResponse {
        //     price: String,
        // }

        // let price_data: PriceResponse = response
        //     .json()
        //     .await
        //     .map_err(|e| Error::ParseError(e.to_string()))?;

        // let price = price_data
        //     .price
        //     .parse::<Decimal>()
        //     .map_err(|e| Error::ParseError(e.to_string()))?;

        // Ok(price)
        Ok(Decimal::new(0, 2))
    }
}

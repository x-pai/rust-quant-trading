// src/data/tushare.rs
use super::DataFetcher;

use crate::error::Error; // 自定义错误类型
use crate::types::Kline; // 假设 Kline 在 types.rs 中定义
use async_trait::async_trait;
use chrono::prelude::*;
use reqwest::Client;
use rust_decimal::Decimal;
use serde_json::json;
use std::str::FromStr;

pub struct TushareDataFetcher {
    api_key: String,
    client: Client,
}

impl TushareDataFetcher {
    pub fn new(api_key: String) -> Self {
        TushareDataFetcher {
            api_key,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl DataFetcher for TushareDataFetcher {
    async fn fetch(&self) -> Result<Vec<Kline>, Error> {
        Err(Error::ApiError("Empty data".to_string())) // 这里可以留空或实现一个基础的 fetch
    }

    async fn fetch_klines(
        &self,
        symbol: &str,
        interval: &str,
        limit: u32,
    ) -> Result<Vec<Kline>, Error> {
        let url = "https://api.tushare.pro";
        let request_body = json!({
            "api_name": "daily",
            "token": self.api_key,
            "params": {
                "ts_code": symbol,
                "start_date": "20220101", // 示例开始日期
                "end_date": "20221231", // 示例结束日期
                "limit": limit
            },
            "fields": "trade_date,open,high,low,close,vol"
        });

        let response = self
            .client
            .post(url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| Error::ApiError(e.to_string()))?
            .json::<serde_json::Value>()
            .await
            .map_err(|e| Error::ApiError(e.to_string()))?;

        // 解析响应数据
        let mut klines = Vec::new();
        if let Some(items) = response["data"]["items"].as_array() {
            for item in items {
                let kline = Kline {
                    timestamp: DateTime::parse_from_str(item[0].as_str().unwrap_or(""), "%Y%m%d")
                        .map_err(|e| Error::ParseError(e.to_string()))?
                        .with_timezone(&Utc),
                    open: Decimal::from_str(item[1].as_str().unwrap_or("0")).unwrap_or_default(),
                    high: Decimal::from_str(item[2].as_str().unwrap_or("0")).unwrap_or_default(),
                    low: Decimal::from_str(item[3].as_str().unwrap_or("0")).unwrap_or_default(),
                    close: Decimal::from_str(item[4].as_str().unwrap_or("0")).unwrap_or_default(),
                    volume: Decimal::from_str(item[5].as_str().unwrap_or("0")).unwrap_or_default(),
                };
                klines.push(kline);
            }
        }
        Ok(klines)
    }

    async fn fetch_ticker(&self, symbol: &str) -> Result<Decimal, Error> {
        let url = "https://api.tushare.pro";
        let request_body = json!({
            "api_name": "daily_basic",
            "token": self.api_key,
            "params": { "ts_code": symbol },
            "fields": "close"
        });

        let response = self
            .client
            .post(url)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| Error::ApiError(e.to_string()))?
            .json::<serde_json::Value>()
            .await
            .map_err(|e| Error::ParseError(e.to_string()))?;

        let close_price_str = response["data"]["items"][0][0]
            .as_str()
            .ok_or_else(|| Error::ApiError("Missing price data".to_string()))?;

        Decimal::from_str(close_price_str).map_err(|e| Error::ParseError(e.to_string()))
    }
}

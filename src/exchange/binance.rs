use super::Exchange;
use async_trait::async_trait;
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::config::TradingConfig;
use crate::crypto_tools;
use crate::error::TradingError;
use crate::types::{Order, OrderType, Position};

pub struct BinanceExchange {
    client: Client,
    config: Arc<TradingConfig>,
    base_url: String,
}

impl BinanceExchange {
    pub fn new(config: Arc<TradingConfig>) -> Self {
        let client = Client::new();
        Self {
            client,
            config,
            base_url: "https://api.binance.com".to_string(),
        }
    }

    async fn sign_request(&self, params: &mut HashMap<String, String>) -> String {
        params.insert(
            "timestamp".to_string(),
            chrono::Utc::now().timestamp_millis().to_string(),
        );

        let query = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&");

        let signature = crypto_tools::hmac_sha256(&self.config.api_secret, &query);
        format!("{}&signature={}", query, signature)
    }
}

#[async_trait]
impl Exchange for BinanceExchange {
    async fn place_order(
        &self,
        symbol: &str,
        order_type: OrderType,
        size: Decimal,
        price: Option<Decimal>,
    ) -> Result<Order, TradingError> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("side".to_string(), order_type.to_string());
        params.insert("type".to_string(), "MARKET".to_string());
        params.insert("quantity".to_string(), size.to_string());

        if let Some(price) = price {
            params.insert("price".to_string(), price.to_string());
        }

        let query = self.sign_request(&mut params).await;
        let url = format!("{}/api/v3/order?{}", self.base_url, query);

        let response = self
            .client
            .post(&url)
            .send()
            .await
            .map_err(|e| TradingError::ApiError(e.to_string()))?;

        let binance_order: BinanceOrder = response
            .json()
            .await
            .map_err(|e| TradingError::ApiError(e.to_string()))?;

        Ok(Order {
            order_id: binance_order.orderId.to_string(),
            symbol: binance_order.symbol,
            order_type,
            size,
            price,
            status: binance_order.status,
        })
    }

    async fn cancel_order(&self, order_id: &str) -> Result<(), TradingError> {
        let mut params = HashMap::new();
        params.insert("orderId".to_string(), order_id.to_string());

        let query = self.sign_request(&mut params).await;
        let url = format!("{}/api/v3/order?{}", self.base_url, query);

        self.client
            .delete(&url)
            .send()
            .await
            .map_err(|e| TradingError::ApiError(e.to_string()))?;

        Ok(())
    }

    async fn get_positions(&self) -> Result<Vec<Position>, TradingError> {
        let mut params = HashMap::new();
        let query = self.sign_request(&mut params).await;
        let url = format!("{}/api/v3/account?{}", self.base_url, query);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| TradingError::ApiError(e.to_string()))?;

        let account: BinanceAccount = response
            .json()
            .await
            .map_err(|e| TradingError::ApiError(e.to_string()))?;

        Ok(account
            .balances
            .into_iter()
            .filter(|balance| balance.free > Decimal::ZERO)
            .map(|balance| Position {
                symbol: balance.asset,
                size: balance.free,
                entry_price: Decimal::ZERO,    // 需要从其他API获取
                current_price: Decimal::ZERO,  // 需要从其他API获取
                unrealized_pnl: Decimal::ZERO, // 需要计算
            })
            .collect())
    }

    async fn get_balance(&self) -> Result<HashMap<String, Decimal>, TradingError> {
        let mut params = HashMap::new();
        let query = self.sign_request(&mut params).await;
        let url = format!("{}/api/v3/account?{}", self.base_url, query);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| TradingError::ApiError(e.to_string()))?;

        let account: BinanceAccount = response
            .json()
            .await
            .map_err(|e| TradingError::ApiError(e.to_string()))?;

        Ok(account
            .balances
            .into_iter()
            .map(|balance| (balance.asset, balance.free))
            .collect())
    }
}

#[derive(Deserialize)]
struct BinanceAccount {
    balances: Vec<BinanceBalance>,
}

#[derive(Deserialize)]
struct BinanceBalance {
    asset: String,
    free: Decimal,
    locked: Decimal,
}

#[derive(Deserialize)]
struct BinanceOrder {
    orderId: u64,
    symbol: String,
    status: String,
}

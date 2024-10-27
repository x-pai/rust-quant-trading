// src/trading_system.rs
use rust_decimal::Decimal;
use std::sync::Arc;
use std::time::Duration;
use tokio;
use tracing::{error, info};

use crate::config::TradingConfig;
use crate::data::DataFetcher;
use crate::error::Error;
use crate::exchange::BinanceExchange;
use crate::exchange::Exchange;
use crate::risk::RiskManager;
// use crate::strategy::MACrossStrategy;
use crate::data::BinanceDataFetcher;
use crate::strategy::MACrossStrategy;
use crate::strategy::Strategy;
use crate::types::{Order, OrderType, Signal};

pub struct TradingSystem {
    config: Arc<TradingConfig>,
    exchange: Box<dyn Exchange>,
    strategy: Box<dyn Strategy>,
    risk_manager: RiskManager,
    data_fetcher: Box<dyn DataFetcher>,
}

impl TradingSystem {
    pub async fn new(config: TradingConfig) -> Result<Self, Error> {
        let config = Arc::new(config);

        // 初始化交易所连接
        let exchange = match config.exchange.as_str() {
            "binance" => Box::new(BinanceExchange::new(config.clone())) as Box<dyn Exchange>,
            _ => {
                error!("Unsupported exchange");
                return Err(Error::ConfigError("Unsupported exchange".to_string()));
            }
        };

        // 初始化策略
        let strategy = Box::new(MACrossStrategy::new(config.clone())) as Box<dyn Strategy>;

        // 初始化风险管理器
        let risk_manager = RiskManager::new(config.clone());

        // 初始化数据获取器
        let data_fetcher =
            Box::new(BinanceDataFetcher::new(config.clone())) as Box<dyn DataFetcher>;

        Ok(Self {
            config,
            exchange,
            strategy,
            risk_manager,
            data_fetcher,
        })
    }

    pub async fn run(&self) -> Result<(), Error> {
        info!("Starting trading system...");

        loop {
            for symbol in &self.config.symbols {
                match self.process_symbol(symbol).await {
                    Ok(_) => info!("Successfully processed symbol: {}", symbol),
                    Err(e) => error!("Error processing symbol {}: {:?}", symbol, e),
                }
            }

            // 等待下一个交易周期
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    }

    async fn process_symbol(&self, symbol: &str) -> Result<(), Error> {
        // 获取市场数据
        let klines = self.data_fetcher.fetch_klines(symbol, "1m", 1000).await?;

        // 生成交易信号
        let signal = self.strategy.generate_signal(&klines).await?;

        // 获取当前价格
        let current_price = self.data_fetcher.fetch_ticker(symbol).await?;

        // 根据信号执行交易
        match signal {
            Signal::Buy => {
                self.handle_buy_signal(symbol, current_price).await?;
            }
            Signal::Sell => {
                self.handle_sell_signal(symbol, current_price).await?;
            }
            Signal::Hold => {
                info!("Hold position for {}", symbol);
            }
        }

        Ok(())
    }

    async fn handle_buy_signal(&self, symbol: &str, price: Decimal) -> Result<(), Error> {
        // 计算建议仓位大小
        let size = self.strategy.calculate_position_size(price);

        // 风险检查
        if self.risk_manager.check_risk(symbol, size, price)? {
            // 执行买入订单
            let order = self
                .exchange
                .place_order(symbol, OrderType::Buy, size, None)
                .await?;

            info!("Buy order executed: {:?}", order);

            // 设置止损
            let stop_loss = self.risk_manager.calculate_stop_loss(price);
            self.exchange
                .place_order(symbol, OrderType::StopLoss, size, Some(stop_loss))
                .await?;

            info!("Stop loss order placed at {}", stop_loss);
        } else {
            info!("Risk check failed for buy signal on {}", symbol);
        }

        Ok(())
    }

    async fn handle_sell_signal(&self, symbol: &str, price: Decimal) -> Result<(), Error> {
        // 获取当前持仓
        let positions = self.exchange.get_positions().await?;

        if let Some(position) = positions.iter().find(|p| p.symbol == symbol) {
            // 执行卖出订单
            let order = self
                .exchange
                .place_order(symbol, OrderType::Sell, position.size, None)
                .await?;

            info!("Sell order executed: {:?}", order);
        } else {
            info!("No position to sell for {}", symbol);
        }

        Ok(())
    }
}

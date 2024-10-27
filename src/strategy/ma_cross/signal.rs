use crate::error::Error;
use crate::notification::NotificationMessage;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use std::sync::Once;
use tracing_subscriber;
use tracing::{error, info};

static INIT: Once = Once::new();

fn init_tracing() {
    INIT.call_once(|| {
        tracing_subscriber::fmt::init();
    });
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MACrossSignal {
    pub symbol: String,
    pub direction: Direction,
    pub price: f64,
    pub timestamp: i64,
    pub fast_ma: f64,
    pub slow_ma: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}

impl MACrossSignal {
    /// 创建新的MA交叉信号
    pub fn new(
        symbol: impl Into<String>,
        direction: Direction,
        price: f64,
        fast_ma: f64,
        slow_ma: f64,
        timestamp: i64,
    ) -> Result<Self, Error> {
        // 验证输入参数
        if price <= 0.0 {
            return Err("Price must be positive".into());
        }
        if fast_ma <= 0.0 || slow_ma <= 0.0 {
            return Err("MA values must be positive".into());
        }
        if timestamp <= 0 {
            return Err("Timestamp must be positive".into());
        }

        Ok(Self {
            symbol: symbol.into(),
            direction,
            price,
            timestamp,
            fast_ma,
            slow_ma,
            volume: None,
            additional_info: None,
        })
    }

    /// 使用当前时间创建信号
    pub fn now(
        symbol: impl Into<String>,
        direction: Direction,
        price: f64,
        fast_ma: f64,
        slow_ma: f64,
    ) -> Result<Self, Error> {
        let timestamp = Utc::now().timestamp();
        Self::new(symbol, direction, price, fast_ma, slow_ma, timestamp)
    }

    /// 添加成交量信息
    pub fn with_volume(mut self, volume: f64) -> Result<Self, Error> {
        if volume < 0.0 {
            return Err("Volume cannot be negative".into());
        }
        self.volume = Some(volume);
        Ok(self)
    }

    /// 添加额外信息
    pub fn with_info(mut self, info: impl Into<String>) -> Self {
        self.additional_info = Some(info.into());
        self
    }

    /// 获取格式化的时间
    pub fn formatted_time(&self) -> String {
        let dt = DateTime::<Utc>::from_timestamp(self.timestamp, 0).unwrap_or_else(|| Utc::now());
        dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
    }

    /// 计算MA差值
    pub fn ma_difference(&self) -> f64 {
        self.fast_ma - self.slow_ma
    }

    /// 判断是否是有效的交叉
    pub fn is_valid_cross(&self, min_difference: f64) -> bool {
        self.ma_difference().abs() >= min_difference
    }

    /// 创建多头信号的便捷方法
    pub fn long(
        symbol: impl Into<String>,
        price: f64,
        fast_ma: f64,
        slow_ma: f64,
    ) -> Result<Self, Error> {
        Self::now(symbol, Direction::Long, price, fast_ma, slow_ma)
    }

    /// 创建空头信号的便捷方法
    pub fn short(
        symbol: impl Into<String>,
        price: f64,
        fast_ma: f64,
        slow_ma: f64,
    ) -> Result<Self, Error> {
        Self::now(symbol, Direction::Short, price, fast_ma, slow_ma)
    }

    /// 创建平仓信号的便捷方法
    pub fn exit(
        symbol: impl Into<String>,
        price: f64,
        fast_ma: f64,
        slow_ma: f64,
    ) -> Result<Self, Error> {
        Self::now(symbol, Direction::Exit, price, fast_ma, slow_ma)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    Long,  // 金叉，做多信号
    Short, // 死叉，做空信号
    Exit,  // 平仓信号
}

impl Direction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Direction::Long => "多头开仓",
            Direction::Short => "空头开仓",
            Direction::Exit => "平仓",
        }
    }

    pub fn is_entry(&self) -> bool {
        matches!(self, Direction::Long | Direction::Short)
    }
}

#[async_trait]
impl NotificationMessage for MACrossSignal {
    fn to_notification_message(&self) -> String {
        let mut msg = format!(
            "MA交叉信号提醒\n\
             币对: {}\n\
             方向: {}\n\
             价格: {:.2}\n\
             快线: {:.2}\n\
             慢线: {:.2}\n\
             时间: {}",
            self.symbol,
            self.direction.as_str(),
            self.price,
            self.fast_ma,
            self.slow_ma,
            self.formatted_time(),
        );

        // 添加可选信息
        if let Some(volume) = self.volume {
            msg.push_str(&format!("\n成交量: {:.2}", volume));
        }

        if let Some(info) = &self.additional_info {
            msg.push_str(&format!("\n备注: {}", info));
        }

        info!(msg);
        msg
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_creation() -> Result<(), Error> {
        // 基本创建测试
        let signal = MACrossSignal::new(
            "BTC/USDT",
            Direction::Long,
            50000.0,
            49800.0,
            49600.0,
            1634567890,
        )?;

        assert_eq!(signal.symbol, "BTC/USDT");
        assert!(matches!(signal.direction, Direction::Long));
        assert_eq!(signal.price, 50000.0);

        // 验证错误情况
        assert!(MACrossSignal::new(
            "BTC/USDT",
            Direction::Long,
            -1.0, // 负价格
            49800.0,
            49600.0,
            1634567890,
        )
        .is_err());

        Ok(())
    }

    #[test]
    fn test_convenience_methods() -> Result<(), Error> {
        // 测试多头信号创建
        let long_signal = MACrossSignal::long("BTC/USDT", 50000.0, 49800.0, 49600.0)?;
        assert!(matches!(long_signal.direction, Direction::Long));

        // 测试空头信号创建
        let short_signal = MACrossSignal::short("BTC/USDT", 50000.0, 49800.0, 49600.0)?;
        assert!(matches!(short_signal.direction, Direction::Short));

        // 测试添加额外信息
        let signal_with_info = long_signal.with_volume(1000.0)?.with_info("强势突破");

        assert_eq!(signal_with_info.volume, Some(1000.0));
        assert_eq!(
            signal_with_info.additional_info,
            Some("强势突破".to_string())
        );

        Ok(())
    }

    #[test]
    fn test_notification_message() -> Result<(), Error> {
        init_tracing();
        let signal = MACrossSignal::new(
            "BTC/USDT",
            Direction::Long,
            50000.0,
            49800.0,
            49600.0,
            1634567890,
        )?
        .with_volume(1000.0)?
        .with_info("突破重要阻力位");

        let message = signal.to_notification_message();

        // 验证消息包含所有必要信息
        assert!(message.contains("BTC/USDT"));
        assert!(message.contains("多头开仓"));
        assert!(message.contains("50000.00"));
        assert!(message.contains("1000.00"));
        assert!(message.contains("突破重要阻力位"));

        Ok(())
    }

    #[test]
    fn test_ma_calculations() -> Result<(), Error> {
        let signal = MACrossSignal::new(
            "BTC/USDT",
            Direction::Long,
            50000.0,
            49800.0,
            49600.0,
            1634567890,
        )?;

        assert_eq!(signal.ma_difference(), 200.0);
        assert!(signal.is_valid_cross(100.0));
        assert!(!signal.is_valid_cross(300.0));

        Ok(())
    }
}

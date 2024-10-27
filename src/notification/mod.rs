pub mod dingtalk;

use async_trait::async_trait;

// 为策略消息定义一个trait
#[async_trait]
pub trait NotificationMessage {
    fn to_notification_message(&self) -> String;
}

use base64::{engine::general_purpose::STANDARD, Engine};
use hmac::{Hmac, Mac};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::Error; // 自定义错误类型

#[derive(Debug, Clone)]
pub struct DingTalkBot {
    webhook: String,
    secret: String,
    client: Client,
}

#[derive(Debug, Serialize)]
struct DingTalkMessage {
    msgtype: String,
    text: TextContent,
}

#[derive(Debug, Serialize)]
struct TextContent {
    content: String,
}

// Markdown 消息的内容结构
#[derive(Debug, Serialize)]
struct DingTalkMarkdownMessage {
    msgtype: String,
    markdown: MarkdownContent,
}

// Markdown 内容结构
#[derive(Debug, Serialize)]
struct MarkdownContent {
    title: String,
    text: String,
}

impl DingTalkBot {
    pub fn new(webhook: String, secret: String) -> Self {
        Self {
            webhook,
            secret,
            client: Client::new(),
        }
    }

    fn generate_signature(&self, timestamp: i64) -> String {
        let string_to_sign = format!("{}\n{}", timestamp, self.secret);
        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(string_to_sign.as_bytes());
        let result = mac.finalize();
        STANDARD.encode(result.into_bytes())
    }

    pub async fn send_text(&self, content: String) -> Result<(), Error> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as i64;

        let sign = self.generate_signature(timestamp);

        let webhook_with_params = format!("{}&timestamp={}&sign={}", self.webhook, timestamp, sign);

        let message = DingTalkMessage {
            msgtype: "text".to_string(),
            text: TextContent { content },
        };

        let response = self
            .client
            .post(&webhook_with_params)
            .json(&message)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err("Failed to send DingTalk message".into());
        }

        Ok(())
    }

    // 发送 Markdown 消息的方法
    pub async fn send_markdown(&self, title: String, text: String) -> Result<(), Error> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as i64;

        let sign = self.generate_signature(timestamp);

        let webhook_with_params = format!("{}&timestamp={}&sign={}", self.webhook, timestamp, sign);

        let markdown_message = DingTalkMarkdownMessage {
            msgtype: "markdown".to_string(),
            markdown: MarkdownContent { title, text },
        };

        let response = self
            .client
            .post(&webhook_with_params)
            .json(&markdown_message)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err("Failed to send DingTalk markdown message".into());
        }

        Ok(())
    }
}

#[tokio::test]
async fn test_send_text() -> anyhow::Result<()> {
    // 加载配置
    use crate::config::TradingConfig;
    let config = TradingConfig::load_trading_config("config.json").await?;

    // 设置你的 webhook 和 secret
    let webhook = config.notification.dingtalk.webhook;
    let secret = config.notification.dingtalk.secret;


    let bot = DingTalkBot::new(webhook.clone(), secret.clone());

    // 尝试发送真实消息
    let result = bot.send_text("Test message from Rust!".to_string()).await;

    // 检查结果是否成功
    match result {
        Ok(_) => {
            println!("Message sent successfully!");
        }
        Err(e) => {
            eprintln!("Failed to send message: {:?}", e);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_send_markdown() -> anyhow::Result<()> {

    // 加载配置
    use crate::config::TradingConfig;
    let config = TradingConfig::load_trading_config("config.json").await?;

    // 设置你的 webhook 和 secret
    let webhook = config.notification.dingtalk.webhook;
    let secret = config.notification.dingtalk.secret;

    let bot = DingTalkBot::new(webhook.clone(), secret.clone());

    // 尝试发送真实消息
    let title = "Test Markdown Message".to_string();
    let text = "### This is a Markdown message from Rust!\n\n* Bullet point 1\n* Bullet point 2\n\n[Click here](https://example.com)".to_string();

    let result = bot.send_markdown(title, text).await;

    // 检查结果是否成功
    match result {
        Ok(_) => {
            println!("Message sent successfully!");
        }
        Err(e) => {
            eprintln!("Failed to send message: {:?}", e);
        }
    }

    Ok(())
}

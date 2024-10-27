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
}

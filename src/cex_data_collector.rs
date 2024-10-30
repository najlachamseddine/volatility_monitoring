use std::str::FromStr;

use eyre::Result;
use tokio::sync::mpsc::Sender;

use async_trait::async_trait;
use futures::StreamExt;
use serde_json::Value;
use tokio_tungstenite::{connect_async, tungstenite::client::IntoClientRequest};
use rust_decimal::Decimal;

#[async_trait]
pub trait CexPool {
    async fn fetch_cex_prices(&self, tx: Sender<Decimal>) -> Result<(), Box<dyn std::error::Error>>;
    fn process_data_event(&self, event: Value) -> Result<Decimal, Box<dyn std::error::Error>>;
}

pub struct BinanceApi {
    base_url: String,
}

impl BinanceApi {
    pub fn new(url: &str) -> Self {
        Self {
            base_url: url.to_string(),
        }
    }
}

#[async_trait]
impl CexPool for BinanceApi {
    async fn fetch_cex_prices(&self, tx: Sender<Decimal>) -> Result<(), Box<dyn std::error::Error>> {
        let request = self.base_url.to_string().into_client_request().unwrap();
        let (stream, _) = connect_async(request).await.unwrap();
        let (_, mut read) = stream.split();

        while let Some(msg) = read.next().await {
            let message = msg?;
            let data: Value = serde_json::from_str(message.to_text()?)?;
            let price = self.process_data_event(data)?;
            let _ = tx.send(price).await;
        }

        Ok(())
    }

    // add option (some/none) and for the unwrap (ok and err)
    fn process_data_event(&self, data: Value) -> Result<Decimal, Box<dyn std::error::Error>>  {
        if let Some(kline) = data["k"].as_object() {
            let close = Decimal::from_str(kline["c"].as_str().unwrap())?;
            return Ok(close);
        }
            Err(format!("Error in processing data to retrieve price for binance").into())
    }
}
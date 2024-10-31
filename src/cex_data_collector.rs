use std::str::FromStr;
use alloy::primitives::U256;
use eyre::Result;
use tokio::sync::mpsc::Sender;
use async_trait::async_trait;
use futures::StreamExt;
use serde_json::Value;
use tokio_tungstenite::{connect_async, tungstenite::client::IntoClientRequest};
use crate::utils::PriceData;
use rust_decimal::Decimal;

#[async_trait]
pub trait CexPool {
    async fn fetch_cex_prices(&self, tx: Sender<PriceData>) -> Result<(), Box<dyn std::error::Error>>;
    fn process_data_event(&self, event: Value) -> Result<U256, Box<dyn std::error::Error>>;
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
    async fn fetch_cex_prices(&self, tx: Sender<PriceData>) -> Result<(), Box<dyn std::error::Error>> {
        let request = self.base_url.to_string().into_client_request().unwrap();
        let (stream, _) = connect_async(request).await.unwrap();
        let (_, mut read) = stream.split();

        while let Some(msg) = read.next().await {
            let message = msg?;
            let data: Value = serde_json::from_str(message.to_text()?)?;
            let price = self.process_data_event(data)?;
            let _ = tx.send(PriceData::new(price, false)).await;
        }

        Ok(())
    }

    fn process_data_event(&self, data: Value) -> Result<U256, Box<dyn std::error::Error>>  {
        if let Some(kline) = data["k"].as_object() {
            let close = Decimal::from_str(kline["c"].as_str().unwrap())? * Decimal::from(10i32.pow(6));
            let close_int = close.trunc().to_string();
            return Ok(U256::from_str(&close_int).unwrap());
        }
            Err(format!("Error in processing data to retrieve price for binance").into())
    }
}
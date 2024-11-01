use crate::utils::PriceData;
use alloy::primitives::U256;
use async_trait::async_trait;
use eyre::Result;
use futures::StreamExt;
use rust_decimal::Decimal;
use serde_json::Value;
use std::str::FromStr;
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::{connect_async, tungstenite::client::IntoClientRequest};
use tracing::{error, info};

#[async_trait]
pub trait CexPool {
    async fn fetch_cex_prices(
        &self,
        tx: Sender<PriceData>,
    ) -> Result<(), Box<dyn std::error::Error>>;
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
    async fn fetch_cex_prices(
        &self,
        tx: Sender<PriceData>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = self.base_url.to_string().into_client_request()?;
        loop {
            match connect_async(request.clone()).await {
                Ok((stream, _response)) => {
                    let (_, mut read) = stream.split();
                    while let Some(message) = read.next().await {
                        match message {
                            Ok(msg) => {
                                let data: Value = serde_json::from_str(msg.to_text()?)?;
                                let price = self.process_data_event(data)?;
                                let _ = tx.send(PriceData::new(price, false)).await;
                            }
                            Err(e) => {
                                info!("Connection error: {}. Reconnecting ", e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => error!("Failed to connect: {}", e),
            }
        }
    }

    fn process_data_event(&self, data: Value) -> Result<U256, Box<dyn std::error::Error>> {
        if let Some(kline) = data["k"].as_object() {
            let close =
                Decimal::from_str(kline["c"].as_str().unwrap())? * Decimal::from(10i32.pow(6));
            let close_int = close.trunc().to_string();
            return Ok(U256::from_str(&close_int).unwrap());
        }
        Err(format!("Error in processing data to retrieve price for binance").into())
    }
}

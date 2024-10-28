use eyre::Result;
use tokio::sync::mpsc::Sender;

use async_trait::async_trait;
use serde_json::Value;
use tokio_tungstenite::{connect_async, tungstenite::client::IntoClientRequest};
use futures::StreamExt;

#[async_trait]
pub trait CexPool {
    async fn fetch_cex_prices(&self, tx: Sender<Value>) -> Result<(), Box<dyn std::error::Error>>;
    // async fn get_pool_address(&self) -> Address;
    // async fn get_price(&self) -> Result<U256, Box<dyn std::error::Error>>;
}

pub struct BinanceApi {
    base_url: String
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
    async fn fetch_cex_prices(&self, tx: Sender<Value>) -> Result<(), Box<dyn std::error::Error>> {
        let request = self.base_url.to_string().into_client_request().unwrap();
        let (stream, _) = connect_async(request).await.unwrap();
        let (_, mut read) = stream.split();

        while let Some(msg) = read.next().await {
            let message = msg?;
            let data: Value = serde_json::from_str(message.to_text()?)?;

            let _ = tx.send(data).await;

            // if let Some(kline) = data["k"].as_object() {
            //     let open_time = kline["t"].as_i64().unwrap_or(0);
            //     let open = kline["o"].as_str().unwrap_or("0.0");
            //     let high = kline["h"].as_str().unwrap_or("0.0");
            //     let low = kline["l"].as_str().unwrap_or("0.0");
            //     let close = kline["c"].as_str().unwrap_or("0.0");

            //     println!(
            //         "Time: {}, Open: {}, High: {}, Low: {}, Close: {}",
            //         open_time, open, high, low, close
            //     );

            // }
        }

        Ok(())
    }
}

// pub async fn fetch_cex_prices(
//     provider_url: &str,
//     tx: Sender<Value>,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     // <Result<(), Box <dyn error>>
//     // let url = Url::parse(provider_url).unwrap();
//     let request = provider_url.into_client_request().unwrap();
//     let (stream, _) = connect_async(request).await.unwrap();
//     let (_, mut read) = stream.split();

//     while let Some(msg) = read.next().await {
//         let message = msg?;
//         let data: Value = serde_json::from_str(message.to_text()?)?;

//         let _ = tx.send(data).await;

//         // if let Some(kline) = data["k"].as_object() {
//         //     let open_time = kline["t"].as_i64().unwrap_or(0);
//         //     let open = kline["o"].as_str().unwrap_or("0.0");
//         //     let high = kline["h"].as_str().unwrap_or("0.0");
//         //     let low = kline["l"].as_str().unwrap_or("0.0");
//         //     let close = kline["c"].as_str().unwrap_or("0.0");

//         //     println!(
//         //         "Time: {}, Open: {}, High: {}, Low: {}, Close: {}",
//         //         open_time, open, high, low, close
//         //     );

//         // }
//     }

//     Ok(())
// }

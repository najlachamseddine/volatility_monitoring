use crate::uniswap_v3_pool::*;
use ethers::{
    providers::{Provider, StreamExt, Ws},
    types::U64,
};
use eyre::Result;
use tokio::sync::mpsc::Sender;
use tokio::time::{timeout, Duration};

use tokio_tungstenite::{connect_async, tungstenite::client::IntoClientRequest};
use serde_json::Value;
use async_trait::async_trait;

#[async_trait]
pub trait DexPool {
    async fn fetch_dex_prices(&self, tx: Sender<SwapFilter>, last_block: U64) -> Result<(), Box<dyn std::error::Error>>;
    // async fn get_pool_address(&self) -> Address;
    // async fn get_price(&self) -> Result<U256, Box<dyn std::error::Error>>;
}

#[async_trait]
impl DexPool for UniswapV3Pool<Provider<Ws>> {
    async fn fetch_dex_prices(&self, tx: Sender<SwapFilter>, last_block: U64) -> Result<(), Box<dyn std::error::Error>> {
        let events = self.event::<SwapFilter>().from_block(last_block);
        let mut stream = events.stream().await.unwrap();
         println!("FETCH DEX PRICES OF THE TRAIT!!");
        // TO DO match X {OK or Err(e)} https://rishabh.io/building-a-rusty-websocket-server-4f3ba4b6b19c
        while let Some(Ok(evt)) = timeout(Duration::from_secs(3600), stream.next())
            .await
            .unwrap_or(None)
        {
            println!("{:#?}", evt);
            let _ = tx.send(evt).await.expect("send my message ");
        }
        Ok(())
    }
}


// pub async fn fetch_dex_prices(
//     provider_url: &str,
//     pool_address: &str,
//     tx: Sender<SwapFilter>,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let provider = Provider::<Ws>::connect(provider_url)
//         .await
//         .unwrap()
//         .interval(Duration::from_millis(50u64));
//     let client = Arc::new(provider);
//     let address = pool_address.parse::<Address>()?;
//     let uniswapv3 = UniswapV3Pool::new(address, Arc::clone(&client));
//     let last_block = client.get_block_number().await?;

//     let events = uniswapv3.event::<SwapFilter>().from_block(last_block);
//     let mut stream = events.stream().await.unwrap();

//     // TO DO match X {OK or Err(e)} https://rishabh.io/building-a-rusty-websocket-server-4f3ba4b6b19c
//     while let Some(Ok(evt)) = timeout(Duration::from_secs(3600), stream.next())
//         .await
//         .unwrap_or(None)
//     {
//         // println!("{:#?}", evt);
//         let _ = tx.send(evt).await.expect("send my message ");
//     }

//     Ok(())
// }

pub async fn fetch_cex_prices(provider_url: &str, tx: Sender<Value>) -> Result<(), Box<dyn std::error::Error>> {
    // <Result<(), Box <dyn error>>
    // let url = Url::parse(provider_url).unwrap();
    let request = provider_url.into_client_request().unwrap();
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

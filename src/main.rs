use std::error::Error;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use volatility_monitoring::cex_data_collector::*;
use volatility_monitoring::dex_data_collector::*;

use ethers::providers::{Middleware, Provider, Ws};
use ethers::types::Address;
use eyre::Result;
use serde_json::Value;
use std::sync::Arc;
use tokio::time::Duration;
use volatility_monitoring::uniswap_v3_pool::*;

const WSS_URL: &str = "wss://arb-mainnet.g.alchemy.com/v2/aZbQQOCV8cExXR7Y0mrzLz4rz1wLDqCB";
const WETH_USDC_POOL_UNISWAP: &str = "0xC6962004f452bE9203591991D15f6b388e09E8D0";
const BINANCE_WSS_URL: &str = "wss://stream.binance.com:9443/ws/ethusdc@kline_1s";
const TIME_WINDOW: i32 = 360;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, mut rx): (Sender<SwapFilter>, Receiver<SwapFilter>) = mpsc::channel(32);
    let (tx2, mut rx2): (Sender<Value>, Receiver<Value>) = mpsc::channel(32);

    let provider = Provider::<Ws>::connect(WSS_URL)
        .await
        .unwrap()
        .interval(Duration::from_millis(50u64));
    let client = Arc::new(provider);
    let address = WETH_USDC_POOL_UNISWAP.parse::<Address>()?;
    let uniswap_pool = UniswapV3Pool::new(address, Arc::clone(&client));
    let last_block = client.get_block_number().await?;

    tokio::spawn(async move {
        keep_connection_alive(client).await;
    });

    tokio::spawn(async move {
        let _ = uniswap_pool.fetch_dex_prices(tx, last_block).await;
    });

    let binance_api = BinanceApi::new(BINANCE_WSS_URL);
    tokio::spawn(async move{
        let _ = binance_api.fetch_cex_prices(tx2).await;
    });

    // while let Some(value) = rx.recv().await {
    //     println!("Dequeued uniswapv3: {}", value.sqrt_price_x96);
    // }

    while let Some(value) = rx2.recv().await {
        println!("Dequeued binance: {}", value);
    }

    Ok(())
}

async fn keep_connection_alive(client: Arc<Provider<Ws>>) {
    let mut interval = tokio::time::interval(Duration::from_millis(30));
    loop {
        interval.tick().await;
        if let Err(e) = client.get_block_number().await {
            eprintln!("Keep-alive ping failed: {:?}", e);
        }
    }
}

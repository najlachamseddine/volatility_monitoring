use std::error::Error;
use bigdecimal::BigDecimal;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use volatility_monitoring::cex_data_collector::*;
use volatility_monitoring::dex_data_collector::*;

use ethers::providers::{Middleware, Provider, Ws};
use ethers::types::{Address};
use eyre::Result;
use serde_json::Value;
use std::sync::Arc;
use tokio::time::Duration;
use volatility_monitoring::uniswap_v3_pool::*;
use rust_decimal::Decimal;

use alloy::primitives::address;
use alloy::providers::{ProviderBuilder, WsConnect};
use volatility_monitoring::utils::Pool; 
use alloy::primitives::U256;


const WSS_URL: &str = "wss://arb-mainnet.g.alchemy.com/v2/aZbQQOCV8cExXR7Y0mrzLz4rz1wLDqCB";
const WETH_USDC_POOL_UNISWAP: &str = "0xC6962004f452bE9203591991D15f6b388e09E8D0";
const BINANCE_WSS_URL: &str = "wss://stream.binance.com:9443/ws/ethusdc@kline_1s";
const TIME_WINDOW: i32 = 360;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, mut rx): (Sender<String>, Receiver<String>) = mpsc::channel(32);
    let (tx2, mut rx2): (Sender<Decimal>, Receiver<Decimal>) = mpsc::channel(32);
    let mut prices: Vec<f64> = Vec::new();

    // let provider = Provider::<Ws>::connect(WSS_URL)
    //     .await
    //     .unwrap()
    //     .interval(Duration::from_millis(50u64));
    // let client = Arc::new(provider);
    // let address = WETH_USDC_POOL_UNISWAP.parse::<Address>()?;
    // let uniswap_pool = UniswapV3Pool::new(address, Arc::clone(&client));
    // let last_block = client.get_block_number().await?;

    // let period = Duration::from_secs(6 * 3600);
    // let interval = Duration::from_secs(60);
    // let start_time = std::time::Instant::now();


    // alloy
    let uniswap_token_pool = Pool::new(address!("C6962004f452bE9203591991D15f6b388e09E8D0"), WSS_URL.to_string());

    // tokio::spawn(async move {
    //     keep_connection_alive(client).await;
    // });

    // tokio::spawn(async move {
    //     let _ = uniswap_pool.fetch_dex_prices(tx, last_block).await;
    // });

    tokio::spawn(async move {
        let _ = uniswap_token_pool.fetch_dex_prices_alloy(tx).await;
    });

    // let binance_api = BinanceApi::new(BINANCE_WSS_URL);
    // tokio::spawn(async move {
    //     let _ = binance_api.fetch_cex_prices(tx2).await;
    // });

    while let Some(value) = rx.recv().await {
        println!("Dequeued uniswapv3: {}", value);
    }

    // while let Some(value) = rx2.recv().await {
    //     println!("Dequeued binance: {}", value);
    // }

    // while let Some(value) = rx.recv().await {
    // if start_time.elapsed() < period {

    // }
    // }

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

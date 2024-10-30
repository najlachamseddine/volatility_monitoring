use std::error::Error;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use volatility_monitoring::cex_data_collector::*;
use volatility_monitoring::dex_data_collector::*;
use eyre::Result;
use alloy::primitives::address;
use volatility_monitoring::utils::{Pool, PriceData}; 

const WSS_URL: &str = "wss://arb-mainnet.g.alchemy.com/v2/aZbQQOCV8cExXR7Y0mrzLz4rz1wLDqCB";
const WETH_USDC_POOL_UNISWAP: &str = "0xC6962004f452bE9203591991D15f6b388e09E8D0";
const BINANCE_WSS_URL: &str = "wss://stream.binance.com:9443/ws/ethusdc@kline_1s";
const TIME_WINDOW: i32 = 360;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, mut rx): (Sender<PriceData>, Receiver<PriceData>) = mpsc::channel(32);
    let tx_clone = tx.clone();

    let uniswap_token_pool = Pool::new(address!("C6962004f452bE9203591991D15f6b388e09E8D0"), WSS_URL.to_string());

    tokio::spawn(async move {
        let _ = uniswap_token_pool.fetch_dex_prices_alloy(tx).await;
    });

    let binance_api = BinanceApi::new(BINANCE_WSS_URL);
    tokio::spawn(async move {
        let _ = binance_api.fetch_cex_prices(tx_clone).await;
    });

    while let Some(value) = rx.recv().await {
        println!("Dequeued uniswapv3: {:?}", value);
    }

    // while let Some(value) = rx.recv().await {
    // if start_time.elapsed() < period {

    // }
    // }

    Ok(())
}


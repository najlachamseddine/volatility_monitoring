use std::error::Error;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use volatility_monitoring::data_collector::*;

use ethers::providers::{Middleware, Provider, Ws};
use eyre::Result;
use std::sync::Arc;
use tokio::time::Duration;
use volatility_monitoring::uniswap_v3_pool::*;

const WSS_URL: &str = "wss://arb-mainnet.g.alchemy.com/v2/aZbQQOCV8cExXR7Y0mrzLz4rz1wLDqCB";
const WETH_USDC_POOL: &str = "0xC6962004f452bE9203591991D15f6b388e09E8D0"; // get be requested with getPool from UniswapV3Factory

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, mut rx): (Sender<SwapFilter>, Receiver<SwapFilter>) = mpsc::channel(32);
    
    let provider = Provider::<Ws>::connect(WSS_URL)
        .await
        .unwrap()
        .interval(Duration::from_millis(50u64));
    let client = Arc::new(provider);

    tokio::spawn(async move {
        keep_connection_alive(client).await;
    });

    fetch_uniswapv3_prices(WSS_URL, WETH_USDC_POOL, tx).await?;

    while let Some(value) = rx.recv().await {
        println!("Dequeued: {}", value.sqrt_price_x96);
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
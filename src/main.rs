use std::error::Error;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use volatility_monitoring::on_chain::*;

use ethers::{
    contract::stream::EventStream,
    providers::{Middleware, Provider, StreamExt, Ws},
    types::Address,
};
use eyre::Result;
use std::sync::Arc;
use tokio::time::{timeout, Duration};
use volatility_monitoring::uniswap_v3_pool::*;

// const WSS_URL: &str = "wss://arbitrum-mainnet.infura.io/ws/v3/9a36ca959f654f67b5cfbbea5f07d18f";
const WSS_URL: &str = "wss://arb-mainnet.g.alchemy.com/v2/aZbQQOCV8cExXR7Y0mrzLz4rz1wLDqCB";
// const WSS_URL: &str = "wss://mainnet.infura.io/ws/v3/c60b0bb42f8a4c6481ecd229eddaca27";
const WETH_USDC_POOL: &str = "0xC6962004f452bE9203591991D15f6b388e09E8D0"; // get be requested with getPool from UniswapV3Factory

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, mut rx): (Sender<f64>, Receiver<f64>) = mpsc::channel(32);
    
    let provider = Provider::<Ws>::connect(WSS_URL)
        .await
        .unwrap()
        .interval(Duration::from_millis(50u64));
    let client = Arc::new(provider);
    // println!("{:#?}", client);
    // let address = WETH_USDC_POOL.parse::<Address>().unwrap();
    // let uniswapv3 = UniswapV3Pool::new(address, Arc::clone(&client));
    // println!(">>>>>>>>>>>>>>>>> uniswapv3 address is {uniswapv3:?}");
    // let last_block = client.get_block_number().await.unwrap();
    // println!("{}", last_block);

    tokio::spawn(async move {
        keep_connection_alive(client).await;
    });

    fetch_uniswapv3_prices(WSS_URL, WETH_USDC_POOL, tx).await?;

    while let Some(value) = rx.recv().await {
        println!("Dequeued: {}", value);
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
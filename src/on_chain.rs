use crate::uniswap_v3_pool::*;
use ethers::{
    contract::stream::EventStream,
    providers::{Middleware, Provider, StreamExt, Ws},
    types::Address,
};
use eyre::Result;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::time::{timeout, Duration};

pub async fn fetch_uniswapv3_prices(
    provider_url: &str,
    pool_address: &str,
    tx: Sender<SwapFilter>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("toto1");
    let provider = Provider::<Ws>::connect(provider_url)
        .await
        .unwrap()
        .interval(Duration::from_millis(50u64));
    let client = Arc::new(provider);
    println!("{:#?}", client);
    let address = pool_address.parse::<Address>()?;
    let uniswapv3 = UniswapV3Pool::new(address, Arc::clone(&client));
    let last_block = client.get_block_number().await?;

    let _s = tokio::spawn(async move {
        let events = uniswapv3.event::<SwapFilter>().from_block(last_block);
        let mut stream = events.stream().await.unwrap();
        println!("toto2");
        while let Some(Ok(evt)) = timeout(Duration::from_secs(3600), stream.next())
            .await
            .unwrap_or(None)
        {
            println!("{:#?}", evt);
            // tokio::spawn(async move {
            let _ = tx.send(evt).await.expect("send my message ");
            // });
        }
    });
    Ok(())
}

use crate::uniswap_v3_pool::*;
use ethers::{
    contract::stream::EventStream, providers::{Middleware, Provider, StreamExt, Ws}, types::Address
};
use eyre::Result;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::time::{timeout, Duration};

pub async fn fetch_uniswapv3_prices(
    provider_url: &str,
    pool_address: &str,
    tx: Sender<f64>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("toto1");
    let provider = Provider::<Ws>::connect(provider_url).await.unwrap().interval(Duration::from_millis(50u64));
    let client = Arc::new(provider);
    println!("{:#?}", client);
    let address = pool_address.parse::<Address>()?;
    let uniswapv3 = UniswapV3Pool::new(address, Arc::clone(&client));
    println!(">>>>>>>>>>>>>>>>> uniswapv3 address is {uniswapv3:?}");
    let last_block = client.get_block_number().await?;
    println!("{}", last_block);

    let s =  listen_specific_events(&uniswapv3).await.unwrap();

    // let s = tokio::spawn(async move {
    //     let events = uniswapv3.event::<SwapFilter>().from_block(last_block);
    //     let event2 = uniswapv3.events();
    //     let _stream1 = event2.stream().await.expect("get an event stream 1");
    //     // println!("{:#?}", event);
    //     let _stream2 = events.stream().await;
    // })
    // .await
    // .unwrap();
    // println!("toto2");
    // while let Some(Ok(evt)) = timeout(Duration::from_secs(360), stream2.next())
    //     .await
    //     .unwrap_or(None)
    // {
    //     println!("{:#?}", evt);
    //     let _ = tx.send(2.0).await.expect("send my message ");
    // }

    Ok(())
}


async fn listen_specific_events(contract: &UniswapV3Pool<Provider<Ws>>) -> Result<()> {
    let events = contract.events();
    let stream = events.stream().await.unwrap();

    let _ = Box::new(stream);

    // while let Some(Ok(f)) = stream.next().await {
    //     println!("SwapFilter event: {f:?}");
    // }

    Ok(())
}
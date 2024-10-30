use crate::uniswap_v3_pool::*;
use alloy_pubsub::PubSubFrontend;
use async_trait::async_trait;
// use ethers::{
//     providers::{Provider, StreamExt, Ws},
//     types::{U256, U64},
// };
use eyre::Result;
use tokio::sync::mpsc::Sender;
use tokio::time::{timeout, Duration};

use alloy::primitives::{U256, U160};
use alloy::{
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::{BlockNumberOrTag, Filter},
    sol,
    sol_types::SolEvent
};
use futures_util::stream::StreamExt;
use crate::utils::Pool;
use uniswap_v3_math::full_math::mul_div;
use ethers::abi::Uint;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UniswapV3Pool,
    "./src/uniswapV3Pool.json"
);

#[async_trait]
pub trait DexPool {
    // async fn fetch_dex_prices(
    //     &self,
    //     tx: Sender<U256>,
    //     last_block: U64,
    // ) -> Result<(), Box<dyn std::error::Error>>;
    async fn fetch_dex_prices_alloy(
        &self,
        tx: Sender<String>,
    ) -> Result<(), Box<dyn std::error::Error>>;
    async fn process_data_event(&self, sqrt_price: U160) -> U256;
}

#[async_trait]
impl DexPool for Pool {
    // async fn fetch_dex_prices(
    //     &self,
    //     tx: Sender<U256>,
    //     last_block: U64,
    // ) -> Result<(), Box<dyn std::error::Error>> {
    //     let events = self.event::<SwapFilter>().from_block(last_block);
    //     let mut stream = events.stream().await.unwrap();
    //     println!("FETCH DEX PRICES OF THE TRAIT!!");
    //     // TO DO match X {OK or Err(e)} https://rishabh.io/building-a-rusty-websocket-server-4f3ba4b6b19c
    //     while let Some(Ok(evt)) = timeout(Duration::from_secs(3600), stream.next())
    //         .await
    //         .unwrap_or(None)
    //     {
    //         println!("{:#?}", evt);
    //         let price = self.process_data_event(evt).await;
    //         let _ = tx.send(price).await.expect("send my message ");
    //     }
    //     Ok(())
    // }

    async fn fetch_dex_prices_alloy(
        &self,
        tx: Sender<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ws = WsConnect::new(&self.rpc_url);
        let uniswap_token_address = self.addr;
        let provider = ProviderBuilder::new().on_ws(ws).await?;
        let filter = Filter::new()
            .address(uniswap_token_address)
            .event("Swap(address,address,int256,int256,uint160,uint128,int24)")
            .from_block(BlockNumberOrTag::Latest);

        // Subscribe to logs.
        let sub = provider.subscribe_logs(&filter).await?;
        let mut stream = sub.into_stream();

        while let Some(log) = stream.next().await {
            let UniswapV3Pool::Swap {sender, recipient, amount0, amount1, sqrtPriceX96, liquidity, tick} = log.log_decode()?.inner.data;
            println!("Uniswap token logs {}", sqrtPriceX96);
            let price = self.process_data_event(sqrtPriceX96).await;
            println!("PRICE !!! {}", price);
            let _ = tx.send("toto".to_string()).await.expect("send my message ");
        }

        Ok(())
    }

    // Result<f64, dyn Error>
    // check overflow
    async fn process_data_event(&self, sqrt_price_x96: U160) -> U256 {
        let q192 = U256::from(1) << 192;
        // let sqrt_price_x96 = event.sqrt_price_x96;
        // // let q96_big = BigDecimal::from_str(&q192.to_string()).unwrap();
        // // let sqrt_price =
        // //     BigDecimal::from_str(&(sqrt_price_x96 * sqrt_price_x96).to_string()).unwrap();
        // // let price = q96_big / sqrt_price;
        // // println!("q192 {} ", q192);
        // // let price = (sqrt_price_x96 * sqrt_price_x96) >> (96 * 2);
        let price = mul_div(U256::from(sqrt_price_x96 * sqrt_price_x96), U256::from(10).pow(U256::from(18)), q192).unwrap();
        price
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

use crate::utils::{Pool, PriceData};
use alloy::primitives::{U160, U256};
use alloy::{
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::{BlockNumberOrTag, Filter},
    sol,
};
use async_trait::async_trait;
use eyre::Result;
use futures_util::stream::StreamExt;
use tokio::sync::mpsc::Sender;
use tracing::{error, info};
use uniswap_v3_math::full_math::mul_div;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    UniswapV3Pool,
    "./src/uniswapV3Pool.json"
);

#[async_trait]
pub trait DexPool {
    async fn fetch_dex_prices(
        &self,
        tx: Sender<PriceData>,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn process_data_event(
        &self,
        sqrt_price: U160,
    ) -> Result<U256, Box<dyn std::error::Error + Send>>;
}

#[async_trait]
impl DexPool for Pool {
    async fn fetch_dex_prices(
        &self,
        tx: Sender<PriceData>,
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
            let UniswapV3Pool::Swap {
                sqrtPriceX96,
                ..
            } = log.log_decode()?.inner.data;
            match self.process_data_event(sqrtPriceX96) {
                Ok(price) => {
                    let body = tx.send(PriceData::new(price, true)).await;
                    match body {
                        Ok(body) => info!("Data sent through channel {:?}", body),
                        Err(e) => error!("Channel error: {}", e),
                    };
                }
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    fn process_data_event(
        &self,
        sqrt_price_x96: U160,
    ) -> Result<U256, Box<dyn std::error::Error + Send>> {
        let price = mul_div(
            U256::from(sqrt_price_x96) * U256::from(sqrt_price_x96),
            U256::from(10).pow(U256::from(18)),
            U256::from(1) << 192,
        );
        match price {
            Ok(p) => Ok(p),
            Err(e) => Err(Box::new(e)),
        }
    }
}

use alloy::primitives::address;
use chrono::prelude::*;
use eyre::Result;
use std::collections::VecDeque;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use volatility_monitoring::cex_data_collector::*;
use volatility_monitoring::dex_data_collector::*;
use volatility_monitoring::math::{compute_deviation, compute_ln_return};
use volatility_monitoring::utils::{Pool, PriceData};

const ARB_WSS_URL: &str = "wss://arb-mainnet.g.alchemy.com/v2/aZbQQOCV8cExXR7Y0mrzLz4rz1wLDqCB";
const BASE_WSS_URL: &str = "wss://base-mainnet.g.alchemy.com/v2/aZbQQOCV8cExXR7Y0mrzLz4rz1wLDqCB";
const WETH_USDC_POOL_UNISWAP: &str = "0xC6962004f452bE9203591991D15f6b388e09E8D0";
const BINANCE_WSS_URL: &str = "wss://stream.binance.com:9443/ws/ethusdc@kline_1s";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, mut rx): (Sender<PriceData>, Receiver<PriceData>) = mpsc::channel(32);
    // let tx_clone = tx.clone();
    // let tx_clone_two = tx.clone();
    let period = 360;
    let interval = Duration::from_secs(60);
    let prices_in_minute = Arc::new(Mutex::new(Vec::new()));
    let mut prices_in_period: Vec<f64> = Vec::new();
    let ln_returns = Arc::new(Mutex::new(VecDeque::new()));
    let mut previous_price = f64::from(-1);

    // let uniswap_token_pool = Pool::new(
    //     address!("C6962004f452bE9203591991D15f6b388e09E8D0"),
    //     ARB_WSS_URL.to_string(),
    // );
    // let sushiswap_token_pool = Pool::new(
    //     address!("57713F7716e0b0F65ec116912F834E49805480d2"),
    //     BASE_WSS_URL.to_string(),
    // );
    // tokio::spawn(async move {
    //     let _ = uniswap_token_pool.fetch_dex_prices(tx).await;
    // });

    // tokio::spawn(async move {
    //     let _ = sushiswap_token_pool.fetch_dex_prices(tx_clone).await;
    // });

    // let binance_api = BinanceApi::new(BINANCE_WSS_URL);
    // tokio::spawn(async move {
    //     let _ = binance_api.fetch_cex_prices(tx_clone_two).await;
    // });

    set_up(tx).await?;

    let prices_minutes_clone = prices_in_minute.clone();
    tokio::spawn(async move {
        loop {
            println!("--------Waiting for one minute to elapse--------");
            tokio::time::sleep(interval).await;
            let mut p = prices_minutes_clone.lock().unwrap();
            if p.len() > 0 {
                let price_t: f64 = p.iter().sum::<f64>() / (p.len() as f64);
                println!(
                    "PRICE AT T MEAN OF ALL THE VALUES ON ONE MINUTE {} ",
                    price_t
                );
                p.clear();
                if previous_price > f64::from(0) {
                    println!("--------TIME T>0--------");
                    let ln_price_t = compute_ln_return(previous_price, price_t);
                    let mut ln_ret = ln_returns.lock().unwrap();
                    if ln_ret.len() == period {
                        ln_ret.pop_front();
                    }
                    ln_ret.push_back(ln_price_t);
                    let variance = 360f64.sqrt() * compute_deviation(ln_ret);
                    println!(
                        "--------New Deviation At {:#?}; Estimated volatility: {}% --------",
                        Local::now(),
                        variance * 100f64
                    );
                    previous_price = price_t;
                    prices_in_period.push(price_t);
                    println!("PRICE AT T ADDED  {:#?} ", prices_in_period);
                } else {
                    println!("--------TIME T==0--------");
                    previous_price = price_t;
                    prices_in_period.push(price_t);
                    println!("PRICE AT T ADDED  {:#?} ", prices_in_period);
                    // continue;
                }
            }
        }
    });

    while let Some(value) = rx.recv().await {
        println!("Dequeued price CEX or DEX: {:?}", value);
        prices_in_minute
            .clone()
            .lock()
            .unwrap()
            .push(f64::from(value.price));
    }

    Ok(())
}


async fn set_up(tx: Sender<PriceData>) -> Result<(), Box<dyn Error>> {
    
    let tx_clone = tx.clone();
    let tx_clone_two = tx.clone();

    let uniswap_token_pool = Pool::new(
        address!("C6962004f452bE9203591991D15f6b388e09E8D0"),
        ARB_WSS_URL.to_string(),
    );
    let sushiswap_token_pool = Pool::new(
        address!("57713F7716e0b0F65ec116912F834E49805480d2"),
        BASE_WSS_URL.to_string(),
    );
    tokio::spawn(async move {
        let _ = uniswap_token_pool.fetch_dex_prices(tx).await;
    });

    tokio::spawn(async move {
        let _ = sushiswap_token_pool.fetch_dex_prices(tx_clone).await;
    });

    let binance_api = BinanceApi::new(BINANCE_WSS_URL);
    tokio::spawn(async move {
        let _ = binance_api.fetch_cex_prices(tx_clone_two).await;
    });
    Ok(())

}
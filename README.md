# ETH/USDC volatility estimator

## On chain data (DEX)

The V3 version of the `ETH/USDC` pool is used. To get the price, we listen to the `Swap` event to retrieve the `sqrtPriceX96` from which we derive the current price of the pool.

For the V2 version of the `ETH/USDC` pool, the price would be deducted from `reserve0` and `reserve1` data.

To add a new on chain `ETH/USDC` V3 pool to listen to, a new `Pool` object needs to be initialized with the address of the pool smart contract address. Then the `fetch_dex_prices` is called on this new object to start listening to contract's `Swap` events.


## Off chain data (CEX)

`Binance kline` API is used though `BinanceApi` object. A websocket is created on the websocket api URL and then `fetch_cex_prices` to start fetch the prices every second.


## Volatitlity

Prices from on chain and off chain sources are sent through a channel. The data is processed by a unique receiver, thus the use of `mpsc::channel`. In case we want to process the two datas sources independently by computing the volatility of each data set source separetly and then do the average, we would use a `broadcast::channel`.

* Each minute, a simple average of the data off/on chain is computed. We get a price at time `t`. The average could also be weighted with the liquidity it represents on each data set.
* We compute the log return `ln_return(t)` between the price at time `t` and the price at time `t-1`, every minute.
* These log returns are stored in a vector to be able to compute the mean at every time interval (i.e each minute). The maximum time window is 6 hours. After 6 hours has elapsed, FIFO is used to get a scroll time window.
* The standard deviation per minute is computed on the log returns
* To get the volatility on 6 hours, we multiply the standard deviation per minute by the square root of the time window (i.e sqrt(360) for 6 hours)

## Install

Build the project with:

```
cargo build
```

Then run:

```
cargo run
```

Exact estimates start to displaying after 3 minutes approximately.

```
--------New interval: Current Volatility Estimation: 0.1000%--------

--------New interval: Current Volatility Estimation: 0.1000%--------

--------New interval: Current Volatility Estimation: 0.1000%--------

--------2024-11-01T16:52:37.696430158+00:00 New Estimated Volatility: 0.5138% --------

--------New interval: Current Volatility Estimation: 0.5138%--------

--------2024-11-01T16:53:37.698351215+00:00 New Estimated Volatility: 1.1772% --------

--------New interval: Current Volatility Estimation: 1.1772%--------

--------2024-11-01T16:54:37.699596923+00:00 New Estimated Volatility: 1.1149% --------

--------New interval: Current Volatility Estimation: 1.1149%--------

--------2024-11-01T16:55:37.701495434+00:00 New Estimated Volatility: 0.9974% --------

--------New interval: Current Volatility Estimation: 0.9974%--------

--------2024-11-01T16:56:37.702513240+00:00 New Estimated Volatility: 0.9116% --------

--------New interval: Current Volatility Estimation: 0.9116%--------

--------2024-11-01T16:57:37.703946939+00:00 New Estimated Volatility: 0.8680% --------

--------New interval: Current Volatility Estimation: 0.8680%--------
```
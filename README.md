# ETH/USDC volatility estimator

## On chain data (DEX)

The V3 version of the `ETH/USDC` pool is used. To get the price, we listen to the `Swap` event to retrieve the `sqrtPriceX96` from which we derive the current price of the pool.

For the V2 version of the `ETH/USDC` pool, the price would be deducted from `reserve0` and `reserve1` data.

To add a new on chain `ETH/USDC` V3 pool to listen to, a new `Pool` object needs to be initialized with the address of the pool smart contract address. Then the `fetch_dex_prices` is called on this new object to start listening to contract's `Swap` events.


## Off chain data (CEX)

`Binance kline` API is used though `BinanceApi` object. A websocket is created on the websocket api URL and then `fetch_cex_prices` to start fetch the prices every second.


## Volatitlity

Prices from on chain and off chain sources are sent through a channel. The data is processed by a unique receiver, thus the use of `mpsc::channel`. In case we want to process the two datas sources independently we would use a `broadcast::channel`.

* Each minute, the average of the data off/on chain is computed. We get a price at time `t`.
* We compute the log return `ln_return(t)` between the price at time `t` and the price at time `t-1`, every minute.
* These log returns are stored in a vector to be able to compute the mean at every time interval (i.e each minute). The maximum time window is 6 hours. After 6 hours has elapsed, FIFO is used to get a scroll time window.
* The standard deviation per minute is computed on the log returns
* To get the volatility on 6 hours, we multiply the standard deviation per minute by the square root of the time window (i.e sqrt(360) for 6 hours)

## Run 

Build the project with:

```
cargo build
```

Then run:

```
cargo run
```

```
--------New minute: Current Volatility Estimation: 0.7293568394437988%--------
-------------------------------------------------------------------
2024-11-01T15:01:24.307147827+00:00 New Estimated Volatility: 0.7124308268363291%
-------------------------------------------------------------------
--------New minute: Current Volatility Estimation: 0.7124308268363291%--------
-------------------------------------------------------------------
2024-11-01T15:02:24.309000570+00:00 New Estimated Volatility: 0.6946508250231922%
-------------------------------------------------------------------
--------New minute: Current Volatility Estimation: 0.6946508250231922%--------
-------------------------------------------------------------------
2024-11-01T15:03:24.311090368+00:00 New Estimated Volatility: 0.6782655775080071%
-------------------------------------------------------------------
--------New minute: Current Volatility Estimation: 0.6782655775080071%--------
-------------------------------------------------------------------
```
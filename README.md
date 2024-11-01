# ETH/USDC volatility estimator

# On chain data (DEX)

The V3 version of the `ETH/USDC` pool is used. To get the price, we listen to the `Swap` event to retrieve the `sqrtPriceX96` from which we derive the current price of the pool.

For the V2 version of the `ETH/USDC` pool, the price would be deducted from `reserve0` and `reserve1` data.

To add a new on chain `ETH/USDC` V3 pool to listen to, a new `Pool` object needs to be initialized with the address of the pool smart contract address. Then the `fetch_dex_prices` is called on this new object to start listening to contract's `Swap` events.


# Off chain data (CEX)

`Binance kline` API is used though `BinanceApi` object. A websocket is created on the websocket api URL and then `fetch_cex_prices` to start fetch the prices every second.


# Volatitlity

Prices from on chain and off chain sources are sent through channel. The data is processed by a unique receiver, thus the use of `mpsc::channel`. In case we want to process the two datas sources independantly we would use a `broadcast::channel`.

* Each minute, the average of the data off/on chain is computed. We get a price at time `t`.
* We compute the log return `ln_return(t)` between the price at time `t` and the price at time `t-1` each minute.
* These log returns are stored in a vector to be able to compute the mean at every interval of time (i.e each minute). The maximum time window is 6 hours.
* The standard deviation per minute is computed on the log returns
*  To get the volatility on 6 hours, we multiply the last result by the sqrt of the time window (i.e sqrt(360) for 6 hours)


use std::collections::VecDeque;
use std::sync::MutexGuard;

pub fn compute_mean(prices: &[f64]) -> f64 {
    let sum: f64 = prices.iter().sum();
    f64::from(sum) / (prices.len() as f64)
}

pub fn compute_ln_return(price_t_1: f64, price_t: f64) -> f64 {
    (price_t / price_t_1).ln()
}

pub fn compute_deviation(returns: MutexGuard<VecDeque<f64>>) -> f64 {
    let mean: f64 = returns.iter().sum::<f64>() / (returns.len() as f64);
    let res: f64 = returns.iter().map(|xt| (xt - mean).powf(2.0)).sum();
    (res / (returns.len() as f64)).sqrt()
}
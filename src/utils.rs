use alloy::primitives::Address;
use alloy::primitives::U256;

pub struct Pool {
    pub addr: Address,
    pub rpc_url: String,
}

impl Pool {
    pub fn new(addr: Address, url: String) -> Self {
        Self { addr: addr, rpc_url: url }
    }
}

#[derive(Debug)]
pub struct PriceData {
    pub price: U256,
    pub on_chain: bool
}

impl PriceData {
    pub fn new(p: U256, is_onchain: bool) -> Self {
        Self { price: p, on_chain: is_onchain }
    }
}
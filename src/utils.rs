use alloy::primitives::Address;

pub struct Pool {
    pub addr: Address,
    pub rpc_url: String,
}

impl Pool {
    pub fn new(addr: Address, url: String) -> Self {
        Self { addr: addr, rpc_url: url }
    }
}
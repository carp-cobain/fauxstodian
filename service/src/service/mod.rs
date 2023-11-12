use crate::driver::SolanaRpcDriver;

mod get_account_balance;
mod grpc_service;
mod parse_pub_key;

pub struct Service {
    driver: SolanaRpcDriver,
}

// Constructor
impl Service {
    pub fn new(driver: SolanaRpcDriver) -> Self {
        Self { driver }
    }
}

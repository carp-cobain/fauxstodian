use crate::driver::SolanaDriver;
use std::sync::Arc;

mod get_account_balance;
mod grpc_service;
mod parse_pub_key;

pub struct Service {
    driver: Arc<Box<dyn SolanaDriver>>,
}

// Constructor
impl Service {
    pub fn new(driver: Arc<Box<dyn SolanaDriver>>) -> Self {
        Self { driver }
    }
}

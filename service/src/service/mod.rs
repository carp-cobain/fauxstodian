use solana_client::rpc_client::RpcClient;

mod get_account_balance;
mod grpc_service;
mod parse_pub_key;

pub struct Service {
    rpc: RpcClient,
}

// Constructor
impl Service {
    pub fn new(rpc: RpcClient) -> Self {
        Self { rpc }
    }
}

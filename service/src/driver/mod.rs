use solana_client::rpc_client::RpcClient;

mod get_account_balance;

pub struct SolanaRpcDriver {
    client: RpcClient,
}

impl SolanaRpcDriver {
    pub fn new(client: RpcClient) -> Self {
        Self { client }
    }
}

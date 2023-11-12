use solana_client::rpc_client::RpcClient;

pub struct SolanaRpc {
    rpc_client: RpcClient,
}

impl SolanaRpc {
    pub fn new(rpc_client: RpcClient) -> Self {
        Self { rpc_client }
    }

    pub fn rpc_client(&self) -> &RpcClient {
        &self.rpc_client
    }
}

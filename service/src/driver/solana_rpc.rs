use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

use super::SolanaDriver;

pub struct SolanaRpc {
    rpc_client: RpcClient,
}

impl SolanaRpc {
    // Create a new solana rpc driver.
    pub fn new(url: &str) -> Self {
        let rpc_client = RpcClient::new(url);
        Self { rpc_client }
    }

    /// Get rpc client reference.
    pub fn rpc_client(&self) -> &RpcClient {
        &self.rpc_client
    }

    /// Query for solana account balance.
    pub fn get_account_balance(&self, pub_key: &Pubkey) -> Result<u64, String> {
        match self.rpc_client().get_account(pub_key) {
            Ok(account) => Ok(account.lamports),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[async_trait::async_trait]
impl SolanaDriver for SolanaRpc {
    async fn get_balance(&self, pub_key: &Pubkey) -> Result<u64, String> {
        self.get_account_balance(pub_key)
    }
}

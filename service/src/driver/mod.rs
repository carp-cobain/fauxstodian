use solana_sdk::pubkey::Pubkey;

mod get_account_balance;

pub mod solana_rpc;

#[async_trait::async_trait]
pub trait SolanaDriver: Send + Sync {
    async fn get_balance(&self, pub_key: &Pubkey) -> Result<u64, String>;
}

#[async_trait::async_trait]
impl SolanaDriver for solana_rpc::SolanaRpc {
    async fn get_balance(&self, pub_key: &Pubkey) -> Result<u64, String> {
        self.get_account_balance(pub_key)
    }
}

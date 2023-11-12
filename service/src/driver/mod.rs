use solana_sdk::pubkey::Pubkey;

pub mod solana_rpc;

#[async_trait::async_trait]
pub trait SolanaDriver: Send + Sync {
    async fn get_balance(&self, pub_key: &Pubkey) -> Result<u64, String>;
}

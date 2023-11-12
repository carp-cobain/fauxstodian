use solana_sdk::pubkey::Pubkey;

// Wire up mods
mod error;
mod helpers;
mod rpc;

// Expose inner types
pub use error::Error;
pub use rpc::SolanaRpc;

/// Driver return type
pub type Result<T> = std::result::Result<T, Error>;

/// Async driver trait for the Solana blockchain.
#[async_trait::async_trait]
pub trait SolanaDriver: Send + Sync {
    async fn get_vault_balance(&self, pda: &Pubkey) -> Result<u64>;
    async fn create_vault(&self, seed: &str, owner: &Pubkey) -> Result<Pubkey>;
    async fn close_vault(&self, pda: &Pubkey, owner: &Pubkey) -> Result<String>;
    async fn change_vault_owner(
        &self,
        pda: &Pubkey,
        owner: &Pubkey,
        new_owner: &Pubkey,
    ) -> Result<String>;
}

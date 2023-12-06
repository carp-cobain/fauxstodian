use solana_sdk::{account::Account, pubkey::Pubkey, signature::Signature};

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
    /// Create a new vault.
    async fn create_vault(&self, seed: &str, owner: &Pubkey) -> Result<(Pubkey, Signature)>;

    /// Return the vault account.
    async fn get_vault_account(&self, pda: &Pubkey) -> Result<Account>;

    /// Transfer ownership of a vault.
    async fn change_vault_owner(
        &self,
        pda: &Pubkey,
        owner: &Pubkey,
        new_owner: &Pubkey,
    ) -> Result<Signature>;

    /// Close an existing vault.
    async fn close_vault(&self, pda: &Pubkey, owner: &Pubkey) -> Result<Signature>;
}

/// Solana driver errors.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error getting account balance: {0}")]
    GetVaultBalanceError(String),
    #[error("error creating public key from seed: {0}")]
    PubkeyWithSeedError(String),
    #[error("error getting latest blockhash: {0}")]
    GetLatestBlockhashError(String),
    #[error("error creating vault: {0}")]
    CreateVaultError(String),
    #[error("error closing vault: {0}")]
    CloseVaultError(String),
    #[error("error changing vault owner: {0}")]
    ChangeVaultOwnerError(String),
}

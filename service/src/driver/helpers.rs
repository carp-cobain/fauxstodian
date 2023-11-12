use solana_sdk::{hash::Hash, rent::Rent};
use vault::state::VaultRecord;

use super::{rpc::SolanaRpc, Error, Result};

impl SolanaRpc {
    /// Calculate vault record rent values
    pub fn calculate_rent(&self) -> (u64, u64) {
        let space = VaultRecord::LEN;
        let lamports = Rent::default().minimum_balance(space);
        (space as u64, lamports)
    }

    /// Get the latest blockhash using the rpc client.
    pub fn get_latest_blockhash(&self) -> Result<Hash> {
        self.rpc_client_ref()
            .get_latest_blockhash()
            .map_err(|err| Error::GetLatestBlockhashError(err.kind.to_string()))
    }
}

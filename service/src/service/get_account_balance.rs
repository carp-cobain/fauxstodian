use solana_sdk::pubkey::Pubkey;
use tonic::Status;

use super::Service;

// Pubkey & account helpers
impl Service {
    /// Query for solana account balance; mapping errors to a gRPC status.
    pub async fn get_account_balance(&self, pub_key: &Pubkey) -> Result<u64, Status> {
        match self.rpc.get_account(pub_key) {
            Ok(account) => Ok(account.lamports),
            Err(err) => Err(Status::from_error(Box::new(err))),
        }
    }
}

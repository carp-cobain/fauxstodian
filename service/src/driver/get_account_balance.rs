use solana_sdk::pubkey::Pubkey;

use crate::driver::solana_rpc::SolanaRpc;

impl SolanaRpc {
    /// Query for solana account balance.
    pub fn get_account_balance(&self, pub_key: &Pubkey) -> Result<u64, String> {
        match self.rpc_client().get_account(pub_key) {
            Ok(account) => Ok(account.lamports),
            Err(err) => Err(err.to_string()),
        }
    }
}

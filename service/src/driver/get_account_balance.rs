use solana_sdk::pubkey::Pubkey;

use super::SolanaRpcDriver;

impl SolanaRpcDriver {
    /// Query for solana account balance.
    pub fn get_account_balance(&self, pub_key: &Pubkey) -> Result<u64, String> {
        match self.client.get_account(pub_key) {
            Ok(account) => Ok(account.lamports),
            Err(err) => Err(err.to_string()),
        }
    }
}

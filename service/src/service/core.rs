use crate::entity::{Account, Balance, Signature};

use super::{Result, Service};

impl Service {
    /// Create and initialize a solana vault PDA.
    pub async fn create_account(&self, seed: &str, owner: &str) -> Result<Account> {
        let owner = self.parse_pubkey(owner)?;
        let pda = self.driver.create_vault(seed, &owner).await?;
        Ok(Account {
            seed: seed.to_string(),
            owner: owner.to_string(),
            pda: pda.to_string(),
        })
    }

    /// Query for solana account balance.
    pub async fn get_balance(&self, key_str: &str) -> Result<Balance> {
        let pda = self.parse_pubkey(&key_str)?;
        let lamports = self.driver.get_vault_balance(&pda).await?;
        Ok(Balance {
            pub_key: key_str.to_string(),
            lamports,
        })
    }

    /// Transfer ownership of a vault PDA.
    pub async fn transfer_ownership(
        &self,
        pda: &str,
        owner: &str,
        new_owner: &str,
    ) -> Result<Signature> {
        let pda = self.parse_pubkey(pda)?;
        let owner = self.parse_pubkey(owner)?;
        let new_owner = self.parse_pubkey(new_owner)?;
        let hash = self
            .driver
            .change_vault_owner(&pda, &owner, &new_owner)
            .await?;
        Ok(Signature { hash })
    }

    /// Close a vault PDA.
    pub async fn close_account(&self, pda: &str, owner: &str) -> Result<Signature> {
        let pda = self.parse_pubkey(pda)?;
        let owner = self.parse_pubkey(owner)?;
        let hash = self.driver.close_vault(&pda, &owner).await?;
        Ok(Signature { hash })
    }
}

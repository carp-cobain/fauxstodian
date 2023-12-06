use crate::entity::{Balance, Signature, VaultAccount};

use super::{Result, Service};

impl Service {
    /// Create and initialize a solana vault PDA.
    pub async fn create_account(&self, seed: &str, owner: &str) -> Result<VaultAccount> {
        let seed = self.validate_seed(seed)?;
        let owner = self.parse_pubkey(owner)?;
        let (pda, signature) = self.driver.create_vault(&seed, &owner).await?;
        Ok(VaultAccount {
            pda: pda.to_string(),
            signature: Some(Signature {
                hash: signature.to_string(),
            }),
        })
    }

    /// Query for solana account balance.
    pub async fn get_balance(&self, pda: &str) -> Result<Balance> {
        let pda = self.parse_pubkey(&pda)?;
        let account = self.driver.get_vault_account(&pda).await?;
        Ok(Balance {
            pda: pda.to_string(),
            lamports: account.lamports,
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
        let signature = self
            .driver
            .change_vault_owner(&pda, &owner, &new_owner)
            .await?;
        Ok(Signature {
            hash: signature.to_string(),
        })
    }

    /// Close a vault PDA.
    pub async fn close_account(&self, pda: &str, owner: &str) -> Result<Signature> {
        let pda = self.parse_pubkey(pda)?;
        let owner = self.parse_pubkey(owner)?;
        let signature = self.driver.close_vault(&pda, &owner).await?;
        Ok(Signature {
            hash: signature.to_string(),
        })
    }
}

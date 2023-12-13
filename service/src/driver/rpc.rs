use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    account::Account, pubkey::Pubkey, signature::Keypair, signature::Signature, signer::Signer,
    system_instruction, transaction::Transaction,
};
use vault::{id, instruction};

use super::{Error, Result, SolanaDriver};

/// The concrete driver type for interacting with the Solana vault program via JSON-RPC.
pub struct SolanaRpc {
    rpc_client: RpcClient,
    signing_keys: Keypair,
}

impl SolanaRpc {
    /// Create a new Solana JSON-RPC driver.
    pub fn new<U: ToString>(url: U, signing_keys: Keypair) -> Self {
        let rpc_client = RpcClient::new(url);
        Self {
            rpc_client,
            signing_keys,
        }
    }

    /// Get a reference to the solana rpc client.
    pub fn rpc_client_ref(&self) -> &RpcClient {
        &self.rpc_client
    }
}

#[async_trait::async_trait]
impl SolanaDriver for SolanaRpc {
    /// Create a new vault with the given seed and owner.
    async fn create_vault(&self, seed: &str, owner: &Pubkey) -> Result<(Pubkey, Signature)> {
        let dart = &self.signing_keys.pubkey();
        let (space, lamports) = self.calculate_rent();

        // Generate the deposit address
        let pda = Pubkey::create_with_seed(dart, seed, &id())
            .map_err(|err| Error::PubkeyWithSeedError(err.to_string()))?;

        // Need to create account and init vault record in one transaction
        let instructions = &[
            system_instruction::create_account_with_seed(
                dart,
                &pda,
                dart,
                seed,
                lamports,
                space,
                &id(),
            ),
            instruction::initialize(&pda, dart, owner),
        ];

        // Build transaction
        let transaction = Transaction::new_signed_with_payer(
            instructions,
            Some(dart),
            &[&self.signing_keys],
            self.get_latest_blockhash()?,
        );

        // Broadcast
        let signature = self
            .rpc_client
            .send_and_confirm_transaction(&transaction)
            .map_err(|err| Error::CreateVaultError(err.kind.to_string()))?;

        Ok((pda, signature))
    }

    /// Get the lamports in a vault.
    async fn get_vault_account(&self, pda: &Pubkey) -> Result<Account> {
        let account = self
            .rpc_client
            .get_account(pda)
            .map_err(|err| Error::GetVaultAccountError(err.to_string()))?;
        Ok(account)
    }

    /// Transfer ownership of a vault.
    async fn change_vault_owner(
        &self,
        pda: &Pubkey,
        owner: &Pubkey,
        new_owner: &Pubkey,
    ) -> Result<Signature> {
        let dart = &self.signing_keys.pubkey();

        let transaction = Transaction::new_signed_with_payer(
            &[instruction::transfer_owner(&pda, &dart, &owner, &new_owner)],
            Some(dart),
            &[&self.signing_keys],
            self.get_latest_blockhash()?,
        );

        let signature: Signature = self
            .rpc_client
            .send_and_confirm_transaction(&transaction)
            .map_err(|err| Error::ChangeVaultOwnerError(err.kind.to_string()))?;

        Ok(signature)
    }

    /// Close a vault and drain lamports to the current owner.
    async fn close_vault(&self, pda: &Pubkey, owner: &Pubkey) -> Result<Signature> {
        let dart = &self.signing_keys.pubkey();

        let transaction = Transaction::new_signed_with_payer(
            &[instruction::close_account(pda, dart, owner)],
            Some(dart),
            &[&self.signing_keys],
            self.get_latest_blockhash()?,
        );

        let signature = self
            .rpc_client
            .send_and_confirm_transaction(&transaction)
            .map_err(|err| Error::CloseVaultError(err.kind.to_string()))?;

        Ok(signature)
    }

    /// Check the solana connection.
    async fn health_check(&self) -> Result<()> {
        self.rpc_client
            .get_health()
            .map_err(|err| Error::HealthCheckError(err.kind.to_string()))
    }
}

use tonic::{Request, Response, Status};

use crate::proto::fauxstodian_server::Fauxstodian;
use crate::proto::{
    CloseAccountRep, CloseAccountReq, CreateAccountRep, CreateAccountReq, GetBalanceRep,
    GetBalanceReq, TransferOwnershipRep, TransferOwnershipReq,
};

use super::Service;

#[tonic::async_trait]
impl Fauxstodian for Service {
    /// Create a new account backed by a solana vault.
    async fn create_account(
        &self,
        request: Request<CreateAccountReq>,
    ) -> Result<Response<CreateAccountRep>, Status> {
        println!(
            "Got a create account request from {:?}",
            request.remote_addr()
        );
        // TODO: Create and init pda using a DART keypair in a transaction...
        let reply = CreateAccountRep {};
        Ok(Response::new(reply))
    }

    /// Get the number of lamports in a solana account.
    async fn get_balance(
        &self,
        request: Request<GetBalanceReq>,
    ) -> Result<Response<GetBalanceRep>, Status> {
        println!("Got a get balance request from {:?}", request.remote_addr());
        let pub_key = self.parse_pub_key(&request.get_ref().pub_key)?;
        let lamports = self.get_account_balance(&pub_key).await?;
        Ok(Response::new(GetBalanceRep {
            pub_key: pub_key.to_string(),
            lamports,
        }))
    }

    /// Transfer ownership of a solana vault.
    async fn transfer_ownership(
        &self,
        request: Request<TransferOwnershipReq>,
    ) -> Result<Response<TransferOwnershipRep>, Status> {
        println!(
            "Got a transfer ownership request from {:?}",
            request.remote_addr()
        );
        // TODO
        let reply = TransferOwnershipRep {};
        Ok(Response::new(reply))
    }

    /// Close an account, withdrawing all lamports to the vault owner.
    async fn close_account(
        &self,
        request: Request<CloseAccountReq>,
    ) -> Result<Response<CloseAccountRep>, Status> {
        println!(
            "Got a close account request from {:?}",
            request.remote_addr()
        );
        // TODO
        let reply = CloseAccountRep {};
        Ok(Response::new(reply))
    }
}
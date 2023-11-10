use std::error::Error;

use tonic::{transport::Server, Request, Response, Status};

use fauxstodian::fauxstodian_server::{Fauxstodian, FauxstodianServer};
use fauxstodian::{
    CloseAccountRep, CloseAccountReq, CreateAccountRep, CreateAccountReq, GetBalanceRep,
    GetBalanceReq, TransferOwnershipRep, TransferOwnershipReq,
};

use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub mod fauxstodian {
    tonic::include_proto!("fauxstodian");
}

pub struct FauxstodianService {
    pub rpc: RpcClient,
}

impl FauxstodianService {
    fn new(rpc: RpcClient) -> Self {
        Self { rpc }
    }
}

#[tonic::async_trait]
impl Fauxstodian for FauxstodianService {
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

        let pub_key = &Pubkey::from_str(&request.get_ref().pub_key).unwrap();
        let lamports = self.rpc.get_account(pub_key).unwrap().lamports;

        let reply = GetBalanceRep {
            pub_key: pub_key.to_string(),
            lamports,
        };

        Ok(Response::new(reply))
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // TODO: Read strings from env var config...
    let listen_addr = "0.0.0.0:50055".parse().unwrap();
    let solana_addr = "http://127.0.0.1:8899";

    let rpc = RpcClient::new(solana_addr);
    let service = FauxstodianService::new(rpc);

    println!("Fauxstodian server listening on {}", listen_addr);

    Server::builder()
        .add_service(FauxstodianServer::new(service))
        .serve(listen_addr)
        .await?;

    Ok(())
}

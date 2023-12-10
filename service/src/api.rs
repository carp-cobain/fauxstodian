use crate::proto::fauxstodian_server::Fauxstodian;
use crate::proto::{
    CloseAccountRep, CloseAccountReq, CreateAccountRep, CreateAccountReq, GetBalanceRep,
    GetBalanceReq, TransferOwnershipRep, TransferOwnershipReq,
};
use crate::service::{Error, Service};
use tonic::{Request, Response, Status};

/// Define the fauxstodian API type.
pub struct FauxstodianApi {
    service: Service,
}

impl FauxstodianApi {
    /// API constructor.
    pub fn new(service: Service) -> Self {
        Self { service }
    }
}

/// Map service errors to grpc status.
impl From<Error> for Status {
    fn from(err: Error) -> Self {
        match err {
            Error::InvalidArgument { message } => Status::invalid_argument(message),
            Error::InternalError { message } => Status::internal(message),
        }
    }
}

#[tonic::async_trait]
impl Fauxstodian for FauxstodianApi {
    /// Create a new account backed by a solana vault.
    async fn create_account(
        &self,
        request: Request<CreateAccountReq>,
    ) -> Result<Response<CreateAccountRep>, Status> {
        println!("Create account request from {:?}", request.remote_addr());
        let reqr = request.get_ref();
        match self.service.create_account(&reqr.seed, &reqr.owner).await {
            Ok(account) => {
                let signature = account.signature_hash();
                println!("Created account; signature = {signature}");
                Ok(Response::new(CreateAccountRep {
                    deposit_address: account.pda,
                }))
            }
            Err(err) => Err(err.into()),
        }
    }

    /// Get the number of lamports in a solana account.
    async fn get_balance(
        &self,
        request: Request<GetBalanceReq>,
    ) -> Result<Response<GetBalanceRep>, Status> {
        println!("Get balance request from {:?}", request.remote_addr());
        match self.service.get_balance(&request.get_ref().pub_key).await {
            Ok(balance) => Ok(Response::new(GetBalanceRep {
                pub_key: balance.pda,
                lamports: balance.lamports,
            })),
            Err(err) => Err(err.into()),
        }
    }

    /// Transfer ownership of a solana vault.
    async fn transfer_ownership(
        &self,
        request: Request<TransferOwnershipReq>,
    ) -> Result<Response<TransferOwnershipRep>, Status> {
        println!(
            "Transfer ownership request from {:?}",
            request.remote_addr()
        );
        let reqr = request.get_ref();
        let future = self
            .service
            .transfer_ownership(&reqr.pda, &reqr.owner, &reqr.new_owner);
        match future.await {
            Ok(signature) => {
                println!("Transfer success; signature = {:?}", signature.hash);
                Ok(Response::new(TransferOwnershipRep {
                    signature: signature.hash,
                }))
            }
            Err(err) => Err(err.into()),
        }
    }

    /// Close an account, withdrawing all lamports to the vault owner.
    async fn close_account(
        &self,
        request: Request<CloseAccountReq>,
    ) -> Result<Response<CloseAccountRep>, Status> {
        println!("Close account request from {:?}", request.remote_addr());
        let reqr = request.get_ref();
        match self.service.close_account(&reqr.pda, &reqr.owner).await {
            Ok(signature) => {
                println!("Account closed; signature = {:?}", signature.hash);
                Ok(Response::new(CloseAccountRep {
                    signature: signature.hash,
                }))
            }
            Err(err) => Err(err.into()),
        }
    }
}

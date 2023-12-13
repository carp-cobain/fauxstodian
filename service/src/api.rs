use crate::proto::fauxstodian_service_server::FauxstodianService;
use crate::proto::{
    CloseAccountRequest, CloseAccountResponse, CreateAccountRequest, CreateAccountResponse,
    GetBalanceRequest, GetBalanceResponse, TransferOwnershipRequest, TransferOwnershipResponse,
};
use crate::service::{Error, Service};
use log::info;
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
impl FauxstodianService for FauxstodianApi {
    /// Create a new account backed by a solana vault.
    async fn create_account(
        &self,
        request: Request<CreateAccountRequest>,
    ) -> Result<Response<CreateAccountResponse>, Status> {
        info!("Create account request from {:?}", request.remote_addr());
        let reqr = request.get_ref();
        match self.service.create_account(&reqr.seed, &reqr.owner).await {
            Ok(account) => {
                let signature = account.signature_hash();
                info!("Created account; signature = {signature}");
                Ok(Response::new(CreateAccountResponse {
                    deposit_address: account.pda,
                }))
            }
            Err(err) => Err(err.into()),
        }
    }

    /// Get the number of lamports in a solana account.
    async fn get_balance(
        &self,
        request: Request<GetBalanceRequest>,
    ) -> Result<Response<GetBalanceResponse>, Status> {
        info!("Get balance request from {:?}", request.remote_addr());
        match self.service.get_balance(&request.get_ref().pub_key).await {
            Ok(balance) => Ok(Response::new(GetBalanceResponse {
                pub_key: balance.pda,
                lamports: balance.lamports,
            })),
            Err(err) => Err(err.into()),
        }
    }

    /// Transfer ownership of a solana vault.
    async fn transfer_ownership(
        &self,
        request: Request<TransferOwnershipRequest>,
    ) -> Result<Response<TransferOwnershipResponse>, Status> {
        info!(
            "Transfer ownership request from {:?}",
            request.remote_addr()
        );
        let reqr = request.get_ref();
        let future = self
            .service
            .transfer_ownership(&reqr.pda, &reqr.owner, &reqr.new_owner);
        match future.await {
            Ok(signature) => {
                info!("Transfer success; signature = {:?}", signature.hash);
                Ok(Response::new(TransferOwnershipResponse {
                    signature: signature.hash,
                }))
            }
            Err(err) => Err(err.into()),
        }
    }

    /// Close an account, withdrawing all lamports to the vault owner.
    async fn close_account(
        &self,
        request: Request<CloseAccountRequest>,
    ) -> Result<Response<CloseAccountResponse>, Status> {
        info!("Close account request from {:?}", request.remote_addr());
        let reqr = request.get_ref();
        match self.service.close_account(&reqr.pda, &reqr.owner).await {
            Ok(signature) => {
                info!("Account closed; signature = {:?}", signature.hash);
                Ok(Response::new(CloseAccountResponse {
                    signature: signature.hash,
                }))
            }
            Err(err) => Err(err.into()),
        }
    }
}

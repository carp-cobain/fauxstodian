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
            Error::ParsePubkeyError { message } => Status::invalid_argument(message),
            Error::DriverError { message } => Status::internal(message),
            Error::Todo => Status::unimplemented(Error::Todo.to_string()),
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
        let inner = request.get_ref();
        match self.service.create_account(&inner.seed, &inner.owner).await {
            Ok(account) => Ok(Response::new(CreateAccountRep {
                deposit_address: account.pda,
            })),
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
                pub_key: balance.pub_key,
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
        let inner = request.into_inner();
        let future = self
            .service
            .transfer_ownership(&inner.pda, &inner.owner, &inner.new_owner);
        match future.await {
            Ok(_) => Ok(Response::new(TransferOwnershipRep {})),
            Err(err) => Err(err.into()),
        }
    }

    /// Close an account, withdrawing all lamports to the vault owner.
    async fn close_account(
        &self,
        request: Request<CloseAccountReq>,
    ) -> Result<Response<CloseAccountRep>, Status> {
        println!("Close account request from {:?}", request.remote_addr());
        let inner = request.into_inner();
        let future = self.service.close_account(&inner.pda, &inner.owner);
        match future.await {
            Ok(_) => Ok(Response::new(CloseAccountRep {})),
            Err(err) => Err(err.into()),
        }
    }
}

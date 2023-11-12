use crate::proto::{GetBalanceRep, GetBalanceReq};
use tonic::{Response, Status};

use super::Service;

impl Service {
    /// Query for solana account balance; mapping results to a gRPC proto types.
    pub async fn get_account_balance(
        &self,
        request: GetBalanceReq,
    ) -> Result<Response<GetBalanceRep>, Status> {
        let pub_key = self.parse_pub_key(&request.pub_key)?;
        match self.driver.get_balance(&pub_key).await {
            Ok(lamports) => Ok(Response::new(GetBalanceRep {
                pub_key: request.pub_key,
                lamports,
            })),
            Err(message) => Err(Status::internal(message)),
        }
    }
}

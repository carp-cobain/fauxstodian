use fauxstodian::{
    api::FauxstodianApi,
    config::Config,
    driver::{SolanaDriver, SolanaRpc},
    proto::fauxstodian_service_server::FauxstodianServiceServer,
    service::Service,
};
use log::info;
use std::{error::Error, sync::Arc};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let config = Config::default();
    info!("Solana rpc url = {}", config.rpc_url);

    let rpc = SolanaRpc::new(config.rpc_url, config.keypair);
    let driver = Arc::new(Box::new(rpc) as Box<dyn SolanaDriver>);
    let api = FauxstodianApi::new(Service::new(driver));

    info!("Fauxstodian server listening on {}", config.listen_addr);

    Server::builder()
        .add_service(FauxstodianServiceServer::new(api))
        .serve(config.listen_addr)
        .await?;

    Ok(())
}

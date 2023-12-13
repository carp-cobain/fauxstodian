use fauxstodian::{
    api::FauxstodianApi,
    config::Config,
    driver::{SolanaDriver, SolanaRpc},
    proto::fauxstodian_service_server::FauxstodianServiceServer,
    service::Service,
    status::health_check,
};
use std::{error::Error, sync::Arc};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let config = Config::default();
    log::info!("Solana rpc url = {}", config.rpc_url);

    // Wire up API
    let rpc = SolanaRpc::new(config.rpc_url, config.keypair);
    let driver = Arc::new(Box::new(rpc) as Box<dyn SolanaDriver>);
    let api = FauxstodianApi::new(Service::new(Arc::clone(&driver)));

    // Start health check task
    let (reporter, health_service) = tonic_health::server::health_reporter();
    tokio::spawn(health_check(reporter, Arc::clone(&driver)));

    // Serve api services
    log::info!("Fauxstodian server listening on {}", config.listen_addr);
    Server::builder()
        .add_service(health_service)
        .add_service(FauxstodianServiceServer::new(api))
        .serve(config.listen_addr)
        .await?;

    Ok(())
}

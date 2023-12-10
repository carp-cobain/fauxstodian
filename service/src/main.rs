use fauxstodian::{
    api::FauxstodianApi,
    config::Config,
    driver::{SolanaDriver, SolanaRpc},
    proto::fauxstodian_server::FauxstodianServer,
    service::Service,
};
use std::{error::Error, sync::Arc};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::default();

    let rpc = SolanaRpc::new(config.rpc_url, config.keypair);
    let driver = Arc::new(Box::new(rpc) as Box<dyn SolanaDriver>);
    let api = FauxstodianApi::new(Service::new(driver));

    println!("Fauxstodian server listening on {}", config.listen_addr);

    Server::builder()
        .add_service(FauxstodianServer::new(api))
        .serve(config.listen_addr)
        .await?;

    Ok(())
}

use fauxstodian::{
    api::FauxstodianApi,
    driver::{SolanaDriver, SolanaRpc},
    proto::fauxstodian_server::FauxstodianServer,
    service::Service,
};
use solana_sdk::{
    signature::Keypair,
    signer::{EncodableKey, Signer},
};
use std::{error::Error, sync::Arc};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // TODO: Read strings from env var config...
    let listen_addr = "0.0.0.0:50055".parse().unwrap();
    let solana_rpc_url = "http://127.0.0.1:8899";
    let keypair_file = "dart.json";

    let dart_keypair = Keypair::read_from_file(keypair_file)?;
    println!(
        "Loaded dart public key: {:?}",
        &dart_keypair.pubkey().to_string()
    );

    let rpc = SolanaRpc::new(solana_rpc_url, dart_keypair);
    let driver = Arc::new(Box::new(rpc) as Box<dyn SolanaDriver>);
    let service = Service::new(driver); //Arc::clone(&driver));
    let api = FauxstodianApi::new(service);

    println!("Fauxstodian server listening on {}", listen_addr);

    Server::builder()
        .add_service(FauxstodianServer::new(api))
        .serve(listen_addr)
        .await?;

    Ok(())
}

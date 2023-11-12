use solana_client::rpc_client::RpcClient;
use std::error::Error;
use std::sync::Arc;
use tonic::transport::Server;

use fauxstodian::driver::{solana_rpc::SolanaRpc, SolanaDriver};
use fauxstodian::proto::fauxstodian_server::FauxstodianServer;
use fauxstodian::service::Service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // TODO: Read strings from env var config...
    let listen_addr = "0.0.0.0:50055".parse().unwrap();
    let solana_addr = "http://127.0.0.1:8899";

    let rpc_client = RpcClient::new(solana_addr);
    let rpc = SolanaRpc::new(rpc_client);
    let driver = Arc::new(Box::new(rpc) as Box<dyn SolanaDriver>);
    let service = Service::new(Arc::clone(&driver));

    println!("Fauxstodian server listening on {}", listen_addr);

    Server::builder()
        .add_service(FauxstodianServer::new(service))
        .serve(listen_addr)
        .await?;

    Ok(())
}

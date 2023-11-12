use std::error::Error;

use solana_client::rpc_client::RpcClient;

use fauxstodian::driver::SolanaRpcDriver;
use fauxstodian::proto::fauxstodian_server::FauxstodianServer;
use fauxstodian::service::Service;

use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // TODO: Read strings from env var config...
    let listen_addr = "0.0.0.0:50055".parse().unwrap();
    let solana_addr = "http://127.0.0.1:8899";

    let client = RpcClient::new(solana_addr);
    let driver = SolanaRpcDriver::new(client);
    let service = Service::new(driver);

    println!("Fauxstodian server listening on {}", listen_addr);

    Server::builder()
        .add_service(FauxstodianServer::new(service))
        .serve(listen_addr)
        .await?;

    Ok(())
}

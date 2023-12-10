use solana_sdk::{signature::Keypair, signer::EncodableKey};
use std::env;
use std::net::SocketAddr;

pub struct Config {
    pub rpc_url: String,
    pub keypair: Keypair,
    pub listen_addr: SocketAddr,
}

impl Config {
    /// Create a new config struct.
    pub fn new(rpc_url: String, keypair: Keypair, listen_addr: SocketAddr) -> Self {
        Self {
            rpc_url,
            keypair,
            listen_addr,
        }
    }
}

impl Default for Config {
    /// Create a default config using env vars, falling back to defaults.
    fn default() -> Self {
        // Default values
        let rpc_url_default = String::from("http://127.0.0.1:8899");
        let keypair_file_default = String::from("keypair.json");
        let listen_addr_default = String::from("0.0.0.0:50055");

        // Read settings from env vars, falling back to defaults
        let rpc_url: String = env::var("SOLANA_RPC_URL").unwrap_or(rpc_url_default);
        let keypair_file = env::var("SOLANA_KEYPAIR_FILE").unwrap_or(keypair_file_default);
        let listen_addr = env::var("GRPC_LISTEN_ADDR").unwrap_or(listen_addr_default);

        // Extra setup, validation.
        let keypair = Keypair::read_from_file(keypair_file).expect("Unable to read keypair file");
        let listen_addr = listen_addr.parse().expect("Unable to parse listen addr");

        // Config
        Config::new(rpc_url, keypair, listen_addr)
    }
}

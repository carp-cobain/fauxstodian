use solana_sdk::{signature::Keypair, signer::EncodableKey};
use std::env;
use std::net::SocketAddr;

/// Fauxstodian configuration.
pub struct Config {
    pub rpc_url: String,
    pub keypair: Keypair,
    pub listen_addr: SocketAddr,
}

impl Config {
    /// Create a new config.
    pub fn new(rpc_url: String, keypair: Keypair, listen_addr: SocketAddr) -> Self {
        Self {
            rpc_url,
            keypair,
            listen_addr,
        }
    }

    /// Load solana json-rpc url
    fn load_rpc_url() -> String {
        env::var("SOLANA_RPC_URL").unwrap_or("http://127.0.0.1:8899".into())
    }

    /// Load solana custodian keypair (signing keys) from file
    fn load_keypair_file() -> Keypair {
        let keypair_file = env::var("SOLANA_KEYPAIR_FILE").unwrap_or("keypair.json".into());
        Keypair::read_from_file(keypair_file).expect("Unable to read keypair file")
    }

    /// Load fauxstodian server gRPC listen address
    fn load_listen_addr() -> SocketAddr {
        let listen_addr = env::var("GRPC_LISTEN_ADDR").unwrap_or("0.0.0.0:50055".into());
        listen_addr.parse().expect("Unable to parse listen addr")
    }
}

impl Default for Config {
    /// Create a default config using env vars.
    fn default() -> Self {
        Config::new(
            Config::load_rpc_url(),
            Config::load_keypair_file(),
            Config::load_listen_addr(),
        )
    }
}

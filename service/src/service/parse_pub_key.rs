use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use tonic::Status;

use super::Service;

impl Service {
    /// Parse a public key from a string, mapping failure to gRPC error status.
    pub fn parse_pub_key(&self, key_str: &str) -> Result<Pubkey, Status> {
        match Pubkey::from_str(key_str) {
            Ok(pub_key) => Ok(pub_key),
            Err(err) => Err(Status::invalid_argument(err.to_string())),
        }
    }
}

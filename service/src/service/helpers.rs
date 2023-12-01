use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

use super::{Result, Service};

impl Service {
    /// Parse a public key from a string.
    pub(crate) fn parse_pubkey(&self, key_str: &str) -> Result<Pubkey> {
        Pubkey::from_str(key_str).map_err(|err| err.into())
    }
}

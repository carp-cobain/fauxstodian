use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

use super::{Error, Result, Service};

impl Service {
    /// Parse a public key from a string.
    pub(crate) fn parse_pubkey(&self, pubkey: &str) -> Result<Pubkey> {
        Pubkey::from_str(pubkey).map_err(|err| err.into())
    }

    /// Ensure seed string is between 1 and 32 (inclusive) bytes.
    pub(crate) fn validate_seed(&self, seed: &str) -> Result<String> {
        let seed = seed.trim();
        let seed_len = seed.len();
        if seed.is_empty() || seed_len > solana_sdk::pubkey::MAX_SEED_LEN {
            Err(Error::InvalidArgument {
                message: "invalid seed length: {seed_len}".to_string(),
            })
        } else {
            Ok(String::from(seed))
        }
    }
}

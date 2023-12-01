use crate::driver::Error as DriverError;
use solana_sdk::pubkey::ParsePubkeyError;

/// Service level errors.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error parsing public key: {message}")]
    ParsePubkeyError { message: String },
    #[error("error calling solana driver: {message}")]
    DriverError { message: String },
    #[error("not implemented")]
    Todo,
}

/// Convert a core driver error into a service level driver error.
impl From<DriverError> for Error {
    fn from(error: DriverError) -> Self {
        Error::DriverError {
            message: error.to_string(),
        }
    }
}

/// Convert a Solana sdk public key parsing error into a service level error.
impl From<ParsePubkeyError> for Error {
    fn from(error: ParsePubkeyError) -> Self {
        Error::ParsePubkeyError {
            message: error.to_string(),
        }
    }
}

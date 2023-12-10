use crate::driver::Error as DriverError;
use solana_sdk::pubkey::ParsePubkeyError;

/// Service level errors.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid argument: {message}")]
    InvalidArgument { message: String },
    #[error("internal error: {message}")]
    InternalError { message: String },
}

/// Convert a core driver error into a service level driver error.
impl From<DriverError> for Error {
    fn from(error: DriverError) -> Self {
        Error::InternalError {
            message: error.to_string(),
        }
    }
}

/// Convert a Solana sdk public key parsing error into a service level error.
impl From<ParsePubkeyError> for Error {
    fn from(error: ParsePubkeyError) -> Self {
        Error::InvalidArgument {
            message: error.to_string(),
        }
    }
}

use crate::driver::SolanaDriver;
use std::sync::Arc;

pub use error::Error;

mod core;
mod error;
mod helpers;

/// Result type to simplify service function signatures.
pub type Result<T> = std::result::Result<T, Error>;

/// Service encapsulates the business logic for fauxstodian.
pub struct Service {
    driver: Arc<Box<dyn SolanaDriver>>,
}

impl Service {
    /// Service constructor.
    pub fn new(driver: Arc<Box<dyn SolanaDriver>>) -> Self {
        Self { driver }
    }
}

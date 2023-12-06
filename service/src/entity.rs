/// Represents a Solana account balance.
#[derive(Debug)]
pub struct Balance {
    pub pda: String,
    pub lamports: u64,
}

/// Represents transaction signature hash.
#[derive(Debug)]
pub struct Signature {
    pub hash: String,
}

/// Represents a newly created account.
#[derive(Debug)]
pub struct VaultAccount {
    pub pda: String,                  // Program Derived Address
    pub signature: Option<Signature>, // Specified after create
}

impl VaultAccount {
    /// Helper for unwrapping the account signature hash.
    pub fn signature_hash(&self) -> String {
        self.signature
            .as_ref()
            .map(|s| s.hash.clone())
            .unwrap_or_default()
    }
}

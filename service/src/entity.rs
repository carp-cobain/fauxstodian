/// Represents a Solana account balance.
#[derive(Debug)]
pub struct Balance {
    pub pub_key: String,
    pub lamports: u64,
}

/// Represents a newly created account.
#[derive(Debug)]
pub struct Account {
    pub seed: String,
    pub owner: String,
    pub pda: String, // Program Derived Address
}

/// Represents transaction signature hash.
#[derive(Debug)]
pub struct Signature {
    pub hash: String,
}

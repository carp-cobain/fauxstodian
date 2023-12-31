use crate::id;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

/// Instructions supported by the vault program.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum VaultInstruction {
    /// Initialize a vault record (by DART on behalf of a given owner).
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[writable]` The vault record account (must be uninitialized).
    /// 1. `[signer]` The securities intermediary (DART)
    /// 2. `[]` The record owner (trader)
    Initialize,

    /// Transfer ownership of a vault record
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[writable]` The vault record account (must be previously initialized).
    /// 1. `[signer]` The securities intermediary (DART)
    /// 2. `[signer]` The current record owner.
    /// 3. `[]` The new record owner
    TransferOwner,

    /// Close a vault record account, draining lamports to the current owner.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[writable]` The vault record account (must be previously initialized).
    /// 1. `[signer]` The securities intermediary (DART)
    /// 2. `[signer, writable]` The record owner (receiver of account lamports).
    CloseAccount,
}

/// Create a `VaultInstruction::Initialize` instruction
pub fn initialize(pda: &Pubkey, dart: &Pubkey, owner: &Pubkey) -> Instruction {
    Instruction::new_with_borsh(
        id(),
        &VaultInstruction::Initialize,
        vec![
            AccountMeta::new(*pda, false),
            AccountMeta::new_readonly(*dart, true),
            AccountMeta::new_readonly(*owner, false),
        ],
    )
}

/// Create a `VaultInstruction::TransferOwner` instruction
pub fn transfer_owner(
    pda: &Pubkey,
    dart: &Pubkey,
    owner: &Pubkey,
    new_owner: &Pubkey,
) -> Instruction {
    Instruction::new_with_borsh(
        id(),
        &VaultInstruction::TransferOwner,
        vec![
            AccountMeta::new(*pda, false),
            AccountMeta::new_readonly(*dart, true),
            AccountMeta::new_readonly(*owner, false),
            AccountMeta::new_readonly(*new_owner, false),
        ],
    )
}

/// Create a `VaultInstruction::CloseAccount` instruction
pub fn close_account(pda: &Pubkey, dart: &Pubkey, owner: &Pubkey) -> Instruction {
    Instruction::new_with_borsh(
        id(),
        &VaultInstruction::CloseAccount,
        vec![
            AccountMeta::new(*pda, false),
            AccountMeta::new_readonly(*dart, true),
            AccountMeta::new(*owner, false),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::program_error::ProgramError;

    #[test]
    fn serialize_initialize() {
        let instruction = VaultInstruction::Initialize;
        let expected = vec![0];
        assert_eq!(instruction.try_to_vec().unwrap(), expected);
        assert_eq!(
            VaultInstruction::try_from_slice(&expected).unwrap(),
            instruction
        );
    }

    #[test]
    fn serialize_transfer_owner() {
        let instruction = VaultInstruction::TransferOwner;
        let expected = vec![1];
        assert_eq!(instruction.try_to_vec().unwrap(), expected);
        assert_eq!(
            VaultInstruction::try_from_slice(&expected).unwrap(),
            instruction
        );
    }

    #[test]
    fn serialize_close_account() {
        let instruction = VaultInstruction::CloseAccount;
        let expected = vec![2];
        assert_eq!(instruction.try_to_vec().unwrap(), expected);
        assert_eq!(
            VaultInstruction::try_from_slice(&expected).unwrap(),
            instruction
        );
    }

    #[test]
    fn deserialize_invalid_instruction() {
        let expected = vec![12]; // Invalid instruction numeric
        let err: ProgramError = VaultInstruction::try_from_slice(&expected)
            .unwrap_err()
            .into();
        assert!(matches!(err, ProgramError::BorshIoError(_)));
    }
}

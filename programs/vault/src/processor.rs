use {
    crate::{error::VaultError, instruction::VaultInstruction, state::VaultRecord},
    borsh::BorshDeserialize,
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        program_pack::IsInitialized,
        pubkey::Pubkey,
    },
};

fn validate_signer(account: &AccountInfo, key: &Pubkey) -> ProgramResult {
    if key != account.key {
        msg!("Account key mismatch");
        return Err(VaultError::IncorrectAccountKey.into());
    }
    if !account.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }
    Ok(())
}

/// Instruction processor
pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let instruction = VaultInstruction::try_from_slice(input)?;
        match instruction {
            VaultInstruction::Initialize => {
                msg!("VaultInstruction::Initialize");
                Processor::process_initialize(program_id, accounts)
            }
            VaultInstruction::TransferOwner => {
                msg!("VaultInstruction::TransferOwner");
                Processor::transfer_owner(program_id, accounts)
            }
            VaultInstruction::CloseAccount => {
                msg!("VaultInstruction::CloseAccount");
                Processor::close_account(program_id, accounts)
            }
        }
    }

    // Initialize a vault record (by DART on behalf of a given owner).
    fn process_initialize(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let pda = next_account_info(account_info_iter)?;
        let dart = next_account_info(account_info_iter)?;
        let owner = next_account_info(account_info_iter)?;

        // Check that the owner of the pda is the program.
        if pda.owner != program_id {
            msg!("invalid program id");
            return Err(ProgramError::IncorrectProgramId);
        }

        if !dart.is_signer {
            msg!("Missing required DART signature in initialize");
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut record = VaultRecord::try_from_slice(*pda.data.borrow())?;
        if record.is_initialized() {
            msg!("Vault record account already initialized");
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        record.dart = *dart.key;
        record.owner = *owner.key;
        record.version = VaultRecord::CURRENT_VERSION;

        borsh::to_writer(&mut pda.data.borrow_mut()[..], &record).map_err(|e| e.into())
    }

    // Transfer ownership of a vault record
    fn transfer_owner(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let pda = next_account_info(account_info_iter)?;
        let dart = next_account_info(account_info_iter)?;
        let owner = next_account_info(account_info_iter)?;
        let new_owner = next_account_info(account_info_iter)?;

        if pda.owner != program_id {
            msg!("invalid program id");
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut record = VaultRecord::try_from_slice(&pda.data.borrow())?;
        if !record.is_initialized() {
            msg!("vault account not initialized");
            return Err(ProgramError::UninitializedAccount);
        }

        // Ensure the current owner on record is correct.
        if record.owner != *owner.key {
            return Err(ProgramError::IllegalOwner);
        }

        validate_signer(dart, &record.dart)?;

        record.owner = *new_owner.key;

        borsh::to_writer(&mut pda.data.borrow_mut()[..], &record).map_err(|e| e.into())
    }

    // Close a vault record account, draining lamports to the current owner.
    fn close_account(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let pda = next_account_info(account_info_iter)?;
        let dart = next_account_info(account_info_iter)?;
        let owner = next_account_info(account_info_iter)?;

        if pda.owner != program_id {
            msg!("invalid program id");
            return Err(ProgramError::IncorrectProgramId);
        }

        let record = VaultRecord::try_from_slice(&pda.data.borrow())?;
        if !record.is_initialized() {
            msg!("record not initialized");
            return Err(ProgramError::UninitializedAccount);
        }

        // Ensure the intermediary signed off on the withdrawal
        validate_signer(dart, &record.dart)?;

        // Ensure the owner on record is correct.
        if record.owner != *owner.key {
            return Err(ProgramError::IllegalOwner);
        }

        let owner_starting_lamports = owner.lamports();
        let pda_lamports = pda.lamports();

        // TODO: Should DART get a fee?

        **pda.lamports.borrow_mut() = 0;
        **owner.lamports.borrow_mut() = owner_starting_lamports
            .checked_add(pda_lamports)
            .ok_or(VaultError::Overflow)?;

        borsh::to_writer(&mut pda.data.borrow_mut()[..], &record).map_err(|e| e.into())
    }
}

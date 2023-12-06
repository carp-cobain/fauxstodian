#![cfg(feature = "test-sbf")]
use {
    solana_program::{
        borsh0_10::get_packed_len, instruction::InstructionError, pubkey::Pubkey, rent::Rent,
        system_instruction,
    },
    solana_program_test::*,
    solana_sdk::{
        signature::{Keypair, Signer},
        transaction::{Transaction, TransactionError},
    },
    vault::{id, instruction, processor::Processor, state::VaultRecord},
};

fn program_test() -> ProgramTest {
    ProgramTest::new("vault", id(), processor!(Processor::process_instruction))
}

// Helper: create and initialize a vault account.
async fn initialize_account(
    context: &mut ProgramTestContext,
    pda: &Keypair,
    dart: &Keypair,
    owner: &Keypair,
) {
    // Rent
    let space = VaultRecord::LEN;
    let lamports = Rent::default().minimum_balance(space);
    println!("rent lamports = {lamports}");

    let transaction = Transaction::new_signed_with_payer(
        &[
            system_instruction::create_account(
                &context.payer.pubkey(),
                &pda.pubkey(),
                lamports,
                space as u64,
                &id(),
            ),
            instruction::initialize(&pda.pubkey(), &dart.pubkey(), &owner.pubkey()),
        ],
        Some(&context.payer.pubkey()),
        &[&context.payer, pda, dart],
        context.last_blockhash,
    );
    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();
}

#[tokio::test]
async fn initialize_success() {
    let mut context = program_test().start_with_context().await;

    let pda = Keypair::new();
    let dart = Keypair::new();
    let owner = Keypair::new();

    initialize_account(&mut context, &pda, &dart, &owner).await;
    let account_data = context
        .banks_client
        .get_account_data_with_borsh::<VaultRecord>(pda.pubkey())
        .await
        .unwrap();
    assert_eq!(account_data.dart, dart.pubkey());
    assert_eq!(account_data.owner, owner.pubkey());
    assert_eq!(account_data.version, VaultRecord::CURRENT_VERSION);
}

#[tokio::test]
async fn initialize_with_seed_success() {
    let mut context = program_test().start_with_context().await;

    let dart = Keypair::new();
    let seed = "U5f76katXToqua7SJzvP7"; // Could be DART account primary key
    let pda = Pubkey::create_with_seed(&dart.pubkey(), seed, &id()).unwrap();
    let owner = Keypair::new();

    // Rent
    let space = get_packed_len::<VaultRecord>();
    let lamports = Rent::default().minimum_balance(space);
    assert_eq!(space, VaultRecord::LEN);

    let transaction = Transaction::new_signed_with_payer(
        &[
            system_instruction::create_account_with_seed(
                &context.payer.pubkey(),
                &pda,
                &dart.pubkey(),
                seed,
                lamports,
                space as u64,
                &id(),
            ),
            instruction::initialize(&pda, &dart.pubkey(), &owner.pubkey()),
        ],
        Some(&context.payer.pubkey()),
        &[&context.payer, &dart],
        context.last_blockhash,
    );
    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();
    let account_data = context
        .banks_client
        .get_account_data_with_borsh::<VaultRecord>(pda)
        .await
        .unwrap();
    assert_eq!(account_data.dart, dart.pubkey());
    assert_eq!(account_data.owner, owner.pubkey());
    assert_eq!(account_data.version, VaultRecord::CURRENT_VERSION);
}

#[tokio::test]
async fn initialize_twice_fail() {
    let mut context = program_test().start_with_context().await;

    let pda = Keypair::new();
    let dart = Keypair::new();
    let owner = Keypair::new();

    // First init (success)
    initialize_account(&mut context, &pda, &dart, &owner).await;

    // Second init (should fail)
    let transaction = Transaction::new_signed_with_payer(
        &[instruction::initialize(
            &pda.pubkey(),
            &dart.pubkey(),
            &owner.pubkey(),
        )],
        Some(&context.payer.pubkey()),
        &[&context.payer, &dart],
        context.last_blockhash,
    );
    assert_eq!(
        context
            .banks_client
            .process_transaction(transaction)
            .await
            .unwrap_err()
            .unwrap(),
        TransactionError::InstructionError(0, InstructionError::AccountAlreadyInitialized)
    );
}

#[tokio::test]
async fn transfer_owner_success() {
    let mut context = program_test().start_with_context().await;

    let pda = Keypair::new();
    let dart = Keypair::new();
    let owner = Keypair::new();

    initialize_account(&mut context, &pda, &dart, &owner).await;

    // The new owner
    let new_owner = Keypair::new();

    // Tx must be signed by DART -and- the existing owner.
    let transaction = Transaction::new_signed_with_payer(
        &[instruction::transfer_owner(
            &pda.pubkey(),
            &dart.pubkey(),
            &owner.pubkey(),
            &new_owner.pubkey(),
        )],
        Some(&context.payer.pubkey()),
        &[&context.payer, &dart],
        context.last_blockhash,
    );
    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    let record = context
        .banks_client
        .get_account_data_with_borsh::<VaultRecord>(pda.pubkey())
        .await
        .unwrap();

    // Ensure the new owner was set in the record.
    assert_eq!(record.owner, new_owner.pubkey());
}

#[tokio::test]
async fn transfer_owner_fail_wrong_owner() {
    let mut context = program_test().start_with_context().await;

    let pda = Keypair::new();
    let dart = Keypair::new();
    let owner = Keypair::new();

    initialize_account(&mut context, &pda, &dart, &owner).await;

    // The new owner
    let new_owner = Keypair::new();

    // Try to use this as the owner
    let wrong_owner = Keypair::new();

    let transaction = Transaction::new_signed_with_payer(
        &[instruction::transfer_owner(
            &pda.pubkey(),
            &dart.pubkey(),
            &wrong_owner.pubkey(),
            &new_owner.pubkey(),
        )],
        Some(&context.payer.pubkey()),
        &[&context.payer, &dart],
        context.last_blockhash,
    );

    assert_eq!(
        context
            .banks_client
            .process_transaction(transaction)
            .await
            .unwrap_err()
            .unwrap(),
        TransactionError::InstructionError(0, InstructionError::IllegalOwner)
    );
}

#[tokio::test]
async fn close_account_success() {
    let mut context = program_test().start_with_context().await;

    let pda = Keypair::new();
    let dart = Keypair::new();
    let owner = Keypair::new();

    initialize_account(&mut context, &pda, &dart, &owner).await;

    let transaction = Transaction::new_signed_with_payer(
        &[instruction::close_account(
            &pda.pubkey(),
            &dart.pubkey(),
            &owner.pubkey(),
        )],
        Some(&context.payer.pubkey()),
        &[&context.payer, &dart],
        context.last_blockhash,
    );
    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    let recipient = context
        .banks_client
        .get_account(owner.pubkey())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        recipient.lamports,
        Rent::default().minimum_balance(get_packed_len::<VaultRecord>())
    );
}

#[tokio::test]
async fn close_account_fail_wrong_owner() {
    let mut context = program_test().start_with_context().await;

    let pda = Keypair::new();
    let dart = Keypair::new();
    let owner = Keypair::new();

    initialize_account(&mut context, &pda, &dart, &owner).await;

    let wrong_owner = Keypair::new();
    let transaction = Transaction::new_signed_with_payer(
        &[instruction::close_account(
            &pda.pubkey(),
            &dart.pubkey(),
            &wrong_owner.pubkey(),
        )],
        Some(&context.payer.pubkey()),
        &[&context.payer, &dart],
        context.last_blockhash,
    );
    assert_eq!(
        context
            .banks_client
            .process_transaction(transaction)
            .await
            .unwrap_err()
            .unwrap(),
        TransactionError::InstructionError(0, InstructionError::IllegalOwner)
    );
}

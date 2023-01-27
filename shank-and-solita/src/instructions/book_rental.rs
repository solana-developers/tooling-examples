use {
    borsh::BorshSerialize,
    solana_program::{
        account_info::{AccountInfo, next_account_info}, 
        entrypoint::ProgramResult, 
        program::invoke_signed,
        pubkey::Pubkey,
        rent::Rent,
        system_instruction,
        sysvar::Sysvar,
    },
};
use crate::state::RentalOrder;

pub fn book_rental(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    rental_order: RentalOrder,
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let rental_order_account = next_account_info(accounts_iter)?;
    let car_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let (rental_order_account_pda, rental_order_account_bump) = Pubkey::find_program_address(
        &[
            RentalOrder::SEED_PREFIX.as_bytes().as_ref(),
            car_account.key.as_ref(),
            payer.key.as_ref(),
        ],
        program_id,
    );
    assert!(&rental_order_account_pda == rental_order_account.key);

    let account_span = (rental_order.try_to_vec()?).len();
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    invoke_signed(
        &system_instruction::create_account(
            &payer.key,
            &rental_order_account.key,
            lamports_required,
            account_span as u64,
            program_id,
        ),
        &[
            payer.clone(), rental_order_account.clone(), system_program.clone()
        ],
        &[&[
            RentalOrder::SEED_PREFIX.as_bytes().as_ref(),
            car_account.key.as_ref(),
            payer.key.as_ref(),
            &[rental_order_account_bump],
        ]]
    )?;
    
    rental_order.serialize(&mut &mut rental_order_account.data.borrow_mut()[..])?;

    Ok(())
}
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
use crate::state::Car;

pub fn add_car(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    car: Car,
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let car_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let (car_account_pda, car_account_bump) = Pubkey::find_program_address(
        &[
            Car::SEED_PREFIX.as_bytes().as_ref(),
            car.year.to_le_bytes().as_ref(),
            car.make.as_bytes().as_ref(),
            car.model.as_bytes().as_ref(),
        ],
        program_id,
    );
    assert!(&car_account_pda == car_account.key);

    let account_span = (car.try_to_vec()?).len();
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    invoke_signed(
        &system_instruction::create_account(
            &payer.key,
            &car_account.key,
            lamports_required,
            account_span as u64,
            program_id,
        ),
        &[
            payer.clone(), car_account.clone(), system_program.clone()
        ],
        &[&[
            Car::SEED_PREFIX.as_bytes().as_ref(),
            car.year.to_le_bytes().as_ref(),
            car.make.as_bytes().as_ref(),
            car.model.as_bytes().as_ref(),
            &[car_account_bump],
        ]]
    )?;
    
    car.serialize(&mut &mut car_account.data.borrow_mut()[..])?;

    Ok(())
}
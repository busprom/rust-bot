use solana_program::{
  msg, program::invoke,
  account_info::AccountInfo,
  entrypoint::ProgramResult
};
use crate::{
  token::{
    create_token_account::process_create_token_account,
    check_count::process_check_count
  }
};


pub fn process_transfer_token<'a>(
  payer: &AccountInfo<'a>,
  mint: &AccountInfo<'a>,
  new_owner: &AccountInfo<'a>,
  token_account: &AccountInfo<'a>,
  vault: &AccountInfo<'a>,
  profit_id: &AccountInfo<'a>,
  token_program: &AccountInfo<'a>,
  rent_info: &AccountInfo<'a>,
  spl_token_program: &AccountInfo<'a>,
  system_program: &AccountInfo<'a>,
  amount: u64
) -> ProgramResult {

  if vault.data_is_empty() {
    process_create_token_account(
      payer,
      new_owner,
      mint,
      vault,
      token_program,
      rent_info,
      system_program,
      spl_token_program
    )?;
  }

  msg!("Transfer token in vault");
  invoke(
    &spl_token::instruction::transfer(
      token_program.key,
      token_account.key,
      vault.key,
      payer.key,
      &[payer.key],
      amount,
    ).unwrap(), 
    &[token_account.clone(), vault.clone(), payer.clone()]
  )?;

  if process_check_count(token_account) == false {
    invoke(
      &spl_token::instruction::close_account(
        token_program.key,
        token_account.key,
        profit_id.key,
        payer.key,
        &[payer.key]
      ).unwrap(),
      &[token_account.clone(), profit_id.clone(), payer.clone()]
    )?;
  }
  
  Ok(())
}
use solana_program::{
  program::invoke, msg,
  account_info::AccountInfo,
  entrypoint::ProgramResult
};
use crate::{PDA_SEED, types::radium::Query};

pub fn process_create_pda<'a>(
  user_owner: &AccountInfo<'a>,
  user_source_token_account: &AccountInfo<'a>,
  token_program: &AccountInfo<'a>,
  rent_program: &AccountInfo<'a>,
  sol: &AccountInfo<'a>,
  query: Query
) -> ProgramResult {

  invoke(
    &solana_program::system_instruction::create_account_with_seed(
      user_owner.key,
      user_source_token_account.key,
      user_owner.key,
      PDA_SEED,
      query.amount_in + 3000000,
      165,
      token_program.key
    ),
    &[user_owner.clone(), user_source_token_account.clone(), rent_program.clone()]
  )?;

  msg!("Go!");

  invoke(
    &spl_token::instruction::initialize_account(
      token_program.key,
      user_source_token_account.key,
      &sol.key,
      user_owner.key
    ).unwrap(), 
    &[user_source_token_account.clone(), user_owner.clone(), sol.clone(), rent_program.clone()]
  )?;

  Ok(())
}
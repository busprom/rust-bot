use solana_program::{
	msg,
	entrypoint::ProgramResult,
	account_info::AccountInfo
};
use crate::{
  token::{
    swap::process_swap,
    create_token_account::process_create_token_account
  },
  types::radium::{Swap, Sell}
};
use spl_token::state::Account;
use solana_program::program_pack::Pack;


pub fn process_sell<'a>(
  token_program: &AccountInfo<'a>,
  amm_id: &AccountInfo<'a>,
  amm_authority: &AccountInfo<'a>,
  amm_open_orders: &AccountInfo<'a>,
  amm_target_orders: &AccountInfo<'a>,
  pool_coin_token_account: &AccountInfo<'a>,
  pool_pc_token_account: &AccountInfo<'a>,
  serum_program_id: &AccountInfo<'a>,
  serum_market: &AccountInfo<'a>,
  serum_bids: &AccountInfo<'a>,
  serum_asks: &AccountInfo<'a>,
  serum_event_queue: &AccountInfo<'a>,
  serum_coin_vault_account: &AccountInfo<'a>,
  serum_pc_vault_account: &AccountInfo<'a>,
  serum_vault_signer: &AccountInfo<'a>,
  user_source_token_account: &AccountInfo<'a>,
  user_dest_token_account: &AccountInfo<'a>,
  user_owner: &AccountInfo<'a>,
  swap_program: &AccountInfo<'a>,
  rent_program: &AccountInfo<'a>,
  system_program: &AccountInfo<'a>,
  spl_token_program: &AccountInfo<'a>,
  sol: &AccountInfo<'a>,
  sell: Sell
) -> ProgramResult {
  msg!("Sell");
  
  process_create_token_account(
    user_owner,
    user_owner,
    sol,
    user_dest_token_account,
    token_program,
    rent_program,
    system_program,
    spl_token_program
  )?;

  let spl_token_account_data = user_source_token_account.try_borrow_data()?;
  let spl_token_account = Account::unpack(&spl_token_account_data)?;

  let mut amount_in = (((spl_token_account.amount * sell.to_sell) / 100) as f32).floor() as u64;

  if sell.to_sell == 100 {
    amount_in = spl_token_account.amount;
  }
  
  let data = Swap {
    instruction: 9,
    amount_in,
    min_amount_out: 0
  };

  drop(spl_token_account_data);

  process_swap(
    token_program,
    amm_id,
    amm_authority,
    amm_open_orders,
    amm_target_orders,
    pool_coin_token_account,
    pool_pc_token_account,
    serum_program_id,
    serum_market,
    serum_bids,
    serum_asks,
    serum_event_queue,
    serum_coin_vault_account,
    serum_pc_vault_account,
    serum_vault_signer,
    user_source_token_account,
    user_dest_token_account,
    user_owner,
    swap_program,
    data
  )?;

  
  Ok(())
}
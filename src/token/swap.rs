use borsh::BorshSerialize;
use solana_program::{
  account_info::AccountInfo,
  instruction::{AccountMeta, Instruction},
  entrypoint::ProgramResult, 
  program::invoke
};
use crate::{
  types::radium::Swap
};

pub fn process_swap<'a>(
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
  data: Swap
) -> ProgramResult {

  let accounts = vec![
    AccountMeta::new_readonly(*token_program.key, false),
    AccountMeta::new(*amm_id.key, false),
    AccountMeta::new_readonly(*amm_authority.key, false),
    AccountMeta::new(*amm_open_orders.key, false),
    AccountMeta::new(*amm_target_orders.key, false),
    AccountMeta::new(*pool_coin_token_account.key, false),
    AccountMeta::new(*pool_pc_token_account.key, false),
    AccountMeta::new_readonly(*serum_program_id.key, false),
    AccountMeta::new(*serum_market.key, false),
    AccountMeta::new(*serum_bids.key, false),
    AccountMeta::new(*serum_asks.key, false),
    AccountMeta::new(*serum_event_queue.key, false),
    AccountMeta::new(*serum_coin_vault_account.key, false),
    AccountMeta::new(*serum_pc_vault_account.key, false),
    AccountMeta::new_readonly(*serum_vault_signer.key, false),
    AccountMeta::new(*user_source_token_account.key, false),
    AccountMeta::new(*user_dest_token_account.key, false),
    AccountMeta::new(*user_owner.key, true),
  ];

  let sign = [
    token_program.clone(),
    amm_id.clone(),
    amm_authority.clone(),
    amm_open_orders.clone(),
    amm_target_orders.clone(),
    pool_coin_token_account.clone(),
    pool_pc_token_account.clone(),
    serum_program_id.clone(),
    serum_market.clone(),
    serum_bids.clone(),
    serum_asks.clone(),
    serum_event_queue.clone(),
    serum_coin_vault_account.clone(),
    serum_pc_vault_account.clone(),
    serum_vault_signer.clone(),
    user_source_token_account.clone(),
    user_dest_token_account.clone(),
    user_owner.clone()
  ];

  invoke(
    &Instruction {
      program_id: *swap_program.key,
      data: Swap::try_to_vec(&data).unwrap(),
      accounts
    },
    &sign
  )?;

  Ok(())
}
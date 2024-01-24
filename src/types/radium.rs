use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct RadiumV4 {
  pub status: u64,
  pub nonce: u64,
  pub max_order: u64,
  pub depth: u64,
  pub base_decimal: u64,
  pub quote_decimal: u64,
  pub state: u64,
  pub reset_flag: u64,
  pub min_size: u64,
  pub vol_ax_cut_ratio: u64,
  pub amount_wave_ratio: u64,
  pub base_lot_size: u64,
  pub quote_lot_size: u64,
  pub min_price_multiplier: u64,
  pub max_price_multiplier: u64,
  pub system_decimal_value: u64,
  pub min_eparate_numerator: u64,
  pub min_separate_denominator: u64,
  pub trade_fee_numerator: u64,
  pub trade_fee_denominator: u64,
  pub pnl_numerator: u64,
  pub pnl_denominator: u64,
  pub swap_fee_numerator: u64,
  pub swap_fee_denominator: u64,
  pub base_need_take_pnl: u64,
  pub quote_need_take_pnl: u64,
  pub quote_total_pnl: u64,
  pub base_total_pnl: u64,
  pub pool_open_time: u64,
  pub punish_pc_amount: u64,
  pub punish_coin_amount: u64,
  pub orderbook_to_init_time: u64,
  pub swap_base_in_amount: u128,
  pub swap_quote_out_amount: u128,
  pub swap_base2_quote_fee: u64,
  pub swap_quote_in_amount: u128,
  pub swap_base_out_amount: u128,
  pub swap_quote2_base_fee: u64,
  pub base_vault: Pubkey,
  pub quote_vault: Pubkey,
  pub base_mint: Pubkey,
  pub quote_mint: Pubkey,
  pub lp_mint: Pubkey,
  pub open_orders: Pubkey,
  pub market_id: Pubkey,
  pub market_program_id: Pubkey,
  pub target_orders: Pubkey,
  pub withdraw_queue: Pubkey,
  pub lp_vault: Pubkey,
  pub owner: Pubkey,
  pub lp_reserve: u64,
  pub padding: [u64; 3]
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Swap {
  pub instruction: u8, //9 - 11
  pub amount_in: u64, 
  pub min_amount_out: u64
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Query {
  pub side: u8, //0 buy / 1 sell,
  pub amount_in: u64, 
  pub min_amount_out: u64,
  pub min_quote_in: u128
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Bot {
  pub side: u8, //0 buy / 1 sell,
  pub amount_in: u64, 
  pub min_amount_out: u64,
  pub min_quote_in: u128,
  pub quote: Pubkey
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Sell {
  pub to_sell: u64
}
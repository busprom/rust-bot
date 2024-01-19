use borsh::BorshDeserialize;
use solana_program::{
	pubkey::Pubkey,
	entrypoint::ProgramResult,
	account_info::{next_account_info, AccountInfo}
};
use crate::{
	utils::mint_token::process_mint_token,
	instruction::SolInstruction
};
pub struct Processor;

impl Processor {
	pub fn process(_program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
		let instruction = SolInstruction::try_from_slice(input)?;
		match instruction {
			SolInstruction::Radium {data} => {
				let account_info_iter = &mut accounts.iter();
				let token_program = next_account_info(account_info_iter)?;
				let amm_id = next_account_info(account_info_iter)?;
				let amm_authority = next_account_info(account_info_iter)?;
				let amm_open_orders = next_account_info(account_info_iter)?;
				let amm_target_orders = next_account_info(account_info_iter)?;
				let pool_coin_token_account = next_account_info(account_info_iter)?;
				let pool_pc_token_account = next_account_info(account_info_iter)?;
				let serum_program_id = next_account_info(account_info_iter)?;
				let serum_market = next_account_info(account_info_iter)?;
				let serum_bids = next_account_info(account_info_iter)?;
				let serum_asks = next_account_info(account_info_iter)?;
				let serum_event_queue = next_account_info(account_info_iter)?;
				let serum_coin_vault_account = next_account_info(account_info_iter)?;
				let serum_pc_vault_account = next_account_info(account_info_iter)?;
				let serum_vault_signer = next_account_info(account_info_iter)?;
				let user_source_token_account = next_account_info(account_info_iter)?;
				let user_dest_token_account = next_account_info(account_info_iter)?;
				let user_owner = next_account_info(account_info_iter)?;
				let swap_program = next_account_info(account_info_iter)?;
				let token_mint = next_account_info(account_info_iter)?;
				let rent_program = next_account_info(account_info_iter)?;
				let system_program = next_account_info(account_info_iter)?;
				let spl_token_program = next_account_info(account_info_iter)?;
				process_mint_token(
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
					token_mint,
					rent_program,
					system_program,
					spl_token_program,
					data
				)
			}
		}
	}
}
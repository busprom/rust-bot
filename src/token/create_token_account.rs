use solana_program::{
  program::{invoke},
  instruction::{AccountMeta, Instruction},
  account_info::AccountInfo,
  entrypoint::ProgramResult
};

pub fn process_create_token_account<'a>(
  payer: &AccountInfo<'a>,
  new_owner: &AccountInfo<'a>,
  mint: &AccountInfo<'a>,
  mint_account: &AccountInfo<'a>,
  token_program: &AccountInfo<'a>,
  rent_program: &AccountInfo<'a>,
  system_program: &AccountInfo<'a>,
  spl_token_program: &AccountInfo<'a>
) -> ProgramResult {

  if mint_account.data_is_empty() {
    invoke(
      &Instruction {
        program_id: *spl_token_program.key,
        data: vec![],
        accounts: vec![
          AccountMeta::new(*payer.key, true),
          AccountMeta::new(*mint_account.key, false),
          AccountMeta::new(*new_owner.key, false),
          AccountMeta::new(*mint.key, false),
          AccountMeta::new_readonly(*system_program.key, false),
          AccountMeta::new_readonly(*token_program.key, false),
          AccountMeta::new_readonly(*rent_program.key, false)
        ]
      },
      &[
        payer.clone(),
        mint_account.clone(),
        new_owner.clone(),
        mint.clone(),
        system_program.clone(),
        token_program.clone(),
        rent_program.clone()
      ]
    )?;
  }

  Ok(())
}
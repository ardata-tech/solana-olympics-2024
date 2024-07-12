use crate::error::TokenSaleError;
use crate::state::{find_vault_pda, TokenBase};
use crate::{
    instruction::accounts::{Context, OpenSaleAccounts},
    require,
};

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey, system_instruction,
};

pub fn process_open_sale(
    program_id: &Pubkey,
    ctx: Context<OpenSaleAccounts>,
    supply: u64,
    price: u64,
    decimals: u8,
    whitelist_root: [u8; 32],
    nonce: u32,
) -> ProgramResult {
    // Account Validations

    // 1. token_base
    //
    // - owner is token_sale (this) program
    // - correct allocation length (TokenBase::LEN)
    // - account is unintialized
    let data = ctx.accounts.token_base.try_borrow_mut_data()?;

    // - owner is token_sale (this) program
    require!(
        ctx.accounts.token_base.owner == program_id,
        ProgramError::InvalidAccountOwner,
        "token_base"
    );

    // - correct allocation length (TokenBase::LEN)
    require!(
        data.len() == TokenBase::LEN,
        TokenSaleError::InvalidAccountDataLength,
        "token_base"
    );

    let token_base = TokenBase::try_from_slice(&data)?;

    // - account is unintialized
    require!(
        token_base.is_uninitialized(),
        ProgramError::AccountAlreadyInitialized,
        "token_base"
    );

    // 2. mint
    //
    // - owner is token_sale (this) program
    // - correct allocation length (TokenBase::LEN)
    // - account is unintialized

    // Processing Instruction

    // create vault ATA
    let (vault_pda, bump) = find_vault_pda(program_id, ctx.accounts.token_base.key);

    // token_base.vault = vault_pda;
    // token_base.vault_bump = vault_bump;
    // token_base.supply = supply;
    // token_base.

    // create token_base
    Ok(())
}

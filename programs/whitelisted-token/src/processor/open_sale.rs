use crate::error::TokenSaleError;
use crate::state::{find_token_base_pda, TokenBase};
use crate::{
    instruction::accounts::{Context, OpenSaleAccounts},
    require,
};
use borsh::BorshDeserialize;
use solana_program::{
    entrypoint::ProgramResult, program_error::ProgramError, program_pack::Pack, pubkey::Pubkey,
};
use spl_token::{error::TokenError, state::Mint};

/// Open a Token Sale with the given config
///
/// Validates the accounts and data passed then
/// initializes the [`TokenBase`] (config)
///
/// Accounts
/// 0. `[WRITE]` `Token Base` config account, PDA generated offchain
/// 1. `[]`         `Mint` account
/// 1. `[]`         `Vault` account
/// 2. `[SIGNER]`   `Sale Authority` account
///
/// Instruction Data
/// - price: u64,
/// - whitelist_root: [u8; 32],
pub fn process_open_sale(
    program_id: &Pubkey,
    ctx: Context<OpenSaleAccounts>,
    price: u64,
    whitelist_root: [u8; 32],
    nonce: u8,
) -> ProgramResult {
    //---------- Account Validations ----------

    // 0. token_base
    //
    // - owner is token_sale (this) program
    // - correct allocation length (TokenBase::LEN)
    // - account is unintialized
    // - token_base seeds must be ["token_base", pubkey(sale_authority), nonce]
    let token_base_data = ctx.accounts.token_base.try_borrow_mut_data()?;
    let mut token_base = TokenBase::try_from_slice(&token_base_data)?;

    // - owner is token_sale (this) program
    require!(
        ctx.accounts.token_base.owner == program_id,
        ProgramError::InvalidAccountOwner,
        "token_base"
    );

    // - correct allocation length (TokenBase::LEN)
    require!(
        token_base_data.len() == TokenBase::LEN,
        TokenSaleError::InvalidAccountDataLength,
        "token_base"
    );

    // - account is unintialized
    require!(
        token_base.is_uninitialized(),
        ProgramError::AccountAlreadyInitialized,
        "token_base"
    );

    // - token_base seeds must be ["token_base", pubkey(sale_authority), nonce]
    let (token_base_pda, token_base_bump) =
        find_token_base_pda(program_id, &token_base.sale_authority, nonce);
    require!(
        *ctx.accounts.token_base.key == token_base_pda,
        TokenSaleError::UnexpectedPDASeeds,
        "token_base"
    );

    // 1. mint
    //
    // - is_initialized is true
    // - mint_authority is token_base sale_authority
    let mint = ctx.accounts.mint;
    let mint_data = mint.try_borrow_data()?;
    let mint_state = Mint::unpack(&mint_data)?;

    // - is_initialized is true
    require!(
        mint_state.is_initialized,
        TokenError::UninitializedState,
        "mint"
    );

    // - mint_authority is token_base sale_authority
    require!(
        mint_state.mint_authority == token_base.sale_authority.into(),
        TokenSaleError::MintAndSaleAuthorityMismatch,
        "mint"
    );

    // 2. vault
    //
    // - not executable
    let vault = ctx.accounts.vault;

    // - not executable
    require!(
        !vault.executable,
        TokenSaleError::VaultMustBeNonExecutable,
        "vault"
    );

    // 3. sale_authority
    //
    // - not executable
    // - must be signer
    let sale_authority = ctx.accounts.sale_authority;

    // - not executable
    require!(
        !sale_authority.executable,
        TokenSaleError::VaultMustBeNonExecutable,
        "sale_authority"
    );

    // - must be signer
    require!(
        !sale_authority.is_signer,
        TokenSaleError::SaleAuthorityNotSigner,
        "sale_authority"
    );

    //---------- Data Validations (if any) ----------

    //---------- Executing Instruction ----------

    token_base.mint = *mint.key;
    token_base.vault = *vault.key;
    token_base.sale_authority = *sale_authority.key;
    token_base.whitelist_root = whitelist_root;
    token_base.price = price;
    token_base.bump = token_base_bump; // store canonical bump

    Ok(())
}

use crate::error::TokenSaleError;
use crate::merkle::WhitelistProof;
use crate::state::{find_token_base_pda, TokenBase};
use crate::{
    instruction::accounts::{BuyTokenAccounts, Context},
    require,
};
use borsh::BorshDeserialize;
use solana_program::{
    entrypoint::ProgramResult, program_error::ProgramError, program_pack::Pack, pubkey::Pubkey,
};
use spl_token::{error::TokenError, state::Mint};

/// Buy N amount of Tokens
///
/// - Initializes Associated Token Account for Buyer
/// - Transfers SOL (lamports) from Buyer to Vault
/// - Mints Token to Buyer account
///
/// Accounts
/// 0. `[WRITE]`    `Token Base` config account, PDA generated offchain
/// 1. `[WRITE]`         `Buyer` token account
/// 1. `[SIGNER]`   `Buyer` account
///
/// Instruction Data
/// - amount: u64,
/// - proof: WhitelistProof
pub fn process_buy_token(
    program_id: &Pubkey,
    ctx: Context<BuyTokenAccounts>,
    amount: u64,
    proof: WhitelistProof,
) -> ProgramResult {
    //---------- Account Validations ----------

    // 0. token_base
    //
    // - owner is token_sale (this) program
    // - correct allocation length (TokenBase::LEN)
    // - account is initialized
    // - token_base seeds must be ["token_base", pubkey(mint)]

    // - owner is token_sale (this) program
    require!(
        ctx.accounts.token_base.owner == program_id,
        ProgramError::InvalidAccountOwner,
        "token_base"
    );

    // - correct allocation length (TokenBase::LEN)
    let token_base_data = ctx.accounts.token_base.try_borrow_mut_data()?;
    require!(
        token_base_data.len() == TokenBase::LEN,
        TokenSaleError::InvalidAccountDataLength,
        "token_base"
    );

    // - account is initialized
    let mut token_base = TokenBase::try_from_slice(&token_base_data)?;
    require!(
        token_base.is_initialized(),
        ProgramError::UninitializedAccount,
        "token_base"
    );

    // - token_base seeds must be ["token_base", pubkey(mint)]
    let (token_base_pda, token_base_bump) = find_token_base_pda(program_id, &token_base.mint);
    require!(
        *ctx.accounts.token_base.key == token_base_pda,
        TokenSaleError::UnexpectedPDASeeds,
        "token_base"
    );

    // 1. buyer_token_account
    //
    // - mint must be token_base mint

    // 2. buyer
    //
    // - not executable
    // - must be signer
    let buyer = ctx.accounts.buyer;

    // - not executable
    require!(
        !buyer.executable,
        TokenSaleError::MustBeNonExecutable,
        "sale_authority"
    );

    // - must be signer
    require!(
        buyer.is_signer,
        TokenSaleError::SaleAuthorityNotSigner,
        "sale_authority"
    );

    //---------- Data Validations (if any) ----------

    //---------- Executing Instruction ----------

    Ok(())
}

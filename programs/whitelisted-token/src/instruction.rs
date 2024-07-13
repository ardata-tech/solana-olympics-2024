use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};

/// TokenSale Instruction List
///
/// For Token Sale Authority:
/// - OpenSale
/// - ConfigureSale
/// - CloseSale
///
/// For Buyer:
/// - BuyToken
///
#[derive(BorshDeserialize, BorshSerialize, Debug, ShankContext, ShankInstruction)]
pub enum TokenSaleInstruction {
    /// Open a token sale by initializing the [`TokenBase`] config
    ///
    /// For Token Sale Authority
    #[account(
        0,
        writable,
        name = "token_base",
        desc = "Account (TokenBase PDA) holding token sale configuration. Seeds ['token_base', `token_base::mint`]"
    )]
    #[account(
        1,
        name = "mint",
        desc = "Account for holding the mint details of the token being sold"
    )]
    #[account(
        2,
        name = "vault",
        desc = "Account for holding the funds raised from token sale"
    )]
    #[account(
        3,
        signer,
        name = "sale_authority",
        desc = "Account who has authority to manage the token sale"
    )]
    OpenSale {
        /// Price of token
        price: u64,
        /// Amount of tokens allowed per buyer wallet
        purchase_limit: u64,
        /// Merkle tree root of whitelist
        whitelist_root: [u8; 32],
    },

    /// Configure TokenBase config for token sale
    ///
    /// - vault
    /// - price
    /// - purchase_limit
    /// - whitelist_root
    ///
    /// For Token Sale Authority
    #[account(
        0,
        writable,
        name = "token_base",
        desc = "Account (TokenBase PDA) holding token sale configuration. Seeds ['token_base', `token_base::mint`]"
    )]
    #[account(
        1,
        signer,
        name = "sale_authority",
        desc = "Account who has authority to manage the token sale"
    )]
    #[account(
        2,
        name = "new_vault",
        desc = "New account for holding the funds raised from token sale"
    )]
    ConfigureSale {
        /// Price of token
        price: u64,
        /// Amount of tokens allowed per buyer wallet
        purchase_limit: u64,
        /// Merkle tree root of whitelist
        whitelist_root: [u8; 32],
    },

    /// Set new whitelist Merkle tree root
    ///
    /// For Token Sale Authority
    #[account(
        0,
        writable,
        name = "token_base",
        desc = "Account (TokenBase PDA) holding token sale configuration. Seeds ['token_base', `token_base::mint`]"
    )]
    #[account(
        1,
        signer,
        name = "sale_authority",
        desc = "Account who has authority to manage the token sale"
    )]
    CloseSale,

    /// Set new whitelist Merkle tree root
    ///
    /// For Buyers
    #[account(
        0,
        writable,
        name = "token_base",
        desc = "Account (TokenBase PDA) holding token sale configuration. Seeds ['token_base', `token_base::mint`]"
    )]
    #[account(
        1,
        signer,
        name = "sale_authority",
        desc = "Account who has authority to manage the token sale"
    )]
    BuyToken { amount: u64 },
}

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
    /// Opens a token sale
    ///
    /// - Initializes [`TokenBase`] config
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

    /// Configures TokenBase config for token sale
    ///
    /// - Atomically updates your [`TokenBase`]:
    /// vault, price, purchase_limit, whitelist_root
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

    /// Close the token sale
    ///
    /// - Closes the [`TokenBase`] account
    /// - Relinquishes rent lamports
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

    /// Buy N amount of Tokens
    ///
    /// - Initializes Associated Token Account for Buyer
    /// - Transfers SOL (lamports) from Buyer to Vault
    /// - Mints Token to Buyer account
    ///
    /// For Buyers
    #[account(
        0,
        name = "token_base",
        desc = "Account (TokenBase PDA) holding token sale configuration. Seeds ['token_base', `token_base::mint`]"
    )]
    #[account(
        1,
        name = "buyer_token_account",
        desc = "Account of owned by the buyer where newly bought tokens get transferred to"
    )]
    #[account(
        2,
        signer,
        name = "buyer",
        desc = "Account who is buying from token sale and will pay for the fees"
    )]
    BuyToken { amount: u64 },
}

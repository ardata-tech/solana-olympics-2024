use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};
use solana_program::pubkey::Pubkey;

/// TokenSale Instruction List
#[derive(BorshDeserialize, BorshSerialize, Debug, ShankContext, ShankInstruction)]
pub enum TokenSaleInstruction {
    /// Open a Token Sale with the given config
    ///
    /// Initializes the TokenBase config
    #[account(
        0,
        writable,
        name = "token_base",
        desc = "Account (TokenBase) holding token sale configuration"
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
        /// Merkle tree root of whitelist
        whitelist_root: [u8; 32],
        /// Randomness (or index) for multiple token bases per admin/s
        nonce: u32,
    },
    // /// Reconfigure the supply / price of a specific TokenBase
    // ConfigureSale {
    //     new_supply: u128,
    //     new_price: u64,
    //     nonce: u32,
    // },
    //
    // CloseSale {
    //     nonce: u32,
    // },
    //
    // BuyToken {
    //     amount: u64,
    // },
}

use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};

/// TokenSale Instruction List
#[derive(BorshDeserialize, BorshSerialize, Debug, ShankContext, ShankInstruction)]
pub enum TokenSaleInstruction {
    /// Open a TokenSale with the given config
    ///
    /// - Create a new mint account
    /// - Create new associated token account (vault) to hold supply for selling
    /// - Initialize token sale with starting configuration
    #[account(
        0,
        writable,
        name = "token_base",
        desc = "Account (TokenBase) holding token sale configuration"
    )]
    OpenSale {
        /// Token sale config
        supply: u64,
        price: u64,
        decimals: u8,
        /// For multiple token bases per admin/s
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

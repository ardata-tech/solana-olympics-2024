use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;
use spl_discriminator::{ArrayDiscriminator, SplDiscriminate};

// TODO: Struct packing, Cache-line optimization
/// TokenBase holding the token sale configuraiton
#[rustfmt::skip] // ensure manual struct ordering
#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount, SplDiscriminate)]
#[discriminator_hash_input("token_sale::state:token_base")]
pub struct TokenBase {
    /// Identifier for this specific data structure
    pub discriminator: [u8; 8],
    /// Amount of lamports to transfer from Buyer to Vault 
    /// when purchasing tokens
    pub price: u64,
    /// Authority that can 
    pub sale_authority: Pubkey,
    /// Mint created external to this program
    pub mint: Pubkey,
    /// Account holding the SOL from token sale
    pub vault: Pubkey,
    /// Account holding the SOL from token sale
    pub whitelist_root: [u8; 32],
    /// Canonical bump for TokenBase PDA
    pub bump: u8
}

impl TokenBase {
    pub const LEN: usize = std::mem::size_of::<TokenBase>();

    pub fn is_initialized(&self) -> bool {
        self.discriminator.as_slice() == TokenBase::SPL_DISCRIMINATOR_SLICE
    }

    pub fn is_uninitialized(&self) -> bool {
        self.discriminator.as_slice() == ArrayDiscriminator::UNINITIALIZED.as_slice()
    }
}

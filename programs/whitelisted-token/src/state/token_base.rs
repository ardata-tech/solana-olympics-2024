use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;
use spl_discriminator::{ArrayDiscriminator, SplDiscriminate};

// TODO: Struct packing, Cache-line optimization
#[rustfmt::skip] // ensure manual struct ordering
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Debug, ShankAccount, SplDiscriminate)]
#[seeds(
    "token_base",
    pub_key("sale_authority", Pubkey),
    nonce("nonce (or index) for multiple token bases per admin/s", u8)
)]
#[discriminator_hash_input("token_sale::state:token_base")]
pub struct TokenBase {
    pub discriminator: [u8; 8],
    pub price: u64,
    pub sale_authority: Pubkey,
    pub mint: Pubkey,
    pub vault: Pubkey, // hold the SOL from token sale
    pub whitelist_root: [u8; 32],
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

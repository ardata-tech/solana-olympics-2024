use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use spl_discriminator::{ArrayDiscriminator, SplDiscriminate};

// TODO: Struct packing, Cache-line optimization
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Debug, SplDiscriminate)]
#[discriminator_hash_input("token_sale::state:token_base")]
// TODO: skip the rust formatter
// #[rustfmt::skip] // ensure manual struct ordering
pub struct TokenBase {
    pub discriminator: [u8; 8],
    pub supply: u64,
    pub price: u64,
    pub bump: u8,
    pub vault: Pubkey,
    pub whitelist_merkle_root: [u8; 32],
    pub nonce: u32,
}

impl TokenBase {
    pub const LEN: usize = std::mem::size_of::<TokenBase>();

    pub fn from(
        supply: u64,
        price: u64,
        vault: Pubkey,
        whitelist_merkle_root: [u8; 32],
        nonce: u32,
        bump: u8,
    ) -> Self {
        Self {
            discriminator: TokenBase::SPL_DISCRIMINATOR.into(),
            supply,
            price,
            vault,
            whitelist_merkle_root,
            nonce,
            bump,
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.discriminator.as_slice() == TokenBase::SPL_DISCRIMINATOR_SLICE
    }

    pub fn is_uninitialized(&self) -> bool {
        self.discriminator.as_slice() == ArrayDiscriminator::UNINITIALIZED.as_slice()
    }
}

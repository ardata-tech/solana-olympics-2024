use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;
use spl_discriminator::{ArrayDiscriminator, SplDiscriminate};

// TODO: Struct packing, Cache-line optimization
/// TokenBase holding the token sale configuraiton
#[rustfmt::skip] // ensure manual struct ordering
#[repr(C)] // use C memory layout
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount, SplDiscriminate)]
#[discriminator_hash_input("token_sale::state:token_base")]
// OPT-OUT: didn't use #[seeds()] because ShankAccount seeds
// helper attribute is buggy. PDA is generated offchain
// instead and seeds are validated on OpenSale
pub struct TokenBase {
    /// Authority that can configure token sale after initialization
    pub sale_authority: Pubkey,
    /// Mint created external to this program
    pub mint: Pubkey,
    /// Account holding the SOL from token sale
    pub vault: Pubkey,
    /// Account holding the SOL from token sale
    pub whitelist_root: [u8; 32],
    /// Merkle root hash used to verify passed Merkle proofs
    /// for whitelist gating
    pub discriminator: [u8; 8],
    /// Amount of lamports to transfer from Buyer to Vault 
    /// when purchasing tokens
    pub price: u64,
    /// Amount of tokens allowed per buyer wallet
    pub purchase_limit: u64,
    /// Canonical bump for TokenBase PDA
    pub bump: u8,

    /// Padding to remove SLOP in C memory layout alignment
    /// Widest scalar = 32bytes
    _padding: [u8; 7]
}

impl TokenBase {
    /// Get known size of TokenBase
    pub const LEN: usize = std::mem::size_of::<TokenBase>();

    /// Is `true` if TokenBase is initialized
    pub fn is_initialized(&self) -> bool {
        self.discriminator.as_slice() == TokenBase::SPL_DISCRIMINATOR_SLICE
    }

    /// Is `true` if TokenBase is uninitialized
    pub fn is_uninitialized(&self) -> bool {
        self.discriminator.as_slice() == ArrayDiscriminator::UNINITIALIZED.as_slice()
    }
}

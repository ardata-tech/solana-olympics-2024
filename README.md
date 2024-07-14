# Token Sale Program in Native Solana

Merkle Tree Whitelist-Gated Token Sale

## Features
**Main Features**
- [x] Merkle Tree Whitelist-Gated Token Sale
- [x] Per Wallet Address Purchase Limit
- [x] Struct packing optimizations
- [x] Verifiable Build with Docker (Custom Docker Image)
- [ ] Frontend client builders (Also used in TS tests)

**Additional Features (If I still have time)**
- [ ] Protections against most-common sea-level attacks
- [ ] Token 2022 Support
- [ ] Multisig support

## Usage

## Installation 

Run

1. `yarn build:token_sale`

Test

1. `yarn build:token_sale`
2. `yarn get:fixtures`
3. `cargo test-sbf -p tests`

## Design Documentation

### Conceptual Roles
- Sale Authority
- Buyer

<hr />

### Instructions

**Admin/s**

`OpenSale`
- initialize token sale with starting configuration

`ConfigureSale`
- change variables in token sale

`CloseSale`
- close token sale

**Buyer**

`BuyToken`
- buy from the token sale

<hr />

### State

**`TokenBase`**

```rust
/// TokenBase holding the token sale configuraiton
pub struct TokenBase {
    /// Authority that can configure token sale after initialization
    pub sale_authority: Pubkey,
    /// Mint created external to this program
    pub mint: Pubkey,
    /// Account holding the SOL from token sale
    pub vault: Pubkey,
    /// Merkle root hash used to verify passed Merkle proof
    /// for whitelist gating
    pub whitelist_root: [u8; 32],
    /// Identifier for this specific structure
    pub discriminator: [u8; 8],
    /// Amount of lamports to transfer from Buyer to Vault 
    /// when purchasing tokens
    pub price: u64,
    /// Default purchase limit per user can be changed
    /// per wallet via AssignLimit
    pub default_purchase_limit: u64,
    /// Canonical bump for TokenBase PDA
    pub bump: u8,

    /// Padding to remove SLOP in C memory layout alignment
    /// Widest scalar = 32bytes
    _padding: [u8; 7]
}
```

**`BuyerFacts`**

```rust
/// BuyerFacts holding per wallet buyer stats
pub struct BuyerFacts {
    /// Token account holding buyer's tokens
    pub token_account: Pubkey,
    /// Identifier for this specific structure
    pub discriminator: [u8; 8],
    /// Amount of tokens allowed for this specific buyer
    pub purchase_limit: u64,

    /// Padding to remove SLOP in C memory layout alignment
    /// Widest scalar = 32bytes
    _padding: [u8; 8]
}
```

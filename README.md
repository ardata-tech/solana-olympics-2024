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

## Development

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

```rust
/// Structure holding the token sale configuraiton
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
    /// Amount of tokens allowed per buyer wallet
    pub purchase_limit: u64,
    /// Canonical bump for TokenBase PDA
    pub bump: u8,

    /// Padding to remove SLOP in C memory layout alignment
    /// Widest scalar = 32bytes
    _padding: [u8; 7]
}
```

# Token Sale Program in Native Solana

Whitelist-Gated Token Sale

## Features
**Main Features**
- [ ] Merkle tree whitelist optimization
- [ ] Struct packing optimizations
- [ ] Verifiable Build with Docker (Custom Docker Image)
- [ ] Jest 100% test coverage
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
    /// Identifier for this specific structure
    pub discriminator: [u8; 8],
    /// Amount of lamports to transfer from Buyer to Vault 
    /// when purchasing tokens
    pub price: u64,
    /// Amount of tokens allowed per buyer wallet
    pub purchase_limit: u64,
    /// Authority that can configure token sale after initialization
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
```

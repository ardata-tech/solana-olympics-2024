# Token Sale Program in Native Solana

Whitelist-Gated Token Sale

## Features
**Main Features**
- [ ] Standard Token Support
- [ ] Merkle tree, struct packing, memory alignment, and inilining optimizations
- [ ] Multiple token bases support (1 admin set with N-many token bases)
- [ ] Jest 100% test coverage (Unit Test)
- [ ] Protections against most-common sea-level attacks
- [ ] Frontend client builders (Also used in TS tests)

**Additional Features**
- [ ] Verifiable Build with Docker (Custom Docker Image)
- [ ] Token 2022 Support
- [ ] Multisig support

**More Features**
- [ ] Super Mario Series Hosted via Solana Program

## Usage

## Installation 

## Development


## Design Documentation

### Roles
- Admin/s
- Buyer

<hr />

### Instructions

**Admin/s**

`OpenSale`
- initialize token sale with starting configuration
- mint new token
- create new token account to hold supply

`ConfigureSale`
- change variables in token sale

`CloseSale`
- close token sale

**Buyer**

`BuyToken`
- buy from the token sale

<hr />

### State

`TokenBase`
- `supply`
- `price`
- `whitelist_merkle_root`
- `nonce` to allow multiple TokenBases
- `bump (canonical)`

use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Debug, Error, FromPrimitive)]
pub enum TokenSaleError {
    #[error("Invalid account data length")]
    InvalidAccountDataLength, // 0

    #[error("Mint and Sale authority don't match")]
    MintAndSaleAuthorityMismatch, // 1

    #[error("Vault must be non-executable")]
    VaultMustBeNonExecutable, // 2

    #[error("Sale authority not a signer")]
    SaleAuthorityNotSigner, // 3
}

// allow .into() for Custom Error to ProgramError conversion
impl From<TokenSaleError> for ProgramError {
    fn from(e: TokenSaleError) -> Self {
        // https://docs.rs/solana-program/latest/solana_program/program_error/enum.ProgramError.html#variant.Custom
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for TokenSaleError {
    fn type_of() -> &'static str {
        "TokenSaleError"
    }
}

impl PrintProgramError for TokenSaleError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}
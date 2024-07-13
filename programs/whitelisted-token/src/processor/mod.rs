use crate::instruction::{
    accounts::{ConfigureSaleAccounts, OpenSaleAccounts},
    TokenSaleInstruction,
};
use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

pub mod open_sale;
use open_sale::*;

pub mod configure_sale;
use configure_sale::*;

/// Program state processor
pub struct Processor {}

impl<'a> Processor {
    /// Process the transaction
    ///
    /// - Deserializes the instruction data
    /// - Routes transaction data to the proper handler
    pub fn process(
        program_id: &Pubkey,
        accounts: &'a [AccountInfo<'a>],
        instruction_data: &[u8],
    ) -> ProgramResult {
        // get instruction
        let instruction = TokenSaleInstruction::try_from_slice(instruction_data)?;
        match instruction {
            TokenSaleInstruction::OpenSale {
                price,
                purchase_limit,
                whitelist_root,
            } => {
                process_open_sale(
                    program_id,
                    OpenSaleAccounts::context(accounts)?,
                    price,
                    purchase_limit,
                    whitelist_root,
                )?;
            }

            TokenSaleInstruction::ConfigureSale {
                price,
                purchase_limit,
                whitelist_root,
            } => {
                process_configure_sale(
                    program_id,
                    ConfigureSaleAccounts::context(accounts)?,
                    price,
                    purchase_limit,
                    whitelist_root,
                )?;
            }

            _ => todo!(),
        }

        Ok(())
    }
}

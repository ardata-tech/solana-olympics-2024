use crate::error::TokenSaleError;
use crate::instruction::{accounts::OpenSaleAccounts, TokenSaleInstruction};
use crate::state::token_base::TokenBase;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

pub mod open_sale;
use open_sale::*;

/// Program state processor
pub struct Processor {}

impl<'a> Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &'a [AccountInfo<'a>],
        instruction_data: &[u8],
    ) -> ProgramResult {
        // process accounts
        // let accounts_info_iter = &mut accounts.iter();
        // let owner_account: &AccountInfo<'a> = next_account_info(accounts_info_iter)?;
        // let counter_account: &AccountInfo<'a> = next_account_info(accounts_info_iter)?;

        // top-level validation (general truths)
        // Self::validate(program_id, owner_account, counter_account)?;

        // get instruction
        let instruction = TokenSaleInstruction::try_from_slice(instruction_data)?;
        match instruction {
            TokenSaleInstruction::OpenSale {
                supply,
                price,
                decimals,
                nonce,
            } => {
                process_open_sale(
                    program_id,
                    OpenSaleAccounts::context(accounts)?,
                    supply,
                    price,
                    decimals,
                    nonce,
                )?;
            }
            _ => todo!(),
        }

        Ok(())
    }

    // This validate function validates the following:
    // - owner is [SIGNER]
    // - counter is [SIGNER, WRITE]
    // - counter is owned by the passed owner
    // - passed counter PDA == computed PDA
    // fn validate(program_id: &Pubkey, owner: &AccountInfo, counter: &AccountInfo) -> ProgramResult {
    //     //----- ASSERTIONS AND SAFETY CHECKS -----
    //     // validate proper payer account permissions
    //     assert!(owner.is_signer);
    //     assert!(owner.is_writable);
    //     // validate proper mint PDA account permissions
    //     assert!(!counter.is_signer); // not
    //     assert!(counter.is_writable);
    //
    //     // verify correctness of PDA derivation with canonical bumps
    //     let (counter_pda, _) =
    //         Pubkey::find_program_address(&[owner.key.as_ref(), b"counter_account"], program_id);
    //     assert_eq!(counter_pda, *counter.key);
    //
    //     Ok(())
    // }
    //
    // pub fn process_initialize_account(
    //     program_id: &Pubkey,
    //     owner: &AccountInfo<'a>,
    //     counter: &AccountInfo<'a>,
    //     bump: u8,
    // ) -> ProgramResult {
    //     // errors
    //     use CounterError::AlreadyInitialized;
    //
    //     // get rent
    //     let counter_rent = match Rent::get() {
    //         Ok(rent_sysvar) => rent_sysvar,
    //         Err(e) => panic!("Error in getting Rent sysvar: {}", e),
    //     }
    //     .minimum_balance(Counter::LEN);
    //
    //     // get counter data
    //     let counter_data = Counter::from(bump);
    //
    //     // create account instruction
    //     let ix = system_instruction::create_account(
    //         owner.key,
    //         counter.key,
    //         counter_rent,
    //         Counter::LEN as u64,
    //         program_id,
    //     );
    //
    //     // create account
    //     if let Err(e) = invoke_signed(
    //         &ix,
    //         &[owner.clone(), counter.clone()],
    //         &[&[owner.key.as_ref(), b"counter_account", &[counter_data.bump]]],
    //     ) {
    //         match e {
    //             // if already initialized
    //             ProgramError::AccountAlreadyInitialized => {
    //                 return Err(ProgramError::from(AlreadyInitialized))
    //             }
    //             // rethrow
    //             _ => return Err(e),
    //         }
    //     };
    //
    //     // store data on new account via borsh
    //     counter_data.serialize(&mut *counter.data.borrow_mut())?;
    //
    //     Ok(())
    // }
    //
    // fn process_increment_counter(counter: &AccountInfo) -> ProgramResult {
    //     let data = counter.data.borrow();
    //     let mut current = Counter::try_from_slice(&data)?;
    //     // increment
    //     current.increment();
    //     current.serialize(&mut *counter.data.borrow_mut())?;
    //
    //     Ok(())
    // }
    //
    // fn process_decrement_counter(counter: &AccountInfo) -> ProgramResult {
    //     let data = counter.data.borrow();
    //     let mut current = Counter::try_from_slice(&data)?;
    //     // decrement
    //     current.decrement();
    //     current.serialize(&mut *counter.data.borrow_mut())?;
    //
    //     Ok(())
    // }
}

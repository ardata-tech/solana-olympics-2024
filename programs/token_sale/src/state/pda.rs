use solana_program::pubkey::Pubkey;

/// Finds the [`TokenBase`] PDA with canonical bump
///
/// - Used for validating TokenBase seeds
pub fn find_token_base_pda(program_id: &Pubkey, mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&["token_base".as_bytes(), mint.as_ref()], program_id)
}

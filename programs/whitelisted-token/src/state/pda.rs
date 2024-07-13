use solana_program::pubkey::Pubkey;

/// Finds the [`TokenBase`] PDA with canonical bump
///
/// Used for validating TokenBase
pub fn find_token_base_pda(
    program_id: &Pubkey,
    sale_authority: &Pubkey,
    nonce: u8,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &["token_base".as_bytes(), sale_authority.as_ref(), &[nonce]],
        program_id,
    )
}

use solana_program::pubkey::Pubkey;

pub fn find_vault_pda(program_id: &Pubkey, token_base: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&["vault".as_bytes(), token_base.as_ref()], program_id)
}

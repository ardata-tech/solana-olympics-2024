/// Onchain Merkle Tree utils
use crate::error::TokenSaleError;
use hex::decode;
use merkletreers::{merkle_proof_check::merkle_proof_check, Leaf, Proof, Root};
use sha256::digest;
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

/// Verify membership
pub fn verify_membership(root: Root, proof: Proof, member: Leaf) -> bool {
    root == merkle_proof_check(proof, member)
}

/// Converts a Solana Pubkey into a Merkle Tree Leaf
pub fn pubkey_to_sha256_leaf(pubkey: &Pubkey) -> Result<Leaf, ProgramError> {
    // Pubkey transformations
    //
    // 1. pubkey -> sha256_digest(pubkey) -> hex
    // 2. hex -> decode(hex) -> Vec<u8>
    // 3. Vec<u8> -> .copy_from_slice(vec) -> [u8; 32] Leaf;
    match decode(digest(pubkey.as_ref())) {
        Ok(decoded) => {
            let mut buffer = [0u8; 32];
            buffer.copy_from_slice(decoded.as_slice());
            Ok(buffer)
        }
        Err(_) => Err(TokenSaleError::FailedToDecodeSha256Hash.into()),
    }
}

/// Onchain Merkle Tree utils
use crate::error::TokenSaleError;
use borsh::{BorshDeserialize, BorshSerialize};
use hex::decode;
use merkletreers::{
    node::{Node, Side},
    {merkle_proof_check::merkle_proof_check, Leaf, Proof, Root},
};
use sha256::digest;
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

/// borsh de/serializable Merkle Proof primitive
pub type WhitelistProof = Vec<WhitelistNode>;

/// borsh de/serializable Merkle Side primitive
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum WhitelistSide {
    LEFT,
    RIGHT,
}

/// borsh de/serializable Merkle Node primitive
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WhitelistNode {
    data: [u8; 32],
    side: WhitelistSide,
}

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

/// Converts WhitelistProof into merkletreers::Proof
pub fn convert_whitelist_proof(w_proof: WhitelistProof) -> Proof {
    let mut merkle_proof = Proof::default();
    for w_node in w_proof {
        // default value
        let mut n = Node {
            data: [0u8; 32],
            side: Side::LEFT,
        };

        n.data = w_node.data;

        n.side = match w_node.side {
            WhitelistSide::LEFT => Side::LEFT,
            WhitelistSide::RIGHT => Side::RIGHT,
        };

        merkle_proof.push(n);
    }

    merkle_proof
}

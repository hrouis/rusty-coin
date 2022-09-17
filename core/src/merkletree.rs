use crate::{Hash, Transaction};
use std::fmt::Error;
use std::ptr::hash;

pub struct MerkleNode {
    pub value: Hash,
    pub children: Vec<MerkleNode>,
    pub parent: MerkleNode,
}

pub struct MerkleHash {
    pub block_id: u32,
    pub hash: Hash,
}

pub fn construct_merkle_tree(
    merkle_node: &mut MerkleNode,
    mut hashes: Vec<Hash>,
) -> Result<&MerkleNode, Error> {
    let size = hashes.len();
    if size > 1 {
        if size % 2 != 0 {
            hashes.extend(hashes.last().unwrap())
        }
        let next_level_hashes = hashes
            .iter()
            .zip(hashes.iter().skip(1))
            .map(|(mut hash1, hash2)| {
                let value = crypto_hash::digest(
                    crypto_hash::Algorithm::SHA256,
                    hash1.extend(hash2).bytes(),
                );
            })
            .collect::<Vec<Hash>>();
        construct_merkle_tree(merkle_node, next_level_hashes)
    }
    Ok(merkle_node)
}

#[cfg(test)]
mod tests {
    fn should_construct_merkle_tree() {}
}

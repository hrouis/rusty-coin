use crate::check_difficulty;
use primitive_types::U256;

use super::Hash;
use super::Hashable;
use super::TimeStamp;
use super::Transaction;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: TimeStamp,
    pub hash: Hash,
    pub prev_block_hash: Hash,
    pub nonce: u64,
    pub difficulty: U256,
    pub transactions: Vec<Transaction>,
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut block_bytes = vec![];
        block_bytes.extend(&self.index.to_le_bytes());
        block_bytes.extend(&self.prev_block_hash);
        block_bytes.extend(&self.nonce.to_le_bytes());
        block_bytes
    }
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: Hash,
        transactions: Vec<Transaction>,
        difficulty: U256,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce: 0,
            difficulty,
            transactions,
        }
    }

    pub fn mine(&mut self) {
        for tmp_nonce in 0..(u64::MAX) {
            self.nonce = tmp_nonce;
            let hash = self.hash();
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}

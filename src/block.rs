use super::Hash;
use super::TimeStamp;
use super::Transaction;
use super::Hashable;

pub struct Block {
    pub index: u32,
    pub timestamp: TimeStamp,
    pub hash: Hash,
    pub prev_block_hash: Hash,
    pub nonce: u64,
    pub difficulty: u128,
    pub transactions: Vec<Transaction>,
}

impl Hashable for Block {
    fn bytes( &self) -> Vec<u8> {
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
        hash: Hash,
        prev_block_hash: Hash,
        nonce: u64,
        difficulty: u128,
        transactions: Vec<Transaction>,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash,
            prev_block_hash,
            nonce,
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

fn check_difficulty(hash: &Hash, target: u128) -> bool {
    super::difficulty_bytes_as_u128(hash) < target
}

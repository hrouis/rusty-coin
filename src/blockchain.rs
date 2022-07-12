use std::cmp;
use std::collections::HashSet;

use super::{Hashable, now, Transaction};
use super::Block;
use super::Hash;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    transaction_pool: Vec<Transaction>,
    unspent_output: HashSet<Hash>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            blocks: vec![],
            transaction_pool: vec![],
            unspent_output: HashSet::new(),
        }
    }

    pub fn add_transaction_to_pool(&mut self, transaction: Transaction) {
        // validate transaction
        // check if transaction is spendable
        if !transaction.is_spendable() {
            //TODO error reject transaction output greater than input
        }
        // check inputs are valid (unspent output in block)
        let input_hashes = transaction.inputs.iter()
            .map(|input| input.hash())
            .collect::<Vec<Hash>>();
        for hash in input_hashes {
            if !self.unspent_output.contains(&hash) {
                //TODO error reject transaction input not spendable
            }
        }
        //TODO complete the validation process ( see spec document)
        self.transaction_pool.push(transaction);
    }

    pub fn create_candidate_block(&mut self, transactions_count: usize) -> Block {
        let mut candidate_index: u32 = 0;
        let mut previous_hash: Hash = Vec::new();
        if let Some(latest_block) = self.blocks.last().cloned() {
            candidate_index = latest_block.index;
            previous_hash = latest_block.hash;
        }
        //Get transactions from pool up to transactions count
        let pool_size = self.transaction_pool.len();
        let block_transaction_count = cmp::min(pool_size, transactions_count);
        let transactions: Vec<Transaction> = self.transaction_pool
            .drain(..block_transaction_count).collect();
        let mut block = Block::new(candidate_index, now(), vec![],
                                   previous_hash, 0, 0, transactions);
        block.hash = block.hash();
        block
    }
}

use std::cmp;
use std::collections::HashSet;

use crate::check_difficulty;

use super::{Hashable, now, Transaction, TxOutput};
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
        // verify transaction
        if !self.verify_transaction(&transaction) {
            return;
        }

        //TODO complete the validation process ( see spec document)
        self.transaction_pool.push(transaction);
    }

    pub fn create_candidate_block(&mut self, transactions_count: usize, miner_address: String) -> Block {
        let mut candidate_index: u32 = 0;
        let mut previous_hash: Hash = Vec::new();
        if let Some(latest_block) = self.blocks.last().cloned() {
            candidate_index = latest_block.index;
            previous_hash = latest_block.hash;
        }
        //Get transactions from pool up to transactions count
        let pool_size = self.transaction_pool.len();
        let block_transaction_count = cmp::min(pool_size, transactions_count);
        let mut transactions: Vec<Transaction> = Vec::new();

        // Add coinbase transaction
        let coinbase = Transaction {
            inputs: vec![],
            outputs: vec![
                TxOutput {
                    address: miner_address,
                    value: 50,
                },
            ],
            timestamp: now(),
        };
        transactions.push(coinbase);

        transactions.extend(self.transaction_pool
            .drain(..block_transaction_count).collect::<Vec<Transaction>>());
        let mut block = Block::new(candidate_index, now(), vec![],
                                   previous_hash, 0, 0, transactions);
        block.hash = block.hash();
        block
    }

    pub fn aggregate_mined_block(&mut self, block: Block) {
        if !check_difficulty(&block.hash, block.difficulty) {
            //TODO raise error block is not mined, does not satisfy POW condition
        }
        let potential_coinbase = block.transactions.first().unwrap();
        if !potential_coinbase.is_coinbase() {
            //TODO raise error first Tx is not coinbase
        }
        let mut output_spent = Vec::new();
        let mut output_created = Vec::new();

        for transaction in block.transactions.iter() {
            if !self.verify_transaction(transaction) {
                //TODO raise error transaction not verified
            }
            output_spent.extend(transaction.input_hashes());
            output_created.extend(transaction.output_hashes());
        }

        // Update unspent output vector
        self.unspent_output.retain(|output| !output_spent.contains(output));
        self.unspent_output.extend(output_created);
        self.blocks.push(block);
    }

    fn verify_transaction(&self, transaction: &Transaction) -> bool {
        // check if transaction is spendable
        if !transaction.is_spendable() {
            //TODO error reject transaction output greater than input
            return false;
        }
        // check inputs are valid (unspent output in block)
        let input_hashes = transaction.input_hashes();
        for hash in input_hashes {
            if !self.unspent_output.contains(&hash) {
                //TODO error reject transaction input not spendable
               return false;
            }
            let tx_pool_hashes = self.transaction_pool.iter()
                .flat_map(|transaction| transaction.input_hashes()).collect::<HashSet<Hash>>();
            if tx_pool_hashes.contains(&hash) {
                //TODO error reject transaction double spending attempt
                return false;
            }
        }
        true
    }
}


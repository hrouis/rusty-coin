use std::collections::HashSet;

use crate::{Hashable, Transaction};

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
        let input_hashes = transaction.inputs.iter().map(|input | input.hash()).collect::<Vec<Hash>>();
        for hash in input_hashes {
            if !self.unspent_output.contains(&hash) {
                //TODO error reject transaction input not spendable
            }
        }
        //TODO complete the validation process ( see spec document)
        self.transaction_pool.push(transaction);
    }
}

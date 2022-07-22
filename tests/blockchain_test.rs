use std::str::FromStr;

use primitive_types::U256;

use rusty_coin::blockchain::Blockchain;
use rusty_coin::{now, Block, Transaction, TxOutput};

#[test]
fn test_scenario_1() {
    let mut blockchain = Blockchain::new();
    let difficulty =
        U256::from_str("0x000fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff")
            .expect("Unable to parse u256");

    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                TxOutput {
                    address: "Alice".to_owned(),
                    value: 50.0,
                },
                TxOutput {
                    address: "Bob".to_owned(),
                    value: 7.0,
                },
            ],
            timestamp: now(),
        }],
        difficulty,
    );

    genesis_block.mine();
    println!("Mined genesis block {:?}", &genesis_block);
    let hash = U256::from(genesis_block.hash.as_slice());
    assert!(hash < difficulty);
    blockchain
        .aggregate_mined_block(genesis_block)
        .expect("Failed to add block!");
    assert_eq!(1, blockchain.len());
}

#[test]
fn test_scenario_2() {
    // Create blockchain and genesis block
    println!("creating the blockchain");
    let mut blockchain = Blockchain::new();
    let difficulty =
        U256::from_str("0x000fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff")
            .expect("Unable to parse u256");
    let first_output = TxOutput {
        address: "Alice".to_owned(),
        value: 50.0,
    };
    let second_output = TxOutput {
        address: "Bob".to_owned(),
        value: 7.0,
    };

    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        vec![Transaction {
            inputs: vec![],
            outputs: vec![first_output.clone(), second_output.clone()],
            timestamp: now(),
        }],
        difficulty,
    );

    genesis_block.mine();
    println!("adding genesis block");
    blockchain
        .aggregate_mined_block(genesis_block)
        .expect("Failed to add block!");
    // Add transactions to the pool
    let transaction = Transaction {
        inputs: vec![first_output.clone()],
        outputs: vec![
            TxOutput {
                address: String::from("ALice"),
                value: 25.0,
            },
            TxOutput {
                address: String::from("Bob"),
                value: 4.995,
            },
        ],
        timestamp: now(),
    };

    blockchain
        .add_transaction_to_pool(transaction)
        .expect("transaction is not valid");
    // Create and mine block
    let mut block = blockchain.create_candidate_block(1, String::from("Alice"), difficulty);
    block.mine();

    // Validate block and add to ledger
    println!("Adding the new block {:?}", block);
    blockchain
        .aggregate_mined_block(block)
        .expect("Block is not valid!");
    assert_eq!(2, blockchain.len());
}

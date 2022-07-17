use baby_coin::{Block, bytes_as_u128, now, Transaction, TxOutput};
use baby_coin::blockchain::Blockchain;

#[test]
fn test_scenario_1() {
    let mut blockchain = Blockchain::new();
    let difficulty = 0x000fffffffffffffffffffffffffffff;

    let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![
        Transaction {
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
        },
    ], difficulty);

    genesis_block.mine();
    println!("Mined genesis block {:?}", &genesis_block);
    let hash = bytes_as_u128(&genesis_block.hash);
    assert!(hash < difficulty);
    blockchain.aggregate_mined_block(genesis_block).expect("Failed to add block!");
    assert_eq!(1, blockchain.len());
}
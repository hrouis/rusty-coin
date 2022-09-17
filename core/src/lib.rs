use std::time::{SystemTime, UNIX_EPOCH};

use primitive_types::U256;

pub use crate::block::Block;
pub use crate::blockchain::Blockchain;
pub use crate::hashable::Hashable;
pub use crate::transaction::Transaction;
pub use crate::transaction::TxOutput;

pub fn now() -> u128 {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_secs() as u128 * 1000 + duration.subsec_millis() as u128
}

fn check_difficulty(hash: &Hash, target: U256) -> bool {
    U256::from(hash.as_slice()) < target
}

type Hash = Vec<u8>;
type Address = String;
type TimeStamp = u128;

pub mod block;
pub mod blockchain;
pub mod hashable;
pub mod merkletree;
pub mod transaction;

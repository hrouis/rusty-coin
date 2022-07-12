use std::time::{SystemTime, UNIX_EPOCH};

pub use crate::block::Block;
pub use crate::blockchain::Blockchain;
pub use crate::hashable::Hashable;
pub use crate::transaction::Transaction;

pub fn now() -> u128 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    duration.as_secs() as u128 * 1000 + duration.subsec_millis() as u128
}

pub fn difficulty_bytes_as_u128(v: &Vec<u8>) -> u128 {
    ((v[31] as u128) << 0xf * 8) |
        ((v[31] as u128) << 0xe * 8) |
        ((v[31] as u128) << 0xd * 8) |
        ((v[31] as u128) << 0xc * 8) |
        ((v[31] as u128) << 0xb * 8) |
        ((v[31] as u128) << 0xa * 8) |
        ((v[31] as u128) << 0x9 * 8) |
        ((v[31] as u128) << 0x8 * 8) |
        ((v[31] as u128) << 0x7 * 8) |
        ((v[31] as u128) << 0x6 * 8) |
        ((v[31] as u128) << 0x5 * 8) |
        ((v[31] as u128) << 0x4 * 8) |
        ((v[31] as u128) << 0x3 * 8) |
        ((v[31] as u128) << 0x2 * 8) |
        ((v[31] as u128) << 0x1 * 8) |
        ((v[31] as u128) << 0x0 * 8)
}


type Hash = Vec<u8>;
type Address = String;
type TimeStamp = u128;


mod block;
mod transaction;
mod hashable;
mod blockchain;

pub fn difficulty_bytes_as_u128 (v: &Vec<u8>) -> u128 {

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
pub use crate::block::Block;

mod transaction;
pub use crate::transaction::Transaction;

mod hashable;
pub use crate::hashable::Hashable;

mod blockchain;
pub use crate::blockchain::Blockchain;
use crate::{Hash, Hashable};
use super::Address;
use super::TimeStamp;

pub struct TxOutput {
    pub address: Address,
    pub value: u64
}

pub struct Transaction {
    pub inputs: Vec<TxOutput>,
    pub outputs: Vec<TxOutput>,
    pub timestamp: TimeStamp
}

impl Hashable for TxOutput {
    fn bytes(&self) -> Vec<u8> {
        let mut to_bytes = vec![];
        to_bytes.extend(&self.address.as_bytes());
        to_bytes.extend(&self.value.to_le_bytes());
        to_bytes
    }
}

impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut tx_output = !vec![];
        tx_output.extend(&self.inputs.iter().flat_map(|input | input.bytes()).collect::<Vec<u8>>());
        tx_output.extend(&self.outputs.iter().flat_map(|output |output.bytes()).collect::<Vec<u8>>());
        tx_output
    }
}
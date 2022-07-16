use crate::Hash;

use super::Address;
use super::Hashable;
use super::TimeStamp;

#[derive(Debug, Clone)]
pub struct TxOutput {
    pub address: Address,
    pub value: f64,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub inputs: Vec<TxOutput>,
    pub outputs: Vec<TxOutput>,
    pub timestamp: TimeStamp,
}

impl Transaction {
    fn compute_inputs(&self) -> f64 {
        self.inputs.iter().map(|input| input.value).sum()
    }

    fn compute_outputs(&self) -> f64 {
        self.outputs.iter().map(|output| output.value).sum()
    }

    pub fn is_spendable(&self) -> bool {
        self.compute_inputs() > self.compute_outputs()
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 0
    }

    pub fn input_hashes(&self) -> Vec<Hash> {
        self.inputs.iter()
            .map(|input| input.hash())
            .collect::<Vec<Hash>>()
    }

    pub fn output_hashes(&self) -> Vec<Hash> {
        self.outputs.iter()
            .map(|output| output.hash())
            .collect::<Vec<Hash>>()
    }
}

impl Hashable for TxOutput {
    fn bytes(&self) -> Vec<u8> {
        let mut to_bytes = vec![];
        to_bytes.extend(self.address.as_bytes());
        to_bytes.extend(self.value.to_le_bytes());
        to_bytes
    }
}

impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut tx_output = vec![];
        tx_output.extend(self.inputs
            .iter()
            .flat_map(|input| input.bytes())
            .collect::<Vec<u8>>());
        tx_output.extend(self.outputs
            .iter()
            .flat_map(|output| output.bytes())
            .collect::<Vec<u8>>());
        tx_output
    }
}
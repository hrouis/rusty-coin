use crypto::digest::Digest;
use crypto::ripemd160::Ripemd160;
use secp256k1::ecdsa::Signature;
use secp256k1::{All, Error, Message, PublicKey, Secp256k1, SecretKey};

use crate::Address;

pub struct HashContext {
    sec_p: Secp256k1<All>,
}

impl HashContext {
    pub fn new() -> Self {
        HashContext {
            sec_p: Secp256k1::new(),
        }
    }

    pub fn generate_key_pair(&self) -> (SecretKey, PublicKey) {
        self.sec_p.generate_keypair(&mut rand::rngs::OsRng)
    }

    pub fn generate_address(&self, pub_key: &PublicKey) -> Address {
        let mut sha256 = crypto::sha2::Sha256::new();
        sha256.input(&pub_key.serialize());
        let mut result: [u8; 32] = [0; 32];
        sha256.result(result.as_mut());
        let mut ripemd160 = Ripemd160::new();
        ripemd160.input(result.as_slice());
        let mut address: [u8; 20] = [0; 20];
        ripemd160.result(address.as_mut());
        bs58::encode(address).into_vec()
    }

    pub fn sign(&self, key: &SecretKey, message_slice: &[u8]) -> Result<Signature, Error> {
        Ok(self
            .sec_p
            .sign_ecdsa(&Message::from_slice(message_slice)?, key))
    }
}

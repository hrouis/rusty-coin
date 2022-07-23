use std::error::Error;

use secp256k1::ecdsa::Signature;
use secp256k1::{All, Message, PublicKey, Secp256k1, SecretKey};

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
        sec_p.generate_keypair(&mut rand::thread_rng())
    }

    pub fn sign(&self, private_key: SecretKey, message: Message) -> Result<Signature, Error> {
        Ok(self.sec_p.sign_ecdsa(&message, &private_key))
    }
}

use secp256k1::ecdsa::Signature;
use secp256k1::{All, Error, Message, PublicKey, Secp256k1, SecretKey};

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

    pub fn sign(&self, key: &SecretKey, message_slice: &[u8]) -> Result<Signature, Error> {
        Ok(self
            .sec_p
            .sign_ecdsa(&Message::from_slice(message_slice)?, key))
    }
}

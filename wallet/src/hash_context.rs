use ripemd::Digest;
use ripemd::Ripemd160;
use secp256k1::ecdsa::Signature;
use secp256k1::{All, Error, Message, PublicKey, Secp256k1, SecretKey};
use sha256::digest_bytes;

use crate::Address;

#[derive(Debug)]
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

    pub fn generate_address(&self, pub_key: &PublicKey, version: &[u8]) -> Address {
        // SHA256 hashing
        let sha256_hash = digest_bytes(&pub_key.serialize());
        // RIPEMD160 hashing
        let mut ripemd160 = Ripemd160::new();
        ripemd160.update(sha256_hash.as_bytes());
        let double_hash_result = ripemd160.finalize();
        // add version
        let mut payload: Vec<u8> = vec![];
        payload.extend(version);
        payload.extend(double_hash_result);
        // double hash payload
        let d_hash_payload = digest_bytes(digest_bytes(payload.as_slice()).as_bytes());
        // Get first 4 bytes from double SHA256 hash
        let payload_suffix = &d_hash_payload[..5];
        payload.extend(payload_suffix.bytes());
        bs58::encode(payload).into_vec()
    }

    pub fn sign(&self, key: &SecretKey, message_slice: &[u8]) -> Result<Signature, Error> {
        Ok(self
            .sec_p
            .sign_ecdsa(&Message::from_slice(message_slice)?, key))
    }
}

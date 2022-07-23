use secp256k1::{PublicKey, SecretKey};

use crate::hash_context::HashContext;

pub struct Account {
    private_key: SecretKey,
    public_key: PublicKey,
    context: HashContext,
}

impl Account {
    pub fn new() -> Self {
        let context = HashContext::new();
        let (private_key, public_key) = hash_context.generate_key_pair();

        Account {
            private_key,
            public_key,
            context,
        }
    }

    pub fn get_public_key(&self) -> PublicKey {
        self.context.
    }

    pub fn sign(&self) {
        self.context.
    }
}

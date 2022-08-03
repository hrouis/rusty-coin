use secp256k1::{PublicKey, SecretKey};

use crate::hash_context::HashContext;
use crate::Address;

#[derive(Debug)]
pub struct Account {
    private_key: SecretKey,
    public_key: PublicKey,
    address: Address,
    context: HashContext,
}

pub const VERSION: u32 = 0x00;

impl Account {
    pub fn new() -> Self {
        let version_vec: [u8; 4] = VERSION.to_le_bytes();
        let context = HashContext::new();
        let (private_key, public_key) = context.generate_key_pair();
        let address = context.generate_address(&public_key, &version_vec);
        Account {
            private_key,
            public_key,
            address,
            context,
        }
    }

    pub fn get_public_key(&self) -> PublicKey {
        self.public_key
    }

    pub fn sign(&self, message_slice: &[u8]) {
        self.context.sign(&self.private_key, message_slice);
    }
}

//Testing

#[cfg(test)]
mod tests {
    use crate::account::Account;

    #[test]
    fn create_account() {
        let account = Account::new();
        println!("{:?}", String::from_utf8(account.address));
    }
}

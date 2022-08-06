# Wallet import Format

source : https://en.bitcoin.it/wiki/Wallet_import_format


* Encoding of a private ECDSA key in order to store it in a key/value local database.

## Steps 

* Take a private key.
* Add a 0x80 byte in front of it for mainnet addresses or 0xef for testnet addresses. Also add a 0x01 byte at the end if the private key will correspond to a compressed public key.
* Perform SHA-256 hash on the extended key.
* Perform SHA-256 hash on result of SHA-256 hash.
* Take the first 4 bytes of the second SHA-256 hash; this is the checksum.
* Add the 4 checksum bytes from point 5 at the end of the extended key from point 2.
* Convert the result from a byte string into a base58 string using Base58Check encoding. This is the wallet import format (WIF).
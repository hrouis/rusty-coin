# Encoding steps

* SHA256 hash of the public key
* The result of the previous step is hashed in RIPEMD160
* Base58Check encoding of the result of double hash (previous step)

# Base58Check encoding
* Add version prefix before the payload
* Double sha256(prefix + payload)
* Add first 4 bytes of the previous step result as a checksum
* encode the result in base 58

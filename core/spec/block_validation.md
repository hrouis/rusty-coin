# Block validation

* The block data structure is syntactically valid
* The block header hash is equal to or less than the target (enforces the Proof-of-Work)
* The block timestamp is less than two hours in the future (allowing for time errors)
* The block size is within acceptable limits
* The first transaction (and only the first) is a coinbase transaction
* All transactions within the block are valid using the transaction checklist discussed in Independent Verification of Transactions


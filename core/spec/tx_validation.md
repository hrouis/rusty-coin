# Transaction validation:

* The transaction’s syntax and data structure must be correct.
* Neither lists of inputs or outputs are empty.
* The transaction size is less than the maximum allowed size for a block excluding witness data, as shown in tx_check.cpp.
* Each output value, as well as the total, must be within the allowed range of values (less than 21m coins, more than the dust threshold).
* None of the inputs have hash=0, N=–1 (coinbase transactions should not be relayed).
* nLocktime is equal to INT_MAX, or nLocktime and nSequence values are satisfied according to MedianTimePast.
* The transaction size in bytes is greater than or equal to 82.
* The number of signature operations (SIGOPS) contained in the transaction is less than the signature operation limit.
* The unlocking script (scriptSig) can only push numbers on the stack, and the locking script (scriptPubkey) must match IsStandard forms (this rejects "nonstandard" transactions).
* A matching transaction in the pool, or in a block in the main branch, must exist.
* For each input, if the referenced output exists in any other transaction in the pool,
the transaction must be rejected.
* For each input, look in the main branch and the transaction pool to find
its parent transaction. If the parent transaction is missing for any input, this will be an orphan transaction.
Add to the orphan transactions pool, if a matching transaction is not already in the pool.
* For each input, if its parent transaction is a coinbase transaction, it must have at least COINBASE_MATURITY (100) confirmations.
* For each input, the referenced output must exist and cannot already be spent.
* Using the parent transactions to get input values, check that each input value, as well as the sum,
are in the allowed range of values (less than 21m coins, more than 0).
* Reject if the sum of input values is less than sum of output values.
* Reject if transaction fee would be too low (minRelayTxFee) to get into an empty block.
* The unlocking scripts for each input must validate against the corresponding output locking scripts.
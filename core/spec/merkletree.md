# Merkle tree

* Transaction are ordered by priority
* Each transaction is hashed which translate to a leaf in the tree
* concatenate the hash of two consecutive tx hashes and hash it the result is an intermediate
node in the tree
* Each level of the tree is a concat + hash of two consecutive nodes from the previous level
* The process is repeated until we end up with one hash which is the root of the tree or Merkle root.
* 
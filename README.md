#treelog-rs

An append-only versioned log that provides cryptographic validation of entries to detect tampering. 
The log consists of a Rust implementation of a merkle tree based data structure defined in [Efficient Data Structures for Tamper-Evident Logging](https://static.usenix.org/event/sec09/tech/full_papers/crosby.pdf). 


<br/><br/><br/>
####Coming soon:

* Indexing hints by passing target version, or version range.
* Customisation of the hashing algorithm used.
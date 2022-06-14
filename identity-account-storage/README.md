# IOTA Identity - Account Storage

This crates defines the [`Storage`](crate::storage::Storage) trait which can be implemented for secure cryptographic operations, such as key generation and signing, as well as key-value like storage of data structures, such as DID Documents.

## Implementations

- [`Stronghold`](crate::storage::Stronghold) implements [`Storage`](crate::storage::Storage) and provides secure data storage and cryptographic operations using [IOTA Stronghold](https://github.com/iotaledger/stronghold.rs).
- [`MemStore`](crate::storage::MemStore) is an in-memory [`Storage`](crate::storage::Storage). It serves as an example implementation for reference and local testing, it is not intended for use in production!

## Test Suite
[`StorageTestSuite`](crate::storage::StorageTestSuite) helps with testing [`Storage`](crate::storage::Storage) implementations. 
# IOTA Identity - Account Storage

This crates defines the [`Storage Trait`](crate::storage::Storage) which can be implemented for secure key operations,
such as key generation and signing, as well as key-value like storage of data structures, such as DID documents.

## Stronghold

[`Stronghold`](crate::storage::Stronghold) implements [`Storage`](crate::storage::Storage) and
provides secure data storage using [IOTA Stronghold](https://github.com/iotaledger/stronghold.rs).

## Memstore

[`MemStore`](crate::storage::MemStore) is an in-memory storage that implements the [`Storage`](crate::storage::Storage).
It acts as an example for implementing [`Storage`](crate::storage::Storage) as well as for testing.

## StorageTestSuite
[`StorageTestSuite`](crate::storage::StorageTestSuite) helps with testing [`Storage`](crate::storage::Storage) implementations. 
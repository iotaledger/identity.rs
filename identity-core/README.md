IOTA Identity - Core  
=== 

The core types and utilities for IOTA identity.

The types and traits exposed by this crate are mainly intended to be used by the other crates constituting the [IOTA Identity Framework](https://wiki.iota.org/identity.rs/introduction). 

## Common Data Types 
This crate exposes some fundamental data types used across the IOTA Identity Framework:
- [`Context`](crate::common::Context): represents [JSON-LD contexts](https://www.w3.org/TR/vc-data-model/#contexts).
- [`Fragment`](crate::common::Fragment): a [DID URL fragment](https://www.w3.org/TR/did-core/#dfn-did-fragments).
- [`Timestamp`](crate::common::Timestamp): an [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339) compliant timestamp.
- [`Url`](crate::common::Url): a URL string. 
- Collections: [`OneOrMany`](crate::common::OneOrMany), [`OneOrSet`](crate::common::OneOrSet), and [`OrderedSet`](crate::common::OrderedSet). 

The above types are located in the [`common` module](crate::common).  

## Cryptographic Primitives 
Cryptographic primitives and traits necessary for DID related operations such as signing and verifying data can be found in the [`crypto` module](crate::crypto). 

While the IOTA Identity Framework strives to keep cryptographic operations as internal implementation details, certain lower-level interfaces require constructs from this module, often through the cryptographic [`KeyPair`](crate::crypto::KeyPair) type. 

## Convenient JSON Serialization
The [`ToJson`](crate::convert::ToJson) and [`FromJson`](crate::convert::FromJson) traits from this crate provide convenience functions to convert most types from the IOTA Identity Framework to and from a few common JSON representations.

## Encoding utilities 
[`BaseEncoding`](crate::utils::BaseEncoding) provides methods to encode and decode binary text with respect to several bases.

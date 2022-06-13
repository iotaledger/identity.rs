IOTA Identity - Core  
=== 

The core types and utilities for IOTA identity.

The types and traits exposed by this crate are mainly intended to be used by the other crates constituting the [IOTA Identity Framework](https://wiki.iota.org/identity.rs/introduction). 

## Common data types 
This crate introduces data types necessary for many concepts within the IOTA Identity Framework. These include 
- [`Context`](crate::common::Context): Representing [JSON-LD contexts](https://www.w3.org/TR/vc-data-model/#contexts)
- [`Fragment`](crate::common::Fragment): Representing a [DID Url fragment](https://www.w3.org/TR/did-core/#dfn-did-fragments)
- [`Timestamp`](crate::common::Timestamp): Representing an [RFC3339 compliant](https://datatracker.ietf.org/doc/html/rfc3339) timestamp. 
- [`Url`](crate::common::Url): Representing a Url. 
- More collections such as [`OneOrMany`](crate::common::OneOrMany), [`OneOrSet`](crate::common::OneOrSet) and [`OrderedSet`](crate::common::OrderedSet). 

All of these data types are located in the [`common` module](crate::common).  

## Cryptographic primitives and traits 
Cryptographic primitives necessary for DID related operations such as signing and verifying data can be found in the [`crypto` module](crate::crypto). 

The Iota Identity Framework strives for high level apis that alleviates users from worrying about cryptography, but lower level apis involving more direct interaction with this module are also offered. For the latter one will typically use the [`KeyPair`](crate::crypto::KeyPair) type representing a pair of cryptographic keys. 

## Convenient JSON serialization
By bringing the [`ToJson`](crate::convert::ToJson) and [`FromJson`](crate::convert::FromJson) traits from this crate into scope, one can conveniently convert most types used in the IOTA Identity Framework to and from a few common JSON representations.  
## Encoding utilities 
One may encode and decode between various base-encoded binary text using the associated methods of [`BaseEncoding` ](crate::utils::BaseEncoding). 

IOTA Identity - DID
=== 

Agnostic implementation of the Decentralized Identifiers (DID) standard from W3C.

## Overview 
Decentralized Identifiers (DID) is a proposed standard from the World Wide Web Consortium (W3C) to enable a
verifiable and decentralized identity. The standard provides a unique identifier (DID), which can be used to look up
more information about the associated identity in the form of a DID Document. The DID Document contains public keys,
to prove control over the identity, and service endpoints which are URI's that can be resolved to find more public
information about the identity. Often the DID Documents are stored on a Distributed Ledger Technology (DLT) such as
Bitcoin, Ethereum and IOTA, but this is not a requirement.

The [IOTA Identity Framework](https://wiki.iota.org/identity.rs/introduction) leverages this crate to build its own [DID method](https://www.w3.org/TR/2020/WD-did-core-20200731/#dfn-did-methods), but the types and traits here are defined according to the [DID specifications v1.0 Working Draft 20200731](https://www.w3.org/TR/2020/WD-did-core-20200731/) which is method agnostic. 

## Central functionality 
When working with the IOTA Identity Framework one will frequently interact with functionality from this crate listed here. 

- [`DID` trait](crate::did::DID)
- [`DIDUrl`](crate::did::DIDUrl)
- [`Service`](crate::service::Service)
- [`VerificationMethod`](crate::verification::VerificationMethod)
- [`MethodRelationship`](crate::verification::MethodRelationship)
- [`MethodScope`](crate::verification::MethodScope)
- [`MethodType`](crate::verification::MethodType)
- [`VerifierOptions`](crate::verifiable::VerifierOptions)
- [`RevocationBitmap`](crate::revocation::RevocationBitmap)
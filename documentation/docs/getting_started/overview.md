---
description: Using IOTA Identity, a new digital identity can be created by anyone or anything at any time by generating a Decentralized Identifier (DID) combined with Verifiable Credentials
image: /img/Identity_icon.png
keywords:
- Identity
- verifiable
- credentials
- Rust
- WASM
- reference
---

# Overview

Using the [standards proposed by W3C](https://www.w3.org/TR/did-core/), this section explains the IOTA Identity implementation. You can use this implementation to create a new digital identity for anyone or anything at any time. To do so, you must first generate a [Decentralized Identifier (DID)](../decentralized_identifiers/overviewy) that will serve as a reference to the [DID Document](../decentralized_identifiers/overview#did-documents). The DID Document contains public keys and other mechanisms to enable the subject to prove their association with the DID.

However, you cannot tell much about the subject from a DID. You need to combine the DID with [Verifiable Credentials](../verifiable_credentials/overview). Verifiable Credentials are statements about the creator of the DID. They can be shared and verified online in a "Bring Your Own Identity" (BYOI) manner, and the DID creator remains in complete control of the process.

You can use this framework in processes such as:

- Address validation: Customers can prove where they live for shipping and billing addresses.
- Age verification: Customers can prove they are 18+ for online purchases.
- (Authority) Login: Customers can prove who they are and gain access to their account,
  without passwords. This can be useful for many websites, including eGovernment and
  banking.


## Implementations

The IOTA Identity framework is developed in the Rust programming language. We also provide bindings, or "Foreign Function Interfaces" (FFI), to other languages. The full set of language bindings currently available is:

- [Rust](../libraries/rust/getting_started)
- [WASM](../libraries/wasm/getting_started)

## Applications

The following applications are currently utilizing the IOTA Identity framework:

- [Selv app](https://selv.iota.org/)
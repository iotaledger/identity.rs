---
sidebar_position: 2
title: Overview
---


# Overview
Using the standards proposed by W3C, this chapter will explain the IOTA Identity implementation. Using this implementation, a new digital identity can be created by anyone or anything at any time. To do so, a Decentralized Identifier (DID) is generated, that serves as a reference to a DID Document. The DID Document contains public keys, and other mechanisms, to enable the subject to prove their association with the DID. 

However a DID alone tells you little about the subject. It must be combined with Verifiable Credentials. Verifiable Credentials are statements about the creator of the DID. They can be shared and verified online in a BYOI manner, and the DID creator remains in complete control of the process. 

This framework can be used in processes such as:
- Address validation: Customers can prove where they live for shipping and billing addresses
- Age verification: Customers can prove they are 18+ for online purchases.
- (Authority) Login: Customers can prove who they are and gain access to their account,
without passwords. This can be useful for many websites, including eGovernment and
banking.



## Implementations
- Rust 
- WASM

## Applications
- [Selv app](https://selv.iota.org/)
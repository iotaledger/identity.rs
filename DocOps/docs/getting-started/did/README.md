---
sidebar_label: Decentralized Identifiers 
---

# Decentralized Identifiers

The Decentralized Identifiers (DID) standard from W3C is the fundamental standard that supports the concept of a decentralized digital identity. A DID is a unique identifier that contains  information that can be resolved to a DID Document. This document contains data such as public keys, enabling the holder to prove ownership over their personal data, but also URIs that link to public information about the identity. This implementation complies to the [DID specifications v1.0 Working Draft 20200731](https://www.w3.org/TR/2020/WD-did-core-20200731/). 

In the IOTA Identity framework, we have implemented the DID standard according to the `iota` DID Method Specification, which can be viewed [here](./specs/method_spec.md). We recommend to see the `iota` DID Method Specification as the golden standard for DID on IOTA. Other implementations of DID on IOTA is recommended to follow the `iota` DID Method Specification. However, it is not necassary to implement a novel Method implementation for every project, so feel free to utilize this framework directly. 

An example of DID conforming to the `iota` method specification:
`did:iota:8dQAzVbbf6FLW9ckwyCBnKmcMGcUV9LYJoXtgQkHcNQy`

### Chapter overview

In this chapter we will cover the basic aspects of the DID standard. We will explore the how and why of DID Document and why IOTA is very suitable for DID.

### What are DIDs and DID Documents?

### Why use DIDs?

### Why use IOTA Identity over other implementations?
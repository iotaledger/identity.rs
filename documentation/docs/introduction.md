---
keywords:
- Identity
- guide
- TOC
- overview
description: The most important concepts that developers will need to know to utilize IOTA Identity to its full potential.
image: /img/Identity_icon.png
---

# IOTA Identity Framework Guide

![IOTA Identity](https://github.com/iotaledger/identity.rs/raw/dev/.meta/identity_banner.png)

The IOTA Identity framework implements the most common standards and patterns for Decentralized Identity in both a DLT agnostic and `iota` method specification manner. It is designed to work for Identity for People, Organizations, Things, and Objects acting as a unifying-layer of trust between everyone and everything.

In this guide, we will go through the most important concepts that developers will need to know to utilize IOTA Identity to its full potential. The guide is programming language agnostic, with language specific guides located in Chapter 4.

## Overview

### [Decentralized Identity](decentralized_identity)

Describes the concept of Decentralized or Self Sovereign Identities (SSI), how it applies to People, Organizations and Things, and why IOTA is used.

### [Getting Started](getting_started/install)

Get started with the IOTA Identity Framework: [Install the library](getting_started/install) and [create and publish](getting_started/create_and_publish) your first DID Document.

### [Decentralized Identifiers (DID)](decentralized_identifiers/overview)

Explains the DID standard from W3C, how to manipulate DID Documents and Merkle Key Collections, the basis of our revocation mechanism.

### [Verifiable Credentials (VC)](verifiable_credentials/overview)

Explains the VC standard from W3C, how to create and revoke VCs, and how to use Verifiable Presentations.

### [DID Communications (DID Comm)](did_communications/overview)

This chapter covers the DID Comm standard, which is being developed by the Decentralized Identity Foundation (DIF). It also describes the different messages agents may send each other and what the expected responses may look like.

### [Advanced Concepts](advanced/overview)

This chapter is meant for those that want to push the IOTA Identity framework to its limits, utilizing the more complex, yet more flexible lower level libraries, allowing developers to optimize their implementation, take control over storage/security, and add features to the framework. 

### Programming Languages

While the framework itself is developed in the Rust programming language, we also provide bindings, or "Foreign Function Interfaces" (FFI), to other languages. These will have separate getting started sections, making the rest of the guide language agnostic, focusing on the conceptual level. The full set of language bindings currently available is:

- [Rust](libraries/rust/getting_started)
- [WASM](libraries/wasm/getting_started)

### [Specification](specs/overview)

While IOTA Identity implements many existing standards, it also adds some additional features we would like to standardize ourselves. This chapter covers these features and how they work in great detail. These are not light reads and can be skipped. 


### [Glossary](glossary)

A list of all terminology used in this guide, the framework and all materials surrounding it. 

### [Contribute](contribute)

A simple guide on how to contribute to the framework.

### [Contact](contact)

How to contact the maintainers.

### [FAQ](faq)

Overview of the most Frequently Asked Questions, and their answers.

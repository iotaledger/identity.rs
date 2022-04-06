---
title: DID Introduction
sidebar_label: Introduction
description: The Decentralized Identifiers (DID) standard from W3C is the fundamental standard that supports the concept of a decentralized digital identity. Explore the basic aspects of the DID standard.
image: /img/Identity_icon.png
keywords:
- public keys
- Method Specification
- Decentralized Identifiers
- overview
- DLT
- tutorial
---

# Decentralized Identifiers (DID)

The Decentralized Identifiers (DID) standard from the World Wide Web Consortium (W3C) is the fundamental standard that supports the concept of a decentralized digital identity. A DID is a unique identifier that contains information that can be resolved to a DID Document. This document contains data such as public keys, enabling the holder to prove ownership over their personal data, but also URIs that link to public information about the identity. This implementation complies with the [DID specifications v1.0 Working](https://www.w3.org/TR/did-core//).

In the IOTA Identity framework, we have implemented the DID standard according to the `iota` [DID Method Specification](../specs/did/iota_did_method_spec.md). We recommend reading the `iota` DID Method Specification as the golden standard for DID on IOTA. Other implementations of DID on IOTA also follow the `iota` DID Method Specification when applicable. However, it is not necessary to implement a novel Method implementation for every project, so feel free to utilize this framework directly. 

Here is an example of a DID conforming to the `iota` method specification:
`did:iota:8dQAzVbbf6FLW9ckwyCBnKmcMGcUV9LYJoXtgQkHcNQy`

## Chapter Overview

In this chapter, we will explain the basic aspects of the DID standard. We will explore the how and why of DID Documents and why IOTA is a suitable technology to host the DID Documents and the rest of a Self-Sovereign Identity (SSI) Framework.

## Decentralized Identifiers

A Decentralized Identifier, or DID, is a unique identifier that is tied to a subject. This subject can be anything: a person, an organization, an IoT device, or even an object. The identifier can be used by the subject to identify themselves through a digital format, providing a basis for online identification. The identifier looks like a set of random characters that includes some prefixes to determine which standard and implementation is used:

`did:iota:8dQAzVbbf6FLW9ckwyCBnKmcMGcUV9LYJoXtgQkHcNQy`

W3C is a well-known standardization body that has defined how DIDs should look and work. This provides a basis for different technologies that implement the DID standard to achieve interoperability. A full list of all implementations can be found [here.](https://www.w3.org/TR/did-spec-registries/#did-methods) Please keep in mind that, unfortunately, most of these methods are outdated and not maintained.

## DID Documents

The purpose of a DID is to help navigate to a DID Document, which is a document containing more information regarding the identity subject. This document contains data, such as public keys, enabling the subject to prove ownership over their personal data, but also URIs that link to public information about the identity.

The identifier contains all information to resolve a DID, providing the latest DID Document. The first three characters `did` indicate that the DID standard from W3C must be used to resolve the identifier. It is followed by a unique method name, in our case `iota`, to indicate that the IOTA method is used. The IOTA method is a specific implementation that follows the following [method spec](../specs/did/iota_did_method_spec.md). This provides unique rules for the protocol to follow to result in the latest DID Document. In our case, it describes how DID Documents are uploaded and queried to and from the IOTA Tangle. Lastly, a DID contains a set of random characters that are unique per identity, making the identity unique and ensuring every identity resolves to a unique DID Document. 

:::note Requires basic knowledge of Asymmetric Encryption

The following and later sections require some basic knowledge of Asymmetric Encryption. Please read or view some materials on the subject before continuing.

:::

A DID Document contains at least two important pieces of data: public keys and services. The public keys can be used to prove ownership over the identity, by cryptographically signing something with the associated private key. The public key can be used to verify that the identity subject signed the data and therefore controls the private key. Ownership over the private keys, therefore, proves ownership over the identity. This also means that it is very important to keep the private keys safe and secure. Additionally, the public keys allow users to send encrypted data to the identity, using their public key, that only the identity owner can decrypt.

:::warning

Never share your private keys, seeds, passphrases with anyone. Not even with IOTA Foundation members. This may lead to loss of IOTA funds or control over your own digital identity.

:::

Services are URIs that point to more information about the identity. This could be something as simple as a website for an organizational identity. These services are publicly available for all to read and should therefore not contain Personal Identifiable Information (PII) in the case of human identities.

## Why Use DIDs?

DIDs allow any subject to have a unique identifier, that they can prove ownership of and, at the same time, provide a way to send them encrypted messages. The Identity is Self-Sovereign, meaning the subject is in control of when the identity is created but also destroyed.

DIDs become more interesting in combination with Verifiable Credentials, which will be covered in a later section. Essentially, Verifiable Credentials (VCs) are signed statements by trusted third parties about a certain identity. The signer, or Issuer, is referenced by the DID and so is the subject, often called the Holder. The Holder controls a copy of this statement and share it with other parties, the Verifiers, that can verify the statement and check which party made the statement, without having to ask the Issuer. Instead, they can verify the signature of the Issuer by checking the Issuers DID Document. This whole setup puts Holders back in control over their own data, but also makes the data much more trustworthy as it has become verifiable.

## Why Use IOTA Identity Over Other Implementations?

IOTA Identity is a framework to implement SSI on IOTA. Inherently, IOTA provides unique features that have a major impact on the usability of the framework.

### Feeless

IOTA is a feeless Distributed Ledger Technology, which means that messages can immutably be stored inside the Tangle at no cost, nor a requirement of holding any cryptocurrency tokens. That means that SSI applications can directly deploy towards the main network without any problems, as compared to most other SSI solutions running on a test network or having cryptocurrency requirements. So not only does it give IOTA Identity predictable costs, but it also turns it into a fair network.

### Ease-of-Use

Without the need for a token, IOTA Identity can directly be used on the main network without having to purchase and manage a cryptocurrency token. Additionally, the framework provides easy-to-use APIs that allow both standardized behavior or a flexible one, but with more complex access. Lastly, IOTA Identity provides a [Stronghold](https://wiki.iota.org/stronghold.rs/welcome/ "Stronghold is an open-source software library that was originally built to protect IOTA Seeds, but can be used to protect any digital secret.") solution for managing secrets securely, without requiring developers to reinvent the security wheel.

### General Purpose DLT

IOTA is a general-purpose DLT as compared to some for-purpose DLTs with restricted use cases. That means that SSI can easily be combined with other DLT features such as payments, data streams, smart contracts, and access control. It will no longer be needed to utilize multiple DLT projects alongside each other.


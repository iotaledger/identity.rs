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
---

# Decentralized Identifiers (DID)

A DID is a unique identifier that can be resolved to a DID Document. This document contains data such as public keys, enabling the holder to prove ownership over their personal data, but also URIs that link to public information about the identity. DIDs are the fundamental building blocks of decentralized digital identity. 
This implementation complies to the [DID specifications v1.0](https://www.w3.org/TR/did-core//) from the World Wide Web Consortium (W3C).

In the IOTA Identity framework, we have implemented the DID standard according to the `iota` [DID Method Specification](../../specs/did/iota_did_method_spec.md). Other implementations of DID on IOTA must follow the `iota` DID Method Specification if they also want to use the `iota` method name. It should not be necessary to re-implement the specification, if you can use library though.

An example of a DID conforming to the `iota` method specification:
`did:iota:0xe4edef97da1257e83cbeb49159cfdd2da6ac971ac447f233f8439cf29376ebfe`

## Chapter Overview

In this chapter, we will explain the basic aspects of the DID standard. We will explore the how and why of DID Documents and why IOTA is a very suitable technology to host the DID Documents and the rest of a Self-Sovereign Identity (SSI) Framework.

## Decentralized Identifiers

A Decentralized Identifier, or DID, is a unique identifier that is tied to a subject. This subject can be anything, like a person, an organization, an IoT device, or even an object. The identifier can be used by the subject to identify themselves through a digital format, providing a basis for online identification. The identifier looks like a set of random characters that includes some prefixes to determine which standard and implementation is used:

`did:iota:0xe4edef97da1257e83cbeb49159cfdd2da6ac971ac447f233f8439cf29376ebfe`

The World Wide Web Consortium (W3C) is a well-known standardization body that has standardized how DIDs should look and work. This provides a basis for different technologies that implement the DID standard to achieve interoperability. A full list of all implementations can be found [here.](https://www.w3.org/TR/did-spec-registries/#did-methods) Please keep in mind that unfortunately most of these methods are outdated and not maintained.

## DID Documents

The purpose of a DID is to help navigate to a DID Document, which is a document containing more information regarding the identity subject. This document contains data such as public keys, enabling the subject to prove ownership over their personal data, but can contain additional information on how to interact with the subject.

The identifier contains all information to resolve a DID, providing the latest DID Document. The first three characters `did` indicate that the DID standard from W3C must be used to resolve the identifier. It is followed by a unique method name, in our case `iota`, to indicate that the IOTA method is used. The IOTA method is a specific implementation following the [IOTA DID Method Specification](../../specs/did/iota_did_method_spec.md). This provides unique rules for the protocol to follow in order to manage a DID Document. In our case, it describes how DID Documents are uploaded and queried to and from the IOTA ledger. Lastly, a DID contains a set of random characters that are unique per identity, this makes the identity unique and makes sure every identity resolves to a unique DID Document.

:::tip Requires basic knowledge of Asymmetric Encryption

The following and later sections require some basic knowledge of Asymmetric Encryption. Please read or view some materials on the subject before continuing.

:::

A DID Document mostly contains two important pieces of data: public keys and services. The public keys can be used to prove ownership over the identity, by cryptographically signing something with the associated private key. The public key can be used to verify that the identity subject signed the data and therefore controls the private key. Ownership over the private keys, therefore, proves ownership over the identity. This also means that it is very important to keep the private keys safe and secure. In addition, the public keys allow users to send encrypted data to the identity, using their public key, that only the identity owner can decrypt.

:::caution

Never share your private keys, seeds, passphrases with anyone. Not even IOTA Foundation members. This may lead to loss of IOTA funds or control over your own digital identity.

:::

Services are URIs that point to more information about the identity. This could be something as simple as a website for an organizational identity. These services are publicly available for all to read and should therefore not contain Personal Identifiable Information (PII) in the case of human identities.

## Why use DIDs?

DIDs allow any subject to have a unique identifier, that they can prove ownership of and at the same time provide a way to send them encrypted messages. The Identity is Self-Sovereign, meaning the subject is in control of when the identity is created but also destroyed.

DIDs become more interesting in combination with Verifiable Credentials, which will be covered in a later section. In essence, Verifiable Credentials (VCs) are signed statements by trusted third parties about a certain identity. The signer, or Issuer, is referenced by the DID and so is the subject, often called the Holder. The Holder controls a copy of this statement and share it with other parties, the Verifiers, that can verify the statement and check which party made the statement, without having to ask the Issuer. Instead, they can verify the signature of the Issuer by checking the Issuers DID Document. This whole setup puts Holders back in control over their own data, but also makes the data much more trustworthy as it has become verifiable.

## Why use IOTA Identity over other implementations?

IOTA Identity is a framework to implement Self-Sovereign Identities on IOTA. Inherently, IOTA provides some unique features that have a major impact on the usability of the framework.

### IOTA ledger benefits

As DID Documents are stored in the ledger state by being covered by the storage deposit, this guarantees that all nodes will have an up-to-date copy of the latest DID Document. Resolving a DID into its document can usually be done by any IOTA node in the network. This solves many issues regarding availability, accessibility or synchronization.

### Layer1 interactions

DID Document are stored in an Alias Outputs, this allows them to directly interact with Layer 1 artifacts like NFTs and native assets. For instance an Alias Output representing a DID can hold native assets or control NFTs. Transferring funds between DIDs is also possible on Layer 1.


### Ease-of-use

Iota Identity helps with abstracting the details of the DID standard by providing easy-to-use APIs that allow standardized behavior. It also allows more flexible and complex management of DID Documents.

### Storage solution

IOTA Identity provides a [Stronghold](https://wiki.iota.org/stronghold.rs/welcome/ "Stronghold is an open-source software library that was originally built to protect IOTA Seeds, but can be used to protect any digital secret.") solution for managing secrets securely, without requiring developers to reinvent the security wheel.

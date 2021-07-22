---
title: Decentralized Identifiers (DID)
sidebar_label: Overview
---

The Decentralized Identifiers (DID) standard from W3C is the fundamental standard that supports the concept of a decentralized digital identity. A DID is a unique identifier that contains  information that can be resolved to a DID Document. This document contains data such as public keys, enabling the holder to prove ownership over their personal data, but also URIs that link to public information about the identity. This implementation complies to the [DID specifications v1.0 Working](https://www.w3.org/TR/did-core//). 

In the IOTA Identity framework, we have implemented the DID standard according to the `iota` DID Method Specification, which can be viewed [here](./specs/iota_did_method_spec.md). We recommend to see the `iota` DID Method Specification as the golden standard for DID on IOTA. Other implementations of DID on IOTA is recommended to follow the `iota` DID Method Specification. However, it is not necassary to implement a novel Method implementation for every project, so feel free to utilize this framework directly. 

An example of DID conforming to the `iota` method specification:
`did:iota:8dQAzVbbf6FLW9ckwyCBnKmcMGcUV9LYJoXtgQkHcNQy`

### Chapter overview

In this chapter we will explain the basic aspects of the DID standard. We will explore the how and why of DID Documents and why IOTA is a very suitable technology to host the DID Documents and the rest of a Self Sovereign Identity Framework.

### Decentralized Identifiers

A Decentralized Identifier, or DID, is a unique identifier that is tied to a subject. This subject can be anything, like a person, an organization, an IoT device or even an object. The identifier can be used by the subject to identify themselves through a digital format, providing a basis for online identification. The identifier looks like a set of random characters that includes some prefixes to determine which standard and implementation is used:

`did:iota:8dQAzVbbf6FLW9ckwyCBnKmcMGcUV9LYJoXtgQkHcNQy`

The World Wide Web Consortium (W3C) is a well-known standardization body that has standardized how DIDs should look like and work. This provides a basis for different technologies that implement the DID standard to achieve interoperability. A full list of all implementations can be found [here.](https://www.w3.org/TR/did-spec-registries/#did-methods) Please keep in mind that unfortuanetly most of these methods are outdated and not maintained. 

### DID Documents

The purpose of a DID is to help navigate to a DID Document, which is a document containing more information regarding the identity subject. This document contains data such as public keys, enabling the subject to prove ownership over their personal data, but also URIs that link to public information about the identity.

The identifier contains all information to resolve a DID, providing the latest DID Document. The first three characters `did` indicate that the DID standard from W3C must be used to resolve the identifier. It is followed by a unique method name, in our case `iota`, to indicate that the IOTA method is used. The IOTA method is a specific implementation that follows the following [method spec](../../specs/iota_did_method_spec.md). This provides unique rules for the protocol to follow in order to result in the latest DID Document. In our case it describes how DID Documents are uploaded and queried to and from the IOTA Tangle. Lastly, a DID contains a set of random characters that are unique per identity, this makes the identity unique and makes sure every identity resolves to a unique DID Document. 

:::tip Requires basic knowledge of Assymtric Encryption

The following and later sections require some basic knowledge of Assymetric Encryption. Please read or view some materials in the subject before continuing. 

:::

A DID Document mostly contains two important pieces of data: public keys and services. The public keys can be used to prove ownership over the identity, by cryptographically signing something with the associated private key. The public key can be used to verify that the identity subject signed the data and therefore controls the private key. Ownership over the private keys therefore prove ownership over the identity. This also means that it is very important to keep the private keys safe and secure. In addition, the public keys allow users to send encrypted data to the identity, using their public key, that only the identity owner can decrypt. 

:::caution

Never share your private keys, seeds, passphrases with anyone. Not even IOTA Foundation members. This may lead to loss of IOTA funds or control over your own digital identity. 

:::

Services are URIs that point to more information about the identity. This could be something as simple as a website for a organizational identity. These services are publicly avaliable for all to read and should therefore not contain Personal Identifiable Information (PII) in the case of human identities. 

### Why use DIDs?

DIDs have some limited use cases. It allows any subject to have a unique identifier, that they can prove ownership off and at the same time provide a way to send them encrypted messages. The Identity is Self Sovereign, meaning the subject is control of when the identity is created, but also destroyed. 

DIDs become really interesting in combination with Verifiable Credentials, that will be covered in a later section. In essence, Verifiable Credentials (VCs) are signed statements by trusted third parties about a certain identity. The signer, or Issuer, is referenced by DID and so is the subject, often called the Holder. The Holder can control a copy of this statement and share this with other parties, the Verifiers, that can verify the statement and check which party gave out the statement, without having to ask the Issuer. Instead, they can verify the signature of the Issuer by checking the Issuers DID Document. This whole set up puts Holders back into control over their own data, but also makes the data much more trustworthy as it has become verifiable. 

### Why use IOTA Identity over other implementations?

IOTA Identity is a framework to implement Self Sovereign Identities on IOTA. Inheritly, IOTA provides some unique features that have a major impact on the usability of the framework.

#### Feeless

IOTA is a feeless Distributed Ledger Technology, that means that messages can immutably be stored inside the Tangle at no-cost, nor a requirements of holding any cryptocurrency tokens. That means that SSI applications can directly deploy towards the main network without any problems, as compared to most other SSI solutions running on a test network or having cryptocurrency requirements. This doesn't just make IOTA Identity have predictable costs and prevent issues around cryptocurrency holding taxes and legislation, it also makes it a fair network as anyone would be able to create one or more identities at no costs. The wealth of someone is irrelevant, making it the most inclusive SSI solution.

#### Ease-of-use

Without the need for a token, IOTA Identity can directly be used on the main network without having to purchase and manage a cryptocurrency token. In addition, the framework provides easy-to-use APIs that allow both standardized behavior or flexible, yet more complex access. Lastly, IOTA Identity provides a Stronghold solution for managing secrets in a secure manner, without requiring developers to reinvent the security wheel.

#### General Purpose DLT

IOTA is a general purpose DLT as compared to some for-purpose DLTs with restricted use cases. That means that SSI can easily be combined with other DLT features such as payments, data streams, smart contracts and access control. It will no longer be needed to utilize multiple DLT projects alongside each other. 


---
title: IOTA DIDComm Specification
sidebar_label: Overview
---

- Version: 0.1
- Status: `IN-PROGRESS`
- Last Updated: 2021-10-18

## Introduction

The IOTA DIDComm Specification standardizes how Self-Sovereign Identities (SSIs) can interact with each other and exchange information. Any applications that implement this standard will naturally be interoperable with each other. This reduces fragmentation in the ecosystem and therefore it is highly recommended to use this specification for any application built on top of the IOTA Identity framework. The specification defines several [protocols](#protocols), that can be used for common interactions like issuing and presenting credentials as well as supporting functions, such as connection management and authentication. Cross-cutting concerns like error handling are discussed in the [resources](#resources) section.

The IOTA DIDComm Specification builds on the [DIDComm Messaging Specification](https://identity.foundation/didcomm-messaging/spec/) developed by the [Decentralized Identity Foundation (DIF)](https://identity.foundation/) and utilises [external protocols](#external-protocols) from the messaging specification for well-established interactions like feature discovery.

This specification is meant to be a complete solution for common use cases and therefore contains protocols for common SSI interactions. It is possible to extend the specification with custom protocols and custom methods in existing protocols to support application-specific requirements. 

The specification itself is technology agnostic. Much like the [DIDComm Messaging Specification](https://identity.foundation/didcomm-messaging/spec/) there are no restrictions on transport layers and a concrete implementation can be done with many different technologies.

## Conformance

The keywords "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL
NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED",  "MAY", and
"OPTIONAL" in this specification are to be interpreted as described in
[BCP 14](https://www.rfc-editor.org/info/bcp14)[[RFC 2119]](https://www.rfc-editor.org/rfc/rfc2119.txt).

## Versioning

Protocols follow [Semantic Versioning 2.0](https://semver.org/) conventions.

## Resources

| Name | Description |
| :--- | :--- |
| [Problem Reports](./resources/problem-reports.md) | Definitions of expected problem reports and guidance on global handling |
| [Credential Kinds](./resources/credential-kinds.md) | Definition of methods to negotiate a specific kind of verifiable credential |

## Protocols

| Name | Version | Description | 
| :--- | :---: | :--- |
| [Connection](./protocols/connection.md) | 0.1 | Establishes a [DIDComm connection](https://identity.foundation/didcomm-messaging/spec/#connections) between two parties. |
| [Authentication](./protocols/authentication.md) | 0.1 | Allows two parties to mutually authenticate, verifying the DID of each other. |
| [Presentation](./protocols/presentation.md) | 0.1 | Allows presentation of verifiable credentials that are issued to a holder and uniquely presented to a third-party verifier. |
| [Issuance](./protocols/issuance.md) | 0.1 | Allows the exchange of a verifiable credential between an issuer and a holder. | 
| [Signing](./protocols/signing.md) | 0.1 | Allows a trusted-party to request the signing of an unsigned verifiable credential by an issuer. |
| [Revocation Options](./protocols/revocation-options.md) | 0.1 | Allows discovery of available [`RevocationInfo`](./protocols/revocation#RevocationInfo) types for use with the [revocation](./protocols/revocation) protocol. |
| [Revocation](./protocols/revocation.md) | 0.1 | Allows to request revocation of an issued credential, either by the holder or a trusted-party. | 
| [Post](./protocols/post.md) | 0.1 | Allows the sending of a single message with arbitrary data. |
| [Termination](./protocols/termination.md) | 0.1 | Indicates the graceful termination of a connection. |

## External Protocols

In addition to the protocols defined in this specification, we RECOMMEND implementors use the following well-known protocols:

| Name | Version | Description |
| :--- | :---: | :--- | 
| [Discover Features](https://github.com/decentralized-identity/didcomm-messaging/blob/9039564e143380a0085a788b6dfd20e63873b9ca/docs/spec-files/feature_discovery.md) | 1.0 | Describes how agents can query one another to discover which features they support, and to what extent. |
| [Trust Ping](https://github.com/decentralized-identity/didcomm-messaging/blob/9039564e143380a0085a788b6dfd20e63873b9ca/docs/spec-files/trustping.md) | 1.0 | A standard way for agents to test connectivity, responsiveness, and security of a DIDComm channel. | 

## Changelog

See [CHANGELOG](./CHANGELOG)

## TODO
TODO: section/page on anonymous encryption/sender authenticated encryption/signed messages (table of comparisons with guarantees? E.g. authentication, eavesdropping protection, integrity etc.). Probably better as a section to avoid duplicating discussions from the DIDComm specification, but they are very light on details in the main document. --- After RFC

TODO: changelog --- After RFC

## Future Work

◈ If necessary, discuss ways for some agent to request the start of an interaction. This has narrow use cases, but might be interesting in the long run.

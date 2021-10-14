---
title: IOTA DIDComm Specification
sidebar_label: Overview
---

*version 0.1, last changed September 2021*

TODO: make clear that we only show message payloads in the protocol examples
TODO: come up with a new protocol, `didcomm://` is VERBOTEN https://identity.foundation/didcomm-messaging/spec/#didcomm-message-uris 


## Conformance

The keywords "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL
NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED",  "MAY", and
"OPTIONAL" in this specification are to be interpreted as described in
[BCP 14](https://www.rfc-editor.org/info/bcp14)[[RFC 2119]](https://www.rfc-editor.org/rfc/rfc2119.txt).

## Resources

| Name | Description |
| :--- | :--- |
| [Problem Reports](./resources/problem-reports.md) | Definitions of expected problem reports and guidance on global handling |
| [Credential Types](./resources/credential-types.md) | Guidance on how to identify a specific type of verifiable credential |

## Versioning



## Protocols

| Name | Version | Description | 
| :--- | :---: | :--- |
| [Connection](./protocols/connection.md) | 0.1 | Allows establishment of a [DIDComm connection](https://identity.foundation/didcomm-messaging/spec/#connections) between two parties. |
| [Authentication](./protocols/authentication.md) | 0.1 | This protocol allows two parties to mutually authenticate, verifying the DID of each other. |
| [Presentation](./protocols/presentation.md) | 0.1 | Allows presentation of verifiable credentials that are issued to a holder and uniquely presented to a third-party verifier. |
| [Issuance](./protocols/issuance.md) | 0.1 | Allows the exchange of a verifiable credential between an issuer and a holder. | 
| [Signing](./protocols/signing.md) | 0.1 | Allows a trusted-party to request the signing of an unsigned verifiable credential by an issuer. |
| [Revocation Options](./protocols/revocation-options.md) | 0.1 | Allows discovery of available [`RevocationInfo`](./protocols/revocation#RevocationInfo) types for use with the [revocation](./protocols/revocation) protocol. |
| [Revocation](./protocols/revocation.md) | 0.1 | Allows to request revocation of an issued credential, either by the holder or a trusted-party. | 
| [Post](./protocols/post.md) | 0.1 | This protocol allows a party to send a generic message. |
| [Termination](./protocols/termination.md) | 0.1 | Indicates the graceful termination of a connection. |

## External Protocols

In addition to the protocols defined in this specification, we recommend implementors use the following well-known protocols:

| Name | Version | Description |
| :--- | :---: | :--- | 
| [Discover Features](https://github.com/decentralized-identity/didcomm-messaging/blob/9039564e143380a0085a788b6dfd20e63873b9ca/docs/spec-files/feature_discovery.md) | 1.0 | Describes how agents can query one another to discover which features it supports, and to what extent. |
| [Trust Ping](https://github.com/decentralized-identity/didcomm-messaging/blob/9039564e143380a0085a788b6dfd20e63873b9ca/docs/spec-files/trustping.md) | 1.0 | A standard way for agents to test connectivity, responsiveness, and security of a DIDComm channel. | 

## Changelog

See [CHANGELOG](./CHANGELOG)

## Future Work

◈ If necessary, discuss ways for some agent to request the start of an interaction. This has narrow use cases, but might be interesting in the long run.

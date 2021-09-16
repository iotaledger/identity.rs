---
title: IOTA DIDComm Specification
sidebar_label: Overview
---

*version 0.1, last changed April 2021*


## Conformance

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL
NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED",  "MAY", and
"OPTIONAL" in this specification are to be interpreted as described in
[BCP 14](https://www.rfc-editor.org/info/bcp14)[[RFC 2119]](https://www.rfc-editor.org/rfc/rfc2119.txt).

## Resources

| Name | Description |
| :--- | :--- |
| [Error Reports](./resources/error-reports.md) | Definitions of expected error reports and guidance on global handling  |

## Protocols

| Name | Version | Description | Messages |
| :--- | :---: | :--- | :--- |
| [presentation](./protocols/presentation.md) | 0.1 | Allows presentation of verifiable credentials that are issued to a holder and uniquely presented to a third-party verifier. | *presentation-offer*, *presentation-request*, *presentation*, *presentation-result* |
| [signing](./protocols/signing.md) | 0.1 | TBD | *signing-request*, *signing-response*, *signing-acknowledgement* |
| [issuance](./protocols/issuance.md) | 0.1 | TBD | *issuance-request*, *issuance-offer*, *issuance-response*, *issuance*, *issuance-acknowledgment* |
| [trust-ping](i_trust-ping.md) | 1.0 | Testing a pairwise channel. | *ping* |

## Changelog

See 

## Future Work

◈ If neccessary, discuss ways for some agent to request the start of an interaction. This has narrow use cases, but might be interesting in the long run.

---
title: Issuance
sidebar_label: Issuance
---

# Issuance

- Version: 0.1
- Status: `IN-PROGRESS`
- Last Updated: 2021-09-16

## Overview

Allows a [holder](#roles) to request a [verifiable credential](https://www.w3.org/TR/vc-data-model/#credentials) from an [issuer](#roles). The [issuer](#roles) may alternatively initiate the issuance without a request from the [holder](#roles).

### Relationships
This protocol may use the [presentation](./presentation.md) and [signing](./signing.md) protocols.

### Example Use-Cases

- A university issues a degree to a graduate that can be verified by potential employers.
- A resident requests proof-of-address from their city council.
- An insurer issues proof that a company has liability issurance.

### Roles
- [Holder](https://www.w3.org/TR/vc-data-model/#dfn-holders): stores one or more verifiable credentials. A holder is usually but not always the subject of those credentials.
- [Issuer](https://www.w3.org/TR/vc-data-model/#dfn-issuers): creates verifiable credentials asserting claims about one or more subjects, transmitted to a holder.

### Interaction

<div style={{textAlign: 'center'}}>

![IssuanceDiagram](/img/didcomm/issuance.drawio.svg)

</div>


## Messages

### 1. issuance-request {#issuance-request}

- Type: `didcomm:iota/issuance/0.1/issuance-request`
- Role: [holder](#roles)

TBD

#### Structure
```json
{
  "subject": DID,             // REQUIRED
  "@context": [string],       // OPTIONAL
  "type": [string],           // REQUIRED
  "trustedIssuers": [string]  // OPTIONAL
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `subject` | [DID](https://www.w3.org/TR/did-core/#dfn-decentralized-identifiers) of the subject of the requested credential. | ✔ |
| `@context` | Array of JSON-LD contexts referencing the credential types. | ✖ |
| `type` | Array of credential types; an issued credential SHOULD match all types specified.[^1] | ✔ |
| `trustedIssuers` | Array of credential issuer IDs or URIs, any of which the holder would accept.[^2] | ✖ |

[^1] The credential `type` could be discovered out-of-band or be pre-sent by an issuer. The types MAY be underspecified if the exact type is not known or if the resulting type depends on the identity or information of the subject or holder. The `type` could be as general as `["VerifiableCredential"]` for example, if the issuer issues only a singular type of credential or decides the credential based on other information related to the subject.
[^2] The [holder](#roles) MAY

#### Examples

1. Requesting a drivers licence credential:

```json
{
  "subject": "did:example:c6ef1fe11eb22cb711e6e227fbc",
  "type": ["VerifiableCredential", "DriversLicence"],
}
```

2. Requesting a university degree as a credential from either supported issuer:

```json
{
  "subject": "did:example:c6ef1fe11eb22cb711e6e227fbc",
  "type": ["VerifiableCredential", "UniversityDegreeCredential"],
  "trustedIssuers":
}
```
### 2. issuance-offer {#issuance-offer}

- Type: `didcomm:iota/issuance/0.1/issuance-offer`
- Role: [issuer](#roles)

TBD

#### Structure
```json
{
  "unsignedCredential": Credential, // REQUIRED
  "requestSignature": bool,         // OPTIONAL
  "expiry": IOSDateTime,            // OPTIONAL
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| [`unsignedCredential`](https://www.w3.org/TR/vc-data-model/#credentials) | Unsigned [verifiable credential](https://www.w3.org/TR/vc-data-model/#credentials) being offered to [holder](#roles). | ✔ |
| `requestSignature` | Indicates if the [issuer](#issuer) requires the acceptance of the credential to be signed by the [holder](#holder). | ✖ |
| `expiry` | Allows the [issuer](#issuer) to specify until when he will uphold the the offer. | ✖ |

#### Examples

1. TBD:

```json
{
 TBD
}
```

### 3. issuance-response {#issuance-response}

- Type: `didcomm:iota/issuance/0.1/issuance-response`
- Role: [holder](#roles)

TBD

#### Structure
```json
{
  "accept": bool,         // REQUIRED
  "disputes": [Dispute],  // OPTIONAL
  "signature": Signature, // OPTIONAL
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `accept` | Indicates if the [holder](#roles) accepts the offered credential from [`issuance-offer`](#issuance-offer). | ✔ |
| `disputes` | Allows the [holder](#roles) to dispute the credential. MAY only be present if the field `accept` is `false`. | ✖ |
| `signature` | Allows the [issuer](#roles) to proof that a credentail was accepted by the [holder](#roles). SHOULD be set if the [issuer](#roles) requested the signature by setting `requestSignature` in the [`issuance-offer`](#issuance-offer). | ✖ |

#### Examples

1. TBD:

```json
{
 TBD
}
```

### 4. issuance {#issuance}

- Type: `didcomm:iota/issuance/0.1/issuance`
- Role: [issuer](#roles)

TBD

#### Structure
```json
{
  "signedCredential": Credential, // REQUIRED
  "signatureChallenge": string,   // OPTIONAL
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `signedCredential` | TBD | ✔ |
| `signatureChallenge` | TBD | ✖ |

#### Examples

1. TBD:

```json
{
 TBD
}
```

### 5. issuance-acknowledgment {#issuance-acknowledgment}

- Type: `didcomm:iota/issuance/0.1/issuance-acknowledgment`
- Role: [holder](#roles)

TBD

#### Structure
```json
{
  "signature": string,          // OPTIONAL
  "signatureChallenge": string, // OPTIONAL
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `signature` | TBD | ✖ |
| `signatureChallenge` | TBD | ✖ |

#### Examples

1. TBD:

```json
{
 TBD
}
```

### Problem Reports

See: https://identity.foundation/didcomm-messaging/spec/#descriptors
TODO

For gerneral guidance see [problem reports](../resources/problem-reports).

Custom error messages for problem-reports that are expected in the course of this protocol. Non-exhaustive, just a normative list of errors that are expected to be thrown.
- e.p.prot.iota.issuance.reject-vc

Also problem reports from embedded protocols can be thrown.

## Considerations

This section is non-normative.

TBD

## Related Work

TBD

## Further Reading

- [Decentralized Identifiers (DIDs) 1.0](https://www.w3.org/TR/did-core/)
- [Verifiable Credentials Data Model 1.0](https://www.w3.org/TR/vc-data-model)
- [Verifiable Credentials Implementation Guidelines 1.0](https://w3c.github.io/vc-imp-guide/)

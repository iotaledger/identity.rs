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

The [holder](#roles) requests a single verifiable credential from the [issuer](#roles).

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
| [`subject`](https://www.w3.org/TR/vc-data-model/#credential-subject-0) | [DID](https://www.w3.org/TR/did-core/#dfn-decentralized-identifiers) of the [credential subject](https://www.w3.org/TR/vc-data-model/#credential-subject-0). | ✔ |
| [`@context`](https://www.w3.org/TR/vc-data-model/#contexts) | Array of JSON-LD contexts referencing the credential types. | ✖ |
| [`type`](https://www.w3.org/TR/vc-data-model/#types) | Array of credential types; an issued credential SHOULD match all types specified.[^1] | ✔ |
| [`trustedIssuer`](https://www.w3.org/TR/vc-data-model/#issuer) | Array of credential issuer IDs or URIs, any of which the holder would accept.[^2] | ✖ |

[^1] The credential `type` could be discovered out-of-band or be pre-sent by an [issuer](#roles). The types MAY be underspecified if the exact type is not known or if the resulting type depends on the identity or information of the subject or holder. The `type` could be as general as `["VerifiableCredential"]` for example, if the issuer issues only a singular type of credential or decides the credential based on other information related to the subject. The [issuer](#roles) SHOULD reject the request with a `problem-report` if it does not support the requested `type`.

[^2] The [holder](#roles) MAY specify one or more `trustedIssuers` they would like to sign the resulting credential. The [issuer](#roles) SHOULD reject the request with a `problem-report` if it does not support any of the requested `trustedIssuers`. However, there are circumstances where a `trustedIssuer` is no longer supported or was compromised, so this behaviour should be decided based on the application.

An [issuer](#roles) wanting to preserve privacy regarding which exact credential types or issuers they support should be careful with the information they disclose in `problem-reports` when rejecting requests. E.g. a `problem-report` with an `invalid-request` code discloses less information than the `invalid-credential-type` or `invalid-trusted-issuer` codes, as the latter two could be used to determine supported types or issuers by process of elimination.

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
  "type": ["VerifiableCredential", "UniversityDegreeCredential", "BachelorOfArtsDegreeCredential"],
  "trustedIssuers": ["did:example:76e12ec712ebc6f1c221ebfeb1f", "did:example:f1befbe122c1f6cbe217ce21e67"]
}
```

### 2. issuance-offer {#issuance-offer}

- Type: `didcomm:iota/issuance/0.1/issuance-offer`
- Role: [issuer](#roles)

The [issuer](#roles) offers a single, unsigned credential to the [holder](#roles), matching the preceding [`issuance-request`](#issuance-request) if present.

#### Structure
```json
{
  "unsignedCredential": Credential, // REQUIRED
  "signatureChallenge": {
    "challenge": string,            // REQUIRED
    "credentialHash": string,       // REQUIRED
  }, // OPTIONAL
  "expiry": ISODateTime             // OPTIONAL
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| [`unsignedCredential`](https://www.w3.org/TR/vc-data-model/#credentials) | Unsigned [verifiable credential](https://www.w3.org/TR/vc-data-model/#credentials) being offered to the[holder](#roles). | ✔ |
| `signatureChallenge` | If present, indicates the [issuer](#issuer) requires the acceptance of the credential to be signed by the [holder](#holder) for non-repudiation. | ✖ |
| `challenge` |  A random string that should be unique per [issuance-offer](#issuance-offer). | ✔ |
| `credentialHash` | The SHA-256? (TODO link) hash of the `unsignedCredential`. | ✔ |
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
- e.p.prot.iota.issuance.invalid-request
- e.p.prot.iota.issuance.invalid-credential-type
- e.p.prot.iota.issuance.invalid-trusted-issuer

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

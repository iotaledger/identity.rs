---
title: Issuance
sidebar_label: Issuance
---

# Issuance

- Version: 0.1
- Status: `IN-PROGRESS`
- Last Updated: 2021-09-17

## Overview

Allows a [holder](#roles) to request a [verifiable credential](https://www.w3.org/TR/vc-data-model/#credentials) from an [issuer](#roles). The [issuer](#roles) may alternatively initiate the issuance without a request from the [holder](#roles). This protocol also allows the [issuer](#roles) to request additional information and to offload the actual singning to a different party.

### Relationships
- [Presentation](./presentation.md): the [issuer](#roles) may request a [verifiable presentation](https://www.w3.org/TR/vc-data-model/#presentations-0) from the [holder](#roles) during the course of this protocol if more information is required.
- [Signing](./signing.md): the [issuer](#roles) may delegate signing to another [issuer](#roles) if they lack the correct authority or private key, in which case the [issuer](#roles) takes on the role of [trusted-party](./signing.md#roles).

### Example Use-Cases

- A university issues a degree to a graduate that can be verified by potential employers.
- A resident requests proof-of-address from their city council.
- An insurer issues proof that a company has liability issurance.

### Roles
- [Holder](https://www.w3.org/TR/vc-data-model/#dfn-holders): stores one or more verifiable credentials. A holder is usually but not always the [subject](https://www.w3.org/TR/vc-data-model/#credential-subject-0) of those credentials.
- [Issuer](https://www.w3.org/TR/vc-data-model/#dfn-issuers): creates verifiable credentials asserting claims about one or more [subjects](https://www.w3.org/TR/vc-data-model/#credential-subject-0), transmitted to a holder.

### Interaction

<div style={{textAlign: 'center'}}>

![IssuanceDiagram](/img/didcomm/issuance.drawio.svg)

</div>


## Messages

### 1. issuance-request {#issuance-request}

- Type: `didcomm:iota/issuance/0.1/issuance-request`
- Role: [holder](#roles)

The [holder](#roles) requests a single verifiable credential from the [issuer](#roles) of a particular type. Optionally, the [holder](#roles) MAY specify one or more [issuers](#roles) from which they would prefer to receive the credential if multiple are available. 

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
| [`subject`](https://www.w3.org/TR/vc-data-model/#credential-subject-0) | [DID](https://www.w3.org/TR/did-core/#dfn-decentralized-identifiers) of the [credential subject](https://www.w3.org/TR/vc-data-model/#credential-subject-0)[^1]. | ✔ |
| [`@context`](https://www.w3.org/TR/vc-data-model/#contexts) | Array of JSON-LD contexts referencing the credential types. | ✖ |
| [`type`](https://www.w3.org/TR/vc-data-model/#types) | Array of credential types; an issued credential SHOULD match all types specified.[^2] | ✔ |
| [`trustedIssuer`](https://www.w3.org/TR/vc-data-model/#issuer) | Array of credential issuer IDs or URIs, any of which the holder would accept.[^3] | ✖ |

[^1] The [holder](#roles) is usually but not always the [subject]((https://www.w3.org/TR/vc-data-model/#credential-subject-0)) of the requested credential. There may be custodial, legal guardianship, or delegation situations where a third-party requests or is issued a credential on behalf of a subject. It is the responsibility of the [issuer](#roles) to ensure authorisation in such cases.

[^2] The credential `type` could be discovered out-of-band or be pre-sent by an [issuer](#roles). The types MAY be underspecified if the exact type is not known or if the resulting type depends on the identity or information of the subject or holder. The `type` could be as general as `["VerifiableCredential"]` for example, if the issuer issues only a singular type of credential or decides the credential based on other information related to the subject. The [issuer](#roles) SHOULD reject the request with a `problem-report` if it does not support the requested `type`.

[^3] The [holder](#roles) MAY specify one or more `trustedIssuers` they would like to sign the resulting credential. The [issuer](#roles) SHOULD reject the request with a `problem-report` if it does not support any of the requested `trustedIssuers`. However, there are circumstances where a `trustedIssuer` is no longer supported or was compromised, so this behaviour should be decided based on the application.

An [issuer](#roles) wanting to preserve privacy regarding which exact credential types or issuers they support should be careful with the information they disclose in `problem-reports` when rejecting requests. E.g. a `problem-report` with an `invalid-request` code discloses less information than the `invalid-credential-type` or `invalid-trusted-issuer` codes, as the latter two could be used to determine supported types or issuers by process of elimination.

#### Examples

1. Request a drivers licence credential:

```json
{
  "subject": "did:example:c6ef1fe11eb22cb711e6e227fbc",
  "type": ["VerifiableCredential", "DriversLicence"],
}
```

2. Request a university degree as a credential from either supported issuer:

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

The [issuer](#roles) offers a single, unsigned credential to the [holder](#roles), matching the preceding [`issuance-request`](#issuance-request) if present. The [issuer](#roles) may set an expiry date for the offer and require a non-repudiable proof by the [holder](#roles) that the offer was received.

#### Structure
```json
{
  "unsignedCredential": Credential, // REQUIRED
  "signatureChallenge": {
    "challenge": string,            // REQUIRED
    "credentialHash": string,       // REQUIRED
  }, // OPTIONAL
  "expiry": DateTime                // OPTIONAL
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| [`unsignedCredential`](https://www.w3.org/TR/vc-data-model/#credentials) | Unsigned [credential](https://www.w3.org/TR/vc-data-model/#credentials) being offered to the [holder](#roles). This MUST NOT include a `proof` section. | ✔ |
| `signatureChallenge` | If present, indicates the [issuer](#issuer) requires the acceptance of the credential to be signed by the [holder](#holder) in the following [issuance-response](#issuance-response) for non-repudiation. | ✖ |
| `challenge` |  A random string that should be unique per [issuance-offer](#issuance-offer). | ✔ |
| `credentialHash` | The [Base58](https://tools.ietf.org/id/draft-msporny-base58-01.html)-encoded [SHA-256 digest](https://www.rfc-editor.org/rfc/rfc4634) of the `unsignedCredential` formatted according to the [JSON Canonicalization Scheme](https://tools.ietf.org/id/draft-rundgren-json-canonicalization-scheme-00.html). | ✔ |
| `expiry` | A string formatted as an [XML DateTime](https://www.w3.org/TR/xmlschema11-2/#dateTime) normalized to UTC 00:00:00 and without sub-second decimal precision. E.g: `"2021-12-30T19:17:47Z"`.[^1] | ✖ |

[^1] If present, an `expiry` indicates that the [issuer](#roles) MAY rescind the offer and abandon the protcol if an affirmative [issuance-response](#issuance-response) is not received before the specified datetime. Note that the `expiry` should override any default message timeouts.

#### Examples

1. Offer a degree credential:

```json
{
 "unsignedCredential": {
    "@context": [
      "https://www.w3.org/2018/credentials/v1",
      "https://www.w3.org/2018/credentials/examples/v1"
    ],
    "id": "6c1a1477-e452-4da7-b2db-65ad0b369d1a",
    "type": ["VerifiableCredential", "UniversityDegreeCredential"],
    "issuer": "did:example:76e12ec712ebc6f1c221ebfeb1f",
    "issuanceDate": "2021-05-03T19:73:24Z",
    "credentialSubject": {
      "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
      "degree": {
        "type": "BachelorDegree",
        "name": "Bachelor of Science and Arts"
      }
    }
  }
}
```

2. A time-limited offer for a degree credential with a signature requested:

```json
{
 "unsignedCredential": {
    "@context": [
      "https://www.w3.org/2018/credentials/v1",
      "https://www.w3.org/2018/credentials/examples/v1"
    ],
    "id": "6c1a1477-e452-4da7-b2db-65ad0b369d1a",
    "type": ["VerifiableCredential", "UniversityDegreeCredential"],
    "issuer": "did:example:76e12ec712ebc6f1c221ebfeb1f",
    "issuanceDate": "2021-05-03T19:73:24Z",
    "credentialSubject": {
      "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
      "degree": {
        "type": "BachelorDegree",
        "name": "Bachelor of Science and Arts"
      }
    }
  },
  "signatureChallenge": {
    "challenge": "d7b7869e-fec3-4de9-84bb-c3a43bacff33",
    "credentialHash": "28Ae7AdqzyMyF9pmnwUNK1Q7VT3EzDDGEj1Huk7uYQT94KYAhQzEPyhoF5Ugs3totUugLPpghGmE9HaG8usJZcZv",
  },
  "expiry": "2021-11-01T21:04:31Z"
}
```

### 3. issuance-response {#issuance-response}

- Type: `didcomm:iota/issuance/0.1/issuance-response`
- Role: [holder](#roles)

The [holder](#roles) responds to a [`issuance-offer`](#issuance-offer) by aceepting or disputing the offer and optionally signing the response for non-repudiation.

#### Structure
```json
{
  "accepted": bool,             // REQUIRED
  "disputes": [Dispute],        // OPTIONAL
  "signature": {
    "signatureChallenge": {
      "challenge": string,      // REQUIRED
      "credentialHash": string, // REQUIRED
    }, // REQUIRED
    "proof": Proof,             // REQUIRED
  } // OPTIONAL
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `accepted` | Indicates if the [holder](#roles) accepts the offered credential from [`issuance-offer`](#issuance-offer). MUST be `false` if any `disputes` are present. | ✔ |
| [`disputes`](https://www.w3.org/TR/vc-data-model/#disputes) | Allows the [holder](#roles) to [`dispute`](https://www.w3.org/TR/vc-data-model/#disputes) one or more claims in the credential. | ✖ |
| `signature` | Allows the [issuer](#roles) to prove that a credential was accepted by the [holder](#roles). SHOULD be present if a `signatureChallenge` was included in the preceding [`issuance-offer`](#issuance-offer). | ✖ |
| `signatureChallenge` | MUST match the `signatureChallenge` in the preceding [`issuance-offer`](#issuance-offer). | ✔ |
| [`proof`](https://w3c-ccg.github.io/ld-proofs/) | Signature of the [holder](#roles) on the `signatureChallenge`. | ✔ |

#### Examples

1. Rejecting a credential offer with disputes:

```json
{
 TBD
}
```

2. Acception a credential offer including a proof:

```json
{
 TBD
}
```

### 4. issuance {#issuance}

- Type: `didcomm:iota/issuance/0.1/issuance`
- Role: [issuer](#roles)

The [issuer](#roles) transmits the signed credential following a [`issuance-response`](#issuance-response) by the [holder](#roles). The [issuer](#roles) may set an expiry until when they expect an achknowledgment and request a cryptographic signature in the acknowledgment for non-repudiation. 

#### Structure
```json
{
  "signedCredential": Credential,   // REQUIRED
  "signatureChallenge": {
    "challenge": string,            // REQUIRED
    "credentialHash": string,       // REQUIRED
  }, // OPTIONAL
  "expiry": DateTime,               // OPTIONAL
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| [`signedCredential`](https://www.w3.org/TR/vc-data-model/#credentials) | [Verifiable credential](https://www.w3.org/TR/vc-data-model/#credentials) signed by the [issuer](#roles). | ✔ |
| `signatureChallenge` | If present, indicates the [issuer](#issuer) requires the receival of the credential to be signed for non-repudiation. | ✖ |
| `challenge` |  A random string that should be unique per [issuance](#issuance). | ✔ |
| `credentialHash` | The SHA-256? (TODO link) hash of the `signedCredential`. | ✔ |
| `expiry` | A string formatted as an [XML Datetime](https://www.w3.org/TR/xmlschema11-2/#dateTime) normalized to UTC 00:00:00 and without sub-second decimal precision. E.g: `"2021-12-30T19:17:47Z"`. | ✖ |

#### Examples

1. Issuing a credential including an expiry and requesting proof:

```json
{
 TBD
}
```

### 5. issuance-acknowledgment {#issuance-acknowledgment}

- Type: `didcomm:iota/issuance/0.1/issuance-acknowledgment`
- Role: [holder](#roles)

The [holder](#roles) responds to an [`issuance`](#issuance) confirming he received the credential, optionally including non-repudiable proof.

#### Structure
```json
{
  "signatureChallenge": string, // OPTIONAL
  "signature": Proof,           // OPTIONAL
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `signature` | TBD | ✖ |
| `signatureChallenge` | TBD | ✖ |

#### Examples

1. Acknowledging the receival of a credential including proof:

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

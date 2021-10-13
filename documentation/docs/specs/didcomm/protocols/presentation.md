---
title: Presentation
sidebar_label: Presentation
---

# Presentation

- Version: 0.1
- Status: `IN-PROGRESS`
- Last Updated: 2021-09-17

## Overview

Allows presentation of [verifiable credentials](https://www.w3.org/TR/vc-data-model) that are issued to a [holder](#roles) and uniquely presented to a third-party [verifier](#roles).

### Relationships
- [Issuance](./issuance): a presentation may be used to provide request extra information from the [holder](#roles) during a credential issuance.

### Example Use-Cases

- A company founder wants to prove they have a bank account in order to apply for insurance.
- A traveller proves to the border agency that he has a valid visa.

### Roles
- [Holder](https://www.w3.org/TR/vc-data-model/#dfn-holders): possesses one or more credentials that are combined in a verifiable presentation to show proof of ownership to the verifier.
- [Verifier](https://www.w3.org/TR/vc-data-model/#dfn-verifier): receives and validates the credentials presented by the holder.

### Interaction

<div style={{textAlign: 'center'}}>

![PresentationDiagram](/img/didcomm/presentation.drawio.svg)

</div>


## Messages

### 1. presentation-offer {#presentation-offer}

- Type: `didcomm:iota/presentation/0.1/presentation-offer`
- Role: [holder](#roles)

Sent by the holder to offer one or more credentials for a verifier to view. 
The context and types are included to allow the verifier to choose whether they are interested in the offer, negotiate the type of credentials they want or accept and by which issuers they trust.

#### Structure
```json
{
  "offers": [{
    "@context": [string],   // OPTIONAL
    "type": [string],       // REQUIRED
    "issuer": string,       // OPTIONAL
  }], // REQUIRED
  "requireSignature": bool, // OPTIONAL
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `offers` | Array of one or more offers, each specifying a single credential possessed by the holder. | ✔ |
| [`@context`](https://www.w3.org/TR/vc-data-model/#contexts) | Array of JSON-LD contexts referenced in the credential. | ✖ |
| [`type`](https://www.w3.org/TR/vc-data-model/#types) | Array of credential types specifying the kind of credential offered.[^1] | ✔ | 
| [`issuer`](https://www.w3.org/TR/vc-data-model/#issuer) | The ID or URI of the credential issuer.[^2] | ✖ |
| `requireSignature` | Request that the verifier sign its [`presentation-request`](#presentation-request) with a proof. It is RECOMMENDED that the holder issues a `problem-report` if the verifier does not sign the message when this is true. | ✖ |

[^1] The types MAY be underspecified to preserve privacy but MUST always include the most general types. For example, a credential with the types `["VerifiableCredential", "DriversLicence", "EUDriversLicence", "GermanDriversLicence"]` could be specified as `["VerifiableCredential", "DriversLicence"]`.

[^2] The issuer is OPTIONAL as the holder may not want to reveal too much information up-front on the exact credentials they possess; they may want a non-repudiable signed request from the verifier first? 

TODO: selective disclosure / ZKP fields?

#### Examples

1. Offer a single verifiable credential:

```json
{
  "offers": [{
    "type": ["VerifiableCredential", "UniversityDegreeCredential"],
    "issuer": "did:example:76e12ec712ebc6f1c221ebfeb1f"
  }]
}
```

2. Offer two verifiable credentials with different issuers:

```json
{
  "offers": [{
    "type": ["VerifiableCredential", "UniversityDegreeCredential"],
    "issuer": "did:example:76e12ec712ebc6f1c221ebfeb1f"
  }, 
  {
    "type": ["VerifiableCredential", "UniversityDegreeCredential"],
    "issuer": "https://example.edu/issuers/565049"
  }]
}
```

### 2. presentation-request {#presentation-request}

- Type: `didcomm:iota/presentation/0.1/presentation-request`
- Role: [verifier](#roles)

Sent by the verifier to request one or more verifiable credentials from a holder. 
The context and types are included, as well as trusted issuers, to allow the holder to determine if he possesses relevant credentials. This message allows a non-repudiable proof, that the verifier requested data. 

#### Structure
```json
{
  "requests": [{
    "@context": [string],       // OPTIONAL
    "type": [string],           // REQUIRED
    "trustedIssuer": [string],  // OPTIONAL
    "optional": bool            // OPTIONAL
  }], // REQUIRED
  "challenge": string,          // REQUIRED
  "proof": Proof                // OPTIONAL
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `requests` | Array of one or more requests, each specifying a single credential possessed by the holder. | ✔ |
| [`@context`](https://www.w3.org/TR/vc-data-model/#contexts) | Array of JSON-LD contexts referenced in a credential. | ✖ |
| [`type`](https://www.w3.org/TR/vc-data-model/#types) | Array of credential types; a presented credential SHOULD match all types specified. | ✔ | 
| [`trustedIssuer`](https://www.w3.org/TR/vc-data-model/#issuer) | Array of credential issuer IDs or URIs; any of which the verifier would accept. | ✖ |
| `optional` | Whether this credential is required (`false`) or optional (`true`) to present by the holder. A holder SHOULD send a problem report if unable to satisfy a non-optional credential request. Default: `false`. | ✖ |
| [`challenge`](https://w3c-ccg.github.io/ld-proofs/#dfn-challenge) | A random string unique per [`presentation-request`](#presentation-request) by a verifier to help mitigate replay attacks. | ✔ |
| [`proof`](https://w3c-ccg.github.io/ld-proofs/) | Signature of the verifier; RECOMMENDED to include if preceded by a [`presentation-offer`](#presentation-offer) with `requireSignature = true`.[^3] | ✖ |

[^3] Verifiers are RECOMMENDED to include a proof whenever possible to avoid rejections from holders that enforce non-repudiation. Holders could use this to prove that a verifier is non-compliant with laws or regulations, e.g. over-requesting information protected by [GDPR](https://gdpr-info.eu/). Holders MAY still choose to accept unsigned [`presentation-requests`](#presentation-request) on a case-by-case basis, even if `requireSignature` was `true` in their [`presentation-offer`](#presentation-offer), as some verifiers may be unable to perform cryptographic signing operations. If the `proof` is invalid, the receiving holder MUST send a `problem-report`.

Note that the `proof` SHOULD NOT be used for authentication of the verifier in general; it is RECOMMENDED to use [Sender Authenticated Encryption](https://identity.foundation/didcomm-messaging/spec/#sender-authenticated-encryption) for authentication of parties in a DIDComm thread.

#### Examples

1. Request a single credential matching both specified types.

```json
{
  "requests": [{
    "type": ["VerifiableCredential", "UniversityDegreeCredential"]
  }],
  "challenge": "06da6f1c-26b0-4976-915d-670b8f407f2d"
}
```

2. Signed request of a required credential from a particular trusted issuer and an optional credential. 

```json
{
  "requests": [{
    "type": ["VerifiableCredential", "UniversityDegreeCredential"],
    "trustedIssuer": ["did:example:76e12ec712ebc6f1c221ebfeb1f"]
  }, {
    "type": ["VerifiableCredential", "DriversLicence"],
    "optional": true
  }], 
  "challenge": "06da6f1c-26b0-4976-915d-670b8f407f2d",
  "proof": { ... }
}
```

3. Request a single credential signed by one of several trusted issuers.

```json
{
  "requests": [{
    "type": ["VerifiableCredential", "UniversityDegreeCredential"],
    "trustedIssuer": ["did:example:76e12ec712ebc6f1c221ebfeb1f", "did:example:f1befbe122c1f6cbe217ce21e67", "did:example:c6ef1fe11eb22cb711e6e227fbc"],
    "optional": false
  }], 
  "challenge": "06da6f1c-26b0-4976-915d-670b8f407f2d",
}
```

### 3. presentation {#presentation}

- Type: `didcomm:iota/presentation/0.1/presentation`
- Role: [holder](#roles)

Sent by the holder to present a [verifiable presentation](https://www.w3.org/TR/vc-data-model/#presentations-0) of one or more [verifiable credentials](https://www.w3.org/TR/vc-data-model/#credentials) for a [verifier](#roles) to review.

#### Structure
```json
{
  "presentation": VerifiablePresentation // REQUIRED
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| [`presentation`](https://www.w3.org/TR/vc-data-model/#presentations-0) | Signed [verifiable presentation](https://www.w3.org/TR/vc-data-model/#presentations-0) containing one or more [verifiable credentials](https://www.w3.org/TR/vc-data-model/#credentials) matching the [presentation-request](#presentation-request).[^4] | ✔ |

[^4] The [`proof`](https://www.w3.org/TR/vc-data-model/#proofs-signatures) section in `presentation` MUST include the `challenge` sent by the verifier in the preceding [`presentation-request`](#presentation-request). The included credentials SHOULD match all `type` fields and one or more `trustedIssuer` if included in the [`presentation-request`](#presentation-request). Revoked, disputed, or otherwise invalid presentations or credentials MUST result in a rejected [`presentation-result`](#presentation-result) sent back to the holder, NOT a separate [`problem-report`]. Other such as the message lacking [sender authenticated encryption](https://identity.foundation/didcomm-messaging/spec/#sender-authenticated-encryption) SHOULD result in a separate [`problem-report`].

TODO: we may want separate problem-reports instead, as mixing disputes with problem-reports if improperly implemented may reveal information to a fake holder trying to discover information about what content a verifier accepts.

#### Examples

1. Presentation of a verifiable presentation credential.

```json
{
  "presentation": {
    "@context": [
      "https://www.w3.org/2018/credentials/v1",
      "https://www.w3.org/2018/credentials/examples/v1"
    ],
    "type": "VerifiablePresentation",
    "verifiableCredential": [{
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
      },
      "proof": { ... }
    }],
    "proof": {
      "challenge": "06da6f1c-26b0-4976-915d-670b8f407f2d",
      ...
    }
  }
}
```

### 4. presentation-result {#presentation-result}

- Type: `didcomm:iota/presentation/0.1/presentation-result`
- Role: [verifier](#roles)

Sent by the verifier to communicate the result of the presentation. It allows the verifier raise problems and disputes encountered in the verification and to specify if the holder may retry a presentation. The message SHOULD be signed by the verifier for non-repudiation.  

#### Structure
```json
{
  "accepted": bool,                   // REQUIRED
  "disputes": [{
    "id": string,                     // OPTIONAL
    "dispute": Dispute,               // REQUIRED
  }], // OPTIONAL
  "problems": [{
    "id": string,                     // OPTIONAL
    "problemReport": ProblemReport,   // REQUIRED
  }], // OPTIONAL
  "allowRetry": bool,                 // OPTIONAL
  "proof": Proof                      // OPTIONAL
}
```

TODO: use DIDComm signed message instead of `proof`?

| Field | Description | Required |
| :--- | :--- | :--- |
| `accepted` | Indicates if the verifier accepted the [`presentation`](#presentation) and credentials. | ✔ |
| `disputes` | Array of disputes | ✖ |
| [`id`](https://www.w3.org/TR/vc-data-model/#identifiers) | Identifier of the credential for which there is a dispute. A holder may omit credential identifiers for privacy reasons. | ✖ |
| [`dispute`](https://www.w3.org/TR/vc-data-model/#disputes) | A [dispute](https://www.w3.org/TR/vc-data-model/#disputes) by the verifier of one or more claims in a presented credential. | ✔ |
| `problems` | Array of problem-reports | ✖ |
| [`id`](https://www.w3.org/TR/vc-data-model/#identifiers) | Identifier of the credential for which there is a problem. A holder may omit credential identifiers for privacy reasons. | ✖ |
| `problemReport` | A [`problem-report`](https://identity.foundation/didcomm-messaging/spec/#problem-reports) indicating something wrong with the credential, e.g. signature validation failed or the credential is expired. | ✔ | 
| `allowRetry` | Indicates if the holder may retry the [`presentation`](#presentation) with different credentials. Default: `false` | ✖ |
| [`proof`](https://w3c-ccg.github.io/ld-proofs/) | Signature of the verifier; RECOMMENDED to include.[^5] | ✖ |

[^5] Similar to [`presentation-request`](#presentation-request), verifiers are RECOMMENDED to include a proof whenever possible for non-repudiation of receipt of the presentation. Holders may choose to blocklist verifiers that refuse to provide non-repudiable signatures.

#### Examples

1. Successful result, including a proof for non-repudiation.

```json
{
  "accepted": true,
  "proof": { ... }
}
```

2. Unsucessful result disputing a credential, allowing the holder to retry. 

```json
{
  "accepted": false,
  "disputes": [{
    "id": "http://example.com/credentials/123",
    "dispute": {
      "@context": [
        "https://www.w3.org/2018/credentials/v1",
        "https://www.w3.org/2018/credentials/examples/v1"
      ],
      "id": "http://example.com/credentials/123",
      "type": ["VerifiableCredential", "DisputeCredential"],
      "credentialSubject": {
        "id": "http://example.com/credentials/245",
        "currentStatus": "Disputed",
        "statusReason": {
          "value": "Address is out of date.",
          "lang": "en"
        },
      },
      "issuer": "did:example:76e12ec712ebc6f1c221ebfeb1f",
      "issuanceDate": "2017-12-05T14:27:42Z",
      "proof": { ... }
    }
  }],
  "allowRetry": true
}
```

3. Unsuccessful result with a `problem-report`, disallowing retries. 

```json
{
  "accepted": false,
  "problems": [{
    "id": "6c1a1477-e452-4da7-b2db-65ad0b369d1a",
    "problemReport": {
      "type": "https://didcomm.org/notify/1.0/problem-report",
      "id": "7c9de639-c51c-4d60-ab95-103fa613c805",
      "pthid": "1e513ad4-48c9-444e-9e7e-5b8b45c5e325",
      "body": {
        "code": "e.p.trust.crypto.credential-proof-invalid",
        "comment": "Signature failed validation for credential {1}.",
        "args": [
          "http://example.com/credentials/123",
        ],
      }
  }],
  "allowRetry": false
}
```

TODO: change problem-report here, or remove them from the result altogether? Example of a hacker trying to brute-force disputes with unsigned credentials, in which case the problem report (trust.crypto) should just end the flow and not return disputes.

### Problem Reports {#problem-reports}

The following problem-report codes may be raised in the course of this protocol and are expected to be recognised and handled in addition to any general problem-reports. Implementers may also introduce their own application-specific problem-reports.

For guidance on problem-reports and a list of general codes see [problem reports](../resources/problem-reports).

| Code | Message | Description |
| :--- | :--- | :--- |
| `e.p.msg.iota.presentation.reject-offer` | [presentation-offer](#presentation-offer) | [Verifier](#roles) rejects a presentation offer for any reason, e.g. unrecognised type or untrusted issuer. |
| `e.p.msg.iota.presentation.reject-offer.invalid-type` | [presentation-offer](#presentation-offer) | [Verifier](#roles) rejects a presentation offer due to a `type` or `@context` being unsupported or otherwise invalid. |
| `e.p.msg.iota.presentation.reject-offer.invalid-issuer` | [presentation-offer](#presentation-offer) | [Verifier](#roles) rejects a presentation offer due to `issuer` being unrecognised, untrusted or otherwise invalid.  |
| `e.p.msg.iota.presentation.reject-offer.reject-require-signature` | [presentation-offer](#presentation-offer) | [Verifier](#roles) rejects a presentation offer due to being unable or unwilling to provide a signature for the following [presentation-request](#presentation-request) |
| `e.p.msg.iota.presentation.reject-request` | [presentation-request](#presentation-request) | [Holder](#roles) rejects a request for any reason. |
| `e.p.msg.iota.presentation.reject-request.invalid-type` | [presentation-request](#presentation-request) | [Holder](#roles) rejects a request due to a `type` or `@context` being unsupported or otherwise invalid. |
| `e.p.msg.iota.presentation.reject-request.invalid-issuer` | [presentation-request](#presentation-request) | [Holder](#roles) rejects a request due to a `trustedIssuer` being unsupported or otherwise invalid. |
| `e.p.msg.iota.presentation.reject-request.missing-signature` | [presentation-request](#presentation-request) | [Holder](#roles) rejects a request due to a missing signature from the [verifier](#roles). The [holder](#roles) may choose to blocklist [verifiers](#roles) that fail to sign requests. |
| `e.p.msg.iota.presentation.reject-presentation` | [presentation](#presentation) | [Verifier](#roles) rejects a presentation and abandons the protocol for any reason other than a disputed or otherwise invalid verifiable presentation, which should instead be communicated via [presentation-result](#presentation-result). |
| `e.p.msg.iota.presentation.reject-result` | [presentation-result](#presentation-result) | [Holder](#roles) rejects a result for any reason. |
| `e.p.msg.iota.presentation.reject-result.missing-signature` | [presentation-result](#presentation-result) | [Holder](#roles) rejects a result due to a missing signature requested from the [verifier](#roles). The [holder](#roles) may blocklist the [verifier](#roles) from future requests. |
| `e.p.msg.iota.presentation.reject-retry` | [presentation-result](#presentation-result) | [Holder](#roles) chooses not to retry the presentation flow and terminates the protocol. |

## Considerations

This section is non-normative.

- **Security**: implementors SHOULD transmit the presentation over an encrypted channel etc. (TODO mention/link to DIDComm encryption?)
- **Authentication**: it is RECOMMENDED to use either the authentication protocol (TODO link?) for once-off authentication, or sender-authenticated encryption (TODO link?) for continuous authentication of both parties in the DIDComm thread. Signatures (`proof` fields) SHOULD NOT be relied upon for this (TODO link?).
- **Authorisation**: establishing whether either party is allowed to request/offer presentations is an application-level concern.
- **Validation**: apart from verifying the presentation and credentials are signed by a trusted issuer, how credential subject matter fields are checked for disputes is out-of-scope.

## Unresolved Questions

- Is a `schema` field needed for the `presentation-offer` and `presentation-request` to identify the types of verifiable credentials and allow forward compatibility for different fields in the message? The E.g. a `SelectiveDisclosure` or ZKP message may only offer or request certain fields in the credential. Does this relate to the [`credentialSchema`](https://www.w3.org/TR/vc-data-model/#data-schemas) field in credentials?
- Are embedded problem-reports the right way to communicate problems with a presentation in [`presentation-result`](#presentation-result)? Can we come up with a more concise form? Are there relevant specifications?
- Identifiers (`id` field) are [optional in verifiable credentials](https://www.w3.org/TR/vc-data-model/#identifiers). The spec suggests content-addressed identifiers when the `id` is not available but their particulars are unclear as there is no spec referenced. This affects the `problems` reported in the [`presentation-result`](#presentation-result).
- We should RECOMMENDED the `id` of a verifiable credential being a UUID (what version?) in issuance. Needs to be a URI https://www.w3.org/TR/vc-data-model/#identifiers, do UUIDs qualify?
- Should we specifically list non-functional requirements e.g in a Goals / Non-Goals section.
- Use `schemas` to negotiate generic form entries as a self-signed credential? E.g. could ask for username, preferred language, comments, any generic information not signed/verified by a third-party issuer from a generic wallet? Similar to Presentation Exchange? https://identity.foundation/presentation-exchange/spec/v1.0.0/

## Related Work

- Aries Hyperledger: https://github.com/hyperledger/aries-rfcs/tree/main/features/0454-present-proof-v2
- Jolocom: https://jolocom.github.io/jolocom-sdk/1.0.0/guides/interaction_flows/#credential-verification
- Presentation Exchange: https://identity.foundation/presentation-exchange/spec/v1.0.0/

## Further Reading

- [Decentralized Identifiers (DIDs) 1.0](https://www.w3.org/TR/did-core/)
- [Verifiable Credentials Data Model 1.0](https://www.w3.org/TR/vc-data-model)
- [Verifiable Credentials Implementation Guidelines 1.0](https://w3c.github.io/vc-imp-guide/)

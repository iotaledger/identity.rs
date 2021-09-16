---
title: Signing
sidebar_label: Signing
---

# Presentation

- Version: 0.1
- Status: `IN-PROGRESS`
- Last Updated: 2021-09-16

## Overview

Allows a trusted-party to request the signing of an unsigned verifiable credential by an issuer.

### Relationships

This protocol may be embedded in the `issuance` protocol.

### Example Use-Cases

- A seperate department requests a signature by the the legal department of a company.
- A subsidiary requests the parent company to sign a credential.
- An IOT device generates an unsigned credential and requests a secure server to sign it.

### Roles
- Trusted-Party: trusted by the issuer to generate unsigned credentials asserting claims about one or more subjects.
- [Issuer](https://www.w3.org/TR/vc-data-model/#dfn-issuers): has the capability to cryptographically sign credentials.

### Interaction

<div style={{textAlign: 'center'}}>

![SigningDiagram](/img/didcomm/signing.drawio.svg)

</div>


## Messages

### 1. signing-request {#signing-request}

- Type: `didcomm:iota/signing/0.1/signing-request`
- Role: [trusted-party](#roles)

Request by a trusted-party for an issuer to sign a credential.

#### Structure
```json
{
  "unsignedCredential": Credential // REQUIRED
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| [`unsignedCredential`](https://www.w3.org/TR/vc-data-model/#credentials) | Unsigned [verifiable credential](https://www.w3.org/TR/vc-data-model/#credentials) requested to be signed by the [issuer](#roles).[^1] | ✔ |

[^1] The initial credential MUST NOT have a `proof` section.

#### Examples

1. Request to sign a bachelors degree.

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

### 2. signing-response {#signing-response}

- Type: `didcomm:iota/signing/0.1/signing-response`
- Role: [issuer](#roles)

Response from the issuer returning the signed credential back to the trusted-party.

#### Structure
```json
{
  "signedCredential": Credential // REQUIRED
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| [`signedCredential`](https://www.w3.org/TR/vc-data-model/#credentials) | Signed [verifiable credential](https://www.w3.org/TR/vc-data-model/#credentials) matching the [signing-request](#signing-request).[^2] | ✔ |

[^2] The [trusted-party](#roles) MUST validate the signature in the `proof` section and issue a problem-report if invalid. The [trusted-party](#roles) SHOULD also verify that the contents of the `signedCredential` sent back by the [issuer](#roles) are complete and unaltered from the [signing-request](#signing-request).

The [issuer](#roles) may request in turn that the credential be signed by a different issuer unknown to the [trusted-party](#roles), by repeating this protocol or through alternative means. In such a case, it is up to the initial [trusted-party](#roles) whether or not to accept the final signature if not signed by the initial [issuer](#roles) they requested.

#### Examples

1. Respond with a signed a bachelors degree.

```json
{
  "signedCredential": {
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
    "proof": {...}
  }
}
```

### 3. signing-acknowledgement {#signing-acknowledgement}

- Type: `didcomm:iota/signing/0.1/signing-acknowledgement`
- Role: [trusted-party](#roles)

Acknowledgement by the [trusted-party](#roles) that the credential was received and accepted. The [issuer](#roles) MAY revoke the credential if no acknowledegment is received.

#### Structure
```json
{
  "accepted": bool // REQUIRED
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `accepted` | Indicates that the `signedCredential` was received and validated by the [trusted-party](#roles).[^3] | ✔ |

[^3] `accepted` MUST be `true`. Invalid signatures or credentials SHOULD result in problem-reports by the [trusted-party](#roles).

#### Examples

1. Accept the credential.

```json
{
  "accepted": true
}
```
### Problem Reports

TODO

e.p.prot.iota.signing.reject-signing-request // issuer rejects request by the trusted-party due to invalid/malformed/incomplete credential, 
e.p.prot.iota.signing.reject-singing-response // the trusted-party rejects the returned credential due to being unsigned/altered, signed with an invalid key etc.

Shared problem reports?
unauthenticated / not-authenticated
unauthorised / not-authorised
unencrypted / not-encrypted // rejection due to security risks of continuing over an unencrypted channel?

## Considerations

This section is non-normative.

- **Security**: implementors SHOULD transmit credentials over an encrypted channel to prevent leaking sensitive information on subjects. (TODO mention/link to DIDComm encryption?)
- **Authentication**: it is RECOMMENDED to use sender-authenticated encryption for continuous authentication of both parties in the DIDComm thread. Anonymous encryption and/or once-off authentication may be insufficient.
- **Authorisation**: establishing whether a trusted-party is allowed to request signing is an application-level concern.
- **Validation**: apart from verifying the proof on the signed credential returned in the [signing-response](#signing-response), how the [issuer](#roles) validates the contents of a well-formed credential from a [trusted-party](#roles) and chooses whether or not to sign it is out-of-scope.

## Related Work

- Aries Hyperledger: https://github.com/hyperledger/aries-rfcs/tree/08653f21a489bf4717b54e4d7fd2d0bdfe6b4d1a/features/0327-crypto-service

## Further Reading

- [Verifiable Credentials Data Model 1.0](https://www.w3.org/TR/vc-data-model)

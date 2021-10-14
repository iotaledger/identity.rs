---
title: Post
sidebar_label: Post
---

# Post

- Version: 0.1
- Status: `IN-PROGRESS`
- Last Updated: 2021-10-14

## Overview

Allows the sending of a generic message.

### Relationships

### Example Use-Cases


### Roles


### Interaction

<div style={{textAlign: 'center'}}>

![SigningDiagram](/img/didcomm/signing.drawio.svg)

</div>


## Messages

### 1. signing-request {#signing-request}

- Type: `didcomm:iota/signing/0.1/signing-request`
- Role: [trusted-party](#roles)

Request by a [trusted-party](#roles) for an [issuer](#roles) to sign a credential.

To authenticate the [trusted-party](#roles), this SHOULD be sent using [sender authenticated encryption](https://identity.foundation/didcomm-messaging/spec/#sender-authenticated-encryption) established in a preceding [authentication](./authentication) protocol. For non-repudiation or auditing, the [issuer](#role) MAY enforce that the [signing-request](#signing-request) be a [signed DIDComm message](https://identity.foundation/didcomm-messaging/spec/#didcomm-signed-message).

#### Structure
```json
{
  "unsignedCredential": Credential // REQUIRED
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| [`unsignedCredential`](https://www.w3.org/TR/vc-data-model/#credentials) | | ✔ |

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

### Problem Reports {#problem-reports}


## Unresolved Questions

## Considerations

## Related Work

## Further Reading

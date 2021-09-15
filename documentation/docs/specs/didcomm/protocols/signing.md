---
title: Signing
sidebar_label: Signing
---

# Presentation

- Version: 0.1
- Status: `IN-PROGRESS`
- Last Updated: 2021-09-15

## Overview

Allows a trusted-party to request the signing of an unsigned verifiable credential by an issuer.

### Relationships

This protocol may be embedded in the `issuance` protocol.

### Example Use-Cases

- A seperate department requests a signature by the the legal department of a company.
- A subsidiary requests the sparent company to sign a credential.

### Roles
- Trusted-Party: A party that is trusted by the issuer to present valid unsigned credential.
- [Issuer](https://www.w3.org/TR/vc-data-model/#dfn-issuers): A party with the capability to cryptographically sign statements.

### Interaction

<div style={{textAlign: 'center'}}>

![SigningDiagram](/img/didcomm/signing.drawio.svg)

</div>


## Messages

### 1. signing-request {#signing-request}

- Type: `didcomm:iota/signing/0.1/signing-request`
- Role: [trusted-party](#roles)

TBD

#### Structure
```json
{
  "unsignedCredential": Unsigned VC// REQUIRED
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `unsignedCredential` | TBD | ✔ |


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


### Problem Reports

TBD

## Considerations

This section is non-normative.

TBD

## Related Work

TBD

## Further Reading

TBD

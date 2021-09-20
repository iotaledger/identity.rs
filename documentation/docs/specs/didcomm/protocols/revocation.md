---
title: Revocation
sidebar_label: Revocation
---

# Revocation

- Version: 0.1
- Status: `IN-PROGRESS`
- Last Updated: 2021-09-20

## Overview
TBD

### Relationships
TBD

### Example Use-Cases
TBD

### Roles
- Trusted-Party: trusted by the issuer to generate unsigned credentials asserting claims about one or more subjects.
- [Issuer](https://www.w3.org/TR/vc-data-model/#dfn-issuers): signer of the verifiable credential.

### Interaction

<div style={{textAlign: 'center'}}>

![RevocationDiagram](/img/didcomm/revocation.drawio.svg)

</div>


## Messages

### 1. revocation-request {#revocation-request}

- Type: `didcomm:iota/revocation/0.1/revocation-request`
- Role: [trusted-party](#roles)

TBD

#### Structure
```json
{
  TBD
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `TBD` | TBD | ✔ |

#### Examples

1. TBD:

```json
{
  TBD
}
```

### 2. revocation-response {#revocation-response}

- Type: `didcomm:iota/revocation/0.1/revocation-response`
- Role: [issuer](#roles)

TBD

#### Structure
```json
{
  TBD
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `TBD` | TBD | ✔ |

#### Examples

1. TBD:

```json
{
  TBD
}
```

### Problem Reports

TBD


## Considerations

This section is non-normative.

TBD

## Unresolved Questions
non-repudiation: should the trusted party be able to prove that the revoker claimed to have revoked the credential by making him include a signature in the `revocation-response`
## Related Work

TBD

## Further Reading

TBD

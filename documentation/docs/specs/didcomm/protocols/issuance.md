---
title: Issuance
sidebar_label: Issuance
---

# Presentation

- Version: 0.1
- Status: `IN-PROGRESS`
- Last Updated: 2021-09-16

## Overview

TBD

### Relationships
TBD

### Example Use-Cases

- TBD

### Roles
- [Holder](https://www.w3.org/TR/vc-data-model/#dfn-holders): entity that stores one or more verifiable credentials. A holder is usually but 
- [Issuer](https://www.w3.org/TR/vc-data-model/#dfn-issuers): TBD.

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
  "types": [string],          // REQUIRED
  "trustedIssuers": [string]  // OPTIONAL
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
| `TBD` | TBD | ✔ |

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
| `TBD` | TBD | ✔ |

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
| `TBD` | TBD | ✔ |

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

## Related Work

TBD

## Further Reading

TBD

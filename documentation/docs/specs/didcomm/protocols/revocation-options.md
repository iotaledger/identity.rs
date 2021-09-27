---
title: Revocation Options
sidebar_label: Revocation Options
---

# Revocation Options

- Version: 0.1
- Status: `IN-PROGRESS`
- Last Updated: 2021-09-27

## Overview
Allows discovery of available [`RevocationInfo`](./revocation#RevocationInfo) types for use with the [revocation](./revocation) protocol.

### Relationships

- [revocation](./revocation): this protocol is used to discover the `revocationType` options available to a [trusted-party](#roles) for use in a [revocation-request](./revocation#revocation-request).

### Roles
- Trusted-Party: has the authority to request the revocation of verifiable credentials.
- Revoker: able to revoke the key used to sign a verifiable credential.

### Interaction

<div style={{textAlign: 'center'}}>

![RevocationOptionsDiagram](/img/didcomm/revocation-options.drawio.svg)

</div>


## Messages
### 1. revocation-options-request {#revocation-options-request}

- Type: `didcomm:iota/revocation-options/0.1/revocation-options-request`
- Role: [trusted-party](#roles)

Empty messsage requesting all available [`RevocationInfo`](./revocation#RevocationInfo) types.

#### Structure
```json
{}
```

### 2. revocation-options {#revocation-options}

- Type: `didcomm:iota/revocation-options/0.1/revocation-options`
- Role: [revoker](#roles)

Response including all available [RevocationInfo](./revocation#RevocationInfo) types supported by the [revoker](#roles).

#### Structure
```json
{
  "revocationInfoTypes": [string], // REQUIRED
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `revocationInfoTypes` | List of supported [RevocationInfo](./revocation#RevocationInfo) types. | ✔ |

#### Examples

1. Response including multiple [RevocationInfo](./revocation#RevocationInfo) types:

```json
{
  "revocationInfoTypes": ["KeyRevocation2021", "CredentialRevocation2021", "CredentialStatusRevocation2021"]
}
```

2. Response including a single [RevocationInfo](./revocation#RevocationInfo) type:

```json
{
  "revocationInfoTypes": ["CredentialRevocation2021"]
}
```

### Problem Reports

TBD


## Considerations

This section is non-normative.

TBD

## Unresolved Questions
- Should revocation-options include the credential status sub-types for `CredentialStatusRevocation2021`?

## Related Work

TBD

## Further Reading

TBD

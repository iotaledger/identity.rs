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
- Trusted-Party: requests supported methods of revocation.
- Revoker: offers supported methods of revocation.

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
| `revocationInfoTypes` | List of supported [RevocationInfo](./revocation#RevocationInfo) types.[^1] | ✔ |

[^1] The actual 

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

For gerneral guidance see [problem reports](../resources/problem-reports).

Custom error messages for problem-reports that are expected in the course of this protocol. Non-exhaustive, just a normative list of errors that are expected to be thrown.
- e.p.prot.iota.revocation-options.reject-revocation-options-request
- e.p.prot.iota.revocation-options.reject-request

## Considerations

This section is non-normative.

- Privacy: similar to [discover features](https://github.com/decentralized-identity/didcomm-messaging/blob/9039564e143380a0085a788b6dfd20e63873b9ca/docs/spec-files/feature_discovery.md), this protocol could be used to fingerprint a server partially or reveal its capabilities. If privacy is a concern, implementors should take care to accept requests only from parties authorised to perform [revocation](./revocation) or return a subset/superset of its actual supported options.
- Authorisation: TODO
- Authentication: TODO

## Unresolved Questions
- Should revocation-options include the credential status sub-types for `CredentialStatusRevocation2021`?

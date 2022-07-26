---
title: Revocation Options
sidebar_label: Revocation Options
---

:::info

The IOTA DIDComm Specification is in the RFC phase and may undergo changes. Suggestions are welcome at [GitHub #464](https://github.com/iotaledger/identity.rs/discussions/464).

:::

- Version: 0.1
- Status: `IN-PROGRESS`
- Last Updated: 2021-10-29

## Overview
Allows discovery of available [`RevocationInfo`](./revocation#RevocationInfo) types for use with the [revocation](./revocation) protocol.

### Relationships

- [revocation](./revocation): this protocol is used to discover the `revocationInfoType` options available to a [trusted-party](#roles) for use in a [revocation-request](./revocation#revocation-request).

### Roles
- Trusted-Party: requests supported methods of revocation.
- Revoker: offers supported methods of revocation.

### Interaction

![RevocationOptionsDiagram](/img/didcomm/revocation-options.drawio.svg)

<div style={{textAlign: 'center'}}>

<sub>For guidance on diagrams see the <a href="../overview#diagrams">corresponding section in the overview</a>.</sub>

</div>


## Messages
### 1. revocation-options-request {#revocation-options-request}

- Type: `iota/revocation-options/0.1/revocation-options-request`
- Role: [trusted-party](#roles)

Empty message requesting all available [`RevocationInfo`](./revocation#RevocationInfo) types.

#### Structure
```json
{}
```

### 2. revocation-options {#revocation-options}

- Type: `iota/revocation-options/0.1/revocation-options`
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

[^1] The actual list of supported types may be vague or exact depending on how much the [revoker](#roles) trusts the requester. The supported types may also differ per requester.

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

### Problem Reports {#problem-reports}

The following problem-report codes may be raised in the course of this protocol and are expected to be recognised and handled in addition to any general problem-reports. Implementers may also introduce their own application-specific problem-reports.

For guidance on problem-reports and a list of general codes see [problem reports](../resources/problem-reports).

| Code | Message | Description |
| :--- | :--- | :--- |
| `e.p.msg.iota.revocation-options.reject-request` | [revocation-options](#revocation-options) | The [revoker](#roles) rejects a request for any reason. |

## Considerations

This section is non-normative.

- **Privacy**: similar to [discover features](https://github.com/decentralized-identity/didcomm-messaging/blob/9039564e143380a0085a788b6dfd20e63873b9ca/docs/spec-files/feature_discovery.md), this protocol could be used to fingerprint a party partially or reveal its capabilities. If privacy is a concern, implementors should take care to accept requests only from parties authorized to perform [revocation](./revocation) or return a subset/superset of its actual supported options.

## Unresolved Questions

- Should revocation-options include the credential status sub-types for `CredentialStatusRevocation2021`?

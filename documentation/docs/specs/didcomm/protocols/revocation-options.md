---
title: Revocation Options
sidebar_label: Revocation Options
---

# Revocation Options

- Version: 0.1
- Status: `IN-PROGRESS`
- Last Updated: 2021-09-21

## Overview
Allows negotiation of available modes revocation.

### Relationships

- [revocation](./revocation): this protocol is used to discover the `revocationType` options available to a [trusted-party](#roles) for use in a [revocation request message](./revocation#revocation-request) in the [revocation](./revocation) protocol.

### Example Use-Cases
TBD

### Roles
TBD

### Interaction

<div style={{textAlign: 'center'}}>

![RevocationOptionsDiagram](/img/didcomm/revocation-options.drawio.svg)

</div>


## Messages
### 1. revocation-request {#revocation-request}

- Type: `didcomm:iota/revocation/0.1/revocation-request`
- Role: [trusted-party](#roles)

TBD

#### Structure
```json

```

| Field | Description | Required |
| :--- | :--- | :--- |
| `type` | TBD | ✔ |

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
TBD

## Related Work

TBD

## Further Reading

TBD

---
title: Revocation
sidebar_label: Revocation
---

# Revocation

- Version: 0.1
- Status: `IN-PROGRESS`
- Last Updated: 2021-09-21

## Overview
Allows to request revocation of an issued credential, either by the holder or a trusted-party. If the revoker is unable to revoke the credential themselves, they may delegate the revocation to a different issuer, in which case they take on the role of trusted-party in their request.

### Relationships
This protocol may rely on a `revocationType` negotiated in the [revocation-options](./revocation-options) protocol.
<!-- [revocation-options](./revocation-o) protocol. -->

### Example Use-Cases
- A member of an organisation asks the organisation to revoke their membership
- A subsidiary of a company asks central to revoke a credential concerning one of their customers. 

### Roles
- Trusted-Party: has the authority to request the revocation of verifiable credentials. May also be the holder of the credential but not necessarily.
- Revoker: able to revoke the key used to sign a verifiable credential. May also be the [issuer](https://www.w3.org/TR/vc-data-model/#dfn-issuers) of the credential but not necessarily.

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
  "revocationType": string,         // REQUIRED
  "revocationInfo": RevocationInfo, // REQUIRED
  "signature": Proof,               // OPTIONAL
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `revocationType` | TBD | ✔ |
| `revocationInfo` | TBD [RevocationInfo](#RevocationInfo) | ✔ |
| `signature` | TBD | ✖ |

#### Examples

1. TBD:

```json
{
  TBD
}
```

### 2. revocation-response {#revocation-response}

- Type: `didcomm:iota/revocation/0.1/revocation-response`
- Role: [revoker](#roles)

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

### RevocationInfo {#RevocationInfo}

The `RevocationInfo` object contains the information necessary for a [revoker](#roles) to revoke a verifiable credential. For instance, this may include the `id` field of the credential, in which case a [revoker](#roles) must maintain a map to the signing key used for each credential to revoke them. It could also be the identifier for the signing key itself on the DID document of the issuer. Implementors are free to construct their own `RevocationInfo` types as different singing keys may require different information for revocation. For example, revoking a `MerkleKeyCollection2021` requires both the key identifier and its index in the collection.

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

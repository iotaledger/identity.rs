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

TODO: revocation list https://w3c-ccg.github.io/vc-status-rl-2020/

### Relationships
This protocol may rely on a `revocationType` negotiated in the [revocation-options](./revocation-options) protocol.

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

Sent by the [trusted-party](#roles) or holder to request revocation of an issued verifiable credential. This message conveys which credential should be revoked and which method should be used. The message may also include a signature so the revoker has proof the request as issued. 

#### Structure
```json
{
  "revocationInfoType": string,     // REQUIRED
  "revocationInfo": RevocationInfo, // REQUIRED
  "signature": Proof,               // OPTIONAL
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `revocationInfoType` | The type of [RevocationInfo](#RevocationInfo). | ✔ |
| [`revocationInfo`](#RevocationInfo) | Contains information sufficient to identity which credential should be revoked. See [`revocationInfo`](#RevocationInfo). | ✔ |
| `signature` | [Proof](https://w3c-ccg.github.io/ld-proofs/) with the signature of the [trusted-party](#roles) on the [revocation-request message](#revocation-request).[^1] | ✖ |

[^1] The `signature` allows for non-repudiation of the request to third-parties. A [revoker](#roles) MAY choose to reject [revocation-requests](#revocation-request) that do not include a `signature`.

#### Examples

1. Request to revoke a credential with the specified identifier:

```json
{
  "revocationInfoType": "CredentialIdRevocation",
  "revocationInfo": {
    "credentialId": "0495e938-3cb7-4228-bb73-c642ec6390c8"
  },
}
```

2. Request to revoke one or more credentials signed by a specific [Ed25519 verification method](https://w3c-ccg.github.io/lds-ed25519-2020/#verification-method), including a signature for non-repudiation:

```json
{
  "revocationInfoType": "Ed25519KeyRevocation",
  "revocationInfo": {
    "key": "did:example:76e12ec712ebc6f1c221ebfeb1f#keys-2"
  },
  "signature": {...},
}
```

### 2. revocation-response {#revocation-response}

- Type: `didcomm:iota/revocation/0.1/revocation-response`
- Role: [revoker](#roles)

Sent by the [revoker](#roles) as soon as the credentials is revoked.

#### Structure
```json
{
  TODO
  revocationStatus: "revoked", "pending-revocation", "unrevoked"
  revoked: true/false
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `TBD` | TBD | ✔ |

The trusted party may check if the crednetial is actually revoked after this message is received.

#### Examples

1. TBD:

```json
{
  TBD
}
```

### RevocationInfo {#RevocationInfo}

The `RevocationInfo` object contains the information necessary for a [revoker](#roles) to revoke a verifiable credential. For instance, this may include the `id` field of the credential, in which case a [revoker](#roles) must maintain a map to the signing key used for each credential to revoke them. It could also be the identifier for the signing key itself on the DID document of the issuer. Implementors are free to construct their own `RevocationInfo` types as different singing keys may require different information for revocation. For example, revoking a `MerkleKeyCollection2021` requires both the key identifier and its index in the collection.

Ed25519 signing keys
{
  "key": "did:iota:123789#keys-2"
}

MerkleKeyCollection signing key
{
  "key": "did:iota:123789#keys-3"
  "index": 6
}

Credential ID
{
  "credentialId": "3264276-fdsG325-6436437"
}

### Problem Reports

TBD


## Considerations

This section is non-normative.

The revoker needs to check if the credential may actually be revoked and if the trusted party actually has the authority to request the revocation.

## Unresolved Questions
non-repudiation: should the trusted party be able to prove that the revoker claimed to have revoked the credential by making him include a signature in the `revocation-response`

TODO: Should this protocol optionally embed the presentation protocol to present relevant information?
## Related Work

TBD

## Further Reading

TBD

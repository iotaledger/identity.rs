---
title: Revocation
sidebar_label: Revocation
---

# Revocation

- Version: 0.1
- Status: `IN-PROGRESS`
- Last Updated: 2021-09-22

## Overview
Allows to request revocation of an issued [verifiable credential](https://www.w3.org/TR/vc-data-model/), either by the holder or a trusted-party. If the revoker is unable to revoke the credential themselves, they may delegate the revocation to a different issuer, in which case they take on the role of trusted-party in their request.

Note that the exact method of revocation is unspecified. The typical procedure is to revoke the verification method with the key used to sign the credential, causing subsequent verification attempts of the credential revocation to fail. However, implementors may instead choose to follow an alternative procedure such as [RevocationList2020](https://w3c-ccgithub.io/vc-status-rl-2020/).

### Relationships
- [revocation-options](./revocation-options): this may be preceded by the the [revocation-options](./revocation-options) protocol for the [trusted-party](#roles) to discover the available [`RevocationInfo` types](#RevocationInfo).

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
  "revocationInfoType": "CredentialRevocation2021",
  "revocationInfo": {
    "credentialId": "0495e938-3cb7-4228-bb73-c642ec6390c8"
  },
}
```

2. Request to revoke all credentials signed by a specific [verification method](https://w3c-ccg.github.io/lds-ed25519-2020/#verification-method) identified by `#keys2`, including a signature for non-repudiation:

```json
{
  "revocationInfoType": "KeyRevocation2021",
  "revocationInfo": {
    "key": "did:example:76e12ec712ebc6f1c221ebfeb1f#keys-2"
  },
  "signature": {...},
}
```

### 2. revocation-response {#revocation-response}

- Type: `didcomm:iota/revocation/0.1/revocation-response`
- Role: [revoker](#roles)

Sent by the [revoker](#roles) as soon as the revocation is performed. It idicates in what state the revocation is.

#### Structure
```json
{
  "status": "revoked" | "pending",
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `status` | Current status of the revocation, either `revoked` or `pending`.[^1] | ✔ |

[^1] The status should be `revoked` if the credential or signing key is confirmed to be revoked, and `pending` if the revocation has been accepted but not yet performed or committed. For instance, a revocation that updates a DID document may require waiting for the update transaction to be confirmed, or it could be queued for a batch update. If the [revoker](#roles) is unable to perform the revocation or rejects the request for any reason, they MUST instead respond with a [`problem-report`](#problem-reports). Care should be taken not to reveal which credentials are under control of the revoker to prevent privacy-revealing brute-force attacks.

The [trusted-party](#roles) SHOULD verify that the credential is actually revoked after this message is received. The [revocation protocol](#Revocation) MAY be polled by a [trusted-party](#roles) by re-sending the same request to confirm revocation if the status of `pending` is received. In the case of a public ledger, however, the [trusted-party](#roles) can query the public state of the verification method themselves to confirm revocation.

#### Examples

1. Response to a [revocation-request](#revocation-request) where the [revoker](#roles) confirms revocation directly:

```json
{
  "status": "revoked",
}
```

2. Response to a [revocation-request](#revocation-request) where the [revoker](#roles) confirms the revocation was scheduled, but can only be confirmed at a later point:

```json
{
  "status": "pending",
}
```


### Problem Reports {#problem-reports}

For gerneral guidance see [problem reports](../resources/problem-reports).

Custom error messages for problem-reports that are expected in the course of this protocol. Non-exhaustive, just a normative list of errors that are expected to be thrown.
- e.p.prot.iota.revocation.reject-revocation

Also problem reports from embedded protocols can be thrown.

## RevocationInfo {#RevocationInfo}

The `RevocationInfo` object contains the information necessary for a [revoker](#roles) to revoke a verifiable credential. For instance, this may include the `id` field of the credential, in which case a [revoker](#roles) must maintain a map to the signing key used for each credential to revoke them. It could also be the identifier for the signing key itself on the DID document of the issuer. Implementors are free to construct their own `RevocationInfo` types as different singing keys may require different information for revocation. For example, revoking a `MerkleKeyCollection2021` requires both the key identifier and its index in the collection.

Implementors MUST adhere to at least one of the types below, either [KeyRevocation2021](#KeyRevocation2021) or [CredentialRevocation2021]. Implementors MAY define additional types as-needed.

### KeyRevocation2021
- Type: `KeyRevocation2021`

Allows a particular cryptographic public key linked as a verification method to be specified for revocation. This may reference any singular verification method such as [Ed25519VerificationKey2018](https://www.w3.org/TR/did-spec-registries/#ed25519verificationkey2018) or [RsaVerificationKey2018](https://www.w3.org/TR/did-spec-registries/#rsaverificationkey2018). Verificaiton methods that hold multiple keys as a collection, such as [MerkleKeyCollection2021](../../did/merkle_key_collection), may encode the index of the key to be revoked in the [query](https://www.w3.org/TR/did-core/#dfn-did-queries) of the [DIDUrl](https://www.w3.org/TR/did-core/#did-url-syntax).

See the [DID Spec Registry for more verification method types](https://www.w3.org/TR/did-spec-registries/#verification-method-types).

Note that revoking a verification method revokes all verifiable credentials signed with its key.

#### Structure

```json
{
  "key": DIDUrl, // REQUIRED
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `key` | String conforming to the [DIDUrl syntax](https://www.w3.org/TR/did-core/#did-url-syntax) identifying a [verification method](https://www.w3.org/TR/did-core/#verification-methods) to be revoked.[^1] | ✔ |

[^1] the [fragment](https://www.w3.org/TR/did-core/#dfn-did-fragments) MUST reference a valid verification method. The DID document referenced need not belong to the revoker necessarily, as they could forward or delegate the request to the actual owner or controller. The [query](https://www.w3.org/TR/did-core/#dfn-did-queries) MAY include extra information needed to identify the particular signing key, for example the index in a [MerkleKeyCollection2021](../../did/merkle_key_collection).

#### Example

1. Specify a single key or verification method to revoke:

```json
{
  "key": "did:example:76e12ec712ebc6f1c221ebfeb1f#keys-1"
}
```

2. Specify a particular key in a [MerkleKeyCollection2021](../../did/merkle_key_collection) to revoke:

```json
{
  "key": "did:example:76e12ec712ebc6f1c221ebfeb1f#keys-2?index=7"
}
```

### CredentialRevocation2021

- Type: `CredentialRevocation2021`

Allows to request the revocation of a verifiable credential by its identifier field. This implies that the revoker needs to keep track of the relevant method of revocation and additional information such as the verification method used to sign it to be able to revoke the credential. 

```json
{
  "credentialId": string,  // REQUIRED
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `credentialId` | TODO | ✔ |

#### Examples

1. Specify the identifier of the credential to revoke:

```json
{
  "credentialId": "1dd5bbc6-b0bc-4f82-94a9-c723e11075b5",
}
```

## Considerations

This section is non-normative.

The revoker needs to check if the credential may actually be revoked and if the trusted party actually has the authority to request the revocation.

## Unresolved Questions
non-repudiation: should the trusted party be able to prove that the revoker claimed to have revoked the credential by making him include a signature in the `revocation-response`

TODO: Should this protocol optionally embed the presentation protocol to present relevant information?
## Related Work

- Aries Hyperledger: https://github.com/hyperledger/aries-rfcs/blob/main/features/0183-revocation-notification/README.md

## Further Reading

- https://www.w3.org/TR/vc-data-model/
- https://hyperledger-indy.readthedocs.io/projects/hipe/en/latest/text/0011-cred-revocation/README.html


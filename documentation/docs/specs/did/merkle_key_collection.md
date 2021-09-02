---
title: Merkle Key Collection Specification
sidebar_label: Merkle Key Collection
---

Version 0.1-draft by Devin Turner, IOTA Foundation

## Abstract

This specification describes a protocol for representing large quantities of cryptographic keys as a single [Verification Method][DID-VMETHODS] inside a [DID Document][DID-DOCUMENT].

## Introduction

The Merkle Key Collection Specification provides authentication and integrity protection to DID Documents without a dependency on any particular Distributed Ledger Technology (DLT).

## Data Structures

### Verification Method

DID Documents **MAY** include Verifications Methods as defined in the [DID Core Specification][DID-VMETHODS]. The following additional rules are defined by this specification:

- The Verification Method `type` **MUST** be `MerkleKeyCollection2021`.

- The Verification Method **MUST** contain a `publicKeyBase58` property. The value is encoded as a [Base58-BTC][BASE58-BTC] string, see [Public Key](#Public-Key) for the format of the encoded data.

### Merkle Tree

A Merkle Key Collection utilitizes a binary [Merkle Tree][MERKLE-TREE] for efficient key storage inside a DID Document. The hashing algorithm is dynamic; referred to as `H`.

The input to the Merkle Tree is a list of entries which are hashed to create the leaves of the tree. The output is a byte vector where the length is determined by the hashing algorithm `H`. Given a list of `n` inputs, `D[n] = {d(0), d(1), ..., d(n-1)}`, the Merkle Tree Root is defined as follows:

:::info
Note that `||` denotes concatenation and `D[k1:k2]` denotes the list `{d(k1), d(k1+1), ..., d(k2-1)}` of length `k2 - k1`.
:::

The Merkle Tree Root of an empty list is the hash of an empty string:

  ```
  MTR({}) = H()
  ```

The Merkle Tree Root of a list with one entry is:

  ```
  MTR({d(0)}) = H(0x00 || d(0))
  ```

The Merkle Tree Root of an n-element list `D[n]` is defined recursively as:

  ```
  k = ... // largest power of two less than `n`.
  MTR(D[n]) = H(0x01 || MTR(D[0:k]) || MTR(D[k:n]))
  ```

Note that the hash calculations for leaves and branches are distinct, this is to provide resistance to second-preimage attacks.

### Inclusion Proof

An inclusion proof allows proving the existence of an entry in the [Merkle Tree][MERKLE-TREE] without requiring the disclosure of the complete tree structure.

An inclusion proof for a leaf node in a given tree is the shortest list of sibling nodes required to compute the root hash. If the root hash computed from the inclusion proof matches a root that is known to be valid, then the inclusion proof shows that the leaf node exists in that tree.

Given a list of `n` inputs, `D[n] = {d(0), d(1), ..., d(n-1)}`, the inclusion proof for the `(m+1)`th input `d(m)`, `0 <= m < n`, is defined as follows:

:::info
Note that `:` denotes list concatenation and `D[k1:k2]` denotes the list `{d(k1), d(k1+1), ..., d(k2-1)}` of length `k2 - k1`.
:::

The inclusion proof for the only leaf node in a one-element tree `D[1] = {d(0)}` is empty:

  ```
  PATH(0, {d(0)}) = {}
  ```

The inclusion proof for the `(m+1)`th element `d(m)` in a list of `n > m` elements is defined recursively as:

  ```
  k = ... // largest power of two less than `n`.
  PATH(m, D[n]) = PATH(m, D[0:k]) : MTR(D[k:n]) where m < k
  PATH(m, D[n]) = PATH(m - k, D[k:n]) : MTR(D[0:k]) where m >= k
  ```

## Key Revocation

There may be situations where an individual entry in the key collection must be revoked, possibly due to security concerns. Key revocation is accomplished by storing a bitmap of the revoked keys in the `revocation` field of the [Verification Method][DID-VMETHODS]. The index of the target leaf node is computed from the inclusion proof nodes and the value is asserted to not be included in the revocation bitmap.

The current version of this specification uses [Roaring Bitmaps][Roaring-Bitmaps] as the format of storing key revocation flags.

:::info
The exact details of updating a [DID Document][DID-DOCUMENT] verification method are beyond the scope of this specification and implementers should refer to the applicable [DID Method][DID-METHODS] Specification for guidance.
:::

## Signature Algorithm

The following algorithm specifies how to create a `MerkleKeySignature2021` digital proof. An `input-document`, `key-collection`, and `target-index` are required inputs.

  1. Generate a Merkle Tree from the public keys of `key-collection`, return the result as `merkle-tree`. See [Merkle Tree](#Merkle-Tree) for details on generating this value.

  2. Generate an Inclusion Proof showing the existence of `key-collection[target-index]` in the Merkle Tree, return the result as `merkle-proof`. See [Inclusion Proof](#Inclusion-Proof) for details on generating this value.

  3. Generate a canonicalized version of `input-document` using [JSON Canonicalization Scheme (JCS)][JCS-RFC], return the result as `message`.

  4. Hash `message` using [SHA-256][SHA-256], return the result as `digest`.

  5. Extract the private key from `key-collection[target-index]`, return the result as `private-key`.

  6. Sign `digest` using the signature algorithm with `private-key`, return the result as `signature`.

  7. Encode `signature` as a [Base58-BTC][BASE58-BTC] string, return the result as `encoded-signature`.

  8. Encode `merkle-proof` as a [Base58-BTC][BASE58-BTC] string in the following form, return the result as `encoded-proof`.

  ```
  [ U32(TOTAL-NODES) | [ [ U8(NODE-TAG) | BYTES(NODE-HASH) ], ... ] ]
  ```

  9. Set the output as `proof.signatureValue` inside `input-document` with the following form.

  ```
  [ encoded-proof | ASCII(.) | encoded-signature ]
  ```

#### Example Proof

```json
{
  "type": "MerkleKeySignature2021",
  "signatureValue": "13nQykUY6HqKLXqPaEa2FGCTrcRBSWfS5LbToBKMnwf98ifM7VLTQCDnkHsdJcZ7Tkc5jArZvu5bk2nS2wSvekMiH4EmUDzRRvExKoDT8sGP9EwHfnvdYG9HPrZqCa3pd9HSLbBhQjQy56TayvZsPkTsrmPSdC87uBbsz1dsM9iee28TdLcFRRfw6Twcv5vv4CfnEFhM86eD9KvxD5KhefZnXEnML1H7HXMqhiTT7xbQAg9fcMarrX8RdS4eb7f7erArY67ok4eiMriJq1KAAHwWcPaxMHH2YxkS6hUdKMc5NiukxgHrVzXbZUoyd8TakoUHYStFcAT2HvxDdst4FuJH8WVQcpkTeBYvo68AtbCKYZUGKxwHKi6UqCoqRamMZTbsTSdQsFqRvVcvjn9Ce1sJ7toCnW3Af9FXMNNkkjxRXtghVTdUWpwsD3K3MLKq1dJfMNb2.2Hfnxou3DyJXMF9XqVdViJfver743buqdCPNX6a8tfzw9qzP3RtRexHQeMDwFrNCq1FfTB6dyHcEGebrdPpuTT2MoaMvEELhMYuFGBubfxhT4yCVwW4vxWDJZXjF97NqCSJJ",
  "verificationMethod": "did:example:123#key-collection",
}
```

## Verification Algorithm

The following algorithm species how to check the authenticity of a DID Document by verifying a `MerkleKeySignature2021` digital proof. An `input-document` and `public-key` are required inputs.

  1. Ensure `input-document` has a `proof` property.

  2. Ensure the `proof.type` property of `input-document` is `MerkleKeySignature2021`.

  3. Extract the verification method by dereferencing `proof.verificationMethod`, return the result as `method`.

  4. Ensure the `type` property of `method` is `MerkleKeyCollection2021`.

  5. Extract and verify the signature algorithm from `method.publicKeyBase58`, return the result as `signature-algorithm` (See [Key Format](#Key-Format) for more info).

  6. Extract and verify the digest algorithm from `method.publicKeyBase58`, return the result as `digest-algorithm` (See [Key Format](#Key-Format) for more info).

  7. Extract the Merkle Tree Root from `method.publicKeyBase58`, return the result as `merkle-root` (See [Key Format](#Key-Format) for more info).

  8. Hash `public-key` with `digest-algorithm`, return the result as `target-hash`.

  9. Extract and decode the [Base58-BTC][BASE58-BTC] inclusion proof and signature from `proof.signatureValue`, return the results as `merkle-proof` and `signature`.

  10. Verify the root hash composed from `target-hash` and `merkle-proof` is equal to `merkle-root`.

  11. Generate a canonicalized version of `input-document` using [JSON Canonicalization Scheme (JCS)][JCS-RFC], return the result as `message`.

  12. Verify the authenticity of `message` using `signature-algorithm` with `signature` and `public-key`.

  13. If all of the checks above pass, return `true`, otherwise return `false`.

## Supported Algorithms

### Supported Signature Algorithms

| Tag    | Name    |
| - | - |
| `0x00` | Ed25519 |

### Supported Digest Algorithms

| Tag    | Name        |
| - | - |
| `0x00` | Sha256      |
| `0x01` | Blake2b-256 |

## Serialization Format

### Public Key

The Merkle Root hash and algorithm identifiers are encoded in the following format:

```
U8(SIGNATURE-TAG) || U8(DIGEST-TAG) || BYTES(MERKLE-ROOT)
```

### Sample Rust source code

```rust
fn encode_public_key(signature: u8, digest: u8, root: &[u8]) -> Vec<u8> {
  let mut encoded: Vec<u8> = Vec::with_capacity(1 + 1 + root.len());
  encoded.push(signature);
  encoded.push(digest);
  encoded.extend_from_slice(root);
  encoded
}
```

## Examples

### Verification Method

```json
{
  "id": "did:example:123#key-collection",
  "controller": "did:example:123",
  "type": "MerkleKeyCollection2021",
  "publicKeyBase58": "11HxitpBaDhvJn8nwddKB1v7Csx1GeAnn4824QQPvD4oYY"
}
```

### DID Document with Verification Method

```json
{
  "id": "did:example:123",
  "verificationMethod": [
    {
      "id": "did:example:123#key-collection",
      "controller": "did:example:123",
      "type": "MerkleKeyCollection2021",
      "publicKeyBase58": "11HxitpBaDhvJn8nwddKB1v7Csx1GeAnn4824QQPvD4oYY"
    }
  ],
  "authentication": [
    "did:example:123#key-collection"
  ]
}
```

## Security Considerations

TODO

[//]: # (sources)

[MERKLE-TREE]: https://en.wikipedia.org/wiki/Merkle_tree
[DID-DOCUMENT]: https://www.w3.org/TR/did-core/#dfn-did-documents
[DID-METHODS]: https://w3c.github.io/did-core/#dfn-did-methods
[DID-VMETHODS]: https://www.w3.org/TR/did-core/#verification-methods
[BASE58-BTC]: https://tools.ietf.org/id/draft-msporny-base58-01.html
[SHA-256]: https://en.wikipedia.org/wiki/SHA-2
[JCS-RFC]: https://tools.ietf.org/html/rfc8785
[Roaring-Bitmaps]: https://roaringbitmap.org/
<!-- [Roaring-Bitmaps-Format]: https://github.com/RoaringBitmap/RoaringFormatSpec/ -->

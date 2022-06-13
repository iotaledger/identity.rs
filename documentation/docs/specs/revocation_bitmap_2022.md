---
title: RevocationBitmap2022 Specification
sidebar_label: Revocation Bitmap Specification
description: The specification for the embedded revocation bitmap.
image: /img/Identity_icon.png
keywords:
  - DID
  - specs
  - specifications
  - revocation
  - bitmap
---

# RevocationBitmap2022

## Abstract

This specification describes an on-tangle mechanism for publishing the revocation status of Verifiable Credentials embedded in an issuer's DID document.

## Introduction

Revocation gives an issuer the possibility to invalidate a credential they issued before its natural expiration date. To that end, issuers embed an identifier in the credential's `credentialStatus` field and verifiers can lookup that identifier in a list to check whether the credential is valid. This document specifies a mechanism of embedding such a list, in form of a bitmap, in an issuer's DID document, where each bitmap index corresponds to a credential the issuer has issued. This mechanism is space-efficient, enables a verifier to check a credential's status in a privacy-preserving manner and without requiring additional lookups or external resources.

## Revocation Bitmap Concept

The revocation status of a verifiable credential is expressed as a binary value. The issuer keeps a bitmap of indices corresponding to verifiable credentials it has issued. If the binary value of the index in the bitmap is 1 (one), the verifiable credential is revoked, if it is 0 (zero) it is not revoked.

## Data Model

### Revocation Bitmap Status

For an issuer to enable verifiers to check the status of a verifiable credential, the [`credentialStatus`](https://www.w3.org/TR/vc-data-model/#status) property MUST be specified with the following properties:

- **id**: A [DID URL](https://www.w3.org/TR/did-core/#did-url-syntax) that can be resolved to a service of the issuer which contains the revocation bitmap;
- **type**: the string `"RevocationBitmap2022"`;
- **revocationBitmapIndex**: an unsigned, 32-bit integer expressed as a string, which is the index of the verifiable credential's revocation status in the issuer's revocation bitmap;

### Revocation Bitmap Service

In order to allow verfiers to check the status of a verifiable credential, the DID document of the issuer may contain a service that MUST have the following properties:

- **id**: a URL identifying the revocation bitmap;
- **type**: the string `"RevocationBitmap2022"`;
- **serviceEndpoint**: a `data` URL [[RFC2397](https://datatracker.ietf.org/doc/html/rfc2397)] embedding a revocation bitmap generated according to the [bitmap generation algorithm](#Bitmap-Generation-Algorithm). The `<mediatype>` MUST be `application/octet-stream` and the `base64` attribute MUST be set.

## Algorithms

The following algorithms describe how to generate and validate revocation bitmaps.

### Bitmap Generation Algorithm

The following process MUST be followed when producing a `RevocationBitmap2022`:

1. Let **bitmap** be a [_roaring bitmap_](https://roaringbitmap.org/) where each bit is initialized to 0 (zero).
2. For each revoked credential with an **identifier** not exceeding an unsigned, 32-bit integer, set the bit in **bitmap** at index **identifier** to 1.
3. Generate the **bitmap serialization** according to the [roaring bitmap serialization format](https://github.com/RoaringBitmap/RoaringFormatSpec/) using the **bitmap** as input.
4. Generate a **compressed bitmap** by using the ZLIB compression algorithm [[RFC 1950](https://datatracker.ietf.org/doc/html/rfc1950)] on the bitmap serialization and base64-encoding [[RFC4648](https://datatracker.ietf.org/doc/html/rfc4648)] the result.
5. Return the **compressed bitmap**.

### Bitmap Expansion Algorithm

The following process MUST be followed when expanding a compressed revocation list bitmap.

1. Let **compressed bitmap** be a compressed revocation bitmap generated using the above algorithm.
2. Generate an **uncompressed bitmap** by base64-decoding [[RFC4648](https://datatracker.ietf.org/doc/html/rfc4648)] the **compressed bitmap** and further decompressing using ZLIB [[RFC 1950](https://datatracker.ietf.org/doc/html/rfc1950)].
3. Generate the **bitmap** by deserializing the **uncompressed bitmap** according to the [roaring bitmap serialization format](https://github.com/RoaringBitmap/RoaringFormatSpec/).
4. Return the **bitmap**.

### Validation Algorithm

The following steps MUST be followed when checking whether a verifiable credential is revoked:

1. Let **credential** be a verifiable credential containing a `credentialStatus` whose `type` is `RevocationBitmap2022`.
2. Let **revocation bitmap uri** be the `id` field of the **credential**'s `credentialStatus`.
3. Verify all proofs associated with the **credential**. If a proof fails, return a validation error.
4. Resolve the **revocation bitmap uri** to a **revocation bitmap service**, and verify that its type is `RevocationBitmap2022`. Return an error otherwise.
5. Let **compressed bitmap** be the `<data>` part of the `serviceEndpoint` data URL [[RFC2397](https://datatracker.ietf.org/doc/html/rfc2397)] of **revocation bitmap service**.
6. Generate a **revocation bitmap** by applying the [bitmap expansion algorithm](#Bitmap-Expansion-Algorithm) to the **compressed bitmap**.
7. Let **revocation index** be the integer value of the `revocationBitmapIndex` property of the **credential**.
8. Let **revoked** be the value of the bit at index **revocation index** in the **revocation bitmap**.
9. Return `true` if **revoked** is 1, `false` otherwise.

## Example

An example of a verifiable credential with a `credentialStatus` of type `RevocationBitmap2022`.

```json
{
  "@context": "https://www.w3.org/2018/credentials/v1",
  "id": "https://example.edu/credentials/3732",
  "type": ["VerifiableCredential", "UniversityDegreeCredential"],
  "credentialSubject": {
    "id": "did:iota:B8DucnzULJ9E8cmaReYoePU2b7UKE9WKxyEVov8tQA7H",
    "GPA": "4.0",
    "degree": "Bachelor of Science and Arts",
    "name": "Alice"
  },
  "issuer": "did:iota:EvaQhPXXsJsGgxSXGhZGMCvTt63KuAFtaGThx6a5nSpw",
  "issuanceDate": "2022-06-13T08:04:36Z",
  "credentialStatus": {
    "id": "did:iota:EvaQhPXXsJsGgxSXGhZGMCvTt63KuAFtaGThx6a5nSpw#revocation",
    "type": "RevocationBitmap2022",
    "revocationBitmapIndex": "5"
  },
  "proof": {
    "type": "JcsEd25519Signature2020",
    "verificationMethod": "did:iota:EvaQhPXXsJsGgxSXGhZGMCvTt63KuAFtaGThx6a5nSpw#key-1",
    "signatureValue": "2eHdbDumMrer4pNVkaiYMqsVqVp2adq7bRcgTJZiw17Zeghk2ZT49YHwLwCCg35YKganBhxP6YSbzYoBK1AuCUv"
  }
}
```

and the corresponding issuer's DID document where credential `"5"` in the `#revocation` service is revoked:

```json
{
  "id": "did:iota:EvaQhPXXsJsGgxSXGhZGMCvTt63KuAFtaGThx6a5nSpw",
  "verificationMethod": [
    {
      "id": "did:iota:EvaQhPXXsJsGgxSXGhZGMCvTt63KuAFtaGThx6a5nSpw#key-1",
      "controller": "did:iota:EvaQhPXXsJsGgxSXGhZGMCvTt63KuAFtaGThx6a5nSpw",
      "type": "Ed25519VerificationKey2018",
      "publicKeyMultibase": "z3hgM9fNkhwgT5mECbj1HdKoFNZgpffwQYEV8WBVHphXq"
    }
  ],
  "capabilityInvocation": [
    {
      "id": "did:iota:EvaQhPXXsJsGgxSXGhZGMCvTt63KuAFtaGThx6a5nSpw#sign-0",
      "controller": "did:iota:EvaQhPXXsJsGgxSXGhZGMCvTt63KuAFtaGThx6a5nSpw",
      "type": "Ed25519VerificationKey2018",
      "publicKeyMultibase": "z83F6zbD3KqaxvQhqo25LvSXzoDdpZmp3EpPVonSVACwZ"
    }
  ],
  "service": [
    {
      "id": "did:iota:EvaQhPXXsJsGgxSXGhZGMCvTt63KuAFtaGThx6a5nSpw#revocation",
      "type": "RevocationBitmap2022",
      "serviceEndpoint": "data:application/octet-stream;base64,ZUp5ek1tQmdZR1NBQUFFZ1ptVUFBQWZPQUlF"
    }
  ]
}
```

## Test Vectors

This section provides test vectors to validate implementations against.

### Test Vector 1

The following data url decodes to a bitmap of length 0 where no index is revoked:

`"data:application/octet-stream;base64,ZUp5ek1tQUFBd0FES0FCcg=="`

### Test Vector 2

The following data url decodes to a bitmap of length 3 where indices `5`, `398`, and `67000` are revoked:

`"data:application/octet-stream;base64,ZUp5ek1tQmdZR0lBQVVZZ1pHQ1FBR0laSUdabDZHUGN3UW9BRXVvQjlB"`.

### Test Vector 3

The following data url decodes to a bitmap of length 16384 where all indices are revoked:

`"data:application/octet-stream;base64,ZUp6dHhERVJBQ0FNQkxESEFWS1lXZkN2Q3E0MmFESmtyMlNrM0ROckFLQ2RBQUFBQUFBQTMzbGhHZm9q"`

## Rationale

This section describes the rationale behind some of the design decisions of this specification.

### Compression

Knowing that messages published to the Tangle cannot exceed [32 KiB](https://github.com/iotaledger/tips/blob/main/tips/TIP-0006/tip-0006.md#message-validation) in size and that the greater the message the more it will cost, the use of compression was assessed.

Although the difference in overall size for the compressed DID document is not significant when adding a compressed bitmap, the size of the decompressed document increases significantly when the bitmap is not compressed. Taking into account that the computational effort to compress and decompress the revocation bitmap would be a cost that only verifiers have to deal with, and that every other user would benefit from a lower memory footprint, we opted for compressing the revocation bitmap.

### Privacy Considerations

Because the revocation bitmap is embedded in the DID document, and thus available without contacting the issuer directly, the issuer cannot correlate how a holder uses their credential. Because of that, it is not necessary for the bitmap to have a minimum size.

### Compressed Bitstring vs. Roaring Bitmap

Because of their space efficiency, a roaring bitmap is preferred for representing a bitmap in-memory. To avoid the dependency on roaring bitmap, we considered to use a compressed bitstring as the serialization format. However, serialization of such a bitstring is 2-3x slower compared to roaring's serialization format, which becomes an issue on resource-constrained devices (e.g. smartphones) or in web browsers.

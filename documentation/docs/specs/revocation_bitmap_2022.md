---
title: Revocation Bitmap
sidebar_label: Revocation Bitmap
description: The specification for the embedded revocation bitmap.
image: /img/Identity_icon.png
keywords:
  - DID
  - specs
  - specifications
  - revocation
  - bitmap
---

# Revocation Bitmap 2022

## Abstract

This specification describes a mechanism for publishing the revocation status of [Verifiable Credentials](../concepts/verifiable_credentials/overview.md) embedded in an issuer's DID document.

## Introduction

Revocation gives an issuer the capability to invalidate a credential they issued before its natural expiration date. To achieve this, issuers can embed an identifier in the `credentialStatus` field of a credential. Verifiers can then lookup that identifier in a separate list, to check whether the credential is still valid. This document specifies a mechanism of embedding such a list, in form of a bitmap, in the DID document of the issuer, where each bitmap index corresponds to a credential they have issued. This mechanism is space-efficient and enables a verifier to check a credential's status in a privacy-preserving manner and without requiring additional lookups or external resources.

## Revocation Bitmap Concept

The revocation status of a verifiable credential is expressed as a binary value. The issuer keeps a bitmap of indices corresponding to verifiable credentials they have issued. If the binary value of the index in the bitmap is 1 (one), the verifiable credential is revoked, if it is 0 (zero) it is not revoked.

## Data Model

### Revocation Bitmap Status

For an issuer to enable verifiers to check the status of a verifiable credential, the [`credentialStatus`](https://www.w3.org/TR/vc-data-model/#status) property MUST be specified with the following properties:

| Property | Description |
| :--- | :--- |
| `id` | The constraints on the `id` property are listed in the [Verifiable Credentials Data Model specification](https://www.w3.org/TR/vc-data-model/). The `id` MUST be a [DID URL](https://www.w3.org/TR/did-core/#did-url-syntax) that is the URL to a [Revocation Bitmap Service](#revocation-bitmap-service) in the DID Document of the issuer. It SHOULD include an `index` query set to the same value as `revocationBitmapIndex`, to uniquely identify the `credentialStatus`. If the `index` query is present, implementations SHOULD reject statuses where the `index` query's value does not match `revocationBitmapIndex`. |
| `type` | The `type` property MUST be `"RevocationBitmap2022"`. |
| `revocationBitmapIndex` | The `revocationBitmapIndex` property MUST be an unsigned, 32-bit integer expressed as a string. This is the index of the credential in the issuer's revocation bitmap. Each index SHOULD be unique among all credentials linking to the same [Revocation Bitmap Service](#revocation-bitmap-service). |

#### Example

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
    "id": "did:iota:EvaQhPXXsJsGgxSXGhZGMCvTt63KuAFtaGThx6a5nSpw?index=5#revocation",
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

### Revocation Bitmap Service

To allow verifiers to check the status of a [Revocation Bitmap Status](#revocation-bitmap-status), the DID document of the credential issuer MUST contain a [service](https://www.w3.org/TR/did-core/#services) with the following properties:

| Property | Description |
| :--- | :--- |
| `id` | The constraints on the `id` property are listed in the [DID Core service specification](https://www.w3.org/TR/did-core/#services). The `id` property MUST be a DID URL uniquely identifying the revocation bitmap. |
| `type` | The `type` property MUST be `"RevocationBitmap2022"`. |
| `serviceEndpoint` | The `serviceEndpoint` MUST be generated according to the [service endpoint generation algorithm](#service-endpoint-generation-algorithm). |

#### Example

An example of an issuer's DID document where credential `"5"` in the `#revocation` service is revoked:

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

## Algorithms

The following algorithms define how to generate, expand and validate revocation bitmaps.

### Service Endpoint Generation Algorithm

The following process MUST be followed when producing a `RevocationBitmap2022` to embed in a service endpoint:

1. Let **bitmap** be a [_roaring bitmap_](https://roaringbitmap.org/) where each bit is initialized to 0.
2. For each revoked credential with an **index** not exceeding an unsigned, 32-bit integer, set the corresponding bit in **bitmap** at **index** to 1.
3. Generate the **bitmap serialization** according to the [roaring bitmap serialization format](https://github.com/RoaringBitmap/RoaringFormatSpec/) using the **bitmap** as input.
4. Generate a **compressed bitmap** by using the ZLIB compression algorithm [[RFC 1950](https://datatracker.ietf.org/doc/html/rfc1950)] on the **bitmap serialization** and base64-encoding [[RFC4648](https://datatracker.ietf.org/doc/html/rfc4648)] the result.
5. Create the **service endpoint** by embedding the **compressed bitmap** in a data URL [[RFC2397](https://datatracker.ietf.org/doc/html/rfc2397)]. On the data url, the `<mediatype>` MUST be `application/octet-stream` and the `base64` attribute MUST be set.
6. Return the **service endpoint**.

### Service Endpoint Expansion Algorithm

The following process MUST be followed when expanding the endpoint from a service of type `RevocationBitmap2022`:

1. Let **service endpoint** be a data url generated using the [service endpoint generation algorithm](#service-endpoint-generation-algorithm).
2. The `<mediatype>` of the **service endpoint** MUST be `application/octet-stream` and the `base64` attribute MUST be set, return an error otherwise. Let **compressed bitmap** be the `<data>` part of the data url.
3. Generate an **uncompressed bitmap** by base64-decoding [[RFC4648](https://datatracker.ietf.org/doc/html/rfc4648)] the **compressed bitmap** and then decompressing the result using ZLIB [[RFC 1950](https://datatracker.ietf.org/doc/html/rfc1950)].
4. Generate the **bitmap** by deserializing the **uncompressed bitmap** according to the [roaring bitmap serialization format](https://github.com/RoaringBitmap/RoaringFormatSpec/).
5. Return the **bitmap**.

### Validation Algorithm

The following steps MUST be followed when checking whether a verifiable credential is revoked:

1. Let **credential** be a verifiable credential containing a `credentialStatus` whose `type` is `RevocationBitmap2022`.
2. Let **revocation bitmap URL** be the `id` field of the **credential**'s `credentialStatus`.
3. Resolve the **revocation bitmap URL** to a **revocation bitmap service** in the issuer's DID document, and verify that the service `type` is `RevocationBitmap2022`. Return an error otherwise.
4. Expand the endpoint of the **revocation bitmap service** into a **revocation bitmap** according to the [service endpoint expansion algorithm](#service-endpoint-expansion-algorithm).
5. Let **revocation index** be the integer value of the `revocationBitmapIndex` property contained in the `credentialStatus` of the **credential**.
6. Let **revoked** be the value of the bit at index **revocation index** in the **revocation bitmap**.
7. Return `true` if **revoked** is 1, `false` otherwise.

## Test Vectors

This section provides test vectors to validate implementations against.

### Test Vector 1

The following data URL decodes to a bitmap of length 0 where no index is revoked:

`"data:application/octet-stream;base64,ZUp5ek1tQUFBd0FES0FCcg=="`

### Test Vector 2

The following data URL decodes to a bitmap of length 3 where indices `5`, `398`, and `67000` are revoked:

`"data:application/octet-stream;base64,ZUp5ek1tQmdZR0lBQVVZZ1pHQ1FBR0laSUdabDZHUGN3UW9BRXVvQjlB"`.

### Test Vector 3

The following data URL decodes to a bitmap of length 16384 where all indices are revoked:

`"data:application/octet-stream;base64,ZUp6dHhERVJBQ0FNQkxESEFWS1lXZkN2Q3E0MmFESmtyMlNrM0ROckFLQ2RBQUFBQUFBQTMzbGhHZm9q"`

## Rationale

This section describes the rationale behind some of the design decisions of this specification.

### Compression and maximum size

Considering that messages published to the Tangle cannot exceed [32 KiB](https://github.com/iotaledger/tips/blob/main/tips/TIP-0006/tip-0006.md#message-validation) in size, and that larger messages have increased requirements, the use of compression was assessed.
The precise size of a serialized bitmap varies based on the number and distribution of revoked indices. When indices are revoked uniformly randomly, roughly 100,000 - 200,000 can be achieved in a DID Document with compression, and significantly more if consecutive indices are revoked.

ZLIB [[RFC 1950](https://datatracker.ietf.org/doc/html/rfc1950)] was chosen for having a free and open source software licence and being one of the most widely used compression schemes, which enhances the accessibility of this specification. Some other assessed algorithms produced only marginally better compression ratios but had far fewer existing implementations across different programming languages.

### Compressed Bitstring vs. Roaring Bitmap

Because of its space efficiency, a roaring bitmap is preferred for representing a bitmap in-memory. To avoid the dependency on roaring bitmap, we considered using a compressed bitstring as the serialization format. However, serialization of such a bitstring was 2-3x slower compared to roaring's serialization format, which becomes an issue on resource-constrained devices (e.g. smartphones) or in web browsers.

### Comparison to `RevocationList2020` and `StatusList2021`

The [RevocationList2020 specification](https://w3c-ccg.github.io/vc-status-rl-2020/) and [StatusList2021 specification](https://w3c-ccg.github.io/vc-status-list-2021/) both describe a similar revocation mechanism using a verifiable credential that contains a bitmap, similar to the `RevocationBitmap2022` approach. The credential is hosted outside of the DID document and the verifier thus needs to fetch it from an external resource, likely one controlled by the issuer. This has privacy implications as the issuer can track where a fetch request for the credential came from and potentially infer who the credential was verified by and for what purpose. The issuer can also potentially infer which credential was checked. Because `RevocationBitmap2022` is embedded in the issuer's DID document, which can be obtained without the their knowledge, this approach does not suffer from these privacy shortcomings. See also the [privacy considerations](#privacy-considerations).

A downside of embedding the revocation list in the DID document is that storage in a distributed ledger (DLT) is usually more expensive than other storage hosting solutions. The DLT might also impose message size limitations, capping the total number of revocations that can be done (see also [compression](#compression)).

Another difference is that `RevocationList2020` specifies a minimum initial size of 131,072 for its bitstring, to mitigate the  potential for correlating individuals when few credentials have been issued. `RevocationBitmap2022` uses a roaring bitmap instead of a bitstring, so the maximum size is not fixed (apart from the upper bound of an unsigned 32-bit integer). This means the bitmap cannot be used to correlate small populations without more information not present in the bitmap itself. However, both schemes still reveal publicly how many credentials have been revoked, which could be used to infer other information if more knowledge about how an issuer assigns credential revocation indexes is known.

`StatusList2021` allows for explicitly stating the purpose of the list, currently either _revocation_ or _suspension_. This specification does not mandate that revoked credentials cannot be unrevoked, which means a `RevocationBitmap2022` can effectively also be used as a suspension list.

### Privacy Considerations

Because the revocation bitmap is embedded in the DID document, and thus available without contacting the issuer directly, the issuer cannot correlate how a holder uses their credential.

An observer finding a service of type `RevocationBitmap2022` in a DID document can infer that this DID belongs to an issuer. However, DIDs of issuers tend to be publicly known, in contrast to DIDs of other entities, so this is unlikely to present an issue. External observers can monitor the frequency of revocations and potentially the total number of issued credentials, depending on how the issuer assigns credential indices (e.g. starting from 0 and incrementing the index for each issued credential).

---
title: IOTA DID Method Specification
sidebar_label: DID Method
description: How IOTA Identity implements the Decentralized Identifiers Standard on the IOTA Tangle.
image: /img/Identity_icon.png
keywords:
- DID
- specs
- specifications
- Decentralized Identifiers
- Tangle
- format
---

# IOTA DID Method Specification

## Abstract

The IOTA DID Method Specification describes a method of implementing the [Decentralized Identifiers](https://www.w3.org/TR/did-core/) (DID) standard on the [IOTA Tangle](https://iota.org), a Distributed Ledger Technology (DLT). It conforms to the [DID specification v1.0](https://www.w3.org/TR/did-core/) and describes how to perform Create, Read, Update and Delete (CRUD) operations for IOTA DID Documents using unspent transaction outputs ([UTXO](https://wiki.iota.org/IOTA-2.0-Research-Specifications/5.1UTXO)) on the IOTA and [Shimmer](https://shimmer.network/) networks, introduced with the [Stardust upgrade](https://blog.shimmer.network/stardust-upgrade-in-a-nutshell/).

## Introduction

### The IOTA Tangle

This specification defines a method of implementing DID on top of the [IOTA Tangle](https://iota.org), which is a Distributed Ledger Technology (DLT) using a Tangle data structure. In contrast to a Blockchain, the Tangle does not store messages in blocks and chain them together, but rather creates a data structure where a message references between one and eight previous messages (used to be two, as the gif shows), creating a parallel structure.

Blockchain | Tangle
:---------:|:---------:

![Blockchain bottleneck](/img/blockchain-bottleneck.gif) | ![Tangle Bottleneck](/img/tangle-bottleneck.gif)

## UTXO Ledger

The unspent transaction output ([UTXO](https://wiki.iota.org/IOTA-2.0-Research-Specifications/5.1UTXO)) model defines a ledger state where outputs are created by a transaction consuming outputs of previous transactions as inputs. IOTA and Shimmer have several output types, the relevant ones for the IOTA DID Method are: Basic Outputs for value transactions, and Alias Outputs.

All types of outputs must hold a minimum amount of IOTA coins in order to be stored on the ledger. For some output types that can hold data, for instance the Alias Output, the amount of IOTA coins held by the output must cover the byte cost of the data stored. This helps to control the ledger size from growing uncontrollably while guaranteeing the data is indefinitely stored on the ledger which is important for resolving DID Documents.
This deposit is fully refundable and can be reclaimed when the output is destroyed.

Data saved in an output and covered by the storage deposit will be stored in *all* nodes on the network and can be retrieved from any node. This provides strong guarantees for any data stored in the ledger.

![Outputs](/img/utxo.png)

**todo outdated image**

### Alias Output

The [Alias Output](https://github.com/lzpap/tips/blob/master/tips/TIP-0018/tip-0018.md#alias-output) is a specific implementation of the [UTXO state machine](https://github.com/lzpap/tips/blob/master/tips/TIP-0018/tip-0018.md#chain-constraint-in-utxo). Some of its relevant properties are:

* **Amount**: the amount of IOTA coins held by the output.
* **Alias ID**: 32 byte array, a unique identifier of the alias, which is the BLAKE2b-256 hash
  of the Output ID that created it.
* **State Index**: A counter that must increase by 1 every time the alias is state transitioned.
* **State Metadata**: Metadata that can only be changed by the state controller. A leading uint16 denotes its length.
* **Unlock Conditions**:
    * State Controller Address Unlock Condition
    * Governor Address Unlock Condition

Consuming an Alias Output in a transaction means that the alias is transformed into the next state. The current state is defined as the consumed alias output, while the next state is defined as the **alias output with the same explicit `AliasID` on the output side**. There are two types of transitions: `state transition` and `governance transition`.

All outputs include an `Unlock Conditions` property. This feature defines how the output can be unlocked and spent. The Alias Output supports two types of controllers that can be set as unlock conditions: the state controller and governor. Each of these can be either an Ed25519 Address, Alias Address or an NFT Address. An Alias Output can have at most one of each unlock condition.

The state controller is responsible for a state transition. It is identified by an incremented `State Index` and can change the fields `Amount`, `State Index`, `State Metadata` among other properties.

The governor, on the other hand, is responsible for a governance transition indicated by an unchanged `State Index`. A governance transition can change the addresses of the state controller and governor. It also allows destroying the Alias Output by consuming it.

## DID Method Name

The namestring to identify this DID method is: `iota`.

A DID that uses this method MUST begin with the following prefix: `did:iota`. Following the generic DID specification, this string MUST be lowercase.

## DID Format

**ToDo**

The DIDs that follow this method have the following format:

```
iota-did = "did:iota:" iota-specific-idstring
iota-specific-idstring = [ iota-network ":" ] iota-tag
iota-network = char{,6}
iota-tag = base-char{44}
char = 0-9 a-z
base-char = 1-9 A-H J-N P-Z a-k m-z
```

### IOTA-Network

**TODO should be replaced by shimmer? **

The iota-network is an identifier of the network where the DID is stored. This network must be an IOTA Tangle, but can either be a public or private network, permissionless or permissioned.

The following values are reserved and cannot reference other networks:

1. `main` references the main network which refers to the Tangle known to host the IOTA cryptocurrency.
2. `dev` references the development network known as "devnet" maintained by the IOTA Foundation.

When no IOTA network is specified, it is assumed that the DID is located on the `main` network. This means that the following DIDs will resolve to the same DID Document:

```
did:iota:main:0xe4edef97da1257e83cbeb49159cfdd2da6ac971ac447f233f8439cf29376ebfe
did:iota:0xe4edef97da1257e83cbeb49159cfdd2da6ac971ac447f233f8439cf29376ebfe
```

### IOTA-Tag

The IOTA tag references an `Alias ID` identitfying the Alias Output where the DID Document is stored.
The tag will not be known before the generation of the DID. It will be assigned when the Alias Output is created.

### Anatomy of the State Metadata

In the `Metadata` of the Alias Output must be a JSON document containing two properties:

* `metadata`: contains metadata about the DID document for example, `created` to indicate the time of
  creation, and `updated` to indicated the time of the last update to the document. It can also include other properties.
* `document`: which contains the actual data of the document. In the example below, this document only contains
  one verification method. The `id` is specified by `did:0:0` referencing the same document since the `id` is unknown
  at the time of publishing.

```Json
{
   "document":{
      "id":"did:0:0",
      "verificationMethod":[
         {
            "id":"did:0:0#key-1",
            "controller":"did:0:0",
            "type":"Ed25519VerificationKey2018",
            "publicKeyMultibase":"z6BNtMbKY78XDVuqfh4u15bZkByu94XNVr9RpqEGCNncn"
         }
      ]
   },
   "metadata":{
      "created":"2022-08-02T21:39:48Z",
      "updated":"2022-08-02T21:39:48Z"
   }
}

```

## Controllers

As of now, only one state controller and one governor can be set for an Alias Output. Support for multiple controllers may be possible depending on future updates of the protocol.

A state controller can directly update the DID Document and the amount of coins held by the Alias Output, but it cannot destroy the output. A governor controller, one the other hand, can indirectly update the DID Document by updating the state controller. It can also destroy the output by performing a governance transition to the empty state.

## CRUD Operations

Create, Read, Update and Delete (CRUD) operations that change the DID Documents are to be submitted to an IOTA Tangle in order to be publicly available.
They have to update or destroy the Alias Output including the DID document.

### Create

Creating a new DID is done by creating a new Alias Output containing a serialized document as described in [Anatomy of serialized document](#anatomy-of-serialized-document). This requires the fulfillment of all the
Stardust requirements including the byte cost.

### Read

In order to read the latest DID document associated with a DID, the Alias Output of containing the DID Document
must be queried using the `Alias ID`. This can be done using an [inx indexer](https://github.com/iotaledger/inx-indexer)
which normal nodes usually include by default.

### Update

Updating can be achieved by a state transition of the Alias Output. A state controller can update the
`State Metadata` in the Alias Output which reflects in updating the DID Document.

### Delete

Temporarily deactivating a DID can be done by deleting the content of the `State Meadata` in the Alias Output. This will render the DID unresolvable. Another option is to set the `deactivated` property in `metadata` to true. In this case the deactivated DID Document can be resolved.

In order to permanently destroy a DID, a new transaction can be published that consumes the Alias Output without having an alias output on the output side with a corresponding explicit Alias ID. This results in destroying the Alias Output and the DID. Note that this operation is irreversible resulting in permanently deleting the DID.

## IOTA Identity standards

The `did:iota` method is implemented in the [IOTA Identity framework](https://github.com/iotaledger/identity.rs). This framework supports a number of operations that are standardized, some are standardized across the SSI community, and some are the invention of the IOTA Foundation.

### Standardized Verification Method Types

The IOTA Identity framework currently supports two Verification Method Types:

* `Ed25519VerificationKey2018`: can be used to sign DID Document updates, Verifiable Credentials, Verifiable Presentations, and arbitrary data with a `JcsEd25519Signature2020`.
* `X25519KeyAgreementKey2019`: can be used to perform [Diffie-Hellman key exchange](https://en.wikipedia.org/wiki/Diffie%E2%80%93Hellman_key_exchange) operations to derive a shared secret between two parties.

### Revocation

Revocation of Verifiable Credentials and signatures can be achieved using the [Revocation Bitmap 2022](../revocation_bitmap_2022.md) where issuers store a bitmap of indices in the DID Document. These indices correspond to verifiable credentials they have issued. If the binary value of the index in the bitmap is 1 (one), the verifiable credential is revoked, if it is 0 (zero) it is not revoked.

### Standardized Services

The IOTA Identity framework also standardized certain `services` that are embedded in the DID Document. It is RECOMMENDED to implement these when implementing the `did:iota` method.

Currently standardized `services`:

* Nothing yet.

## Security Considerations

### Private Key Management

All private keys or seeds used for the `did:iota` method should be equally well protected by the users. The signing key is especially important as it controls how keys are added or removed, providing full control over the identity. The IOTA Identity framework utilizes the [Stronghold project](https://github.com/iotaledger/stronghold.rs), a secure software implementation isolating digital secrets from exposure to hacks or leaks. Developers may choose to add other ways to manage the private keys in a different manner.

## Privacy Considerations

### Personal Identifiable Information

The public IOTA Tangle networks are immutable networks. This means that once something is uploaded, it can never be completely removed. That directly conflicts with certain privacy laws such as GDPR, which have a 'right-to-be-forgotten' for Personal Identifiable Information (PII). As such, users should NEVER upload any PII to the Tangle, including inside DID Documents. The IOTA Identity framework allows Verifiable Credentials to be published to the Tangle directly, however this feature should only be utilized by Identity for Organisation and Identity for Things.

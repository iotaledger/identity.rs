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

Draft 11 August 2022

## Abstract

The IOTA DID Method Specification describes a method of implementing the [Decentralized Identifiers](https://www.w3.org/TR/did-core/) (DID) standard on [IOTA](https://iota.org), a Distributed Ledger Technology (DLT). It conforms to the [DID specification v1.0](https://www.w3.org/TR/did-core/) and describes how to perform Create, Read, Update and Delete (CRUD) operations for IOTA DID Documents using unspent transaction outputs ([UTXO](https://wiki.iota.org/IOTA-2.0-Research-Specifications/5.1UTXO)) on the IOTA and [Shimmer](https://shimmer.network/) networks, introduced with the [Stardust upgrade](https://blog.shimmer.network/stardust-upgrade-in-a-nutshell/).

## Introduction

### UTXO Ledger

The unspent transaction output ([UTXO](https://wiki.iota.org/IOTA-2.0-Research-Specifications/5.1UTXO)) model defines a ledger state where outputs are created by a transaction consuming outputs of previous transactions as inputs. IOTA and Shimmer have several output types, the relevant ones for the IOTA DID Method are: Basic Outputs for value transactions, and Alias Outputs.

All outputs must hold a minimum amount of coins to be stored on the ledger. For output types that can hold arbitrary data, for instance the Alias Output, the amount of coins held by the output must cover the byte cost of the data stored. This helps control the ledger size from growing uncontrollably while guaranteeing the data is stored on the ledger indefinitely, which is important for resolving DID Documents. This deposit is fully refundable and can be reclaimed when the output is destroyed.

Data saved in an output and covered by the storage deposit will be stored in *all* nodes on the network and can be retrieved from any node. This provides strong guarantees for any data stored in the ledger.


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

Consuming an Alias Output in a transaction means that the alias is transformed into the next state. The current state is defined as the consumed alias output, while the next state is defined as the **alias output with the same explicit `Alias ID` on the output side**. There are two types of transitions: `state transition` and `governance transition`.

All outputs include an `Unlock Conditions` property. This feature defines how the output can be unlocked and spent. The Alias Output supports two types of controllers that can be set as unlock conditions: the state controller and governor. Each of these can be either an Ed25519 Address, Alias Address or an NFT Address. An Alias Output can have at most one of each unlock condition.

The state controller is responsible for a state transition. It is identified by an incremented `State Index` and can change the fields `Amount`, `State Index`, `State Metadata` among other properties.

The governor, on the other hand, is responsible for a governance transition indicated by an unchanged `State Index`. A governance transition can change the addresses of the state controller and governor. It also allows destroying the Alias Output by consuming it.

### Ledger and DID
Storing DID Documents in the ledger state means they inherently benefit from the guarantees the ledger provides.

1. Conflicts among nodes are sorted out and dealt with by the ledger.
2. Replay attacks are not possible since transactions need to be confirmed by the ledger.
3. The Alias Output includes the `State Index` which provides linear history for updates of a DID Document.

## DID Method Name

The namestring to identify this DID method is: `iota`.

A DID that uses this method MUST begin with the following prefix: `did:iota`. Following the generic DID specification, this string MUST be lowercase.

## DID Format

The DIDs that follow this method have the following format:

```
iota-did = "did:iota:" iota-specific-idstring
iota-specific-idstring = [ iota-network ":" ] iota-tag 
iota-network = char{,6}
iota-tag = "0x" lower case hex-encoded Alias ID 
char = 0-9 a-z
```

It starts with the string "did:iota:", followed by an optional network name (0-6 char) and a colon, then the tag.
The tag starts with "0x" followed by a hex-encoded `Alias ID` with lower case a-f.

### IOTA-Network

The iota-network is an identifier of the network where the DID is stored. This network must be an IOTA Ledger, but can either be a public or private network, permissionless or permissioned.

The following values are reserved and cannot reference other networks:

1. `iota` references the main network which refers to the ledger known to host the IOTA cryptocurrency.
2. `atoi` references the development network maintained by the IOTA Foundation.
3. `smr`  references the shimmer network.
4. `rms`  references the development network of shimmer.

When no IOTA network is specified, it is assumed that the DID is located on the `iota` network. This means that the following DIDs will resolve to the same DID Document:

```
did:iota:iota:0xe4edef97da1257e83cbeb49159cfdd2da6ac971ac447f233f8439cf29376ebfe
did:iota:0xe4edef97da1257e83cbeb49159cfdd2da6ac971ac447f233f8439cf29376ebfe
```

### IOTA-Tag
An IOTA-tag is a hex-encoded `Alias ID`. The `Alias ID` itself is a unique identifier of the alias, which is the BLAKE2b-256 hash of the Output ID that created it.
This tag identifies the Alias Output where the DID Document is stored, and it will not be known before the generation of the DID since it will be assigned when the Alias Output is created.

### Anatomy of the State Metadata

In the `State Metadata` of the Alias Output must be a byte packed payload with header fields as follows:

| Name          | Type                                                | Description                                                  |
|---------------|-----------------------------------------------------|--------------------------------------------------------------|
| Document Type | 3 Bytes ascii string                                | Set value to "DID" to denote a DID Document.                 |
| Version       | An unsigned 8-bit integer encoded in Little Endian. | Set value **1** to denote the version number of this method  |
| Encoding      | An unsigned 8-bit integer encoded in Little Endian. | Set value to **1** to denote plain text without compression. |
| Payload       | ByteArray                                           | Byte-packed playload.                                        |

#### Payload

The payload must contain the following fields:

* `metadata`: contains metadata about the DID document for example, `created` to indicate the time of
  creation, and `updated` to indicated the time of the last update to the document. It can also include other properties.
* `document`: which contains the actual data of the document. In the example below, this document only contains one verification method. The `id` is specified by `did:0:0` referencing the same document since the `id` is unknown at the time of publishing.

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

A state controller can directly update the DID Document and the amount of coins held by the Alias Output, but it cannot destroy the output. A governor, on the other hand, can indirectly update the DID Document by updating the state controller. The governor can also destroy the output by performing a governance transition without producing an Alias Output with the same `Alias ID`.

As of now, only one state controller and one governor can be set for an Alias Output. Support for multiple controllers may be possible depending on future updates of the protocol.

## CRUD Operations

Create, Read, Update and Delete (CRUD) operations that change the DID Documents are done through state or governance transitions of the Alias Output in order to be available on the ledger

**These operations require fund transfer to cover storage deposit. Transactions must be carefully done in order to avoid fund loss.** For example, the amount of funds in the inputs should equal these in the outputs. Additionally, private keys of controllers must be stored securely.

### Create
In order to create a simple self controlled DID two things are required:
1. An Ed25519 Address of which the private key is available.
2. A Basic, Alias or NFT Output with enough coins to cover the storage deposit.

Creation steps:
1. Create the content of the DID Document like verification methods, services, etc.
2. Use placeholder `did:0:0` instead of the actual DID in the document.
3. Create the payload and the headers as described in the [Anatomy of the State Metadata](#anatomy-of-the-state-metadata).
4. Create a new Alias Output with the payload and the headers stored in its `State Metadata`.
5. Set the Ed25519 Address as the state controller as well as the governor.
6. Set enough coins in the output to cover the storage deposit.
7. Publish a new transaction with the existing output as input and the newly created Alias Output as output. Other outputs can be created to hold the rest of the coins.
8. Query the newly created output using the address and retrieve the `Alias ID`.
9. Once the transaction is confirmed, the DID is published and can be formatted by using the `Alias ID` as the tag in [DID Format](#did-format).

Note that state controller and governor don't have to be the same, nor they have to be Ed255519 Addresses. This was chosen for the sake of simplicity.

### Read

The following steps can be used to read the latest DID document associated with a DID.
1. Extract the `iota-tag` from the DID, see [DID Format](#did-format).
2. Query the Alias Output using a node running [inx indexer](https://github.com/iotaledger/inx-indexer). Normal nodes usually include this indexer by default.
3. Retrieve the value of `State Metadata` from the returned output.
4. Check if its content matches the [Anatomy of the State Metadata](#anatomy-of-the-state-metadata).
5. Retrieve the value of `document` from the payload.
6. Replace the placeholder `did:0:0` with the actual DID.

### Update

Updating a DID Document can be achieved by a state transition of the Alias Output. A state controller can update the
`State Metadata` in the Alias Output which reflects in updating the DID Document.

1. Create the content of the updated DID Document.
2. Use placeholder `did:0:0` instead of the actual DID in the document.
3. Create the payload and the headers as described in the [Anatomy of the State Metadata](#anatomy-of-the-state-metadata).
4. Create a new Alias Output with the same `Alias ID` as the current output.
5. Increment the `State Index`.
6. Set the state controller and the governor.
7. Publish a new transaction with the current Alias Output as input and the newly created one as output.

Note, if the updated document requires more coins to cover the storage deposit, another input will be required to cover that.

### Delete

#### Deactivate
Temporarily deactivating a DID can be done by deleting the content of the `State Meadata` in the Alias Output. This will render the DID unresolvable.
1. Create and new Alias Output with the same `Alias ID` as the current output with empty `State Metadata`.
2. Increment the `State Index`.
3. Set the state controller and the governor.
4. Publish a new transaction with the current Alias Output as input and the newly created one as output.

Another option is to update the DID Document and set the `deactivated` property in `metadata` to true. In this case the deactivated DID Document will be deactivated, but it can still be resolved, see [Update](#update).

#### Destroy
In order to permanently destroy a DID, a new transaction can be published that consumes the Alias Output without having an alias output on the output side with a corresponding explicit `Alias ID`. This results in destroying the Alias Output and the DID. Note that this operation is irreversible resulting in permanently deleting the DID.

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
The `did:iota` method is implemented on the [IOTA](https://iota.org), a public permissionless and feeless Distributed Ledger Technology (DLT), making it resistant against almost all censorship attack vectors. Up until the `Coordicide` update for the IOTA network, a reliability on the coordinator exists for resolving ordering conflicts. This has a minor censorship possibility, that, in the wrost case, can prevent transactions from getting confirmed.


### Private Key Management

All private keys or seeds used for the `did:iota` method should be equally well protected by the users. Private keys of the state controller and the governor are especially important as they control how keys are added or removed, providing full control over the identity. The IOTA Identity framework utilizes the [Stronghold project](https://github.com/iotaledger/stronghold.rs), a secure software implementation isolating digital secrets from exposure to hacks or leaks. Developers may choose to add other ways to manage the private keys in a different manner.

## Privacy Considerations

### Personal Identifiable Information

The public IOTA Tangle networks are immutable networks. This means that once something is uploaded, it can never be completely removed. For example, destroying an Alias Output will remove it from the ledger state, but it can still be stored in permanodes or by any party that records historical ledger states.

That directly conflicts with certain privacy laws such as GDPR, which have a 'right-to-be-forgotten' for Personal Identifiable Information (PII). As such, users should NEVER upload any PII to the Tangle, including inside DID Documents. The IOTA Identity framework allows Verifiable Credentials to be published to the Tangle directly, however this feature should only be utilized by Identity for Organisation and Identity for Things.

### Correlation Risks

As with any DID method, identities can be linked if they are used too often and their usage somehow becomes public. Additionally, a DID can be correlated with funds if the Alias Output used to store the DID Document or any of its controllers is used for holding, transferring or controlling coins or NFTs. 
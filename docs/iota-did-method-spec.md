# IOTA DID Method Specification

Version 0.1-draft by Jelle Millenaar, IOTA Foundation

## Abstract

The IOTA DID Method Specification describes a method of implementing the [Decentralized Identifiers](https://www.w3.org/TR/did-core/) (DID) standard on the IOTA Tangle, a Distributed Ledger Technology (DLT). It conforms to the [DID specifications v1.0 Working Draft 20200731](https://www.w3.org/TR/2020/WD-did-core-20200731/) and described how to publish DID Document Create, Read, Update and Delete (CRUD) operations to the IOTA Tangle. 

## Introduction

### The IOTA Tangle

This specification defines a method of implementing DID on top of the IOTA Tangle, which is a Distributed Ledger Technology (DLT) using a Tangle data structure. In contrast to a Blockchain, the Tangle does not store messages in blocks and chain them together, but rather creates a data structure where a message references two previous messages, creating a parallel structure. 

For this method, the most important features of IOTA are: 

* The lack of fees, requiring no cryptocurrency tokens to be owned in order to submit a message to the DLT.
* Pure data messages are possible to be stored immutably. 
* Few nodes store the entire Tangle, requiring additional logic to prove the immutability of data. 

## DID Method Name

The namestring to identify this DID method is: `iota`.

A DID that uses this method MUST begin with the following prefix: `did:iota`. Following the generic DID specification, this string MUST be completely in lowercase.

## DID Format

The DIDs that follow this method have the following format:
```
iota-did = "did:iota:" iota-specific-idstring
iota-specific-idstring = [ iota-network ":" ] [ network-shard ":" ] iota-tag
iota-network = 6* char
network-shard = 6* char
iota-tag = 44* base-char
char = 0-9 a-z
base-char = 1-9 A-H J-N P-Z a-k m-z
```

### IOTA-Network

The iota-network is an identifer of the network where the DID is stored. This network must be an IOTA Tangle, but can either be a public or private network, permissionless or permissioned.

The following values are reserved and cannot reference other network:
1. `main` reference the main network which references to the Tangle known to host the IOTA cryptocurrency
2. `dev` references the test network known as "devnet" or "testnet" maintained by the IOTA Foundation.
3. `com` references the test network known as "comnet" or "community network" maintained by the IOTA community.

Note that when no IOTA network is specified, it is assumed that the DID is located on the `main` network. This means that the following DIDs will resolve to the same DID Document:
```
did:iota:main:H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV
did:iota:H3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV
```

### Network-Shard

In a future update to the network, IOTA will likely have shards containing subsets of the Tangle. While this is currently not the case, the space is in the DID is reserved and implementations should be able to handle this extra field. The value can currently be ignored. 

### IOTA-Tag

The IOTA tag references an indexation which resolves to the initial DID Messages. 

#### Generation

The following steps MUST be taken to generate a valid Tag:
* Generate an asymmetric keypair using a supported verification method type.
* Hash the public key using `BLAKE2b-256` and encode using base58.

Note that this public key MUST also be embedded into the DID Document (See ...).

## DID Messages

### Authentication Chain

### Differentiation Chain

### DID Document

### Key Revocation?

## CRUD Operations

### Create

### Read

### Update

### Delete

## Privacy and Security Considerations

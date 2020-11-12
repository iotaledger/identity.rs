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

Note that this public key MUST also be embedded into the DID Document (See [CRUD: Create](#Create)).

## DID Messages

DID Documents associated to the `did:iota` method consist of a chain of data messages, also known as zero-value transactions, published to a Tangle called "DID messages". The Tangle has no understanding of "DID messages" and acts purely as an immutable database. The chain of DID messages and the resulting DID Documents must therefore be validated on the client side. 

A DID Message can be part of one of two different message chains, the "Authentication Chain" (Auth Chain) and the "Differentiation Chain" (Diff Chain). The Auth Chain is a chain of "DID Auth Messages" that contain full DID Documents. The Diff Chain is a chain of "DID Diff Messages" that contain JSON objects which only list the differences between one DID Document and the next state. 

### Autonomy of Auth DID Messages

An Auth DID Message MUST contain a valid DID Document according to the W3C DID standard. In addition, the message has additional restrictions:

* The DID Document MUST always contain atleast one or more authentication keys, in the `authentication` object of the DID Document. 
* The first DID Document in the chain MUST contain an authentication key that, when hashed using the `Blake2b-256` hashing function, equals the tag section of the DID.
* Auth DID Messages have at least the following attributes:
    * prevMsg (required): The MessageId of an IOTA Message. This MessageId should reference the previous Auth DID Message that is updated by this new Auth DID Message. If this is the first message, the field should be submitted with an empty string. 
    * diffChain (required): A valid index of the Tangle (previously known as an Address or Tag). The value points to an indexation of the Tangle, which will be the location of the Diff Chain, originating from the DID Document contained in the same DID Auth Message.
    * proof (required): A proof object as defined by [Autonomy of the Proof object](#autonomy-of-the-proof-object). 

### Autonomy of the Diff DID Messages

TODO: Diff ID

A Diff DID Message does not contain a valid DID Document. Instead the chain creates a list of changes compared to the first Auth DID Message which references the location of this Diff (Multiple Auth DID Messages should not reference the same Diff Id). 

* A Diff DID Message is NOT allowed to add, remove or update authentication keys. This must be done via an Auth DID Message.
* The latest Diff DID Message is a collection of all changes, including the previous Diff DID Message.
* Diff DID Messages have at least the following attributes:
    * id (required): A DID URL that contains a fragment referencing the Diff Id.
    * prevMsg (required): The MessageId of an IOTA Tangle Message. This MessageId should reference the previous Diff DID Message or, if it is the first in the Diff Chain, the MessageId of the Auth DID Message.
    * diff (required): A Differentiation object containing all the changes compared to the DID Document in the first Auth DID Message which references the location of this Diff Chain (See [Autonomy of the Diff object](#autonomy-of-the-diff-object)).
    * proof (required): A proof object as defined by [Autonomy of the Proof object](#autonomy-of-the-proof-object). 

### Autonomy of the Proof object

The proof object includes at least the following attributes:
* id (required): A DID URL that contains a fragment, listing the key used for signing the signature.
* created (required): The string value of an ISO8601 combined date and time string.
* signature (required): One of any number of valid representations of signature values generated by the Signature Algorithm.


### Autonomy of the Diff object

TODO: With guidance from Tensor

## CRUD Operations

Create, Read, Update and Delete (CRUD) operations that change the DID Documents are to be submitted to an IOTA Tangle in order to be publicly avaliable. They will either have to be a valid Auth DID Message or Diff DID Message, submitted on the correct indexation for the identity. 

### Create

To generate a new DID, the method described in [generation](#generation) must be followed. A basic DID Document must be created that includes the public key used in the DID creation process as a public key included in the `authentication` object. This DID Document must be formatted as a Auth DID Message and published to an IOTA Tangle on the indexation generated out of the public key used in the DID creation process. 

A valid indexation for the Auth DID Messages is the hash generated from hashing the `Blake2b-256` algorithm encoded in base58. 

### Read

To read the latest DID Document associated with a DID, the following process must be executed:
* The indexation to query the Auth DID Messages DID is the `tag` part of the DID. 
* Query all the Auth DID Messages and order based on prevMsg linkages.
* When order cannot be determined through the prevMsg, dismiss the later  Auth DID Messages based on the milestone index. 
* Validate the first Auth DID Message as containing 

#### Determining Order

To determine order of any DID messages, the following algorithm must be applied:
* Order is initially established by recreating the chain based on the `prevMsg` linkages. 
* When two or more Messages compete, the older messages are to be dismissed.
* To determine which messages are to be dismissed, the order is based on which milestone confirmed the message.

### Update

### Delete

## Privacy and Security Considerations

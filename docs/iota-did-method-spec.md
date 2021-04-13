# IOTA DID Method Specification

Version 0.2-draft by Jelle Millenaar, IOTA Foundation

## Abstract

The IOTA DID Method Specification describes a method of implementing the [Decentralized Identifiers](https://www.w3.org/TR/did-core/) (DID) standard on the IOTA Tangle, a Distributed Ledger Technology (DLT). It conforms to the [DID specifications v1.0 Working Draft 20200731](https://www.w3.org/TR/2020/WD-did-core-20200731/) and described how to publish DID Document Create, Read, Update and Delete (CRUD) operations to the IOTA Tangle. In addition, it lists additional non-standardized features that are built for the IOTA Identity implementation. 

## Introduction

### The IOTA Tangle

This specification defines a method of implementing DID on top of the IOTA Tangle, which is a Distributed Ledger Technology (DLT) using a Tangle data structure. In contrast to a Blockchain, the Tangle does not store messages in blocks and chain them together, but rather creates a data structure where a message references between one and eight previous messages, creating a parallel structure. 

For this method, the most important features of IOTA are: 

* The lack of fees, requiring no cryptocurrency tokens to be owned in order to submit a message to the DLT.
* The DLT has a public and permissionless network which runs the IOTA cryptocurrency.
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
iota-network = char{,6}
network-shard = char{,6}
iota-tag = base-char{44}
char = 0-9 a-z
base-char = 1-9 A-H J-N P-Z a-k m-z
```

### IOTA-Network

The iota-network is an identifer of the network where the DID is stored. This network must be an IOTA Tangle, but can either be a public or private network, permissionless or permissioned.

The following values are reserved and cannot reference other network:
1. `main` reference the main network which references to the Tangle known to host the IOTA cryptocurrency
2. `dev` references the test network known as "devnet" or "testnet" maintained by the IOTA Foundation.

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

TODO: Add MerkleKeyCollection fields

DID Documents associated to the `did:iota` method consist of a chain of data messages, also known as zero-value transactions, published to a Tangle called "DID messages". The Tangle has no understanding of "DID messages" and acts purely as an immutable database. The chain of DID messages and the resulting DID Document must therefore be validated on the client side. 

A DID message can be part of one of two different message chains, the "Integration Chain" (Int Chain) and the "Differentiation Chain" (Diff Chain). The Integration Chain is a chain of "DID Integration Messages" that contain JSON formatted DID Documents as per the W3C standard for DID. The Diff Chain is a chain of "DID Diff Messages" that contain JSON objects which only list the differences between the previous DID Document and the next state. 

### Previous Message Id

Any DID message uploaded to the Tangle, with the exception of the very first DID message that creates the DID, MUST contain a `previous_message_id` field. This field MUST carry the MessageId, an IOTA indexation for a single message, of the previous DID Document that is updated with this DID message. This value SHOULD be used to order DID messages during the resolving procedure. If two or more DID messages reference the same `previous_message_id` an ordering conflict is identified and is resolved using a deterministic ordering mechanism. 

### Signing Key

DID Documents published to the Tangle must be cryptographically signed. As such the DID Document MUST include one verification method with a public key. It is recommended, for security reasons, to not use this keypair for other purposes as the control over this private key is vital for controlling the identity. It is RECOMMENDED to name this public key #_sign-x, where x is the index of the signing key, which is incredemented everytime the signing key is updated, starting at index 1. 

### Autonomy of DID Integration Messages

A DID Integration message MUST contain a valid DID Document according to the W3C DID standard. In addition, the message has additional restrictions:

* The DID Document MUST always contain one or more verification methods with a public key, in the `verificationMethod` object of the DID Document. It is RECOMMENDED to only use this key for updating the DID Document and name this public key #_sign-x. 
* The first DID Document in the chain MUST contain an `verificationMethod` that contains a public key that, when hashed using the `Blake2b-256` hashing function, equals the tag section of the DID. This prevents the creation of conflicting entry messages of the chain by adversaries.
* An Integration DID message must be published to an IOTA Tangle on an indexation that is generated by the `BLAKE2b-256` of the public key, created in the [generation](#generation) event, encoded in `hex`. 
* DID Integration messages SHOULD contain all cumulative changes from the Diff Chain associated to the last Integration Chain message. Any changes added in the Diff Chain that are not added to the new DID Integration message will be lost. 
* DID Integration Messages have at least the following attributes:
    * `previous_message_id` (REQUIRED): This field provides an immutable link to the previous DID document that is updated and is used for basic ordering of the DID messages, creating a chain. The value of `previous_message_id` MUST be a string that contains an IOTA MessageId from the previous DID message it updates, which MUST reference an Int Chain message. The field can be ommited, otherwise the field is REQUIRED. Read the [Previous Message Id](#previous-message-id) section for more information. 
    * `proof` (REQUIRED): This field provides a cryptographic proof on the message that proves ownership over the DID Document. The value of the `proof` object MUST contain an object as defined by [Autonomy of the Proof object](#autonomy-of-the-proof-object).

### Autonomy of the Diff DID Messages

A Diff DID message does not contain a valid DID Document. Instead the chain creates a list of changes compared to the a DID Integration message that is used as a basis. The Diff DID messages are hosted on a different indexation on the Tangle, which allows skipping older Diff DID messages during a query, optimizing the client verification speed significantly.  

* A Diff DID message is NOT allowed to add, remove or update any keys used to sign the Diff DID messages. This must be done via an Integration DID message.
* A Diff DID message must be published to an IOTA Tangle on an indexation that is generated by the hash, generated by the `BLAKE2b-256` hashing algorithm, of the `previous_message_id` of the latest Integration DID message and encoded in `hex`. 
* Diff DID Messages have at least the following attributes:
    * `id` (REQUIRED): This field helps link the update to a DID. The value of `id` MUST be a string that references the DID that this update applies to. 
    * `previous_message_id` (REQUIRED): This field provides an immutable link to the previous DID document that is updated and is used for basic ordering of the DID messages, creating a chain. The value of `previous_message_id` MUST be a string that contains an IOTA MessageId from the previous DID message it updates, which references either a Diff or Int Chain message. Read the [Previous Message Id](#previous-message-id) section for more information.
    * `diff` (REQUIRED): A Differentiation object containing all the changes compared to the DID Document it references in the `previous_message_id` field. The value of `diff` MUST be an escaped JSON string following the [Autonomy of the Diff object](#autonomy-of-the-diff-object) definition.
    * `proof` (REQUIRED): This field provides a cryptographic proof on the message that proves ownership over the DID Document. The value of the `proof` object MUST contain an object as defined by [Autonomy of the Proof object](#autonomy-of-the-proof-object).

### Autonomy of the Proof object

Following the proof format in the [Verifiable Credential standard](https://www.w3.org/TR/vc-data-model/#proofs-signatures), at least one proof mechanism, and the details necessary to evaluate that, MUST be expressed for a DID Document uploaded to the Tangle. 

The proof object is an embedded proof that contains all information to be verifiable. It contains one or more cryptographic proofs that can be used to detect tampering and verify the authorship of a DID creation or update. It mostly follows LD-Proofs standard.

**Type**

The proof object MUST include a `type` property. This property references a verification method type's signature algorithm, as standardized in the [did spec registries](https://www.w3.org/TR/did-spec-registries/#verification-method-types) or standardized via the method specification. 

The IOTA Identity implementation currently supports:
- `JcsEd25519Signature2020`

**Verification Method**

The proof object MUST include a `verificationMethod` which references a verification method embedded in the same DID Document. 

**Signature**

Depending on the verification method, a set of fields are REQUIRED to provide the cryptographic proof of the DID Document. For example, the `JcsEd25519Signature2020` method has a `signatureValue` field. 

### Autonomy of the Diff object

TODO: With guidance from Tensor

## CRUD Operations

Create, Read, Update and Delete (CRUD) operations that change the DID Documents are to be submitted to an IOTA Tangle in order to be publicly avaliable. They will either have to be a valid Int DID message or Diff DID message, submitted on the correct indexation for the identity. 

### Create

To generate a new DID, the method described in [generation](#generation) must be followed. A basic DID Document must be created that includes the public key used in the DID creation process as an `verificationMethod`. This DID Document must be formatted as an Integration DID message and published to an IOTA Tangle on the indexation generated out of the public key used in the DID creation process. 

### Read

To read the latest DID Document associated with a DID, the following process must be executed:
* Query all the Integration DID messages from the indexation, which is the `tag` part of the DID.
* Order the messages based on `previous_message_id` linkages. See [Determining Order](#determining-order) for more details.
* Validate the first Integration DID message as containing a public key inside the `verificationMethod` field that, when hashed using the `BLAKE2b-256` hashing algorithm, equals the `tag` field of the DID. 
* Verify the signatures of all the DID messages. Signatures must be signed using a public key avaliable in the previous DID message. 
* Ignore any messages that are not signed correctly, afterwards ignore any messages that are not first in the ordering for their specific location in the chain.
* If a URL parameter is added to the Resolution of `diff=false`, the following steps can be skipped and you should now have the latest DID Document.
* Query all the Differentiation DID messages from the indexation, generated by the MessageId from the last valid Integration DID message, hashed using `Blake2b-256` and encoded in `hex`.
* Order and validate signatures as similarly as to the Integration DID messages.
* Ignore messages with illegal operations such as removing or updating a signing key.
* Apply all valid Differentation updates to the state generated from the Integration DID messages. This will provide you with the latest DID Document.

#### Determining Order

To determine order of any DID messages, the following algorithm must be applied:
* Order is initially established by recreating the chain based on the `previous_message_id` linkages. 
* When two or more Messages compete, an order must be established between them.
* To determine the order, check which milestone confirmed the messages.
* If multiple messages are confirmed by the same milestone, we order based on the IOTA MessageId with alphabetical ordering.

### Update

In order to update a DID Document, either an Integration or a Differentation DID message needs to be generated. It is RECOMMENDED to use only Integration DID messages if the DID Document is updated very infrequently and it is expected to never go beyond 100 updates in the lifetime of the DID. If that is not the case, it is RECOMMENDED to use as much Differentiation DID messages instead with a maximum of around 100 updates per Diff chain. 

#### Creating an Integration DID message

An Integration DID message is unrestricted in its operations and may add or remove any fields to the DID Document. In order to query the DID Document, every Integration DID message must be processed, therefore it is RECOMMENDED to reduce the usage of these messages unless the DID Document in updated very infrequently. 

In order to create a valid Integeration DID message, the following steps must be executed:
* Create a new DID Document that contains all the new target state. This SHOULD include the new desired changes and all the changes inside the previous Diff Chain, otherwise these changes are lost!
* Retrieve the IOTA MessageId from the previous Integration DID message and a keypair for signing the DID Document.
* Set the `previous_message_id` field to the IOTA MessageId value.
* Create a `proof` object referencing the public key used inside the `verificationMethod` field and the signature suite in the `type` field. 
* Create a cryptographic signature using the same keypair and add the result in the appropriate field(s) inside the `proof` object.

#### Creating an Differentiation DID message

An Differentiation DID message is restricted in its usage. It may not update any signing keys that are used in the Diff chain. If this is desireable, it is REQUIRED to use an Integretation DID message. If the current Diff chain becomes to long (currently RECOMMENDED to end at a length of 100), it is RECOMMENDED to use a single Integration DID message to reset this length.

In order to create a valid Integration DID message, the following steps must be executed:
* Create and serialize a `diff` JSON object that contains all the changes. 
* Set the `did` field to the DID value that this update applies to.
* Retrieve the IOTA MessageId from the previous Diff chain message, or Integration DID message if this message is the first in the Diff chain. and a keypair for signing the DID Document.
* Set the `previous_message_id` field to the IOTA MessageId value.
* Create a `proof` object referencing the public key used inside the `verificationMethod` field and the signature suite in the `type` field. 
* Create a cryptographic signature using the same keypair and add the result in the appropriate field(s) inside the `proof` object.

### Delete

In order to deactivate a DID document, an Integration DID message must be published that removes all content from a DID Document, effectively deactivating the DID Document. Keep in mind that this is irreversable. 

## IOTA Identity standards

The `did:iota` method is implemented in the IOTA Identity framework. This framework supports a number of operations that are standardized, some are standardized across the SSI community, and some are the invention of the IOTA Foundation. 

### Standardized Verification Method Types

We support two different Verification Method Types. Verification methods that can be used for signing DID Document, and verification methods that are only used for signing [W3C standardized Verifiable Credentials](https://www.w3.org/TR/vc-data-model/). This set differs as the IOTA Identity implements revocation through public key removal, which means every Verifiable Credential must be signed by a different keypair. We create collection of keypairs in a Merkle Tree in order to save space in the DID Document.

Verification Methods that can be used to sign DID Documents and do other reusable activities:
* `Ed25519VerificationKey2018` to create a `JcsEd25519Signature2020`.

Verification Methods that can be used to sign Verifiable Credentials:
* `MerkleKeyCollection2021` to create a `MerkleKeySignature2021`

### Revocation

As mentioned above, revocation is done through deactivating public keys in the IOTA Identity framework. IOTA Identity has a highlky scalable solution since some identities might sign and want to revoke thousands if not millions of verifiable credentials. It is also GDPR compliant, leaving no trace, not even a hash, of a verifiable credential on the Tangle. Below will be a brief overview of how this mechanism works, but you may read all the details in the MerkleKeyCollection standardization document. 

#### Key Collections

Instead of storing individual public keys in a DID Document, IOTA Identity introduces the `MerkleKeyCollection2021` verification method. It supports a REQUIRED `publicKeyBase58` field that MUST contain the top hash of a Merkle Tree. In this Merkle Tree all the individual leaves are a public keys using the Signature Algorithm of choice and Digest Algorithm of choice for the Merkle Tree process. This process allows the creation of millions of public keys, within a single verification Method and without bloating the DID Document. For specific info such as the maximum depth of the Merkle Tree, supported signature algorithms and digest algorithms can be found in the specification document. 

#### Verifiable Credential Proofs

In addition to the normal signature, a `MerkleKeySignature2021` requires additional proof data to validate the signature's origin and validaty. The public key itself needs to be revealed inside the `signatureValue` field, but also the individual hashes inside the Merkle Tree and their relative location in the path to create a valid `Proof of Inclusion`. 

#### Revocation List

In order to revoke a public key, and therefore any Verifiable Credential or other statements, the DID Document must be updated with a revocation list. The REQUIRED `revocation` field is introduced inside the `MerkleKeyCollection2021` verification method, which lists the indices from the leaves of the Merkle Tree that are revoked. These indices are further compressed via [Roaring Bitmaps](https://roaringbitmap.org/).

## Standardized Services




## Privacy and Security Considerations

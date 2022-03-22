---
description: Glossary for IOTA Identity, defines the terms used in this specification and throughout decentralized identifier infrastructure.
image: /img/Identity_icon.png
keywords:
- W3C
- terminology
- IOTA
- verification method
- verifiable data registry
- reference
---

# Glossary

This section defines the terms used in this specification, (sourced from W3)[https://www.w3.org/TR/did-core/#terminology], and throughout decentralized identifier infrastructure. A link to these terms is included whenever they appear in this specification.

The first part of the glossary describes the terminology by the W3C. The [second describes the terminology for IOTA-related topics]().

## W3C Terminology

### Authentication
A process (typically some type of protocol) by which an entity can prove it has a specific attribute or controls a specific secret using one or more verification methods. With DIDs, a common example would be proving control of the private key associated with a public key published in a DID document.

### Decentralized Identifier (DID)
A globally unique persistent identifier that does not require a centralized registration authority because it is generated or registered cryptographically. The generic format of a DID is defined in the DID Core specification. A specific DID scheme is defined in a DID method specification. Many, but not all, DID methods make use of distributed ledger technology (DLT) or some other form of decentralized network.

### Decentralized identity management
An type of identity management that is based on the use of decentralized identifiers. Decentralized identity management extends authority for identifier generation, registration, and assignment beyond traditional roots of trust such as X.500 directory services, the Domain Name System, and most national ID systems.

### DID Controller
An entity that has the capability to make changes to a DID document. A DID may have more than one DID controller. The DID controller(s) can be denoted by the optional controller property at the top level of the DID document. Note that one DID controller may be the DID subject.

### DID Delegate
An entity to whom a DID controller has granted permission to use a verification method associated with a DID via a DID document. For example, a parent who controls a child's DID document might permit the child to use their personal device for authentication purposes. In this case, the child is the DID delegate. The child's personal device would contain the private cryptographic material enabling the child to authenticate using the DID. However, the child may not be permitted to add other personal devices without the parent's permission.

### DID Document
A set of data describing the DID subject, including mechanisms, such as public keys and pseudonymous biometrics, that the DID subject or a DID delegate can use to authenticate itself and prove its association with the DID. A DID document may also contain other attributes or claims describing the DID subject. A DID document may have one or more different representations as defined in § 6. Core Representations or in the W3C DID Specification Registries [DID-SPEC-REGISTRIES].

### DID Fragment
The portion of a DID URL that follows the first hash sign character (#). DID fragment syntax is identical to URI fragment syntax.

### DID Method
A definition of how a specific DID scheme must be implemented to work with a specific verifiable data registry. A DID method is defined by a DID method specification, which must specify the precise operations by which DIDs are created, resolved and deactivated, where DID documents are written and updated. See § 7. Methods.

### DID Path
The portion of a DID URL that begins with and includes the first forward-slash (/) character and ends with either a question mark (?) character or a fragment hash sign (#) character (or the end of the DID URL). DID path syntax is identical to URI path syntax. See § 3.2.3 Path.

### DID Query
The portion of a DID URL that follows and includes the first question mark character (?). DID query syntax is identical to URI query syntax. See § 3.2.4 Query.

### DID Resolution
The function that takes, as an input, a DID and a set of input metadata and returns a DID document in a conforming representation plus additional metadata. This function relies on the "Read" operation of the applicable DID method. The inputs and outputs of this function are defined in § 8. Resolution.

### DID Resolver
A DID resolver is a software or hardware component that performs the DID resolution function by taking a DID as input and producing a conforming DID document as output.

### DID Scheme
The formal syntax of a decentralized identifier. The generic DID scheme begins with the prefix "did:" as defined in the section of the DID Core specification. Each DID method specification must define a specific DID scheme that works with that particular DID method. In a specific DID method scheme, the DID method name must follow the first colon and terminate with the second colon, such as "did:example:".

### DID Subject
The entity identified by a DID and described by a DID document. A DID has exactly one DID subject. Anything can be a DID subject: a person, group, organization, physical thing, digital thing, logical thing, and so on.

### DID URL
A DID plus any additional syntactic component that conforms to the definition in § 3.2 DID URL Syntax. This includes an optional DID path, optional DID query (and its leading ? character), and optional DID fragment (and its leading # character).

### DID URL Dereferencing
The function that takes as its input a DID URL, a DID document, plus a set of dereferencing options, and returns a resource. This resource may be a DID document plus additional metadata, or it may be a secondary resource contained within the DID document, or it may be a resource entirely external to the DID document. If the function begins with a DID URL, it uses the DID resolution function to fetch a DID document indicated by the DID contained within the DID URL. The dereferencing function can then perform additional processing on the DID document to return the dereferenced resource indicated by the DID URL. The inputs and outputs of this function are defined in § 8.2 DID URL Dereferencing.

## IOTA Terminology

### Distributed Ledger (DLT)
A distributed database in which the various nodes use a consensus protocol to maintain a shared ledger in which each transaction is cryptographically signed and chained to the previous transaction.

### Public Key Description
A data object contained inside a DID document that contains all the metadata necessary to use a public key or verification key.

### Resource
As defined by [RFC3986]: "...the term 'resource' is used in a general sense for whatever might be identified by a URI." Similarly, any resource may serve as a DID subject identified by a DID.

### Representation
As defined for HTTP by [RFC7231]: "information that is intended to reflect a past, current, or desired state of a given resource, in a format that can be readily communicated via the protocol, and that consists of a set of representation metadata and a potentially unbounded stream of representation data." A DID document is a representation of information describing a DID subject. The § 6. Core Representations section of the DID Core specification defines several representation formats for a DID document.

### Service
A means of communicating or interacting with the DID subject or associated entities via one or more service endpoints. Examples include discovery services, agent services, social networking services, file storage services, and verifiable credential repository services.

### Service Endpoint
A network address (such as an HTTP URL) at which a service operates on behalf of a DID subject.

### Uniform Resource Identifier (URI)
The standard identifier format for all resources on the World Wide Web as defined by [RFC3986]. A DID is a type of URI scheme.

### Verifiable Credential
A standard data model and representation format for cryptographically-verifiable digital credentials as defined by the W3C [VC-DATA-MODEL].

### Verifiable Data Registry
A system that facilitates the creation, verification, updating, or deactivation of decentralized identifiers and DID documents. A verifiable data registry may also be used for other cryptographically-verifiable data structures such as verifiable credentials. For more information, see [VC-DATA-MODEL].

### Verifiable Timestamp
A verifiable timestamp enables a third-party to verify that a data object existed at a specific moment in time and that it has not been modified or corrupted since that moment in time. If the data integrity could were to be reasonably modified or corrupted since that moment in time, the timestamp is not verifiable.

### Verification Method
A set of parameters that can be used together with a process or protocol to independently verify a proof. For example, a public key can be used as a verification method with respect to a digital signature; in such usage, it verifies that the signer possessed the associated private key.

"Verification" and "proof" in this definition are intended to apply broadly. For example, a public key might be used during Diffie-Hellman key exchange to negotiate a shared symmetric key for encryption. This guarantees the integrity of the key agreement process. It is thus another type of verification method, even though descriptions of the process might not use the words "verification" or "proof."

### Verification Relationship
An expression of the relationship between the DID subject and a verification method. An example of a verification relationship is § 5.4.1 authentication.

### Universally Unique Identifier (UUID)
A type of globally unique identifier defined by [RFC4122]. UUIDs are similar to DIDs in that they do not require a centralized registration authority. UUIDs differ from DIDs in that they are not resolvable or cryptographically-verifiable.
In addition to the terminology above, this specification also uses terminology from the [INFRA] specification to formally define the abstract data model. When [INFRA] terminology is used, such as string, ordered set, and map, it is linked directly to that specification.
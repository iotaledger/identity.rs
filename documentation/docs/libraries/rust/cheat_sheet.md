---
title:  Rust Cheat Sheet
sidebar_label: Cheat Sheet
description: IOTA Identity Rust Library Cheat Sheet
image: /img/Identity_icon.png
keywords:
- rust
- identity
- decentralized identifiers
- did
- verifiable credentials
- verifiable presentations
- create
- update
- resolve
- remove
---

## Import the Library
To include IOTA Identity in your project add it as a dependency in your Cargo.toml:

```rust
[dependencies]
identity = { git = "https://github.com/iotaledger/identity.rs", branch = "main"}
```

## Decentralized Identifiers (DID)

A DID is a unique identifier that contains information that can be resolved to a DID Document. This document contains data such as public keys, enabling the holder to prove ownership over their personal data, but also URIs that link to public information about the identity. This implementation complies to the DID specifications v1.0 Working.

### [Create](../../decentralized_identifiers/create.mdx)

#### identity::prelude::KeyPair::new_ed25519(count)

Generates a new Ed255119 KeyPair.

```rs
KeyPair::new_ed25519()?;
```

####  identity::prelude::IotaDocument::from_keypair(keypair)

Create a new DID Document from the given KeyPair.
The DID Document will be pre-populated with a single authentication method based on the provided KeyPair.

```rs
IotaDocument::from_keypair(keypair:KeyPair)?;
```

####  identity::prelude::IotaDocument::from_keypair_with_network(keypair, network)

Create a new DID Document from the given KeyPair and network.
The DID Document will be pre-populated with a single authentication method based on the provided KeyPair.

```rs
IotaDocument::from_keypair_with_network(keypair:KeyPair, network: str)?;
```

### [Publish](../../decentralized_identifiers/create.mdx)

####  identity::prelude::Client.publish_document(document)

Publish an IotaDocument to the Tangle using a Client.

```rs
Client.publishDocument(document: IotaDocument);
```

### [Update](../../decentralized_identifiers/update.mdx)

#### identity::prelude::IotaDocument.insert_method(scope, method)

Add a new VerificationMethod to a IotaDocument.

```rs
IotaDocument.insert_method(scope:  identity::did::MethodScope, method: identity::iota::IotaVerificationMethod)
```

#### identity::prelude::IotaDocument.insert_service(service)

Add a new Service to a IotaDocument.

```rs
IotaDocument.insert_service(service: identity::did::Service)
```

### [Resolve](../../decentralized_identifiers/resolve.mdx)

####  identity::did::resolution::resolve(did, input, method)

Resolves a DID into a DID Document by using the “Read” operation of the DID method.

```rs
resolution::resolve(did: str, input: InputMetadata, method: R)
```

#### identity::prelude::Client::resolve_history(did: &'_ IotaDID)

Returns the MessageHistory of the given IotaDID.

```rs
client.resolve_history(did: IotaDID)
```

## Verifiable Credentials (VC)

A Verifiable Credential can be verified by anyone, allowing you to take control of it and share it with anyone.

### [Create](../../verifiable_credentials/create.mdx)

#### identity::prelude::common::issue_degree(issuer, subject)

Take two IotaDocuments for issuer and subject, and creates an unsigned credential with claims about subject by issuer.
```rs
common::issue_degree(issuer: IotaDocument, subject: IotaDocument)
```

### [Sign](../../verifiable_credentials/create.mdx)

####  identity::prelude::IotaDocument.sign_data(data, private_key)

Sign the provided data with the default authentication method.

```rs
IotaDocument.sign_data(data: X, private_key: Key)
```

### [Revoke](../../verifiable_credentials/revoke.mdx)

#### identity::prelude::IotaDocument.remove_method(did)

Removes all references to the specified Verification Method,  effectively revoking the VC as it will no longer be able to verify.

```rs
IotaDocument.removeMethod(did: IotaDID)
```

####  identity::iota::IotaVerificationMethod.revoke_merkle_key(index)

Revokes the public key of a Merkle Key Collection at the specified index, instead of revoking the entire verification method.

```rs
IotaVerificationMethod.revoke_merkle_key(index: usize)
```

## Verifiable Presentations (VP)

A Verifiable Presentation is the format in which you can share a (collection of) Verifiable Credential(s). It is signed by the subject, to prove control over the Verifiable Credential with a nonce or timestamp.

### [Create](../../verifiable_credentials/verifiable_presentations.mdx)

#### identity::credential::PresentationBuilder.build()

Returns a new Presentation based on the PresentationBuilder configuration. 

````rs
PresentationBuilder::default().build()
````

### [Sign](../../verifiable_credentials/verifiable_presentations.mdx)

####  identity::prelude::IotaDocument.sign_data(data, private_key)

Sign the provided data (in this case a Presentation) with the default authentication method.

```rs
IotaDocument.sign_data(data: Presentaton, private_key: Key)
```

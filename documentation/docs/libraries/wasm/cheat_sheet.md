---
title:  Wasm Cheat Sheet
sidebar_label: Cheat Sheet
description: IOTA Identity Wasm Library Cheat Sheet
image: /img/Identity_icon.png
keywords:
- wasm
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

```bash
npm install @iota/identity-wasm 
```

## Decentralized Identifiers (DID)

A DID is a unique identifier that contains information that can be resolved to a DID Document. This document contains data such as public keys, enabling the holder to prove ownership over their personal data, but also URIs that link to public information about the identity. This implementation complies to the DID specifications v1.0 Working.

### [Create](../../decentralized_identifiers/create)

####  [new KeyPair( _type)](api_reference#KeyPair)

Generates a new  [KeyPair](api_reference#KeyPair) object of the dedired _type.

```js
new KeyPair(type_: number)
```

#### [new Document(type_, network, tag)](api_reference#new_Document_new)
Create a new [Document](api_reference#Document) from the given [KeyPair](api_reference#KeyPair).

```js
new Document(type_:number, network:string | undefined, tag:string | undefined)
```
####  [Document.fromKeyPair(key, network)](api_reference#Document.fromKeyPair)
Creates a new DID Document from the given [KeyPair](api_reference#KeyPair) and optional network.

```js
Document.fromKeyPair(key:keyPair, network:string|undefined)
```

### [Publish](../../decentralized_identifiers/create)

#### [Client.publishDocument(document)](api_reference#Client+publishDocument)

```js
client.publishDocument(document: any);
```
### [Update](../../decentralized_identifiers/update)

#### [Document.insertMethod(verificationMethod, scope)](api_reference#documentinsertmethodmethod-scope-⇒-boolean)

Add a new [VerificationMethod](api_reference#VerificationMethod).

```js
Document.insertMethod(verificationMethod: VerificationMethod, scope: string|undefined)
```

#### [Document.insertService(service)](api_reference#documentinsertserviceservice-⇒-boolean)

Add a new [Service](api_reference#Service).

```js
Document.insertService(service: Service)
```

### [Resolve](../../decentralized_identifiers/resolve) 

#### [resolve(did)](api_reference#clientresolvedid-⇒-promiseany)

Use a [Client](api_reference#Client) to resolve a DID [Document](api_reference#Document).

```js
client.resolve(did:string)
```

#### [resolveHistory(did)](api_reference#clientresolvehistorydid-⇒-promiseany)

Use a [Client](api_reference#Client) to return the message history of a DID [Document](api_reference#Document).

```js
client.resolveHistory(did:string)
```

## Verifiable Credentials (VC)

A [Verifiable Credential](api_reference#VerifiableCredential) can be verified by anyone, allowing you to take control of it and share it with anyone.

### [Create](../../verifiable_credentials/create)

#### [VerifiableCredential.extend(value)](api_reference#verifiablecredentialextendvalue-⇒-codeverifiablecredentialcode)

Create an [VerifiableCredential](api_reference#VerifiableCredential) from the given value.

```js
VerifiableCredential.extend(value:any)
```
### [Sign](../../verifiable_credentials/create)

#### [Document.sign(keyPair)](api_reference#Documentsignkey)

Signs a DID [Document](api_reference#Document) with the default authentication method using a [KeyPair](api_reference#KeyPair).

```js
Document.sign(key:keyPair)
```

#### [Document.signCredential(data, args)](api_reference#documentsigncredentialdata-args-⇒-codeverifiablecredentialcode)

Use a [Document](api_reference#Document) to sign a [VerifiableCredential](api_reference#VerifiableCredential) with a [KeyPair](api_reference#KeyPair). 

```js
Document.sign(key: keyPair)
```

### [Revoke](../verifiable_credentials/revoke)

#### [Document.removeMethod(DID)](api_reference#Document+removeMethod)

Remove a public key ([DID](api_reference#DID)) that signed a [VerifiableCredential](api_reference#VerifiableCredential) from a [Document](api_reference#Document),  effectively revoking the VC as it will no longer be able to verify.

```js
Document.removeMethod(did: DID)
```

#### [Document.revokeMerkleKey(query, index)](api_reference#Document+revokeMerkleKey)

[Revoke a single key from a MerkleKeyCollection](../verifiable_credentials/merkle_key_collection), instead of revoking the entire verification method.

## Verifiable Presentations (VP)

A Verifiable Presentation is the format in which you can share a (collection of) Verifiable Credential(s). It is signed by the subject, to prove control over the Verifiable Credential with a nonce or timestamp.

### [Create](../verifiable_credentials/verifiable_presentations)

#### [new VerifiablePresentation(holder_doc, credential_data, presentation_type, presentation_id)](api_reference#new-verifiablepresentationholder_doc-credential_data-presentation_type-presentation_id)

````js
new VerifiablePresentation(holder_doc: Document,credential_data: any, presentation_type: string|undefined, presentation_id: string | undefined )
````

### [Sign](../verifiable_credentials/verifiable_presentations)

#### [Document.signPresentation(data, args)](api_reference#documentsignpresentationdata-args-⇒-codeverifiablepresentationcode)

Use a [Document](api_reference#Document) to sign a [Verifiable Presentation](api_reference#VerifiablePresentation)

```js
Document.signPresentation(data: any, args: any)
```
---
title:  Wasm Cheat Sheet
sidebar_label: Cheat Sheet
description: IOTA Identity Wasm Library Cheat Sheet.
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
- reference
---

## Install the Library

You can install the main Identity package using [npm](https://www.npmjs.com/).

### Latest Stable Release

This version matches the `main` branch of this repository. It is **stable** and will have **changelogs**.

```bash
npm install @iota/identity-wasm 
```

### Development Release

This version matches the `dev` branch of this repository. It has all the **latest features**, but it **may also have undocumented breaking changes**.

```bash
npm install @iota/identity-wasm@dev
```

## Decentralized Identifiers (DID)

A DID is a unique identifier that contains information that can be resolved to a DID Document. This document contains data such as public keys, enabling the holder to prove ownership over their personal data, but also URIs that link to public information about the identity. This implementation complies with the DID specifications v1.0 Working.

### [Create](../../decentralized_identifiers/create.mdx)

####  [new KeyPair( _type)](api_reference#KeyPair)

Generates a new [KeyPair](api_reference#KeyPair) object of the desired _type.

##### Parameters

* type_ : One of enum identity.KeyType {Ed25519}
 
```js
new KeyPair(type_: number);
```

##### Returns
<details>
<summary>

* [KeyPair](api_reference#KeyPair).

</summary>

```js
{
  type: string,// example 'ed25519'
  public: string(44),// example '5Geq8HMRHxBhyesEbzbq8nG78ZRXFcHRUVhny4kfrHRh',
  secret: string(44),// example 'GeKQa6EhNXkbo74JLuvyxRFR3rouy2iPViwtk9JM9Dyn'
}
```
</details>

#### [new Document(type_, network, tag)](api_reference#new_Document_new)

Create a new [Document](api_reference#Document) from the given [KeyPair](api_reference#KeyPair).

##### Parameters

* type_ : One of enum identity.KeyType{Ed25519}.
* network: (optional) The string representation of the chosen network, "mainNet" for example.
* tag: (optional) String.

```js
new Document(type_:number, network:string | undefined, tag:string | undefined);
```

#### Returns

<details>
<summary>

Object consisting of:
  * [KeyPair](api_reference#KeyPair).
  * [Document](api_reference#Document).
  
</summary>

```js
{
  key: {
    type: string,// example 'ed25519',
    public: string(44),// example 'C7XrAUAUuM14sRdtN2daAkfA1hwoFxbWDcE5YxXqwveN',
    secret: string(44),// example 'EsmpiVppDZi6ocYhmMGzerSufwUb7rFcWcmwMNmGRaXA'
  },
  doc: {
    id: string(53),// example 'did:iota:9WnLBWvqU8ULtc6HZ8t9yCwa4FZRqQE3hX6wt7qoJZYK',
    authentication: [ [Object] ],
    created: Date, //exaple '2021-10-04T14:55:41Z',
    updated: Date,// example'2021-10-04T14:55:41Z'
  }
}
```
</details>

####  [Document.fromKeyPair(key, network)](api_reference#Document.fromKeyPair)

Creates a new DID Document from the given [KeyPair](api_reference#KeyPair) and optional network.

##### Parameters

* key: [KeyPair](api_reference#KeyPair).
* network: (optional) The string representation of the chosen network, "mainNet" for example.

```js
Document.fromKeyPair(key:keyPair, network:string|undefined);
```

#### Returns

<details>
<summary>

* [Document](api_reference#Document).

</summary>

```js
{
  id: string(53),// example 'did:iota:4sKRTsLb7xpRC2gBuVN3gpHvw4NtvZGFHXjcHGTnKBGn',
  authentication: [
    {
      id: string(53+),// example 'did:iota:4sKRTsLb7xpRC2gBuVN3gpHvw4NtvZGFHXjcHGTnKBGn#authentication',
      controller:string(53),// example  'did:iota:4sKRTsLb7xpRC2gBuVN3gpHvw4NtvZGFHXjcHGTnKBGn',
      type: string,// example 'Ed25519VerificationKey2018',
      publicKeyBase58: string(44),// example '5Geq8HMRHxBhyesEbzbq8nG78ZRXFcHRUVhny4kfrHRh'
    }
  ],
  created: Date, //example '2021-10-04T14:55:41Z',
  updated: Date, //example '2021-10-04T14:55:41Z'
}
```
</details>

### [Publish](../../decentralized_identifiers/create.mdx)

#### [Client.publishDocument(document)](api_reference#Client+publishDocument)

```js
client.publishDocument(document: any);
```
#### Parameters

* document: Any. 

#### Returns

<details>
<summary>
Promise
</summary>

```js
{
  network: string, //example 'dev'
  messageId: string(64), //example 'cd4e6274a8c3d75fda5ef276562d87aa06366d7e8d87639f03354b24f3de8010'
  networkId: int, // example 6514788332515804000
  nonce: int, // example 865163
}
```
</details>

### [Update](https://wiki.iota.org/identity.rs/decentralized_identifiers/update)

#### [Document.insertMethod(verificationMethod, scope)](api_reference#documentinsertmethodmethod-scope-⇒-boolean)

Add a new [VerificationMethod](api_reference#VerificationMethod).

##### Parameters

* verificationMethod: [VerificationMethod](api_reference#VerificationMethod).
* scope: (optional) string.

```js
Document.insertMethod(verificationMethod: VerificationMethod, scope: string|undefined);
```
##### Returns

* Boolean.

#### [Document.insertService(service)](api_reference#documentinsertserviceservice-⇒-boolean)

Add a new [Service](api_reference#Service).

#### Parameters 

* service: [Service](api_reference#Service).

```js
Document.insertService(service: Service);
```
##### Returns
 
* Boolean

### [Resolve](../../decentralized_identifiers/resolve.mdx) 

#### [resolve(did)](api_reference#clientresolvedid-⇒-promiseany)

Use a [Client](api_reference#Client) to resolve a DID [Document](api_reference#Document).

##### Parameters

* did: string. The [Document](api_reference#Document)'s identifier. 

```js
client.resolve(did:string);
```
##### Returns

<details>
<summary>

* [Document](api_reference#Document).
 
</summary>

```js
{
  id: string(53),// example 'did:iota:4sKRTsLb7xpRC2gBuVN3gpHvw4NtvZGFHXjcHGTnKBGn',
  authentication: [
    {
      id: string(53+),// example 'did:iota:4sKRTsLb7xpRC2gBuVN3gpHvw4NtvZGFHXjcHGTnKBGn#authentication',
      controller:string(53),// example  'did:iota:4sKRTsLb7xpRC2gBuVN3gpHvw4NtvZGFHXjcHGTnKBGn',
      type: string,// example 'Ed25519VerificationKey2018',
      publicKeyBase58: string(44),// example '5Geq8HMRHxBhyesEbzbq8nG78ZRXFcHRUVhny4kfrHRh'
    }
  ],
  created: Date, //example '2021-10-04T14:55:41Z',
  updated: Date, //example '2021-10-04T14:55:41Z'
}
```

</details>

#### [resolveHistory(did)](api_reference#clientresolvehistorydid-⇒-promiseany)

Use a [Client](api_reference#Client) to return the message history of a DID [Document](api_reference#Document).

##### Parameters

* did: string. The [Document](api_reference#Document)'s identifier.

```js
client.resolveHistory(did:string);
```
##### Returns


<details>
<summary>

The message history of a given [Document](api_reference#Document).

</summary>

```js
{
  integrationChainData: [
    {
      id: string, // example 'did:iota:HzXqCmUPyvibPSwsf2TiPL6GnB6dFPgaf2xm62Dcr2z2',
      authentication: [Array],
      created: Date,// example '2021-10-05T08:46:37Z',
      updated: Date,// example'2021-10-05T08:46:37Z',
      proof: [Object]
    }
  ],
  integrationChainSpam: [],
  diffChainData: [],
  diffChainSpam: []
}

```

</details>

## Verifiable Credentials (VC)

A [VerifiableCredential](api_reference#VerifiableCredential) can be verified by anyone, allowing you to take control of it and share it with anyone.

### [Create](../../verifiable_credentials/create.mdx)

#### [VerifiableCredential.extend(value)](api_reference#verifiablecredentialextendvalue-⇒-codeverifiablecredentialcode)

Create a [VerifiableCredential](api_reference#VerifiableCredential) from the given value.

```js
VerifiableCredential.extend(value:any);
```

#### Returns


<details>
<summary>

* [VerifiableCredential](api_reference#VerifiableCredential)

</summary>

```js
{
  '@context': 'https://www.w3.org/2018/credentials/v1',
  id: string, // example 'https://example.edu/credentials/3732',
  type: [ 'VerifiableCredential', 'UniversityDegreeCredential' ],
  credentialSubject: {
    id: string(53), // example 'did:iota:95J1FTxKWMEZEGDByw1SwLuDCk6G7rKnM9niQfbjxjaX',
    GPA:  string, // example'4.0',
    degreeName:  string, // example 'Bachelor of Science and Arts',
    degreeType:  string, // example 'BachelorDegree',
    name:  string, // example 'Alice'
  },
  issuer:  string(53), //'did:iota:8X22fe2H6NrHP9sr77S96ym7xhDg5aEgiywi375noTHX',
  issuanceDate: Date, //'2021-10-05T08:58:33Z'
}
```
</details>

### [Sign](../../verifiable_credentials/create.mdx)

#### [Document.sign(keyPair)](api_reference#Documentsignkey)

Signs a DID [Document](api_reference#Document) with the default authentication method using a [KeyPair](api_reference#KeyPair).

##### Parameters

* [KeyPair](api_reference#KeyPair).

```js
Document.sign(key:keyPair);
```

##### Returns


<details>
<summary>

* [VerifiableCredential](api_reference#VerifiableCredential)

</summary>

```js
{
  '@context': 'https://www.w3.org/2018/credentials/v1',
  id: string, // example 'https://example.edu/credentials/3732',
  type: [ 'VerifiableCredential', 'UniversityDegreeCredential' ],
  credentialSubject: {
    id: string(53), // example 'did:iota:95J1FTxKWMEZEGDByw1SwLuDCk6G7rKnM9niQfbjxjaX',
    GPA:  string, // example'4.0',
    degreeName:  string, // example 'Bachelor of Science and Arts',
    degreeType:  string, // example 'BachelorDegree',
    name:  string, // example 'Alice'
  },
  issuer:  string(53), //'did:iota:8X22fe2H6NrHP9sr77S96ym7xhDg5aEgiywi375noTHX',
  issuanceDate: Date, //'2021-10-05T08:58:33Z'
}
```

</details>

#### [Document.signCredential(data, args)](api_reference#documentsigncredentialdata-args-⇒-codeverifiablecredentialcode)

Use a [Document](api_reference#Document) to sign a [VerifiableCredential](api_reference#VerifiableCredential). 

##### Parameters

* data: Any.
* args: Any.
 
```js
Document.signCredential(data: any, args: any);
```

##### Returns

<details>
<summary>

* [VerifiableCredential](api_reference#VerifiableCredential)

</summary>

```js
{
  '@context': 'https://www.w3.org/2018/credentials/v1',
  id: string, // example 'https://example.edu/credentials/3732',
  type: [ 'VerifiableCredential', 'UniversityDegreeCredential' ],
  credentialSubject: {
    id: string(53), // example 'did:iota:95J1FTxKWMEZEGDByw1SwLuDCk6G7rKnM9niQfbjxjaX',
    GPA:  string, // example'4.0',
    degreeName:  string, // example 'Bachelor of Science and Arts',
    degreeType:  string, // example 'BachelorDegree',
    name:  string, // example 'Alice'
  },
  issuer:  string(53), //'did:iota:8X22fe2H6NrHP9sr77S96ym7xhDg5aEgiywi375noTHX',
  issuanceDate: Date, //'2021-10-05T08:58:33Z'
}
```

</details>

### [Revoke](https://wiki.iota.org/identity.rs/verifiable_credentials/revoke)

#### [Document.removeMethod(DID)](api_reference#Document+removeMethod)

Remove a public key ([DID](api_reference#DID)) that signed a [VerifiableCredential](api_reference#VerifiableCredential) from a [Document](api_reference#Document), effectively revoking the VC as it will no longer be able to verify.

##### Parameters

* did: [DID](api_reference#DID).
 
```js
Document.removeMethod(did: DID);
```
##### Returns

* Void

## Verifiable Presentations (VP)

A Verifiable Presentation is the format that you can share a (collection of) Verifiable Credential(s). It is signed by the subject to prove control over the Verifiable Credential with a nonce or timestamp.

### [Create](../../verifiable_credentials/verifiable_presentations.mdx)

#### [new VerifiablePresentation(holder_doc, credential_data, presentation_type, presentation_id)](api_reference#new-verifiablepresentationholder_doc-credential_data-presentation_type-presentation_id)

##### Parameters

* holder_doc: [Document](api_reference#Document). 
* credential_data: any.
* presentation_type: (optional) string.
* presentation_id : (optional) string.
 
````js
new VerifiablePresentation(holder_doc: Document, credential_data: any, presentation_type: string|undefined, presentation_id: string | undefined )
````

##### Returns

<details>
<summary>

* [Verifiable Presentation](api_reference#VerifiablePresentation) containing a [VerifiableCredential](api_reference#VerifiableCredential).
 
</summary>

```js
{
  '@context': string, // example 'https://www.w3.org/2018/credentials/v1',
  type: string, // example  'VerifiablePresentation',
  verifiableCredential: {
    '@context':string, // example  'https://www.w3.org/2018/credentials/v1',
    id: string, // example 'https://example.edu/credentials/3732',
    type: array, // example [ 'VerifiableCredential', 'UniversityDegreeCredential' ],
            credentialSubject: {
      id: string(53), // example 'did:iota:95J1FTxKWMEZEGDByw1SwLuDCk6G7rKnM9niQfbjxjaX',
              GPA:  string, // example'4.0',
              degreeName:  string, // example 'Bachelor of Science and Arts',
              degreeType:  string, // example 'BachelorDegree',
              name:  string, // example 'Alice'
    },
    issuer:  string(53), //'did:iota:8X22fe2H6NrHP9sr77S96ym7xhDg5aEgiywi375noTHX',
    issuanceDate: Date, //'2021-10-05T08:58:33Z'  
    proof: {
      type: string, // example 'JcsEd25519Signature2020',
      verificationMethod:  string, // example '#newKey',
      signatureValue:  string(88), // example '56aGDSPBTW3p1AJQWnn1eRwixfGBfq1Gpj3NtSjhA2Qqgk3LrpeNYocvmV73ru4WLRYHPTyHwVKSGfd6JbSsyFRZ'
    }
  },
  holder:  string(53), // 'did:iota:4SgsnbN67cJDwGgpU4woVYNY37kfN4Dr8rSfyDPbVTsR'
}
```

</details>

### [Sign](https://wiki.iota.org/identity.rs/verifiable_credentials/verifiable_presentations)

#### [Document.signPresentation(data, args)](api_reference#documentsignpresentationdata-args-⇒-codeverifiablepresentationcode)

Use a [Document](api_reference#Document) to sign a [Verifiable Presentation](api_reference#VerifiablePresentation).

##### Parameters

* data: any.
* args: any.
 
```js
Document.signPresentation(data: any, args: any);
```

##### Returns

<details>
<summary>

* [Verifiable Presentation](api_reference#VerifiablePresentation) containing a [VerifiableCredential](api_reference#VerifiableCredential).
 
</summary>

```js
{
  '@context': string, // example 'https://www.w3.org/2018/credentials/v1',
  type: string, // example  'VerifiablePresentation',
  verifiableCredential: {
    '@context':string, // example  'https://www.w3.org/2018/credentials/v1',
    id: string, // example 'https://example.edu/credentials/3732',
    type: array, // example [ 'VerifiableCredential', 'UniversityDegreeCredential' ],
            credentialSubject: {
      id: string(53), // example 'did:iota:95J1FTxKWMEZEGDByw1SwLuDCk6G7rKnM9niQfbjxjaX',
              GPA:  string, // example'4.0',
              degreeName:  string, // example 'Bachelor of Science and Arts',
              degreeType:  string, // example 'BachelorDegree',
              name:  string, // example 'Alice'
    },
    issuer:  string(53), //'did:iota:8X22fe2H6NrHP9sr77S96ym7xhDg5aEgiywi375noTHX',
    issuanceDate: Date, //'2021-10-05T08:58:33Z'  
    proof: {
      type: string, // example 'JcsEd25519Signature2020',
      verificationMethod:  string, // example '#newKey',
      signatureValue:  string(88), // example '56aGDSPBTW3p1AJQWnn1eRwixfGBfq1Gpj3NtSjhA2Qqgk3LrpeNYocvmV73ru4WLRYHPTyHwVKSGfd6JbSsyFRZ'
    }
  },
  holder:  string(53), // 'did:iota:4SgsnbN67cJDwGgpU4woVYNY37kfN4Dr8rSfyDPbVTsR'
}
```

</details>

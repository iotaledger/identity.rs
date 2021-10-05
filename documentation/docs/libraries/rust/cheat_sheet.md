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
##### Returns

* Ed255119 KeyPair.

```js
{
  type: string,// example 'ed25519'
  public: string(44),// example '5Geq8HMRHxBhyesEbzbq8nG78ZRXFcHRUVhny4kfrHRh',
  secret: string(44),// example 'GeKQa6EhNXkbo74JLuvyxRFR3rouy2iPViwtk9JM9Dyn'
}
```
####  identity::prelude::IotaDocument::from_keypair(keypair)

Create a new DID Document from the given KeyPair.
The DID Document will be pre-populated with a single authentication method based on the provided KeyPair.

```rs
IotaDocument::from_keypair(keypair:KeyPair)?;
```

#### Returns

* IotaDocument

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


####  identity::prelude::IotaDocument::from_keypair_with_network(keypair, network)

Create a new DID Document from the given KeyPair and network.
The DID Document will be pre-populated with a single authentication method based on the provided KeyPair.

```rs
IotaDocument::from_keypair_with_network(keypair:KeyPair, network: str)?;
```

#### Returns

* IotaDocument

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

### [Publish](../../decentralized_identifiers/create.mdx)

####  identity::prelude::Client.publish_document(document)

Publish an IotaDocument to the Tangle using a Client.

```rs
Client.publishDocument(document: IotaDocument);
```

#### Returns

* Receipt

```js
 {
    network: string,// example Mainnet,
    message_id: MessageId,// example MessageId(a4b879ea338041560319ee0aad6826e5ea2a4872f25c59cca091c2a03d87b325),
    network_id: int, // example 1454675179895816119,
    nonce: int, // example 7225257,
}

```
### [Update](../../decentralized_identifiers/update.mdx)

#### identity::prelude::IotaDocument.insert_method(scope, method)

Add a new VerificationMethod to a IotaDocument.

```rs
IotaDocument.insert_method(scope:  identity::did::MethodScope, method: identity::iota::IotaVerificationMethod)
```

##### Returns

* Boolean.

#### identity::prelude::IotaDocument.insert_service(service)

Add a new Service to a IotaDocument.

```rs
IotaDocument.insert_service(service: identity::did::Service)
```

##### Returns

* Boolean.

### [Resolve](../../decentralized_identifiers/resolve.mdx)

####  identity::did::resolution::resolve(did, input, method)

Resolves a DID into a DID Document by using the “Read” operation of the DID method.

```rs
resolution::resolve(did: str, input: InputMetadata, method: R)
```

##### Returns

* IotaDocument

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
#### identity::prelude::Client::resolve_history(did: &'_ IotaDID)

Returns the MessageHistory of the given IotaDID.

```rs
client.resolve_history(did: IotaDID)
```

##### Returns

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

## Verifiable Credentials (VC)

A Verifiable Credential can be verified by anyone, allowing you to take control of it and share it with anyone.

### [Create](../../verifiable_credentials/create.mdx)

#### identity::prelude::common::issue_degree(issuer, subject)

Take two IotaDocuments for issuer and subject, and creates an unsigned credential with claims about subject by issuer.

```rs
common::issue_degree(issuer: IotaDocument, subject: IotaDocument)
```

#### Returns

* VerifiableCredential

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

### [Sign](../../verifiable_credentials/create.mdx)

####  identity::prelude::IotaDocument.sign_data(data, private_key)

Sign the provided data with the default authentication method.

```rs
IotaDocument.sign_data(data: X, private_key: Key)
```

#### Returns

* VerifiableCredential

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

### [Revoke](../../verifiable_credentials/revoke.mdx)

#### identity::prelude::IotaDocument.remove_method(did)

Removes all references to the specified Verification Method,  effectively revoking the VC as it will no longer be able to verify.

```rs
IotaDocument.removeMethod(did: IotaDID)
```

##### Returns

* Void

####  identity::iota::IotaVerificationMethod.revoke_merkle_key(index)

Revokes the public key of a Merkle Key Collection at the specified index, instead of revoking the entire verification method.

```rs
IotaVerificationMethod.revoke_merkle_key(index: usize)
```

##### Returns

* Boolean.

## Verifiable Presentations (VP)

A Verifiable Presentation is the format in which you can share a (collection of) Verifiable Credential(s). It is signed by the subject, to prove control over the Verifiable Credential with a nonce or timestamp.

### [Create](../../verifiable_credentials/verifiable_presentations.mdx)

#### identity::credential::PresentationBuilder.build()

Returns a new Presentation based on the PresentationBuilder configuration. 

````rs
PresentationBuilder::default().build()
````

##### Returns

* Verifiable Presentation

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

### [Sign](../../verifiable_credentials/verifiable_presentations.mdx)

####  identity::prelude::IotaDocument.sign_data(data, private_key)

Sign the provided data (in this case a Presentation) with the default authentication method.

```rs
IotaDocument.sign_data(data: Presentaton, private_key: Key)
```
##### Returns

* Verifiable Presentation

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
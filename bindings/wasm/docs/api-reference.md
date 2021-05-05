## Classes

<dl>
<dt><a href="#DID">DID</a></dt>
<dd></dd>
<dt><a href="#Document">Document</a></dt>
<dd></dd>
<dt><a href="#KeyCollection">KeyCollection</a></dt>
<dd></dd>
<dt><a href="#KeyPair">KeyPair</a></dt>
<dd></dd>
<dt><a href="#NewDocument">NewDocument</a></dt>
<dd></dd>
<dt><a href="#Service">Service</a></dt>
<dd></dd>
<dt><a href="#VerifiableCredential">VerifiableCredential</a></dt>
<dd></dd>
<dt><a href="#VerifiablePresentation">VerifiablePresentation</a></dt>
<dd></dd>
<dt><a href="#VerificationMethod">VerificationMethod</a></dt>
<dd></dd>
</dl>

## Members

<dl>
<dt><a href="#Digest">Digest</a></dt>
<dd></dd>
<dt><a href="#KeyType">KeyType</a></dt>
<dd></dd>
</dl>

## Functions

<dl>
<dt><a href="#start">start()</a></dt>
<dd><p>Initializes the console error panic hook for better error messages</p>
</dd>
<dt><a href="#publish">publish(document, params)</a> ⇒ <code>any</code></dt>
<dd><p>Publishes a DID Document to the Tangle, params looks like { node: &quot;<a href="http://localhost:14265&quot;">http://localhost:14265&quot;</a>, network: &quot;main&quot; }</p>
</dd>
<dt><a href="#resolve">resolve(did, params)</a> ⇒ <code>any</code></dt>
<dd><p>Resolves the latest DID Document from the Tangle, params looks like { node: &quot;<a href="http://localhost:14265&quot;">http://localhost:14265&quot;</a>, network: &quot;main&quot; }</p>
</dd>
<dt><a href="#checkCredential">checkCredential(data, params)</a> ⇒ <code>any</code></dt>
<dd><p>Validates a credential with the DID Document from the Tangle, params looks like { node: &quot;<a href="http://localhost:14265&quot;">http://localhost:14265&quot;</a>, network: &quot;main&quot; }</p>
</dd>
<dt><a href="#checkPresentation">checkPresentation(data, params)</a> ⇒ <code>any</code></dt>
<dd><p>Validates a presentation with the DID Document from the Tangle, params looks like { node: &quot;<a href="http://localhost:14265&quot;">http://localhost:14265&quot;</a>, network: &quot;main&quot; }</p>
</dd>
</dl>

<a name="DID"></a>

## DID
**Kind**: global class  

* [DID](#DID)
    * [new DID(key, network, shard)](#new_DID_new)
    * _instance_
        * [.network](#DID+network) ⇒ <code>string</code>
        * [.shard](#DID+shard) ⇒ <code>string</code> \| <code>undefined</code>
        * [.tag](#DID+tag) ⇒ <code>string</code>
        * [.toString()](#DID+toString) ⇒ <code>string</code>
    * _static_
        * [.fromBase58(key, network, shard)](#DID.fromBase58) ⇒ [<code>DID</code>](#DID)
        * [.parse(input)](#DID.parse) ⇒ [<code>DID</code>](#DID)

<a name="new_DID_new"></a>

### new DID(key, network, shard)
Creates a new `DID` from a `KeyPair` object.


| Param | Type |
| --- | --- |
| key | [<code>KeyPair</code>](#KeyPair) | 
| network | <code>string</code> \| <code>undefined</code> | 
| shard | <code>string</code> \| <code>undefined</code> | 

<a name="DID+network"></a>

### did.network ⇒ <code>string</code>
Returns the IOTA tangle network of the `DID`.

**Kind**: instance property of [<code>DID</code>](#DID)  
<a name="DID+shard"></a>

### did.shard ⇒ <code>string</code> \| <code>undefined</code>
Returns the IOTA tangle shard of the `DID` (if any).

**Kind**: instance property of [<code>DID</code>](#DID)  
<a name="DID+tag"></a>

### did.tag ⇒ <code>string</code>
Returns the unique tag of the `DID`.

**Kind**: instance property of [<code>DID</code>](#DID)  
<a name="DID+toString"></a>

### did.toString() ⇒ <code>string</code>
Returns the `DID` object as a string.

**Kind**: instance method of [<code>DID</code>](#DID)  
<a name="DID.fromBase58"></a>

### DID.fromBase58(key, network, shard) ⇒ [<code>DID</code>](#DID)
Creates a new `DID` from a base58-encoded public key.

**Kind**: static method of [<code>DID</code>](#DID)  

| Param | Type |
| --- | --- |
| key | <code>string</code> | 
| network | <code>string</code> \| <code>undefined</code> | 
| shard | <code>string</code> \| <code>undefined</code> | 

<a name="DID.parse"></a>

### DID.parse(input) ⇒ [<code>DID</code>](#DID)
Parses a `DID` from the input string.

**Kind**: static method of [<code>DID</code>](#DID)  

| Param | Type |
| --- | --- |
| input | <code>string</code> | 

<a name="Document"></a>

## Document
**Kind**: global class  

* [Document](#Document)
    * [new Document(type_, network, tag)](#new_Document_new)
    * _instance_
        * [.id](#Document+id) ⇒ [<code>DID</code>](#DID)
        * [.proof](#Document+proof) ⇒ <code>any</code>
        * [.previousMessageId](#Document+previousMessageId) ⇒ <code>string</code>
        * [.setPreviousMessageId](#Document+setPreviousMessageId)
        * [.insertMethod(method, scope)](#Document+insertMethod) ⇒ <code>boolean</code>
        * [.removeMethod(did)](#Document+removeMethod)
        * [.insertService(service)](#Document+insertService) ⇒ <code>boolean</code>
        * [.removeService(did)](#Document+removeService)
        * [.sign(key)](#Document+sign)
        * [.verify()](#Document+verify) ⇒ <code>boolean</code>
        * [.signCredential(data, args)](#Document+signCredential) ⇒ [<code>VerifiableCredential</code>](#VerifiableCredential)
        * [.signPresentation(data, args)](#Document+signPresentation) ⇒ [<code>VerifiablePresentation</code>](#VerifiablePresentation)
        * [.signData(data, args)](#Document+signData) ⇒ <code>any</code>
        * [.verifyData(data)](#Document+verifyData) ⇒ <code>boolean</code>
        * [.resolveKey(query)](#Document+resolveKey) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
        * [.revokeMerkleKey(query, index)](#Document+revokeMerkleKey) ⇒ <code>boolean</code>
        * [.diff(other, message, key)](#Document+diff) ⇒ <code>any</code>
        * [.merge(diff)](#Document+merge)
        * [.toJSON()](#Document+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromKeyPair(key)](#Document.fromKeyPair) ⇒ [<code>Document</code>](#Document)
        * [.fromAuthentication(method)](#Document.fromAuthentication) ⇒ [<code>Document</code>](#Document)
        * [.fromJSON(json)](#Document.fromJSON) ⇒ [<code>Document</code>](#Document)

<a name="new_Document_new"></a>

### new Document(type_, network, tag)
Creates a new DID Document from the given KeyPair.


| Param | Type |
| --- | --- |
| type_ | <code>number</code> | 
| network | <code>string</code> \| <code>undefined</code> |
| tag | <code>string</code> \| <code>undefined</code> | 

<a name="Document+id"></a>

### document.id ⇒ [<code>DID</code>](#DID)
Returns the DID Document `id`.

**Kind**: instance property of [<code>Document</code>](#Document)  
<a name="Document+proof"></a>

### document.proof ⇒ <code>any</code>
Returns the DID Document `proof` object.

**Kind**: instance property of [<code>Document</code>](#Document)  
<a name="Document+previousMessageId"></a>

### document.previousMessageId ⇒ <code>string</code>
**Kind**: instance property of [<code>Document</code>](#Document)
<a name="Document+setPreviousMessageId"></a>

### document.setPreviousMessageId
**Kind**: instance property of [<code>Document</code>](#Document)

| Param | Type |
| --- | --- |
| value | <code>string</code> |

<a name="Document+insertMethod"></a>

### document.insertMethod(method, scope) ⇒ <code>boolean</code>
**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| method | [<code>VerificationMethod</code>](#VerificationMethod) | 
| scope | <code>string</code> \| <code>undefined</code> | 

<a name="Document+removeMethod"></a>

### document.removeMethod(did)
**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| did | [<code>DID</code>](#DID) | 

<a name="Document+insertService"></a>

### document.insertService(service) ⇒ <code>boolean</code>
**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| service | [<code>Service</code>](#Service) | 

<a name="Document+removeService"></a>

### document.removeService(did)
**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| did | [<code>DID</code>](#DID) | 

<a name="Document+sign"></a>

### document.sign(key)
Signs the DID Document with the default authentication method.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| key | [<code>KeyPair</code>](#KeyPair) | 

<a name="Document+verify"></a>

### document.verify() ⇒ <code>boolean</code>
Verify the signature with the authentication_key

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+signCredential"></a>

### document.signCredential(data, args) ⇒ [<code>VerifiableCredential</code>](#VerifiableCredential)
**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| data | <code>any</code> | 
| args | <code>any</code> | 

<a name="Document+signPresentation"></a>

### document.signPresentation(data, args) ⇒ [<code>VerifiablePresentation</code>](#VerifiablePresentation)
**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| data | <code>any</code> | 
| args | <code>any</code> | 

<a name="Document+signData"></a>

### document.signData(data, args) ⇒ <code>any</code>
Creates a signature for the given `data` with the specified DID Document
Verification Method.

An additional `proof` property is required if using a Merkle Key
Collection verification Method.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| data | <code>any</code> | 
| args | <code>any</code> | 

<a name="Document+verifyData"></a>

### document.verifyData(data) ⇒ <code>boolean</code>
Verifies the authenticity of `data` using the target verification method.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| data | <code>any</code> | 

<a name="Document+resolveKey"></a>

### document.resolveKey(query) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| query | <code>string</code> | 

<a name="Document+revokeMerkleKey"></a>

### document.revokeMerkleKey(query, index) ⇒ <code>boolean</code>
**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| query | <code>string</code> | 
| index | <code>number</code> | 

<a name="Document+diff"></a>

### document.diff(other, message, key) ⇒ <code>any</code>
Generate the difference between two DID Documents and sign it

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| other | [<code>Document</code>](#Document) | 
| message | <code>string</code> | 
| key | [<code>KeyPair</code>](#KeyPair) | 

<a name="Document+merge"></a>

### document.merge(diff)
Verifies the `diff` signature and merges the changes into `self`.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| diff | <code>string</code> | 

<a name="Document+toJSON"></a>

### document.toJSON() ⇒ <code>any</code>
Serializes a `Document` object as a JSON object.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document.fromKeyPair"></a>

### Document.fromKeyPair(key) ⇒ [<code>Document</code>](#Document)
Creates a new DID Document from the given KeyPair.

**Kind**: static method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| key | [<code>KeyPair</code>](#KeyPair) | 

<a name="Document.fromAuthentication"></a>

### Document.fromAuthentication(method) ⇒ [<code>Document</code>](#Document)
Creates a new DID Document from the given verification [`method`][`Method`].

**Kind**: static method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| method | [<code>VerificationMethod</code>](#VerificationMethod) | 

<a name="Document.fromJSON"></a>

### Document.fromJSON(json) ⇒ [<code>Document</code>](#Document)
Deserializes a `Document` object from a JSON object.

**Kind**: static method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="KeyCollection"></a>

## KeyCollection
**Kind**: global class  

* [KeyCollection](#KeyCollection)
    * [new KeyCollection(type_, count)](#new_KeyCollection_new)
    * _instance_
        * [.length](#KeyCollection+length) ⇒ <code>number</code>
        * [.isEmpty()](#KeyCollection+isEmpty) ⇒ <code>boolean</code>
        * [.keypair(index)](#KeyCollection+keypair) ⇒ [<code>KeyPair</code>](#KeyPair) \| <code>undefined</code>
        * [.public(index)](#KeyCollection+public) ⇒ <code>string</code> \| <code>undefined</code>
        * [.secret(index)](#KeyCollection+secret) ⇒ <code>string</code> \| <code>undefined</code>
        * [.merkleRoot(digest)](#KeyCollection+merkleRoot) ⇒ <code>string</code>
        * [.merkleProof(digest, index)](#KeyCollection+merkleProof) ⇒ <code>string</code> \| <code>undefined</code>
        * [.toJSON()](#KeyCollection+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#KeyCollection.fromJSON) ⇒ [<code>KeyCollection</code>](#KeyCollection)

<a name="new_KeyCollection_new"></a>

### new KeyCollection(type_, count)
Creates a new `KeyCollection` with the specified key type.


| Param | Type |
| --- | --- |
| type_ | <code>number</code> | 
| count | <code>number</code> | 

<a name="KeyCollection+length"></a>

### keyCollection.length ⇒ <code>number</code>
Returns the number of keys in the collection.

**Kind**: instance property of [<code>KeyCollection</code>](#KeyCollection)  
<a name="KeyCollection+isEmpty"></a>

### keyCollection.isEmpty() ⇒ <code>boolean</code>
Returns `true` if the collection contains no keys.

**Kind**: instance method of [<code>KeyCollection</code>](#KeyCollection)  
<a name="KeyCollection+keypair"></a>

### keyCollection.keypair(index) ⇒ [<code>KeyPair</code>](#KeyPair) \| <code>undefined</code>
Returns the keypair at the specified `index`.

**Kind**: instance method of [<code>KeyCollection</code>](#KeyCollection)  

| Param | Type |
| --- | --- |
| index | <code>number</code> | 

<a name="KeyCollection+public"></a>

### keyCollection.public(index) ⇒ <code>string</code> \| <code>undefined</code>
Returns the public key at the specified `index` as a base58-encoded string.

**Kind**: instance method of [<code>KeyCollection</code>](#KeyCollection)  

| Param | Type |
| --- | --- |
| index | <code>number</code> | 

<a name="KeyCollection+secret"></a>

### keyCollection.secret(index) ⇒ <code>string</code> \| <code>undefined</code>
Returns the secret key at the specified `index` as a base58-encoded string.

**Kind**: instance method of [<code>KeyCollection</code>](#KeyCollection)  

| Param | Type |
| --- | --- |
| index | <code>number</code> | 

<a name="KeyCollection+merkleRoot"></a>

### keyCollection.merkleRoot(digest) ⇒ <code>string</code>
**Kind**: instance method of [<code>KeyCollection</code>](#KeyCollection)  

| Param | Type |
| --- | --- |
| digest | <code>number</code> | 

<a name="KeyCollection+merkleProof"></a>

### keyCollection.merkleProof(digest, index) ⇒ <code>string</code> \| <code>undefined</code>
**Kind**: instance method of [<code>KeyCollection</code>](#KeyCollection)  

| Param | Type |
| --- | --- |
| digest | <code>number</code> | 
| index | <code>number</code> | 

<a name="KeyCollection+toJSON"></a>

### keyCollection.toJSON() ⇒ <code>any</code>
Serializes a `KeyCollection` object as a JSON object.

**Kind**: instance method of [<code>KeyCollection</code>](#KeyCollection)  
<a name="KeyCollection.fromJSON"></a>

### KeyCollection.fromJSON(json) ⇒ [<code>KeyCollection</code>](#KeyCollection)
Deserializes a `KeyCollection` object from a JSON object.

**Kind**: static method of [<code>KeyCollection</code>](#KeyCollection)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="KeyPair"></a>

## KeyPair
**Kind**: global class  

* [KeyPair](#KeyPair)
    * [new KeyPair(type_)](#new_KeyPair_new)
    * _instance_
        * [.public](#KeyPair+public) ⇒ <code>string</code>
        * [.secret](#KeyPair+secret) ⇒ <code>string</code>
        * [.toJSON()](#KeyPair+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromBase58(type_, public_key, secret_key)](#KeyPair.fromBase58) ⇒ [<code>KeyPair</code>](#KeyPair)
        * [.fromJSON(json)](#KeyPair.fromJSON) ⇒ [<code>KeyPair</code>](#KeyPair)

<a name="new_KeyPair_new"></a>

### new KeyPair(type_)
Generates a new `KeyPair` object.


| Param | Type |
| --- | --- |
| type_ | <code>number</code> | 

<a name="KeyPair+public"></a>

### keyPair.public ⇒ <code>string</code>
Returns the public key as a base58-encoded string.

**Kind**: instance property of [<code>KeyPair</code>](#KeyPair)  
<a name="KeyPair+secret"></a>

### keyPair.secret ⇒ <code>string</code>
Returns the secret key as a base58-encoded string.

**Kind**: instance property of [<code>KeyPair</code>](#KeyPair)  
<a name="KeyPair+toJSON"></a>

### keyPair.toJSON() ⇒ <code>any</code>
Serializes a `KeyPair` object as a JSON object.

**Kind**: instance method of [<code>KeyPair</code>](#KeyPair)  
<a name="KeyPair.fromBase58"></a>

### KeyPair.fromBase58(type_, public_key, secret_key) ⇒ [<code>KeyPair</code>](#KeyPair)
Parses a `KeyPair` object from base58-encoded public/secret keys.

**Kind**: static method of [<code>KeyPair</code>](#KeyPair)  

| Param | Type |
| --- | --- |
| type_ | <code>number</code> | 
| public_key | <code>string</code> | 
| secret_key | <code>string</code> | 

<a name="KeyPair.fromJSON"></a>

### KeyPair.fromJSON(json) ⇒ [<code>KeyPair</code>](#KeyPair)
Deserializes a `KeyPair` object from a JSON object.

**Kind**: static method of [<code>KeyPair</code>](#KeyPair)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="NewDocument"></a>

## NewDocument
**Kind**: global class  

* [NewDocument](#NewDocument)
    * [.key](#NewDocument+key) ⇒ [<code>KeyPair</code>](#KeyPair)
    * [.doc](#NewDocument+doc) ⇒ [<code>Document</code>](#Document)

<a name="NewDocument+key"></a>

### newDocument.key ⇒ [<code>KeyPair</code>](#KeyPair)
**Kind**: instance property of [<code>NewDocument</code>](#NewDocument)  
<a name="NewDocument+doc"></a>

### newDocument.doc ⇒ [<code>Document</code>](#Document)
**Kind**: instance property of [<code>NewDocument</code>](#NewDocument)  
<a name="Service"></a>

## Service
**Kind**: global class  

* [Service](#Service)
    * _instance_
        * [.toJSON()](#Service+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(value)](#Service.fromJSON) ⇒ [<code>Service</code>](#Service)

<a name="Service+toJSON"></a>

### service.toJSON() ⇒ <code>any</code>
Serializes a `Service` object as a JSON object.

**Kind**: instance method of [<code>Service</code>](#Service)  
<a name="Service.fromJSON"></a>

### Service.fromJSON(value) ⇒ [<code>Service</code>](#Service)
Deserializes a `Method` object from a JSON object.

**Kind**: static method of [<code>Service</code>](#Service)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="VerifiableCredential"></a>

## VerifiableCredential
**Kind**: global class  

* [VerifiableCredential](#VerifiableCredential)
    * _instance_
        * [.toJSON()](#VerifiableCredential+toJSON) ⇒ <code>any</code>
    * _static_
        * [.extend(value)](#VerifiableCredential.extend) ⇒ [<code>VerifiableCredential</code>](#VerifiableCredential)
        * [.issue(issuer_doc, subject_data, credential_type, credential_id)](#VerifiableCredential.issue) ⇒ [<code>VerifiableCredential</code>](#VerifiableCredential)
        * [.fromJSON(json)](#VerifiableCredential.fromJSON) ⇒ [<code>VerifiableCredential</code>](#VerifiableCredential)

<a name="VerifiableCredential+toJSON"></a>

### verifiableCredential.toJSON() ⇒ <code>any</code>
Serializes a `VerifiableCredential` object as a JSON object.

**Kind**: instance method of [<code>VerifiableCredential</code>](#VerifiableCredential)  
<a name="VerifiableCredential.extend"></a>

### VerifiableCredential.extend(value) ⇒ [<code>VerifiableCredential</code>](#VerifiableCredential)
**Kind**: static method of [<code>VerifiableCredential</code>](#VerifiableCredential)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="VerifiableCredential.issue"></a>

### VerifiableCredential.issue(issuer_doc, subject_data, credential_type, credential_id) ⇒ [<code>VerifiableCredential</code>](#VerifiableCredential)
**Kind**: static method of [<code>VerifiableCredential</code>](#VerifiableCredential)  

| Param | Type |
| --- | --- |
| issuer_doc | [<code>Document</code>](#Document) | 
| subject_data | <code>any</code> | 
| credential_type | <code>string</code> \| <code>undefined</code> | 
| credential_id | <code>string</code> \| <code>undefined</code> | 

<a name="VerifiableCredential.fromJSON"></a>

### VerifiableCredential.fromJSON(json) ⇒ [<code>VerifiableCredential</code>](#VerifiableCredential)
Deserializes a `VerifiableCredential` object from a JSON object.

**Kind**: static method of [<code>VerifiableCredential</code>](#VerifiableCredential)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="VerifiablePresentation"></a>

## VerifiablePresentation
**Kind**: global class  

* [VerifiablePresentation](#VerifiablePresentation)
    * [new VerifiablePresentation(holder_doc, credential_data, presentation_type, presentation_id)](#new_VerifiablePresentation_new)
    * _instance_
        * [.toJSON()](#VerifiablePresentation+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#VerifiablePresentation.fromJSON) ⇒ [<code>VerifiablePresentation</code>](#VerifiablePresentation)

<a name="new_VerifiablePresentation_new"></a>

### new VerifiablePresentation(holder_doc, credential_data, presentation_type, presentation_id)

| Param | Type |
| --- | --- |
| holder_doc | [<code>Document</code>](#Document) | 
| credential_data | <code>any</code> | 
| presentation_type | <code>string</code> \| <code>undefined</code> | 
| presentation_id | <code>string</code> \| <code>undefined</code> | 

<a name="VerifiablePresentation+toJSON"></a>

### verifiablePresentation.toJSON() ⇒ <code>any</code>
Serializes a `VerifiablePresentation` object as a JSON object.

**Kind**: instance method of [<code>VerifiablePresentation</code>](#VerifiablePresentation)  
<a name="VerifiablePresentation.fromJSON"></a>

### VerifiablePresentation.fromJSON(json) ⇒ [<code>VerifiablePresentation</code>](#VerifiablePresentation)
Deserializes a `VerifiablePresentation` object from a JSON object.

**Kind**: static method of [<code>VerifiablePresentation</code>](#VerifiablePresentation)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="VerificationMethod"></a>

## VerificationMethod
**Kind**: global class  

* [VerificationMethod](#VerificationMethod)
    * [new VerificationMethod(key, tag)](#new_VerificationMethod_new)
    * _instance_
        * [.id](#VerificationMethod+id) ⇒ [<code>DID</code>](#DID)
        * [.controller](#VerificationMethod+controller) ⇒ [<code>DID</code>](#DID)
        * [.type](#VerificationMethod+type) ⇒ <code>string</code>
        * [.data](#VerificationMethod+data) ⇒ <code>any</code>
        * [.toJSON()](#VerificationMethod+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromDID(did, key, tag)](#VerificationMethod.fromDID) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
        * [.createMerkleKey(digest, did, keys, tag)](#VerificationMethod.createMerkleKey) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
        * [.fromJSON(value)](#VerificationMethod.fromJSON) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)

<a name="new_VerificationMethod_new"></a>

### new VerificationMethod(key, tag)
Creates a new `VerificationMethod` object from the given `key`.


| Param | Type |
| --- | --- |
| key | [<code>KeyPair</code>](#KeyPair) | 
| tag | <code>string</code> \| <code>undefined</code> | 

<a name="VerificationMethod+id"></a>

### verificationMethod.id ⇒ [<code>DID</code>](#DID)
Returns the `id` DID of the `VerificationMethod` object.

**Kind**: instance property of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod+controller"></a>

### verificationMethod.controller ⇒ [<code>DID</code>](#DID)
Returns the `controller` DID of the `VerificationMethod` object.

**Kind**: instance property of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod+type"></a>

### verificationMethod.type ⇒ <code>string</code>
Returns the `VerificationMethod` type.

**Kind**: instance property of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod+data"></a>

### verificationMethod.data ⇒ <code>any</code>
Returns the `VerificationMethod` public key data.

**Kind**: instance property of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod+toJSON"></a>

### verificationMethod.toJSON() ⇒ <code>any</code>
Serializes a `VerificationMethod` object as a JSON object.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod.fromDID"></a>

### VerificationMethod.fromDID(did, key, tag) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
Creates a new `VerificationMethod` object from the given `did` and `key`.

**Kind**: static method of [<code>VerificationMethod</code>](#VerificationMethod)  

| Param | Type |
| --- | --- |
| did | [<code>DID</code>](#DID) | 
| key | [<code>KeyPair</code>](#KeyPair) | 
| tag | <code>string</code> \| <code>undefined</code> | 

<a name="VerificationMethod.createMerkleKey"></a>

### VerificationMethod.createMerkleKey(digest, did, keys, tag) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
Creates a new Merkle Key Collection Method from the given key collection.

**Kind**: static method of [<code>VerificationMethod</code>](#VerificationMethod)  

| Param | Type |
| --- | --- |
| digest | <code>number</code> | 
| did | [<code>DID</code>](#DID) | 
| keys | [<code>KeyCollection</code>](#KeyCollection) | 
| tag | <code>string</code> \| <code>undefined</code> | 

<a name="VerificationMethod.fromJSON"></a>

### VerificationMethod.fromJSON(value) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
Deserializes a `VerificationMethod` object from a JSON object.

**Kind**: static method of [<code>VerificationMethod</code>](#VerificationMethod)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="Digest"></a>

## Digest
**Kind**: global variable  
<a name="KeyType"></a>

## KeyType
**Kind**: global variable  
<a name="start"></a>

## start()
Initializes the console error panic hook for better error messages

**Kind**: global function  
<a name="publish"></a>

## publish(document, params) ⇒ <code>any</code>
Publishes a DID Document to the Tangle, params looks like { node: "http://localhost:14265", network: "main" }

**Kind**: global function  

| Param | Type |
| --- | --- |
| document | <code>any</code> | 
| params | <code>any</code> | 

<a name="resolve"></a>

## resolve(did, params) ⇒ <code>any</code>
Resolves the latest DID Document from the Tangle, params looks like { node: "http://localhost:14265", network: "main" }

**Kind**: global function  

| Param | Type |
| --- | --- |
| did | <code>string</code> | 
| params | <code>any</code> | 

<a name="checkCredential"></a>

## checkCredential(data, params) ⇒ <code>any</code>
Validates a credential with the DID Document from the Tangle, params looks like { node: "http://localhost:14265", network: "main" }

**Kind**: global function  

| Param | Type |
| --- | --- |
| data | <code>string</code> | 
| params | <code>any</code> | 

<a name="checkPresentation"></a>

## checkPresentation(data, params) ⇒ <code>any</code>
Validates a presentation with the DID Document from the Tangle, params looks like { node: "http://localhost:14265", network: "main" }

**Kind**: global function  

| Param | Type |
| --- | --- |
| data | <code>string</code> | 
| params | <code>any</code> | 


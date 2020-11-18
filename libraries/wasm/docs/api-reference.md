## Classes

<dl>
<dt><a href="#DID">DID</a></dt>
<dd></dd>
<dt><a href="#Doc">Doc</a></dt>
<dd></dd>
<dt><a href="#Key">Key</a></dt>
<dd></dd>
<dt><a href="#NewDoc">NewDoc</a></dt>
<dd></dd>
<dt><a href="#PubKey">PubKey</a></dt>
<dd></dd>
<dt><a href="#VerifiableCredential">VerifiableCredential</a></dt>
<dd></dd>
<dt><a href="#VerifiablePresentation">VerifiablePresentation</a></dt>
<dd></dd>
</dl>

## Functions

<dl>
<dt><a href="#publish">publish(doc, params)</a> ⇒ <code>any</code></dt>
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
<dt><a href="#start">start()</a></dt>
<dd><p>Initializes the console error panic hook for better error messages</p>
</dd>
</dl>

<a name="DID"></a>

## DID
**Kind**: global class  

* [DID](#DID)
    * [new DID(key, network)](#new_DID_new)
    * _instance_
        * [.network](#DID+network) ⇒ <code>string</code>
        * [.shard](#DID+shard) ⇒ <code>string</code> \| <code>undefined</code>
        * [.tag](#DID+tag) ⇒ <code>string</code>
        * [.address](#DID+address) ⇒ <code>string</code>
        * [.toString()](#DID+toString) ⇒ <code>string</code>
    * _static_
        * [.fromBase58Key(key, network)](#DID.fromBase58Key) ⇒ [<code>DID</code>](#DID)
        * [.fromBase64Key(key, network)](#DID.fromBase64Key) ⇒ [<code>DID</code>](#DID)
        * [.parse(input)](#DID.parse) ⇒ [<code>DID</code>](#DID)

<a name="new_DID_new"></a>

### new DID(key, network)
Creates a new `DID` from a `Key` object.


| Param | Type |
| --- | --- |
| key | [<code>Key</code>](#Key) | 
| network | <code>string</code> \| <code>undefined</code> | 

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
<a name="DID+address"></a>

### did.address ⇒ <code>string</code>
Returns the IOTA tangle address of the `DID`.

**Kind**: instance property of [<code>DID</code>](#DID)  
<a name="DID+toString"></a>

### did.toString() ⇒ <code>string</code>
Returns the `DID` object as a string.

**Kind**: instance method of [<code>DID</code>](#DID)  
<a name="DID.fromBase58Key"></a>

### DID.fromBase58Key(key, network) ⇒ [<code>DID</code>](#DID)
Creates a new `DID` from a base58-encoded public key.

**Kind**: static method of [<code>DID</code>](#DID)  

| Param | Type |
| --- | --- |
| key | <code>string</code> | 
| network | <code>string</code> \| <code>undefined</code> | 

<a name="DID.fromBase64Key"></a>

### DID.fromBase64Key(key, network) ⇒ [<code>DID</code>](#DID)
Creates a new `DID` from a base64-encoded public key.

**Kind**: static method of [<code>DID</code>](#DID)  

| Param | Type |
| --- | --- |
| key | <code>string</code> | 
| network | <code>string</code> \| <code>undefined</code> | 

<a name="DID.parse"></a>

### DID.parse(input) ⇒ [<code>DID</code>](#DID)
Parses a `DID` from the input string.

**Kind**: static method of [<code>DID</code>](#DID)  

| Param | Type |
| --- | --- |
| input | <code>string</code> | 

<a name="Doc"></a>

## Doc
**Kind**: global class  

* [Doc](#Doc)
    * [new Doc(authentication)](#new_Doc_new)
    * _instance_
        * [.id](#Doc+id) ⇒ <code>string</code>
        * [.authChain](#Doc+authChain) ⇒ <code>string</code>
        * [.diffChain](#Doc+diffChain) ⇒ <code>string</code>
        * [.proof](#Doc+proof) ⇒ <code>any</code>
        * [.sign(key)](#Doc+sign) ⇒ <code>any</code>
        * [.verify()](#Doc+verify) ⇒ <code>boolean</code>
        * [.diff(other, key)](#Doc+diff) ⇒ <code>any</code>
        * [.verifyDiff(diff)](#Doc+verifyDiff) ⇒ <code>boolean</code>
        * [.updateService(did, url, service_type)](#Doc+updateService)
        * [.clearServices()](#Doc+clearServices)
        * [.updateAuth(public_key)](#Doc+updateAuth) ⇒ <code>boolean</code>
        * [.clearAuth()](#Doc+clearAuth)
        * [.updateAssert(public_key)](#Doc+updateAssert) ⇒ <code>boolean</code>
        * [.clearAssert()](#Doc+clearAssert)
        * [.updateVerification(public_key)](#Doc+updateVerification) ⇒ <code>boolean</code>
        * [.clearVerification()](#Doc+clearVerification)
        * [.updateDelegation(public_key)](#Doc+updateDelegation) ⇒ <code>boolean</code>
        * [.clearDelegation()](#Doc+clearDelegation)
        * [.updateInvocation(public_key)](#Doc+updateInvocation) ⇒ <code>boolean</code>
        * [.clearInvocation()](#Doc+clearInvocation)
        * [.updateAgreement(public_key)](#Doc+updateAgreement) ⇒ <code>boolean</code>
        * [.clearAgreement()](#Doc+clearAgreement)
        * [.resolveKey(ident, scope)](#Doc+resolveKey) ⇒ [<code>PubKey</code>](#PubKey)
        * [.toJSON()](#Doc+toJSON) ⇒ <code>any</code>
    * _static_
        * [.generateRandom(key_type, network, tag)](#Doc.generateRandom) ⇒ [<code>NewDoc</code>](#NewDoc)
        * [.generateEd25519(network, tag)](#Doc.generateEd25519) ⇒ [<code>NewDoc</code>](#NewDoc)
        * [.fromJSON(json)](#Doc.fromJSON) ⇒ [<code>Doc</code>](#Doc)

<a name="new_Doc_new"></a>

### new Doc(authentication)

| Param | Type |
| --- | --- |
| authentication | [<code>PubKey</code>](#PubKey) | 

<a name="Doc+id"></a>

### doc.id ⇒ <code>string</code>
**Kind**: instance property of [<code>Doc</code>](#Doc)  
<a name="Doc+authChain"></a>

### doc.authChain ⇒ <code>string</code>
**Kind**: instance property of [<code>Doc</code>](#Doc)  
<a name="Doc+diffChain"></a>

### doc.diffChain ⇒ <code>string</code>
**Kind**: instance property of [<code>Doc</code>](#Doc)  
<a name="Doc+proof"></a>

### doc.proof ⇒ <code>any</code>
**Kind**: instance property of [<code>Doc</code>](#Doc)  
<a name="Doc+sign"></a>

### doc.sign(key) ⇒ <code>any</code>
**Kind**: instance method of [<code>Doc</code>](#Doc)  

| Param | Type |
| --- | --- |
| key | [<code>Key</code>](#Key) | 

<a name="Doc+verify"></a>

### doc.verify() ⇒ <code>boolean</code>
Verify the signature with the authentication_key

**Kind**: instance method of [<code>Doc</code>](#Doc)  
<a name="Doc+diff"></a>

### doc.diff(other, key) ⇒ <code>any</code>
Generate the difference between two DID Documents and sign it

**Kind**: instance method of [<code>Doc</code>](#Doc)  

| Param | Type |
| --- | --- |
| other | [<code>Doc</code>](#Doc) | 
| key | [<code>Key</code>](#Key) | 

<a name="Doc+verifyDiff"></a>

### doc.verifyDiff(diff) ⇒ <code>boolean</code>
Verify the signature in a diff with the authentication_key

**Kind**: instance method of [<code>Doc</code>](#Doc)  

| Param | Type |
| --- | --- |
| diff | <code>string</code> | 

<a name="Doc+updateService"></a>

### doc.updateService(did, url, service_type)
**Kind**: instance method of [<code>Doc</code>](#Doc)  

| Param | Type |
| --- | --- |
| did | [<code>DID</code>](#DID) | 
| url | <code>string</code> | 
| service_type | <code>string</code> | 

<a name="Doc+clearServices"></a>

### doc.clearServices()
**Kind**: instance method of [<code>Doc</code>](#Doc)  
<a name="Doc+updateAuth"></a>

### doc.updateAuth(public_key) ⇒ <code>boolean</code>
**Kind**: instance method of [<code>Doc</code>](#Doc)  

| Param | Type |
| --- | --- |
| public_key | [<code>PubKey</code>](#PubKey) | 

<a name="Doc+clearAuth"></a>

### doc.clearAuth()
**Kind**: instance method of [<code>Doc</code>](#Doc)  
<a name="Doc+updateAssert"></a>

### doc.updateAssert(public_key) ⇒ <code>boolean</code>
**Kind**: instance method of [<code>Doc</code>](#Doc)  

| Param | Type |
| --- | --- |
| public_key | [<code>PubKey</code>](#PubKey) | 

<a name="Doc+clearAssert"></a>

### doc.clearAssert()
**Kind**: instance method of [<code>Doc</code>](#Doc)  
<a name="Doc+updateVerification"></a>

### doc.updateVerification(public_key) ⇒ <code>boolean</code>
**Kind**: instance method of [<code>Doc</code>](#Doc)  

| Param | Type |
| --- | --- |
| public_key | [<code>PubKey</code>](#PubKey) | 

<a name="Doc+clearVerification"></a>

### doc.clearVerification()
**Kind**: instance method of [<code>Doc</code>](#Doc)  
<a name="Doc+updateDelegation"></a>

### doc.updateDelegation(public_key) ⇒ <code>boolean</code>
**Kind**: instance method of [<code>Doc</code>](#Doc)  

| Param | Type |
| --- | --- |
| public_key | [<code>PubKey</code>](#PubKey) | 

<a name="Doc+clearDelegation"></a>

### doc.clearDelegation()
**Kind**: instance method of [<code>Doc</code>](#Doc)  
<a name="Doc+updateInvocation"></a>

### doc.updateInvocation(public_key) ⇒ <code>boolean</code>
**Kind**: instance method of [<code>Doc</code>](#Doc)  

| Param | Type |
| --- | --- |
| public_key | [<code>PubKey</code>](#PubKey) | 

<a name="Doc+clearInvocation"></a>

### doc.clearInvocation()
**Kind**: instance method of [<code>Doc</code>](#Doc)  
<a name="Doc+updateAgreement"></a>

### doc.updateAgreement(public_key) ⇒ <code>boolean</code>
**Kind**: instance method of [<code>Doc</code>](#Doc)  

| Param | Type |
| --- | --- |
| public_key | [<code>PubKey</code>](#PubKey) | 

<a name="Doc+clearAgreement"></a>

### doc.clearAgreement()
**Kind**: instance method of [<code>Doc</code>](#Doc)  
<a name="Doc+resolveKey"></a>

### doc.resolveKey(ident, scope) ⇒ [<code>PubKey</code>](#PubKey)
**Kind**: instance method of [<code>Doc</code>](#Doc)  

| Param | Type |
| --- | --- |
| ident | <code>any</code> | 
| scope | <code>string</code> \| <code>undefined</code> | 

<a name="Doc+toJSON"></a>

### doc.toJSON() ⇒ <code>any</code>
Serializes a `Doc` object as a JSON object.

**Kind**: instance method of [<code>Doc</code>](#Doc)  
<a name="Doc.generateRandom"></a>

### Doc.generateRandom(key_type, network, tag) ⇒ [<code>NewDoc</code>](#NewDoc)
Generates a keypair and DID Document, supported key_type is "Ed25519VerificationKey2018"

**Kind**: static method of [<code>Doc</code>](#Doc)  

| Param | Type |
| --- | --- |
| key_type | <code>string</code> | 
| network | <code>string</code> \| <code>undefined</code> | 
| tag | <code>string</code> \| <code>undefined</code> | 

<a name="Doc.generateEd25519"></a>

### Doc.generateEd25519(network, tag) ⇒ [<code>NewDoc</code>](#NewDoc)
Generates an Ed25519 keypair and DID Document

**Kind**: static method of [<code>Doc</code>](#Doc)  

| Param | Type |
| --- | --- |
| network | <code>string</code> \| <code>undefined</code> | 
| tag | <code>string</code> \| <code>undefined</code> | 

<a name="Doc.fromJSON"></a>

### Doc.fromJSON(json) ⇒ [<code>Doc</code>](#Doc)
Deserializes a `Doc` object from a JSON object.

**Kind**: static method of [<code>Doc</code>](#Doc)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="Key"></a>

## Key
**Kind**: global class  

* [Key](#Key)
    * [new Key(key_type)](#new_Key_new)
    * _instance_
        * [.public](#Key+public) ⇒ <code>string</code>
        * [.private](#Key+private) ⇒ <code>string</code>
        * [.toJSON()](#Key+toJSON) ⇒ <code>any</code>
    * _static_
        * [.generateEd25519()](#Key.generateEd25519) ⇒ [<code>Key</code>](#Key)
        * [.fromBase58(public_key, private_key)](#Key.fromBase58) ⇒ [<code>Key</code>](#Key)
        * [.fromBase64(public_key, private_key)](#Key.fromBase64) ⇒ [<code>Key</code>](#Key)
        * [.fromJSON(json)](#Key.fromJSON) ⇒ [<code>Key</code>](#Key)

<a name="new_Key_new"></a>

### new Key(key_type)
Generates a new `Key` object.


| Param | Type |
| --- | --- |
| key_type | <code>string</code> | 

<a name="Key+public"></a>

### key.public ⇒ <code>string</code>
Returns the public key as a base58-encoded string.

**Kind**: instance property of [<code>Key</code>](#Key)  
<a name="Key+private"></a>

### key.private ⇒ <code>string</code>
Returns the private key as a base58-encoded string.

**Kind**: instance property of [<code>Key</code>](#Key)  
<a name="Key+toJSON"></a>

### key.toJSON() ⇒ <code>any</code>
Serializes a `Key` object as a JSON object.

**Kind**: instance method of [<code>Key</code>](#Key)  
<a name="Key.generateEd25519"></a>

### Key.generateEd25519() ⇒ [<code>Key</code>](#Key)
Generates a new `Key` object suitable for ed25519 signatures.

**Kind**: static method of [<code>Key</code>](#Key)  
<a name="Key.fromBase58"></a>

### Key.fromBase58(public_key, private_key) ⇒ [<code>Key</code>](#Key)
Parses a `Key` object from base58-encoded public/private keys.

**Kind**: static method of [<code>Key</code>](#Key)  

| Param | Type |
| --- | --- |
| public_key | <code>string</code> | 
| private_key | <code>string</code> | 

<a name="Key.fromBase64"></a>

### Key.fromBase64(public_key, private_key) ⇒ [<code>Key</code>](#Key)
Parses a `Key` object from base64-encoded public/private keys.

**Kind**: static method of [<code>Key</code>](#Key)  

| Param | Type |
| --- | --- |
| public_key | <code>string</code> | 
| private_key | <code>string</code> | 

<a name="Key.fromJSON"></a>

### Key.fromJSON(json) ⇒ [<code>Key</code>](#Key)
Deserializes a `Key` object from a JSON object.

**Kind**: static method of [<code>Key</code>](#Key)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="NewDoc"></a>

## NewDoc
**Kind**: global class  

* [NewDoc](#NewDoc)
    * [.key](#NewDoc+key) ⇒ [<code>Key</code>](#Key)
    * [.doc](#NewDoc+doc) ⇒ [<code>Doc</code>](#Doc)

<a name="NewDoc+key"></a>

### newDoc.key ⇒ [<code>Key</code>](#Key)
**Kind**: instance property of [<code>NewDoc</code>](#NewDoc)  
<a name="NewDoc+doc"></a>

### newDoc.doc ⇒ [<code>Doc</code>](#Doc)
**Kind**: instance property of [<code>NewDoc</code>](#NewDoc)  
<a name="PubKey"></a>

## PubKey
**Kind**: global class  

* [PubKey](#PubKey)
    * [new PubKey(did, key_type, key_data, tag)](#new_PubKey_new)
    * _instance_
        * [.id](#PubKey+id) ⇒ [<code>DID</code>](#DID)
        * [.controller](#PubKey+controller) ⇒ [<code>DID</code>](#DID)
        * [.toJSON()](#PubKey+toJSON) ⇒ <code>any</code>
    * _static_
        * [.generateEd25519(did, key_data, tag)](#PubKey.generateEd25519) ⇒ [<code>PubKey</code>](#PubKey)
        * [.fromJSON(json)](#PubKey.fromJSON) ⇒ [<code>PubKey</code>](#PubKey)

<a name="new_PubKey_new"></a>

### new PubKey(did, key_type, key_data, tag)

| Param | Type |
| --- | --- |
| did | [<code>DID</code>](#DID) | 
| key_type | <code>string</code> | 
| key_data | <code>string</code> | 
| tag | <code>string</code> \| <code>undefined</code> | 

<a name="PubKey+id"></a>

### pubKey.id ⇒ [<code>DID</code>](#DID)
Returns the `id` DID of the `PubKey` object.

**Kind**: instance property of [<code>PubKey</code>](#PubKey)  
<a name="PubKey+controller"></a>

### pubKey.controller ⇒ [<code>DID</code>](#DID)
Returns the `controller` DID of the `PubKey` object.

**Kind**: instance property of [<code>PubKey</code>](#PubKey)  
<a name="PubKey+toJSON"></a>

### pubKey.toJSON() ⇒ <code>any</code>
Serializes a `PubKey` object as a JSON object.

**Kind**: instance method of [<code>PubKey</code>](#PubKey)  
<a name="PubKey.generateEd25519"></a>

### PubKey.generateEd25519(did, key_data, tag) ⇒ [<code>PubKey</code>](#PubKey)
Generates a new `PubKey` object suitable for ed25519 signatures.

**Kind**: static method of [<code>PubKey</code>](#PubKey)  

| Param | Type |
| --- | --- |
| did | [<code>DID</code>](#DID) | 
| key_data | <code>string</code> | 
| tag | <code>string</code> \| <code>undefined</code> | 

<a name="PubKey.fromJSON"></a>

### PubKey.fromJSON(json) ⇒ [<code>PubKey</code>](#PubKey)
Deserializes a `PubKey` object from a JSON object.

**Kind**: static method of [<code>PubKey</code>](#PubKey)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="VerifiableCredential"></a>

## VerifiableCredential
**Kind**: global class  

* [VerifiableCredential](#VerifiableCredential)
    * [new VerifiableCredential(issuer_doc, issuer_key, subject_data, credential_type, credential_id)](#new_VerifiableCredential_new)
    * _instance_
        * [.sign(issuer, key)](#VerifiableCredential+sign)
        * [.verify(issuer)](#VerifiableCredential+verify) ⇒ <code>boolean</code>
        * [.toJSON()](#VerifiableCredential+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#VerifiableCredential.fromJSON) ⇒ [<code>VerifiableCredential</code>](#VerifiableCredential)

<a name="new_VerifiableCredential_new"></a>

### new VerifiableCredential(issuer_doc, issuer_key, subject_data, credential_type, credential_id)

| Param | Type |
| --- | --- |
| issuer_doc | [<code>Doc</code>](#Doc) | 
| issuer_key | [<code>Key</code>](#Key) | 
| subject_data | <code>any</code> | 
| credential_type | <code>string</code> \| <code>undefined</code> | 
| credential_id | <code>string</code> \| <code>undefined</code> | 

<a name="VerifiableCredential+sign"></a>

### verifiableCredential.sign(issuer, key)
Signs the credential with the given issuer `Doc` and `Key` object.

**Kind**: instance method of [<code>VerifiableCredential</code>](#VerifiableCredential)  

| Param | Type |
| --- | --- |
| issuer | [<code>Doc</code>](#Doc) | 
| key | [<code>Key</code>](#Key) | 

<a name="VerifiableCredential+verify"></a>

### verifiableCredential.verify(issuer) ⇒ <code>boolean</code>
Verifies the credential signature against the issuer `Doc`.

**Kind**: instance method of [<code>VerifiableCredential</code>](#VerifiableCredential)  

| Param | Type |
| --- | --- |
| issuer | [<code>Doc</code>](#Doc) | 

<a name="VerifiableCredential+toJSON"></a>

### verifiableCredential.toJSON() ⇒ <code>any</code>
Serializes a `VerifiableCredential` object as a JSON object.

**Kind**: instance method of [<code>VerifiableCredential</code>](#VerifiableCredential)  
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
    * [new VerifiablePresentation(holder_doc, holder_key, credential_data, presentation_type, presentation_id)](#new_VerifiablePresentation_new)
    * _instance_
        * [.sign(holder, key)](#VerifiablePresentation+sign)
        * [.verify(holder)](#VerifiablePresentation+verify) ⇒ <code>boolean</code>
        * [.toJSON()](#VerifiablePresentation+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#VerifiablePresentation.fromJSON) ⇒ [<code>VerifiablePresentation</code>](#VerifiablePresentation)

<a name="new_VerifiablePresentation_new"></a>

### new VerifiablePresentation(holder_doc, holder_key, credential_data, presentation_type, presentation_id)

| Param | Type |
| --- | --- |
| holder_doc | [<code>Doc</code>](#Doc) | 
| holder_key | [<code>Key</code>](#Key) | 
| credential_data | <code>any</code> | 
| presentation_type | <code>string</code> \| <code>undefined</code> | 
| presentation_id | <code>string</code> \| <code>undefined</code> | 

<a name="VerifiablePresentation+sign"></a>

### verifiablePresentation.sign(holder, key)
Signs the presentation with the given holder `Doc` and `Key` object.

**Kind**: instance method of [<code>VerifiablePresentation</code>](#VerifiablePresentation)  

| Param | Type |
| --- | --- |
| holder | [<code>Doc</code>](#Doc) | 
| key | [<code>Key</code>](#Key) | 

<a name="VerifiablePresentation+verify"></a>

### verifiablePresentation.verify(holder) ⇒ <code>boolean</code>
Verifies the presentation signature against the holder `Doc`.

**Kind**: instance method of [<code>VerifiablePresentation</code>](#VerifiablePresentation)  

| Param | Type |
| --- | --- |
| holder | [<code>Doc</code>](#Doc) | 

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

<a name="publish"></a>

## publish(doc, params) ⇒ <code>any</code>
Publishes a DID Document to the Tangle, params looks like { node: "http://localhost:14265", network: "main" }

**Kind**: global function  

| Param | Type |
| --- | --- |
| doc | <code>any</code> | 
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

<a name="start"></a>

## start()
Initializes the console error panic hook for better error messages

**Kind**: global function  

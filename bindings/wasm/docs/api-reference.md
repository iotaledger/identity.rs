## Classes

<dl>
<dt><a href="#CoreDID">CoreDID</a></dt>
<dd><p>A method-agnostic Decentralized Identifier (DID).</p>
</dd>
<dt><a href="#CoreDocument">CoreDocument</a></dt>
<dd><p>A method-agnostic DID Document.</p>
</dd>
<dt><a href="#Credential">Credential</a></dt>
<dd></dd>
<dt><a href="#CredentialValidationOptions">CredentialValidationOptions</a></dt>
<dd><p>Options to declare validation criteria when validating credentials.</p>
</dd>
<dt><a href="#CredentialValidator">CredentialValidator</a></dt>
<dd></dd>
<dt><a href="#DIDUrl">DIDUrl</a></dt>
<dd><p>A method agnostic DID Url.</p>
</dd>
<dt><a href="#DomainLinkageConfiguration">DomainLinkageConfiguration</a></dt>
<dd><p>DID Configuration Resource which contains Domain Linkage Credentials.
It can be placed in an origin&#39;s <code>.well-known</code> directory to prove linkage between the origin and a DID.
See: <a href="https://identity.foundation/.well-known/resources/did-configuration/#did-configuration-resource">https://identity.foundation/.well-known/resources/did-configuration/#did-configuration-resource</a></p>
<p>Note:</p>
<ul>
<li>Only <a href="https://identity.foundation/.well-known/resources/did-configuration/#linked-data-proof-format">Linked Data Proof Format</a>
is supported.</li>
</ul>
</dd>
<dt><a href="#DomainLinkageValidator">DomainLinkageValidator</a></dt>
<dd><p>A validator for a Domain Linkage Configuration and Credentials.</p>
</dd>
<dt><a href="#Duration">Duration</a></dt>
<dd><p>A span of time.</p>
</dd>
<dt><a href="#Ed25519">Ed25519</a></dt>
<dd></dd>
<dt><a href="#IotaDID">IotaDID</a></dt>
<dd><p>A DID conforming to the IOTA DID method specification.</p>
</dd>
<dt><a href="#IotaDocument">IotaDocument</a></dt>
<dd></dd>
<dt><a href="#IotaDocumentMetadata">IotaDocumentMetadata</a></dt>
<dd><p>Additional attributes related to an IOTA DID Document.</p>
</dd>
<dt><a href="#IotaIdentityClientExt">IotaIdentityClientExt</a></dt>
<dd><p>An extension interface that provides helper functions for publication
and resolution of DID documents in Alias Outputs.</p>
</dd>
<dt><a href="#KeyPair">KeyPair</a></dt>
<dd></dd>
<dt><a href="#LinkedDomainService">LinkedDomainService</a></dt>
<dd></dd>
<dt><a href="#MethodData">MethodData</a></dt>
<dd><p>Supported verification method data formats.</p>
</dd>
<dt><a href="#MethodScope">MethodScope</a></dt>
<dd><p>Supported verification method types.</p>
</dd>
<dt><a href="#MethodType">MethodType</a></dt>
<dd><p>Supported verification method types.</p>
</dd>
<dt><a href="#Presentation">Presentation</a></dt>
<dd></dd>
<dt><a href="#PresentationValidationOptions">PresentationValidationOptions</a></dt>
<dd><p>Options to declare validation criteria when validating presentation.</p>
</dd>
<dt><a href="#PresentationValidator">PresentationValidator</a></dt>
<dd></dd>
<dt><a href="#Proof">Proof</a></dt>
<dd><p>A digital signature.</p>
<p>For field definitions see: <a href="https://w3c-ccg.github.io/security-vocab/">https://w3c-ccg.github.io/security-vocab/</a></p>
</dd>
<dt><a href="#ProofOptions">ProofOptions</a></dt>
<dd><p>Holds additional options for creating signatures.
See <code>IProofOptions</code>.</p>
</dd>
<dt><a href="#ProofPurpose">ProofPurpose</a></dt>
<dd><p>Associates a purpose with a <a href="#Proof">Proof</a>.</p>
<p>See <a href="https://w3c-ccg.github.io/security-vocab/#proofPurpose">https://w3c-ccg.github.io/security-vocab/#proofPurpose</a></p>
</dd>
<dt><a href="#Resolver">Resolver</a></dt>
<dd><p>Convenience type for resolving DID documents from different DID methods.</p>
<p>Also provides methods for resolving DID Documents associated with
verifiable <code>Credentials</code> and <code>Presentations</code>.</p>
<h1 id="configuration">Configuration</h1>
<p>The resolver will only be able to resolve DID documents for methods it has been configured for in the constructor.</p>
</dd>
<dt><a href="#RevocationBitmap">RevocationBitmap</a></dt>
<dd><p>A compressed bitmap for managing credential revocation.</p>
</dd>
<dt><a href="#Service">Service</a></dt>
<dd><p>A DID Document Service used to enable trusted interactions associated with a DID subject.</p>
</dd>
<dt><a href="#Timestamp">Timestamp</a></dt>
<dd></dd>
<dt><a href="#VerificationMethod">VerificationMethod</a></dt>
<dd><p>A DID Document Verification Method.</p>
</dd>
<dt><a href="#VerifierOptions">VerifierOptions</a></dt>
<dd><p>Holds additional proof verification options.
See <code>IVerifierOptions</code>.</p>
</dd>
<dt><a href="#X25519">X25519</a></dt>
<dd><p>An implementation of <code>X25519</code> Elliptic-curve Diffie-Hellman (ECDH) cryptographic key exchange.</p>
</dd>
</dl>

## Members

<dl>
<dt><a href="#KeyType">KeyType</a></dt>
<dd></dd>
<dt><a href="#MethodRelationship">MethodRelationship</a></dt>
<dd></dd>
<dt><a href="#StatusCheck">StatusCheck</a></dt>
<dd><p>Controls validation behaviour when checking whether or not a credential has been revoked by its
<a href="https://www.w3.org/TR/vc-data-model/#status"><code>credentialStatus</code></a>.</p>
</dd>
<dt><a href="#Strict">Strict</a></dt>
<dd><p>Validate the status if supported, reject any unsupported
<a href="https://www.w3.org/TR/vc-data-model/#status"><code>credentialStatus</code></a> types.</p>
<p>Only <code>RevocationBitmap2022</code> is currently supported.</p>
<p>This is the default.</p>
</dd>
<dt><a href="#SkipUnsupported">SkipUnsupported</a></dt>
<dd><p>Validate the status if supported, skip any unsupported
<a href="https://www.w3.org/TR/vc-data-model/#status"><code>credentialStatus</code></a> types.</p>
</dd>
<dt><a href="#SkipAll">SkipAll</a></dt>
<dd><p>Skip all status checks.</p>
</dd>
<dt><a href="#SubjectHolderRelationship">SubjectHolderRelationship</a></dt>
<dd><p>Declares how credential subjects must relate to the presentation holder during validation.
See <code>PresentationValidationOptions::subject_holder_relationship</code>.</p>
<p>See also the <a href="https://www.w3.org/TR/vc-data-model/#subject-holder-relationships">Subject-Holder Relationship</a> section of the specification.</p>
</dd>
<dt><a href="#AlwaysSubject">AlwaysSubject</a></dt>
<dd><p>The holder must always match the subject on all credentials, regardless of their <a href="https://www.w3.org/TR/vc-data-model/#nontransferable-property"><code>nonTransferable</code></a> property.
This variant is the default used if no other variant is specified when constructing a new
<code>PresentationValidationOptions</code>.</p>
</dd>
<dt><a href="#SubjectOnNonTransferable">SubjectOnNonTransferable</a></dt>
<dd><p>The holder must match the subject only for credentials where the <a href="https://www.w3.org/TR/vc-data-model/#nontransferable-property"><code>nonTransferable</code></a> property is <code>true</code>.</p>
</dd>
<dt><a href="#Any">Any</a></dt>
<dd><p>The holder is not required to have any kind of relationship to any credential subject.</p>
</dd>
<dt><a href="#FailFast">FailFast</a></dt>
<dd><p>Declares when validation should return if an error occurs.</p>
</dd>
<dt><a href="#AllErrors">AllErrors</a></dt>
<dd><p>Return all errors that occur during validation.</p>
</dd>
<dt><a href="#FirstError">FirstError</a></dt>
<dd><p>Return after the first error occurs.</p>
</dd>
<dt><a href="#StateMetadataEncoding">StateMetadataEncoding</a></dt>
<dd></dd>
</dl>

## Functions

<dl>
<dt><a href="#start">start()</a></dt>
<dd><p>Initializes the console error panic hook for better error messages</p>
</dd>
</dl>

<a name="CoreDID"></a>

## CoreDID
A method-agnostic Decentralized Identifier (DID).

**Kind**: global class  

* [CoreDID](#CoreDID)
    * _instance_
        * [.setMethodName(value)](#CoreDID+setMethodName)
        * [.setMethodId(value)](#CoreDID+setMethodId)
        * [.scheme()](#CoreDID+scheme) ⇒ <code>string</code>
        * [.authority()](#CoreDID+authority) ⇒ <code>string</code>
        * [.method()](#CoreDID+method) ⇒ <code>string</code>
        * [.methodId()](#CoreDID+methodId) ⇒ <code>string</code>
        * [.join(segment)](#CoreDID+join) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.toUrl()](#CoreDID+toUrl) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.intoUrl()](#CoreDID+intoUrl) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.toString()](#CoreDID+toString) ⇒ <code>string</code>
        * [.toCoreDid()](#CoreDID+toCoreDid) ⇒ [<code>CoreDID</code>](#CoreDID)
        * [.toJSON()](#CoreDID+toJSON) ⇒ <code>any</code>
        * [.clone()](#CoreDID+clone) ⇒ [<code>CoreDID</code>](#CoreDID)
    * _static_
        * [.parse(input)](#CoreDID.parse) ⇒ [<code>CoreDID</code>](#CoreDID)
        * [.validMethodName(value)](#CoreDID.validMethodName) ⇒ <code>boolean</code>
        * [.validMethodId(value)](#CoreDID.validMethodId) ⇒ <code>boolean</code>
        * [.fromJSON(json)](#CoreDID.fromJSON) ⇒ [<code>CoreDID</code>](#CoreDID)

<a name="CoreDID+setMethodName"></a>

### coreDID.setMethodName(value)
Set the method name of the `CoreDID`.

**Kind**: instance method of [<code>CoreDID</code>](#CoreDID)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="CoreDID+setMethodId"></a>

### coreDID.setMethodId(value)
Set the method-specific-id of the `DID`.

**Kind**: instance method of [<code>CoreDID</code>](#CoreDID)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="CoreDID+scheme"></a>

### coreDID.scheme() ⇒ <code>string</code>
Returns the `CoreDID` scheme.

E.g.
- `"did:example:12345678" -> "did"`
- `"did:iota:smr:12345678" -> "did"`

**Kind**: instance method of [<code>CoreDID</code>](#CoreDID)  
<a name="CoreDID+authority"></a>

### coreDID.authority() ⇒ <code>string</code>
Returns the `CoreDID` authority: the method name and method-id.

E.g.
- `"did:example:12345678" -> "example:12345678"`
- `"did:iota:smr:12345678" -> "iota:smr:12345678"`

**Kind**: instance method of [<code>CoreDID</code>](#CoreDID)  
<a name="CoreDID+method"></a>

### coreDID.method() ⇒ <code>string</code>
Returns the `CoreDID` method name.

E.g.
- `"did:example:12345678" -> "example"`
- `"did:iota:smr:12345678" -> "iota"`

**Kind**: instance method of [<code>CoreDID</code>](#CoreDID)  
<a name="CoreDID+methodId"></a>

### coreDID.methodId() ⇒ <code>string</code>
Returns the `CoreDID` method-specific ID.

E.g.
- `"did:example:12345678" -> "12345678"`
- `"did:iota:smr:12345678" -> "smr:12345678"`

**Kind**: instance method of [<code>CoreDID</code>](#CoreDID)  
<a name="CoreDID+join"></a>

### coreDID.join(segment) ⇒ [<code>DIDUrl</code>](#DIDUrl)
Construct a new `DIDUrl` by joining with a relative DID Url string.

**Kind**: instance method of [<code>CoreDID</code>](#CoreDID)  

| Param | Type |
| --- | --- |
| segment | <code>string</code> | 

<a name="CoreDID+toUrl"></a>

### coreDID.toUrl() ⇒ [<code>DIDUrl</code>](#DIDUrl)
Clones the `CoreDID` into a `DIDUrl`.

**Kind**: instance method of [<code>CoreDID</code>](#CoreDID)  
<a name="CoreDID+intoUrl"></a>

### coreDID.intoUrl() ⇒ [<code>DIDUrl</code>](#DIDUrl)
Converts the `CoreDID` into a `DIDUrl`, consuming it.

**Kind**: instance method of [<code>CoreDID</code>](#CoreDID)  
<a name="CoreDID+toString"></a>

### coreDID.toString() ⇒ <code>string</code>
Returns the `CoreDID` as a string.

**Kind**: instance method of [<code>CoreDID</code>](#CoreDID)  
<a name="CoreDID+toCoreDid"></a>

### coreDID.toCoreDid() ⇒ [<code>CoreDID</code>](#CoreDID)
**Kind**: instance method of [<code>CoreDID</code>](#CoreDID)  
<a name="CoreDID+toJSON"></a>

### coreDID.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>CoreDID</code>](#CoreDID)  
<a name="CoreDID+clone"></a>

### coreDID.clone() ⇒ [<code>CoreDID</code>](#CoreDID)
Deep clones the object.

**Kind**: instance method of [<code>CoreDID</code>](#CoreDID)  
<a name="CoreDID.parse"></a>

### CoreDID.parse(input) ⇒ [<code>CoreDID</code>](#CoreDID)
Parses a `CoreDID` from the given `input`.

### Errors

Throws an error if the input is not a valid `CoreDID`.

**Kind**: static method of [<code>CoreDID</code>](#CoreDID)  

| Param | Type |
| --- | --- |
| input | <code>string</code> | 

<a name="CoreDID.validMethodName"></a>

### CoreDID.validMethodName(value) ⇒ <code>boolean</code>
Validates whether a string is a valid DID method name.

**Kind**: static method of [<code>CoreDID</code>](#CoreDID)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="CoreDID.validMethodId"></a>

### CoreDID.validMethodId(value) ⇒ <code>boolean</code>
Validates whether a string is a valid `DID` method-id.

**Kind**: static method of [<code>CoreDID</code>](#CoreDID)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="CoreDID.fromJSON"></a>

### CoreDID.fromJSON(json) ⇒ [<code>CoreDID</code>](#CoreDID)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>CoreDID</code>](#CoreDID)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="CoreDocument"></a>

## CoreDocument
A method-agnostic DID Document.

**Kind**: global class  

* [CoreDocument](#CoreDocument)
    * [new CoreDocument(values)](#new_CoreDocument_new)
    * _instance_
        * [.id()](#CoreDocument+id) ⇒ [<code>CoreDID</code>](#CoreDID)
        * [.setId(id)](#CoreDocument+setId)
        * [.controller()](#CoreDocument+controller) ⇒ [<code>Array.&lt;CoreDID&gt;</code>](#CoreDID)
        * [.setController(controllers)](#CoreDocument+setController)
        * [.alsoKnownAs()](#CoreDocument+alsoKnownAs) ⇒ <code>Array.&lt;string&gt;</code>
        * [.setAlsoKnownAs(urls)](#CoreDocument+setAlsoKnownAs)
        * [.verificationMethod()](#CoreDocument+verificationMethod) ⇒ [<code>Array.&lt;VerificationMethod&gt;</code>](#VerificationMethod)
        * [.authentication()](#CoreDocument+authentication) ⇒ <code>Array.&lt;(DIDUrl\|VerificationMethod)&gt;</code>
        * [.assertionMethod()](#CoreDocument+assertionMethod) ⇒ <code>Array.&lt;(DIDUrl\|VerificationMethod)&gt;</code>
        * [.keyAgreement()](#CoreDocument+keyAgreement) ⇒ <code>Array.&lt;(DIDUrl\|VerificationMethod)&gt;</code>
        * [.capabilityDelegation()](#CoreDocument+capabilityDelegation) ⇒ <code>Array.&lt;(DIDUrl\|VerificationMethod)&gt;</code>
        * [.capabilityInvocation()](#CoreDocument+capabilityInvocation) ⇒ <code>Array.&lt;(DIDUrl\|VerificationMethod)&gt;</code>
        * [.properties()](#CoreDocument+properties) ⇒ <code>Map.&lt;string, any&gt;</code>
        * [.setPropertyUnchecked(key, value)](#CoreDocument+setPropertyUnchecked)
        * [.service()](#CoreDocument+service) ⇒ [<code>Array.&lt;Service&gt;</code>](#Service)
        * [.insertService(service)](#CoreDocument+insertService)
        * [.removeService(didUrl)](#CoreDocument+removeService) ⇒ [<code>Service</code>](#Service) \| <code>undefined</code>
        * [.resolveService(query)](#CoreDocument+resolveService) ⇒ [<code>Service</code>](#Service) \| <code>undefined</code>
        * [.methods(scope)](#CoreDocument+methods) ⇒ [<code>Array.&lt;VerificationMethod&gt;</code>](#VerificationMethod)
        * [.verificationRelationships()](#CoreDocument+verificationRelationships) ⇒ <code>Array.&lt;(DIDUrl\|VerificationMethod)&gt;</code>
        * [.insertMethod(method, scope)](#CoreDocument+insertMethod)
        * [.removeMethod(did)](#CoreDocument+removeMethod) ⇒ [<code>VerificationMethod</code>](#VerificationMethod) \| <code>undefined</code>
        * [.resolveMethod(query, scope)](#CoreDocument+resolveMethod) ⇒ [<code>VerificationMethod</code>](#VerificationMethod) \| <code>undefined</code>
        * [.attachMethodRelationship(didUrl, relationship)](#CoreDocument+attachMethodRelationship) ⇒ <code>boolean</code>
        * [.detachMethodRelationship(didUrl, relationship)](#CoreDocument+detachMethodRelationship) ⇒ <code>boolean</code>
        * [.verifyData(data, options)](#CoreDocument+verifyData) ⇒ <code>boolean</code>
        * [.revokeCredentials(serviceQuery, indices)](#CoreDocument+revokeCredentials)
        * [.unrevokeCredentials(serviceQuery, indices)](#CoreDocument+unrevokeCredentials)
        * [.signData(data, privateKey, methodQuery, options)](#CoreDocument+signData) ⇒ <code>any</code>
        * [.clone()](#CoreDocument+clone) ⇒ [<code>CoreDocument</code>](#CoreDocument)
        * [._shallowCloneInternal()](#CoreDocument+_shallowCloneInternal) ⇒ [<code>CoreDocument</code>](#CoreDocument)
        * [.toJSON()](#CoreDocument+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#CoreDocument.fromJSON) ⇒ [<code>CoreDocument</code>](#CoreDocument)

<a name="new_CoreDocument_new"></a>

### new CoreDocument(values)
Creates a new `CoreDocument` with the given properties.


| Param | Type |
| --- | --- |
| values | <code>ICoreDocument</code> | 

<a name="CoreDocument+id"></a>

### coreDocument.id() ⇒ [<code>CoreDID</code>](#CoreDID)
Returns a copy of the DID Document `id`.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  
<a name="CoreDocument+setId"></a>

### coreDocument.setId(id)
Sets the DID of the document.

### Warning

Changing the identifier can drastically alter the results of
[`Self::resolve_method`](CoreDocument::resolve_method()),
[`Self::resolve_service`](CoreDocument::resolve_service()) and the related [DID URL dereferencing](https://w3c-ccg.github.io/did-resolution/#dereferencing) algorithm.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| id | [<code>CoreDID</code>](#CoreDID) | 

<a name="CoreDocument+controller"></a>

### coreDocument.controller() ⇒ [<code>Array.&lt;CoreDID&gt;</code>](#CoreDID)
Returns a copy of the document controllers.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  
<a name="CoreDocument+setController"></a>

### coreDocument.setController(controllers)
Sets the controllers of the DID Document.

Note: Duplicates will be ignored.
Use `null` to remove all controllers.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| controllers | [<code>CoreDID</code>](#CoreDID) \| [<code>Array.&lt;CoreDID&gt;</code>](#CoreDID) \| <code>null</code> | 

<a name="CoreDocument+alsoKnownAs"></a>

### coreDocument.alsoKnownAs() ⇒ <code>Array.&lt;string&gt;</code>
Returns a copy of the document's `alsoKnownAs` set.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  
<a name="CoreDocument+setAlsoKnownAs"></a>

### coreDocument.setAlsoKnownAs(urls)
Sets the `alsoKnownAs` property in the DID document.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| urls | <code>string</code> \| <code>Array.&lt;string&gt;</code> \| <code>null</code> | 

<a name="CoreDocument+verificationMethod"></a>

### coreDocument.verificationMethod() ⇒ [<code>Array.&lt;VerificationMethod&gt;</code>](#VerificationMethod)
Returns a copy of the document's `verificationMethod` set.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  
<a name="CoreDocument+authentication"></a>

### coreDocument.authentication() ⇒ <code>Array.&lt;(DIDUrl\|VerificationMethod)&gt;</code>
Returns a copy of the document's `authentication` set.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  
<a name="CoreDocument+assertionMethod"></a>

### coreDocument.assertionMethod() ⇒ <code>Array.&lt;(DIDUrl\|VerificationMethod)&gt;</code>
Returns a copy of the document's `assertionMethod` set.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  
<a name="CoreDocument+keyAgreement"></a>

### coreDocument.keyAgreement() ⇒ <code>Array.&lt;(DIDUrl\|VerificationMethod)&gt;</code>
Returns a copy of the document's `keyAgreement` set.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  
<a name="CoreDocument+capabilityDelegation"></a>

### coreDocument.capabilityDelegation() ⇒ <code>Array.&lt;(DIDUrl\|VerificationMethod)&gt;</code>
Returns a copy of the document's `capabilityDelegation` set.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  
<a name="CoreDocument+capabilityInvocation"></a>

### coreDocument.capabilityInvocation() ⇒ <code>Array.&lt;(DIDUrl\|VerificationMethod)&gt;</code>
Returns a copy of the document's `capabilityInvocation` set.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  
<a name="CoreDocument+properties"></a>

### coreDocument.properties() ⇒ <code>Map.&lt;string, any&gt;</code>
Returns a copy of the custom DID Document properties.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  
<a name="CoreDocument+setPropertyUnchecked"></a>

### coreDocument.setPropertyUnchecked(key, value)
Sets a custom property in the DID Document.
If the value is set to `null`, the custom property will be removed.

### WARNING

This method can overwrite existing properties like `id` and result in an invalid document.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| key | <code>string</code> | 
| value | <code>any</code> | 

<a name="CoreDocument+service"></a>

### coreDocument.service() ⇒ [<code>Array.&lt;Service&gt;</code>](#Service)
Returns a set of all [Service](#Service) in the document.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  
<a name="CoreDocument+insertService"></a>

### coreDocument.insertService(service)
Add a new [Service](#Service) to the document.

Errors if there already exists a service or verification method with the same id.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| service | [<code>Service</code>](#Service) | 

<a name="CoreDocument+removeService"></a>

### coreDocument.removeService(didUrl) ⇒ [<code>Service</code>](#Service) \| <code>undefined</code>
Remove a [Service](#Service) identified by the given [DIDUrl](#DIDUrl) from the document.

Returns `true` if the service was removed.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| didUrl | [<code>DIDUrl</code>](#DIDUrl) | 

<a name="CoreDocument+resolveService"></a>

### coreDocument.resolveService(query) ⇒ [<code>Service</code>](#Service) \| <code>undefined</code>
Returns the first [Service](#Service) with an `id` property matching the provided `query`,
if present.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| query | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 

<a name="CoreDocument+methods"></a>

### coreDocument.methods(scope) ⇒ [<code>Array.&lt;VerificationMethod&gt;</code>](#VerificationMethod)
Returns a list of all [VerificationMethod](#VerificationMethod) in the DID Document,
whose verification relationship matches `scope`.

If `scope` is not set, a list over the **embedded** methods is returned.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| scope | [<code>MethodScope</code>](#MethodScope) \| <code>undefined</code> | 

<a name="CoreDocument+verificationRelationships"></a>

### coreDocument.verificationRelationships() ⇒ <code>Array.&lt;(DIDUrl\|VerificationMethod)&gt;</code>
Returns an array of all verification relationships.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  
<a name="CoreDocument+insertMethod"></a>

### coreDocument.insertMethod(method, scope)
Adds a new `method` to the document in the given `scope`.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| method | [<code>VerificationMethod</code>](#VerificationMethod) | 
| scope | [<code>MethodScope</code>](#MethodScope) | 

<a name="CoreDocument+removeMethod"></a>

### coreDocument.removeMethod(did) ⇒ [<code>VerificationMethod</code>](#VerificationMethod) \| <code>undefined</code>
Removes all references to the specified Verification Method.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| did | [<code>DIDUrl</code>](#DIDUrl) | 

<a name="CoreDocument+resolveMethod"></a>

### coreDocument.resolveMethod(query, scope) ⇒ [<code>VerificationMethod</code>](#VerificationMethod) \| <code>undefined</code>
Returns a copy of the first verification method with an `id` property
matching the provided `query` and the verification relationship
specified by `scope`, if present.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| query | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| scope | [<code>MethodScope</code>](#MethodScope) \| <code>undefined</code> | 

<a name="CoreDocument+attachMethodRelationship"></a>

### coreDocument.attachMethodRelationship(didUrl, relationship) ⇒ <code>boolean</code>
Attaches the relationship to the given method, if the method exists.

Note: The method needs to be in the set of verification methods,
so it cannot be an embedded one.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| didUrl | [<code>DIDUrl</code>](#DIDUrl) | 
| relationship | <code>number</code> | 

<a name="CoreDocument+detachMethodRelationship"></a>

### coreDocument.detachMethodRelationship(didUrl, relationship) ⇒ <code>boolean</code>
Detaches the given relationship from the given method, if the method exists.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| didUrl | [<code>DIDUrl</code>](#DIDUrl) | 
| relationship | <code>number</code> | 

<a name="CoreDocument+verifyData"></a>

### coreDocument.verifyData(data, options) ⇒ <code>boolean</code>
Verifies the authenticity of `data` using the target verification method.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| data | <code>any</code> | 
| options | [<code>VerifierOptions</code>](#VerifierOptions) | 

<a name="CoreDocument+revokeCredentials"></a>

### coreDocument.revokeCredentials(serviceQuery, indices)
If the document has a `RevocationBitmap` service identified by `serviceQuery`,
revoke all specified `indices`.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| serviceQuery | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| indices | <code>number</code> \| <code>Array.&lt;number&gt;</code> | 

<a name="CoreDocument+unrevokeCredentials"></a>

### coreDocument.unrevokeCredentials(serviceQuery, indices)
If the document has a `RevocationBitmap` service identified by `serviceQuery`,
unrevoke all specified `indices`.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| serviceQuery | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| indices | <code>number</code> \| <code>Array.&lt;number&gt;</code> | 

<a name="CoreDocument+signData"></a>

### coreDocument.signData(data, privateKey, methodQuery, options) ⇒ <code>any</code>
Creates a signature for the given `data` with the specified DID Document
Verification Method.

NOTE: use `signSelf` or `signDocument` for DID Documents.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| data | <code>any</code> | 
| privateKey | <code>Uint8Array</code> | 
| methodQuery | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| options | [<code>ProofOptions</code>](#ProofOptions) | 

<a name="CoreDocument+clone"></a>

### coreDocument.clone() ⇒ [<code>CoreDocument</code>](#CoreDocument)
Deep clones the `CoreDocument`.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  
<a name="CoreDocument+_shallowCloneInternal"></a>

### coreDocument.\_shallowCloneInternal() ⇒ [<code>CoreDocument</code>](#CoreDocument)
### Warning
This is for internal use only. Do not rely on or call this method.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  
<a name="CoreDocument+toJSON"></a>

### coreDocument.toJSON() ⇒ <code>any</code>
Serializes to a plain JS representation.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  
<a name="CoreDocument.fromJSON"></a>

### CoreDocument.fromJSON(json) ⇒ [<code>CoreDocument</code>](#CoreDocument)
Deserializes an instance from a plain JS representation.

**Kind**: static method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="Credential"></a>

## Credential
**Kind**: global class  

* [Credential](#Credential)
    * [new Credential(values)](#new_Credential_new)
    * _instance_
        * [.context()](#Credential+context) ⇒ <code>Array.&lt;(string\|Record.&lt;string, any&gt;)&gt;</code>
        * [.id()](#Credential+id) ⇒ <code>string</code> \| <code>undefined</code>
        * [.type()](#Credential+type) ⇒ <code>Array.&lt;string&gt;</code>
        * [.credentialSubject()](#Credential+credentialSubject) ⇒ <code>Array.&lt;Subject&gt;</code>
        * [.issuer()](#Credential+issuer) ⇒ <code>string</code> \| <code>Issuer</code>
        * [.issuanceDate()](#Credential+issuanceDate) ⇒ [<code>Timestamp</code>](#Timestamp)
        * [.expirationDate()](#Credential+expirationDate) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
        * [.credentialStatus()](#Credential+credentialStatus) ⇒ <code>Array.&lt;Status&gt;</code>
        * [.credentialSchema()](#Credential+credentialSchema) ⇒ <code>Array.&lt;Schema&gt;</code>
        * [.refreshService()](#Credential+refreshService) ⇒ <code>Array.&lt;RefreshService&gt;</code>
        * [.termsOfUse()](#Credential+termsOfUse) ⇒ <code>Array.&lt;Policy&gt;</code>
        * [.evidence()](#Credential+evidence) ⇒ <code>Array.&lt;Evidence&gt;</code>
        * [.nonTransferable()](#Credential+nonTransferable) ⇒ <code>boolean</code> \| <code>undefined</code>
        * [.proof()](#Credential+proof) ⇒ [<code>Proof</code>](#Proof) \| <code>undefined</code>
        * [.properties()](#Credential+properties) ⇒ <code>Map.&lt;string, any&gt;</code>
        * [.toJSON()](#Credential+toJSON) ⇒ <code>any</code>
        * [.clone()](#Credential+clone) ⇒ [<code>Credential</code>](#Credential)
    * _static_
        * [.BaseContext()](#Credential.BaseContext) ⇒ <code>string</code>
        * [.BaseType()](#Credential.BaseType) ⇒ <code>string</code>
        * [.createDomainLinkageCredential(values)](#Credential.createDomainLinkageCredential) ⇒ [<code>Credential</code>](#Credential)
        * [.fromJSON(json)](#Credential.fromJSON) ⇒ [<code>Credential</code>](#Credential)

<a name="new_Credential_new"></a>

### new Credential(values)
Constructs a new `Credential`.


| Param | Type |
| --- | --- |
| values | <code>ICredential</code> | 

<a name="Credential+context"></a>

### credential.context() ⇒ <code>Array.&lt;(string\|Record.&lt;string, any&gt;)&gt;</code>
Returns a copy of the JSON-LD context(s) applicable to the `Credential`.

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+id"></a>

### credential.id() ⇒ <code>string</code> \| <code>undefined</code>
Returns a copy of the unique `URI` identifying the `Credential` .

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+type"></a>

### credential.type() ⇒ <code>Array.&lt;string&gt;</code>
Returns a copy of the URIs defining the type of the `Credential`.

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+credentialSubject"></a>

### credential.credentialSubject() ⇒ <code>Array.&lt;Subject&gt;</code>
Returns a copy of the `Credential` subject(s).

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+issuer"></a>

### credential.issuer() ⇒ <code>string</code> \| <code>Issuer</code>
Returns a copy of the issuer of the `Credential`.

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+issuanceDate"></a>

### credential.issuanceDate() ⇒ [<code>Timestamp</code>](#Timestamp)
Returns a copy of the timestamp of when the `Credential` becomes valid.

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+expirationDate"></a>

### credential.expirationDate() ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
Returns a copy of the timestamp of when the `Credential` should no longer be considered valid.

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+credentialStatus"></a>

### credential.credentialStatus() ⇒ <code>Array.&lt;Status&gt;</code>
Returns a copy of the information used to determine the current status of the `Credential`.

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+credentialSchema"></a>

### credential.credentialSchema() ⇒ <code>Array.&lt;Schema&gt;</code>
Returns a copy of the information used to assist in the enforcement of a specific `Credential` structure.

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+refreshService"></a>

### credential.refreshService() ⇒ <code>Array.&lt;RefreshService&gt;</code>
Returns a copy of the service(s) used to refresh an expired `Credential`.

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+termsOfUse"></a>

### credential.termsOfUse() ⇒ <code>Array.&lt;Policy&gt;</code>
Returns a copy of the terms-of-use specified by the `Credential` issuer.

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+evidence"></a>

### credential.evidence() ⇒ <code>Array.&lt;Evidence&gt;</code>
Returns a copy of the human-readable evidence used to support the claims within the `Credential`.

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+nonTransferable"></a>

### credential.nonTransferable() ⇒ <code>boolean</code> \| <code>undefined</code>
Returns whether or not the `Credential` must only be contained within a [Presentation](#Presentation)
with a proof issued from the `Credential` subject.

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+proof"></a>

### credential.proof() ⇒ [<code>Proof</code>](#Proof) \| <code>undefined</code>
Returns a copy of the proof used to verify the `Credential`.

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+properties"></a>

### credential.properties() ⇒ <code>Map.&lt;string, any&gt;</code>
Returns a copy of the miscellaneous properties on the `Credential`.

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+toJSON"></a>

### credential.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+clone"></a>

### credential.clone() ⇒ [<code>Credential</code>](#Credential)
Deep clones the object.

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential.BaseContext"></a>

### Credential.BaseContext() ⇒ <code>string</code>
Returns the base JSON-LD context.

**Kind**: static method of [<code>Credential</code>](#Credential)  
<a name="Credential.BaseType"></a>

### Credential.BaseType() ⇒ <code>string</code>
Returns the base type.

**Kind**: static method of [<code>Credential</code>](#Credential)  
<a name="Credential.createDomainLinkageCredential"></a>

### Credential.createDomainLinkageCredential(values) ⇒ [<code>Credential</code>](#Credential)
**Kind**: static method of [<code>Credential</code>](#Credential)  

| Param | Type |
| --- | --- |
| values | <code>IDomainLinkageCredential</code> | 

<a name="Credential.fromJSON"></a>

### Credential.fromJSON(json) ⇒ [<code>Credential</code>](#Credential)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>Credential</code>](#Credential)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="CredentialValidationOptions"></a>

## CredentialValidationOptions
Options to declare validation criteria when validating credentials.

**Kind**: global class  

* [CredentialValidationOptions](#CredentialValidationOptions)
    * [new CredentialValidationOptions(options)](#new_CredentialValidationOptions_new)
    * _instance_
        * [.toJSON()](#CredentialValidationOptions+toJSON) ⇒ <code>any</code>
        * [.clone()](#CredentialValidationOptions+clone) ⇒ [<code>CredentialValidationOptions</code>](#CredentialValidationOptions)
    * _static_
        * [.default()](#CredentialValidationOptions.default) ⇒ [<code>CredentialValidationOptions</code>](#CredentialValidationOptions)
        * [.fromJSON(json)](#CredentialValidationOptions.fromJSON) ⇒ [<code>CredentialValidationOptions</code>](#CredentialValidationOptions)

<a name="new_CredentialValidationOptions_new"></a>

### new CredentialValidationOptions(options)
Creates a new `CredentialValidationOptions` from the given fields.

Throws an error if any of the options are invalid.


| Param | Type |
| --- | --- |
| options | <code>ICredentialValidationOptions</code> | 

<a name="CredentialValidationOptions+toJSON"></a>

### credentialValidationOptions.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>CredentialValidationOptions</code>](#CredentialValidationOptions)  
<a name="CredentialValidationOptions+clone"></a>

### credentialValidationOptions.clone() ⇒ [<code>CredentialValidationOptions</code>](#CredentialValidationOptions)
Deep clones the object.

**Kind**: instance method of [<code>CredentialValidationOptions</code>](#CredentialValidationOptions)  
<a name="CredentialValidationOptions.default"></a>

### CredentialValidationOptions.default() ⇒ [<code>CredentialValidationOptions</code>](#CredentialValidationOptions)
Creates a new `CredentialValidationOptions` with defaults.

**Kind**: static method of [<code>CredentialValidationOptions</code>](#CredentialValidationOptions)  
<a name="CredentialValidationOptions.fromJSON"></a>

### CredentialValidationOptions.fromJSON(json) ⇒ [<code>CredentialValidationOptions</code>](#CredentialValidationOptions)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>CredentialValidationOptions</code>](#CredentialValidationOptions)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="CredentialValidator"></a>

## CredentialValidator
**Kind**: global class  

* [CredentialValidator](#CredentialValidator)
    * [.validate(credential, issuer, options, fail_fast)](#CredentialValidator.validate)
    * [.checkStructure(credential)](#CredentialValidator.checkStructure)
    * [.checkExpiresOnOrAfter(credential, timestamp)](#CredentialValidator.checkExpiresOnOrAfter)
    * [.checkIssuedOnOrBefore(credential, timestamp)](#CredentialValidator.checkIssuedOnOrBefore)
    * [.verifySignature(credential, trustedIssuers, options)](#CredentialValidator.verifySignature)
    * [.checkSubjectHolderRelationship(credential, holder, relationship)](#CredentialValidator.checkSubjectHolderRelationship)
    * [.checkStatus(credential, trustedIssuers, statusCheck)](#CredentialValidator.checkStatus)
    * [.extractIssuer(credential)](#CredentialValidator.extractIssuer) ⇒ [<code>CoreDID</code>](#CoreDID)

<a name="CredentialValidator.validate"></a>

### CredentialValidator.validate(credential, issuer, options, fail_fast)
Validates a `Credential`.

The following properties are validated according to `options`:
- the issuer's signature,
- the expiration date,
- the issuance date,
- the semantic structure.

### Warning
The lack of an error returned from this method is in of itself not enough to conclude that the credential can be
trusted. This section contains more information on additional checks that should be carried out before and after
calling this method.

#### The state of the issuer's DID Document
The caller must ensure that `issuer` represents an up-to-date DID Document. The convenience method
`Resolver::resolveCredentialIssuer` can help extract the latest available state of the issuer's DID Document.

#### Properties that are not validated
 There are many properties defined in [The Verifiable Credentials Data Model](https://www.w3.org/TR/vc-data-model/) that are **not** validated, such as:
`credentialStatus`, `type`, `credentialSchema`, `refreshService`, **and more**.
These should be manually checked after validation, according to your requirements.

### Errors
An error is returned whenever a validated condition is not satisfied.

**Kind**: static method of [<code>CredentialValidator</code>](#CredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| issuer | [<code>CoreDocument</code>](#CoreDocument) \| <code>IToCoreDocument</code> | 
| options | [<code>CredentialValidationOptions</code>](#CredentialValidationOptions) | 
| fail_fast | <code>number</code> | 

<a name="CredentialValidator.checkStructure"></a>

### CredentialValidator.checkStructure(credential)
Validates the semantic structure of the `Credential`.

### Warning
This does not validate against the credential's schema nor the structure of the subject claims.

**Kind**: static method of [<code>CredentialValidator</code>](#CredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 

<a name="CredentialValidator.checkExpiresOnOrAfter"></a>

### CredentialValidator.checkExpiresOnOrAfter(credential, timestamp)
Validate that the credential expires on or after the specified timestamp.

**Kind**: static method of [<code>CredentialValidator</code>](#CredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| timestamp | [<code>Timestamp</code>](#Timestamp) | 

<a name="CredentialValidator.checkIssuedOnOrBefore"></a>

### CredentialValidator.checkIssuedOnOrBefore(credential, timestamp)
Validate that the credential is issued on or before the specified timestamp.

**Kind**: static method of [<code>CredentialValidator</code>](#CredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| timestamp | [<code>Timestamp</code>](#Timestamp) | 

<a name="CredentialValidator.verifySignature"></a>

### CredentialValidator.verifySignature(credential, trustedIssuers, options)
Verify the signature using the DID Document of a trusted issuer.

# Warning
The caller must ensure that the DID Documents of the trusted issuers are up-to-date.
### Errors
This method immediately returns an error if
the credential issuer' url cannot be parsed to a DID belonging to one of the trusted issuers. Otherwise an attempt
to verify the credential's signature will be made and an error is returned upon failure.

**Kind**: static method of [<code>CredentialValidator</code>](#CredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| trustedIssuers | <code>Array.&lt;(CoreDocument\|IToCoreDocument)&gt;</code> | 
| options | [<code>VerifierOptions</code>](#VerifierOptions) | 

<a name="CredentialValidator.checkSubjectHolderRelationship"></a>

### CredentialValidator.checkSubjectHolderRelationship(credential, holder, relationship)
Validate that the relationship between the `holder` and the credential subjects is in accordance with
`relationship`. The `holder` parameter is expected to be the URL of the holder.

**Kind**: static method of [<code>CredentialValidator</code>](#CredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| holder | <code>string</code> | 
| relationship | <code>number</code> | 

<a name="CredentialValidator.checkStatus"></a>

### CredentialValidator.checkStatus(credential, trustedIssuers, statusCheck)
Checks whether the credential status has been revoked.

Only supports `BitmapRevocation2022`.

**Kind**: static method of [<code>CredentialValidator</code>](#CredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| trustedIssuers | <code>Array.&lt;(CoreDocument\|IToCoreDocument)&gt;</code> | 
| statusCheck | <code>number</code> | 

<a name="CredentialValidator.extractIssuer"></a>

### CredentialValidator.extractIssuer(credential) ⇒ [<code>CoreDID</code>](#CoreDID)
Utility for extracting the issuer field of a `Credential` as a DID.

### Errors

Fails if the issuer field is not a valid DID.

**Kind**: static method of [<code>CredentialValidator</code>](#CredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 

<a name="DIDUrl"></a>

## DIDUrl
A method agnostic DID Url.

**Kind**: global class  

* [DIDUrl](#DIDUrl)
    * _instance_
        * [.did()](#DIDUrl+did) ⇒ [<code>CoreDID</code>](#CoreDID)
        * [.urlStr()](#DIDUrl+urlStr) ⇒ <code>string</code>
        * [.fragment()](#DIDUrl+fragment) ⇒ <code>string</code> \| <code>undefined</code>
        * [.setFragment(value)](#DIDUrl+setFragment)
        * [.path()](#DIDUrl+path) ⇒ <code>string</code> \| <code>undefined</code>
        * [.setPath(value)](#DIDUrl+setPath)
        * [.query()](#DIDUrl+query) ⇒ <code>string</code> \| <code>undefined</code>
        * [.setQuery(value)](#DIDUrl+setQuery)
        * [.join(segment)](#DIDUrl+join) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.toString()](#DIDUrl+toString) ⇒ <code>string</code>
        * [.toJSON()](#DIDUrl+toJSON) ⇒ <code>any</code>
        * [.clone()](#DIDUrl+clone) ⇒ [<code>DIDUrl</code>](#DIDUrl)
    * _static_
        * [.parse(input)](#DIDUrl.parse) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.fromJSON(json)](#DIDUrl.fromJSON) ⇒ [<code>DIDUrl</code>](#DIDUrl)

<a name="DIDUrl+did"></a>

### didUrl.did() ⇒ [<code>CoreDID</code>](#CoreDID)
Return a copy of the `CoreDID` section of the `DIDUrl`.

**Kind**: instance method of [<code>DIDUrl</code>](#DIDUrl)  
<a name="DIDUrl+urlStr"></a>

### didUrl.urlStr() ⇒ <code>string</code>
Return a copy of the relative DID Url as a string, including only the path, query, and fragment.

**Kind**: instance method of [<code>DIDUrl</code>](#DIDUrl)  
<a name="DIDUrl+fragment"></a>

### didUrl.fragment() ⇒ <code>string</code> \| <code>undefined</code>
Returns a copy of the `DIDUrl` method fragment, if any. Excludes the leading '#'.

**Kind**: instance method of [<code>DIDUrl</code>](#DIDUrl)  
<a name="DIDUrl+setFragment"></a>

### didUrl.setFragment(value)
Sets the `fragment` component of the `DIDUrl`.

**Kind**: instance method of [<code>DIDUrl</code>](#DIDUrl)  

| Param | Type |
| --- | --- |
| value | <code>string</code> \| <code>undefined</code> | 

<a name="DIDUrl+path"></a>

### didUrl.path() ⇒ <code>string</code> \| <code>undefined</code>
Returns a copy of the `DIDUrl` path.

**Kind**: instance method of [<code>DIDUrl</code>](#DIDUrl)  
<a name="DIDUrl+setPath"></a>

### didUrl.setPath(value)
Sets the `path` component of the `DIDUrl`.

**Kind**: instance method of [<code>DIDUrl</code>](#DIDUrl)  

| Param | Type |
| --- | --- |
| value | <code>string</code> \| <code>undefined</code> | 

<a name="DIDUrl+query"></a>

### didUrl.query() ⇒ <code>string</code> \| <code>undefined</code>
Returns a copy of the `DIDUrl` method query, if any. Excludes the leading '?'.

**Kind**: instance method of [<code>DIDUrl</code>](#DIDUrl)  
<a name="DIDUrl+setQuery"></a>

### didUrl.setQuery(value)
Sets the `query` component of the `DIDUrl`.

**Kind**: instance method of [<code>DIDUrl</code>](#DIDUrl)  

| Param | Type |
| --- | --- |
| value | <code>string</code> \| <code>undefined</code> | 

<a name="DIDUrl+join"></a>

### didUrl.join(segment) ⇒ [<code>DIDUrl</code>](#DIDUrl)
Append a string representing a path, query, and/or fragment, returning a new `DIDUrl`.

Must begin with a valid delimiter character: '/', '?', '#'. Overwrites the existing URL
segment and any following segments in order of path, query, then fragment.

I.e.
- joining a path will clear the query and fragment.
- joining a query will clear the fragment.
- joining a fragment will only overwrite the fragment.

**Kind**: instance method of [<code>DIDUrl</code>](#DIDUrl)  

| Param | Type |
| --- | --- |
| segment | <code>string</code> | 

<a name="DIDUrl+toString"></a>

### didUrl.toString() ⇒ <code>string</code>
Returns the `DIDUrl` as a string.

**Kind**: instance method of [<code>DIDUrl</code>](#DIDUrl)  
<a name="DIDUrl+toJSON"></a>

### didUrl.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>DIDUrl</code>](#DIDUrl)  
<a name="DIDUrl+clone"></a>

### didUrl.clone() ⇒ [<code>DIDUrl</code>](#DIDUrl)
Deep clones the object.

**Kind**: instance method of [<code>DIDUrl</code>](#DIDUrl)  
<a name="DIDUrl.parse"></a>

### DIDUrl.parse(input) ⇒ [<code>DIDUrl</code>](#DIDUrl)
Parses a `DIDUrl` from the input string.

**Kind**: static method of [<code>DIDUrl</code>](#DIDUrl)  

| Param | Type |
| --- | --- |
| input | <code>string</code> | 

<a name="DIDUrl.fromJSON"></a>

### DIDUrl.fromJSON(json) ⇒ [<code>DIDUrl</code>](#DIDUrl)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>DIDUrl</code>](#DIDUrl)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="DomainLinkageConfiguration"></a>

## DomainLinkageConfiguration
DID Configuration Resource which contains Domain Linkage Credentials.
It can be placed in an origin's `.well-known` directory to prove linkage between the origin and a DID.
See: <https://identity.foundation/.well-known/resources/did-configuration/#did-configuration-resource>

Note:
- Only [Linked Data Proof Format](https://identity.foundation/.well-known/resources/did-configuration/#linked-data-proof-format)
  is supported.

**Kind**: global class  

* [DomainLinkageConfiguration](#DomainLinkageConfiguration)
    * [new DomainLinkageConfiguration(linked_dids)](#new_DomainLinkageConfiguration_new)
    * _instance_
        * [.linkedDids()](#DomainLinkageConfiguration+linkedDids) ⇒ [<code>Array.&lt;Credential&gt;</code>](#Credential)
        * [.issuers()](#DomainLinkageConfiguration+issuers) ⇒ <code>Array.&lt;string&gt;</code>
        * [.toJSON()](#DomainLinkageConfiguration+toJSON) ⇒ <code>any</code>
        * [.clone()](#DomainLinkageConfiguration+clone) ⇒ [<code>DomainLinkageConfiguration</code>](#DomainLinkageConfiguration)
    * _static_
        * [.fromJSON(json)](#DomainLinkageConfiguration.fromJSON) ⇒ [<code>DomainLinkageConfiguration</code>](#DomainLinkageConfiguration)

<a name="new_DomainLinkageConfiguration_new"></a>

### new DomainLinkageConfiguration(linked_dids)
Constructs a new `DomainLinkageConfiguration`.


| Param | Type |
| --- | --- |
| linked_dids | [<code>Array.&lt;Credential&gt;</code>](#Credential) | 

<a name="DomainLinkageConfiguration+linkedDids"></a>

### domainLinkageConfiguration.linkedDids() ⇒ [<code>Array.&lt;Credential&gt;</code>](#Credential)
List of the Domain Linkage Credentials.

**Kind**: instance method of [<code>DomainLinkageConfiguration</code>](#DomainLinkageConfiguration)  
<a name="DomainLinkageConfiguration+issuers"></a>

### domainLinkageConfiguration.issuers() ⇒ <code>Array.&lt;string&gt;</code>
List of the issuers of the Domain Linkage Credentials.

**Kind**: instance method of [<code>DomainLinkageConfiguration</code>](#DomainLinkageConfiguration)  
<a name="DomainLinkageConfiguration+toJSON"></a>

### domainLinkageConfiguration.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>DomainLinkageConfiguration</code>](#DomainLinkageConfiguration)  
<a name="DomainLinkageConfiguration+clone"></a>

### domainLinkageConfiguration.clone() ⇒ [<code>DomainLinkageConfiguration</code>](#DomainLinkageConfiguration)
Deep clones the object.

**Kind**: instance method of [<code>DomainLinkageConfiguration</code>](#DomainLinkageConfiguration)  
<a name="DomainLinkageConfiguration.fromJSON"></a>

### DomainLinkageConfiguration.fromJSON(json) ⇒ [<code>DomainLinkageConfiguration</code>](#DomainLinkageConfiguration)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>DomainLinkageConfiguration</code>](#DomainLinkageConfiguration)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="DomainLinkageValidator"></a>

## DomainLinkageValidator
A validator for a Domain Linkage Configuration and Credentials.

**Kind**: global class  

* [DomainLinkageValidator](#DomainLinkageValidator)
    * [.validateLinkage(options)](#DomainLinkageValidator.validateLinkage)
    * [.validateCredential(options)](#DomainLinkageValidator.validateCredential)

<a name="DomainLinkageValidator.validateLinkage"></a>

### DomainLinkageValidator.validateLinkage(options)
Validates the linkage between a domain and a DID.
[`DomainLinkageConfiguration`] is validated according to [DID Configuration Resource Verification](https://identity.foundation/.well-known/resources/did-configuration/#did-configuration-resource-verification).

Linkage is valid if no error is thrown.

# Note:
- Only [Linked Data Proof Format](https://identity.foundation/.well-known/resources/did-configuration/#linked-data-proof-format)
  is supported.
- Only the Credential issued by `issuer` is verified.

# Errors
 - Semantic structure of `configuration` is invalid.
 - `configuration` includes multiple credentials issued by `issuer`.
 - Validation of the matched Domain Linkage Credential fails.

**Kind**: static method of [<code>DomainLinkageValidator</code>](#DomainLinkageValidator)  

| Param | Type |
| --- | --- |
| options | <code>IValidateLinkage</code> | 

<a name="DomainLinkageValidator.validateCredential"></a>

### DomainLinkageValidator.validateCredential(options)
Validates a [Domain Linkage Credential](https://identity.foundation/.well-known/resources/did-configuration/#domain-linkage-credential).
Error will be thrown in case the validation fails.

**Kind**: static method of [<code>DomainLinkageValidator</code>](#DomainLinkageValidator)  

| Param | Type |
| --- | --- |
| options | <code>IValidateCredential</code> | 

<a name="Duration"></a>

## Duration
A span of time.

**Kind**: global class  

* [Duration](#Duration)
    * _instance_
        * [.toJSON()](#Duration+toJSON) ⇒ <code>any</code>
    * _static_
        * [.seconds(seconds)](#Duration.seconds) ⇒ [<code>Duration</code>](#Duration)
        * [.minutes(minutes)](#Duration.minutes) ⇒ [<code>Duration</code>](#Duration)
        * [.hours(hours)](#Duration.hours) ⇒ [<code>Duration</code>](#Duration)
        * [.days(days)](#Duration.days) ⇒ [<code>Duration</code>](#Duration)
        * [.weeks(weeks)](#Duration.weeks) ⇒ [<code>Duration</code>](#Duration)
        * [.fromJSON(json)](#Duration.fromJSON) ⇒ [<code>Duration</code>](#Duration)

<a name="Duration+toJSON"></a>

### duration.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>Duration</code>](#Duration)  
<a name="Duration.seconds"></a>

### Duration.seconds(seconds) ⇒ [<code>Duration</code>](#Duration)
Create a new `Duration` with the given number of seconds.

**Kind**: static method of [<code>Duration</code>](#Duration)  

| Param | Type |
| --- | --- |
| seconds | <code>number</code> | 

<a name="Duration.minutes"></a>

### Duration.minutes(minutes) ⇒ [<code>Duration</code>](#Duration)
Create a new `Duration` with the given number of minutes.

**Kind**: static method of [<code>Duration</code>](#Duration)  

| Param | Type |
| --- | --- |
| minutes | <code>number</code> | 

<a name="Duration.hours"></a>

### Duration.hours(hours) ⇒ [<code>Duration</code>](#Duration)
Create a new `Duration` with the given number of hours.

**Kind**: static method of [<code>Duration</code>](#Duration)  

| Param | Type |
| --- | --- |
| hours | <code>number</code> | 

<a name="Duration.days"></a>

### Duration.days(days) ⇒ [<code>Duration</code>](#Duration)
Create a new `Duration` with the given number of days.

**Kind**: static method of [<code>Duration</code>](#Duration)  

| Param | Type |
| --- | --- |
| days | <code>number</code> | 

<a name="Duration.weeks"></a>

### Duration.weeks(weeks) ⇒ [<code>Duration</code>](#Duration)
Create a new `Duration` with the given number of weeks.

**Kind**: static method of [<code>Duration</code>](#Duration)  

| Param | Type |
| --- | --- |
| weeks | <code>number</code> | 

<a name="Duration.fromJSON"></a>

### Duration.fromJSON(json) ⇒ [<code>Duration</code>](#Duration)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>Duration</code>](#Duration)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="Ed25519"></a>

## Ed25519
**Kind**: global class  

* [Ed25519](#Ed25519)
    * [.PRIVATE_KEY_LENGTH()](#Ed25519.PRIVATE_KEY_LENGTH) ⇒ <code>number</code>
    * [.PUBLIC_KEY_LENGTH()](#Ed25519.PUBLIC_KEY_LENGTH) ⇒ <code>number</code>
    * [.SIGNATURE_LENGTH()](#Ed25519.SIGNATURE_LENGTH) ⇒ <code>number</code>
    * [.sign(message, privateKey)](#Ed25519.sign) ⇒ <code>Uint8Array</code>
    * [.verify(message, signature, publicKey)](#Ed25519.verify)

<a name="Ed25519.PRIVATE_KEY_LENGTH"></a>

### Ed25519.PRIVATE\_KEY\_LENGTH() ⇒ <code>number</code>
Length in bytes of an Ed25519 private key.

**Kind**: static method of [<code>Ed25519</code>](#Ed25519)  
<a name="Ed25519.PUBLIC_KEY_LENGTH"></a>

### Ed25519.PUBLIC\_KEY\_LENGTH() ⇒ <code>number</code>
Length in bytes of an Ed25519 public key.

**Kind**: static method of [<code>Ed25519</code>](#Ed25519)  
<a name="Ed25519.SIGNATURE_LENGTH"></a>

### Ed25519.SIGNATURE\_LENGTH() ⇒ <code>number</code>
Length in bytes of an Ed25519 signature.

**Kind**: static method of [<code>Ed25519</code>](#Ed25519)  
<a name="Ed25519.sign"></a>

### Ed25519.sign(message, privateKey) ⇒ <code>Uint8Array</code>
Computes an EdDSA signature using an Ed25519 private key.

NOTE: this differs from [Document.signData](#Document+signData) which uses JCS
to canonicalize JSON messages.

The private key must be a 32-byte seed in compliance with [RFC 8032](https://datatracker.ietf.org/doc/html/rfc8032#section-3.2).
Other implementations often use another format. See [this blog post](https://blog.mozilla.org/warner/2011/11/29/ed25519-keys/) for further explanation.

**Kind**: static method of [<code>Ed25519</code>](#Ed25519)  

| Param | Type |
| --- | --- |
| message | <code>Uint8Array</code> | 
| privateKey | <code>Uint8Array</code> | 

<a name="Ed25519.verify"></a>

### Ed25519.verify(message, signature, publicKey)
Verifies an EdDSA signature against an Ed25519 public key.

NOTE: this differs from [Document.verifyData](#Document+verifyData) which uses JCS
to canonicalize JSON messages.

**Kind**: static method of [<code>Ed25519</code>](#Ed25519)  

| Param | Type |
| --- | --- |
| message | <code>Uint8Array</code> | 
| signature | <code>Uint8Array</code> | 
| publicKey | <code>Uint8Array</code> | 

<a name="IotaDID"></a>

## IotaDID
A DID conforming to the IOTA DID method specification.

**Kind**: global class  

* [IotaDID](#IotaDID)
    * [new IotaDID(bytes, network)](#new_IotaDID_new)
    * _instance_
        * [.networkStr()](#IotaDID+networkStr) ⇒ <code>string</code>
        * [.tag()](#IotaDID+tag) ⇒ <code>string</code>
        * [.toCoreDid()](#IotaDID+toCoreDid) ⇒ [<code>CoreDID</code>](#CoreDID)
        * [.scheme()](#IotaDID+scheme) ⇒ <code>string</code>
        * [.authority()](#IotaDID+authority) ⇒ <code>string</code>
        * [.method()](#IotaDID+method) ⇒ <code>string</code>
        * [.methodId()](#IotaDID+methodId) ⇒ <code>string</code>
        * [.join(segment)](#IotaDID+join) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.toUrl()](#IotaDID+toUrl) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.toAliasId()](#IotaDID+toAliasId) ⇒ <code>string</code>
        * [.intoUrl()](#IotaDID+intoUrl) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.toString()](#IotaDID+toString) ⇒ <code>string</code>
        * [.toJSON()](#IotaDID+toJSON) ⇒ <code>any</code>
        * [.clone()](#IotaDID+clone) ⇒ [<code>IotaDID</code>](#IotaDID)
    * _static_
        * [.METHOD](#IotaDID.METHOD) ⇒ <code>string</code>
        * [.DEFAULT_NETWORK](#IotaDID.DEFAULT_NETWORK) ⇒ <code>string</code>
        * [.fromAliasId(aliasId, network)](#IotaDID.fromAliasId) ⇒ [<code>IotaDID</code>](#IotaDID)
        * [.placeholder(network)](#IotaDID.placeholder) ⇒ [<code>IotaDID</code>](#IotaDID)
        * [.parse(input)](#IotaDID.parse) ⇒ [<code>IotaDID</code>](#IotaDID)
        * [.fromJSON(json)](#IotaDID.fromJSON) ⇒ [<code>IotaDID</code>](#IotaDID)

<a name="new_IotaDID_new"></a>

### new IotaDID(bytes, network)
Constructs a new `IotaDID` from a byte representation of the tag and the given
network name.

See also [placeholder](#IotaDID.placeholder).


| Param | Type |
| --- | --- |
| bytes | <code>Uint8Array</code> | 
| network | <code>string</code> | 

<a name="IotaDID+networkStr"></a>

### did.networkStr() ⇒ <code>string</code>
Returns the Tangle network name of the `IotaDID`.

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+tag"></a>

### did.tag() ⇒ <code>string</code>
Returns a copy of the unique tag of the `IotaDID`.

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+toCoreDid"></a>

### did.toCoreDid() ⇒ [<code>CoreDID</code>](#CoreDID)
Returns the DID represented as a `CoreDID`.

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+scheme"></a>

### did.scheme() ⇒ <code>string</code>
Returns the `DID` scheme.

E.g.
- `"did:example:12345678" -> "did"`
- `"did:iota:main:12345678" -> "did"`

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+authority"></a>

### did.authority() ⇒ <code>string</code>
Returns the `DID` authority: the method name and method-id.

E.g.
- `"did:example:12345678" -> "example:12345678"`
- `"did:iota:main:12345678" -> "iota:main:12345678"`

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+method"></a>

### did.method() ⇒ <code>string</code>
Returns the `DID` method name.

E.g.
- `"did:example:12345678" -> "example"`
- `"did:iota:main:12345678" -> "iota"`

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+methodId"></a>

### did.methodId() ⇒ <code>string</code>
Returns the `DID` method-specific ID.

E.g.
- `"did:example:12345678" -> "12345678"`
- `"did:iota:main:12345678" -> "main:12345678"`

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+join"></a>

### did.join(segment) ⇒ [<code>DIDUrl</code>](#DIDUrl)
Construct a new `DIDUrl` by joining with a relative DID Url string.

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  

| Param | Type |
| --- | --- |
| segment | <code>string</code> | 

<a name="IotaDID+toUrl"></a>

### did.toUrl() ⇒ [<code>DIDUrl</code>](#DIDUrl)
Clones the `DID` into a `DIDUrl`.

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+toAliasId"></a>

### did.toAliasId() ⇒ <code>string</code>
Returns the hex-encoded AliasId with a '0x' prefix, from the DID tag.

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+intoUrl"></a>

### did.intoUrl() ⇒ [<code>DIDUrl</code>](#DIDUrl)
Converts the `DID` into a `DIDUrl`, consuming it.

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+toString"></a>

### did.toString() ⇒ <code>string</code>
Returns the `DID` as a string.

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+toJSON"></a>

### did.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+clone"></a>

### did.clone() ⇒ [<code>IotaDID</code>](#IotaDID)
Deep clones the object.

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID.METHOD"></a>

### IotaDID.METHOD ⇒ <code>string</code>
The IOTA DID method name (`"iota"`).

**Kind**: static property of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID.DEFAULT_NETWORK"></a>

### IotaDID.DEFAULT\_NETWORK ⇒ <code>string</code>
The default Tangle network (`"iota"`).

**Kind**: static property of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID.fromAliasId"></a>

### IotaDID.fromAliasId(aliasId, network) ⇒ [<code>IotaDID</code>](#IotaDID)
Constructs a new `IotaDID` from a hex representation of an Alias Id and the given
network name.

**Kind**: static method of [<code>IotaDID</code>](#IotaDID)  

| Param | Type |
| --- | --- |
| aliasId | <code>string</code> | 
| network | <code>string</code> | 

<a name="IotaDID.placeholder"></a>

### IotaDID.placeholder(network) ⇒ [<code>IotaDID</code>](#IotaDID)
Creates a new placeholder [`IotaDID`] with the given network name.

E.g. `did:iota:smr:0x0000000000000000000000000000000000000000000000000000000000000000`.

**Kind**: static method of [<code>IotaDID</code>](#IotaDID)  

| Param | Type |
| --- | --- |
| network | <code>string</code> | 

<a name="IotaDID.parse"></a>

### IotaDID.parse(input) ⇒ [<code>IotaDID</code>](#IotaDID)
Parses a `IotaDID` from the input string.

**Kind**: static method of [<code>IotaDID</code>](#IotaDID)  

| Param | Type |
| --- | --- |
| input | <code>string</code> | 

<a name="IotaDID.fromJSON"></a>

### IotaDID.fromJSON(json) ⇒ [<code>IotaDID</code>](#IotaDID)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>IotaDID</code>](#IotaDID)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="IotaDocument"></a>

## IotaDocument
**Kind**: global class  

* [IotaDocument](#IotaDocument)
    * [new IotaDocument(network)](#new_IotaDocument_new)
    * _instance_
        * [.id()](#IotaDocument+id) ⇒ [<code>IotaDID</code>](#IotaDID)
        * [.controller()](#IotaDocument+controller) ⇒ [<code>Array.&lt;IotaDID&gt;</code>](#IotaDID)
        * [.alsoKnownAs()](#IotaDocument+alsoKnownAs) ⇒ <code>Array.&lt;string&gt;</code>
        * [.setAlsoKnownAs(urls)](#IotaDocument+setAlsoKnownAs)
        * [.properties()](#IotaDocument+properties) ⇒ <code>Map.&lt;string, any&gt;</code>
        * [.setPropertyUnchecked(key, value)](#IotaDocument+setPropertyUnchecked)
        * [.service()](#IotaDocument+service) ⇒ [<code>Array.&lt;Service&gt;</code>](#Service)
        * [.insertService(service)](#IotaDocument+insertService)
        * [.removeService(did)](#IotaDocument+removeService) ⇒ [<code>Service</code>](#Service) \| <code>undefined</code>
        * [.resolveService(query)](#IotaDocument+resolveService) ⇒ [<code>Service</code>](#Service) \| <code>undefined</code>
        * [.methods(scope)](#IotaDocument+methods) ⇒ [<code>Array.&lt;VerificationMethod&gt;</code>](#VerificationMethod)
        * [.insertMethod(method, scope)](#IotaDocument+insertMethod)
        * [.removeMethod(did)](#IotaDocument+removeMethod) ⇒ [<code>VerificationMethod</code>](#VerificationMethod) \| <code>undefined</code>
        * [.resolveMethod(query, scope)](#IotaDocument+resolveMethod) ⇒ [<code>VerificationMethod</code>](#VerificationMethod) \| <code>undefined</code>
        * [.attachMethodRelationship(didUrl, relationship)](#IotaDocument+attachMethodRelationship) ⇒ <code>boolean</code>
        * [.detachMethodRelationship(didUrl, relationship)](#IotaDocument+detachMethodRelationship) ⇒ <code>boolean</code>
        * [.signCredential(credential, privateKey, methodQuery, options)](#IotaDocument+signCredential) ⇒ [<code>Credential</code>](#Credential)
        * [.signPresentation(presentation, privateKey, methodQuery, options)](#IotaDocument+signPresentation) ⇒ [<code>Presentation</code>](#Presentation)
        * [.signData(data, privateKey, methodQuery, options)](#IotaDocument+signData) ⇒ <code>any</code>
        * [.verifyData(data, options)](#IotaDocument+verifyData) ⇒ <code>boolean</code>
        * [.pack()](#IotaDocument+pack) ⇒ <code>Uint8Array</code>
        * [.packWithEncoding(encoding)](#IotaDocument+packWithEncoding) ⇒ <code>Uint8Array</code>
        * [.metadata()](#IotaDocument+metadata) ⇒ [<code>IotaDocumentMetadata</code>](#IotaDocumentMetadata)
        * [.metadataCreated()](#IotaDocument+metadataCreated) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
        * [.setMetadataCreated(timestamp)](#IotaDocument+setMetadataCreated)
        * [.metadataUpdated()](#IotaDocument+metadataUpdated) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
        * [.setMetadataUpdated(timestamp)](#IotaDocument+setMetadataUpdated)
        * [.metadataDeactivated()](#IotaDocument+metadataDeactivated) ⇒ <code>boolean</code> \| <code>undefined</code>
        * [.setMetadataDeactivated(deactivated)](#IotaDocument+setMetadataDeactivated)
        * [.metadataStateControllerAddress()](#IotaDocument+metadataStateControllerAddress) ⇒ <code>string</code> \| <code>undefined</code>
        * [.metadataGovernorAddress()](#IotaDocument+metadataGovernorAddress) ⇒ <code>string</code> \| <code>undefined</code>
        * [.setMetadataPropertyUnchecked(key, value)](#IotaDocument+setMetadataPropertyUnchecked)
        * [.revokeCredentials(serviceQuery, indices)](#IotaDocument+revokeCredentials)
        * [.unrevokeCredentials(serviceQuery, indices)](#IotaDocument+unrevokeCredentials)
        * [.clone()](#IotaDocument+clone) ⇒ [<code>IotaDocument</code>](#IotaDocument)
        * [._shallowCloneInternal()](#IotaDocument+_shallowCloneInternal) ⇒ [<code>IotaDocument</code>](#IotaDocument)
        * [.toJSON()](#IotaDocument+toJSON) ⇒ <code>any</code>
        * [.toCoreDocument()](#IotaDocument+toCoreDocument) ⇒ [<code>CoreDocument</code>](#CoreDocument)
    * _static_
        * [.newWithId(id)](#IotaDocument.newWithId) ⇒ [<code>IotaDocument</code>](#IotaDocument)
        * [.unpackFromOutput(did, aliasOutput, allowEmpty, tokenSupply)](#IotaDocument.unpackFromOutput) ⇒ [<code>IotaDocument</code>](#IotaDocument)
        * [.unpackFromBlock(network, block, protocol_parameters)](#IotaDocument.unpackFromBlock) ⇒ [<code>Array.&lt;IotaDocument&gt;</code>](#IotaDocument)
        * [.fromJSON(json)](#IotaDocument.fromJSON) ⇒ [<code>IotaDocument</code>](#IotaDocument)

<a name="new_IotaDocument_new"></a>

### new IotaDocument(network)
Constructs an empty DID Document with a [placeholder](#IotaDID.placeholder) identifier
for the given `network`.


| Param | Type |
| --- | --- |
| network | <code>string</code> | 

<a name="IotaDocument+id"></a>

### iotaDocument.id() ⇒ [<code>IotaDID</code>](#IotaDID)
Returns a copy of the DID Document `id`.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  
<a name="IotaDocument+controller"></a>

### iotaDocument.controller() ⇒ [<code>Array.&lt;IotaDID&gt;</code>](#IotaDID)
Returns a copy of the list of document controllers.

NOTE: controllers are determined by the `state_controller` unlock condition of the output
during resolution and are omitted when publishing.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  
<a name="IotaDocument+alsoKnownAs"></a>

### iotaDocument.alsoKnownAs() ⇒ <code>Array.&lt;string&gt;</code>
Returns a copy of the document's `alsoKnownAs` set.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  
<a name="IotaDocument+setAlsoKnownAs"></a>

### iotaDocument.setAlsoKnownAs(urls)
Sets the `alsoKnownAs` property in the DID document.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| urls | <code>string</code> \| <code>Array.&lt;string&gt;</code> \| <code>null</code> | 

<a name="IotaDocument+properties"></a>

### iotaDocument.properties() ⇒ <code>Map.&lt;string, any&gt;</code>
Returns a copy of the custom DID Document properties.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  
<a name="IotaDocument+setPropertyUnchecked"></a>

### iotaDocument.setPropertyUnchecked(key, value)
Sets a custom property in the DID Document.
If the value is set to `null`, the custom property will be removed.

### WARNING

This method can overwrite existing properties like `id` and result in an invalid document.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| key | <code>string</code> | 
| value | <code>any</code> | 

<a name="IotaDocument+service"></a>

### iotaDocument.service() ⇒ [<code>Array.&lt;Service&gt;</code>](#Service)
Return a set of all [Service](#Service) in the document.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  
<a name="IotaDocument+insertService"></a>

### iotaDocument.insertService(service)
Add a new [Service](#Service) to the document.

Returns `true` if the service was added.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| service | [<code>Service</code>](#Service) | 

<a name="IotaDocument+removeService"></a>

### iotaDocument.removeService(did) ⇒ [<code>Service</code>](#Service) \| <code>undefined</code>
Remove a [Service](#Service) identified by the given [DIDUrl](#DIDUrl) from the document.

Returns `true` if a service was removed.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| did | [<code>DIDUrl</code>](#DIDUrl) | 

<a name="IotaDocument+resolveService"></a>

### iotaDocument.resolveService(query) ⇒ [<code>Service</code>](#Service) \| <code>undefined</code>
Returns the first [Service](#Service) with an `id` property matching the provided `query`,
if present.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| query | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 

<a name="IotaDocument+methods"></a>

### iotaDocument.methods(scope) ⇒ [<code>Array.&lt;VerificationMethod&gt;</code>](#VerificationMethod)
Returns a list of all [VerificationMethod](#VerificationMethod) in the DID Document,
whose verification relationship matches `scope`.

If `scope` is not set, a list over the **embedded** methods is returned.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| scope | [<code>MethodScope</code>](#MethodScope) \| <code>undefined</code> | 

<a name="IotaDocument+insertMethod"></a>

### iotaDocument.insertMethod(method, scope)
Adds a new `method` to the document in the given `scope`.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| method | [<code>VerificationMethod</code>](#VerificationMethod) | 
| scope | [<code>MethodScope</code>](#MethodScope) | 

<a name="IotaDocument+removeMethod"></a>

### iotaDocument.removeMethod(did) ⇒ [<code>VerificationMethod</code>](#VerificationMethod) \| <code>undefined</code>
Removes all references to the specified Verification Method.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| did | [<code>DIDUrl</code>](#DIDUrl) | 

<a name="IotaDocument+resolveMethod"></a>

### iotaDocument.resolveMethod(query, scope) ⇒ [<code>VerificationMethod</code>](#VerificationMethod) \| <code>undefined</code>
Returns a copy of the first verification method with an `id` property
matching the provided `query` and the verification relationship
specified by `scope`, if present.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| query | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| scope | [<code>MethodScope</code>](#MethodScope) \| <code>undefined</code> | 

<a name="IotaDocument+attachMethodRelationship"></a>

### iotaDocument.attachMethodRelationship(didUrl, relationship) ⇒ <code>boolean</code>
Attaches the relationship to the given method, if the method exists.

Note: The method needs to be in the set of verification methods,
so it cannot be an embedded one.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| didUrl | [<code>DIDUrl</code>](#DIDUrl) | 
| relationship | <code>number</code> | 

<a name="IotaDocument+detachMethodRelationship"></a>

### iotaDocument.detachMethodRelationship(didUrl, relationship) ⇒ <code>boolean</code>
Detaches the given relationship from the given method, if the method exists.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| didUrl | [<code>DIDUrl</code>](#DIDUrl) | 
| relationship | <code>number</code> | 

<a name="IotaDocument+signCredential"></a>

### iotaDocument.signCredential(credential, privateKey, methodQuery, options) ⇒ [<code>Credential</code>](#Credential)
Creates a signature for the given `Credential` with the specified DID Document
Verification Method.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| privateKey | <code>Uint8Array</code> | 
| methodQuery | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| options | [<code>ProofOptions</code>](#ProofOptions) | 

<a name="IotaDocument+signPresentation"></a>

### iotaDocument.signPresentation(presentation, privateKey, methodQuery, options) ⇒ [<code>Presentation</code>](#Presentation)
Creates a signature for the given `Presentation` with the specified DID Document
Verification Method.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| presentation | [<code>Presentation</code>](#Presentation) | 
| privateKey | <code>Uint8Array</code> | 
| methodQuery | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| options | [<code>ProofOptions</code>](#ProofOptions) | 

<a name="IotaDocument+signData"></a>

### iotaDocument.signData(data, privateKey, methodQuery, options) ⇒ <code>any</code>
Creates a signature for the given `data` with the specified DID Document
Verification Method.

NOTE: use `signSelf` or `signDocument` for DID Documents.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| data | <code>any</code> | 
| privateKey | <code>Uint8Array</code> | 
| methodQuery | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| options | [<code>ProofOptions</code>](#ProofOptions) | 

<a name="IotaDocument+verifyData"></a>

### iotaDocument.verifyData(data, options) ⇒ <code>boolean</code>
Verifies the authenticity of `data` using the target verification method.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| data | <code>any</code> | 
| options | [<code>VerifierOptions</code>](#VerifierOptions) | 

<a name="IotaDocument+pack"></a>

### iotaDocument.pack() ⇒ <code>Uint8Array</code>
Serializes the document for inclusion in an Alias Output's state metadata
with the default [StateMetadataEncoding](#StateMetadataEncoding).

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  
<a name="IotaDocument+packWithEncoding"></a>

### iotaDocument.packWithEncoding(encoding) ⇒ <code>Uint8Array</code>
Serializes the document for inclusion in an Alias Output's state metadata.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| encoding | <code>number</code> | 

<a name="IotaDocument+metadata"></a>

### iotaDocument.metadata() ⇒ [<code>IotaDocumentMetadata</code>](#IotaDocumentMetadata)
Returns a copy of the metadata associated with this document.

NOTE: Copies all the metadata. See also `metadataCreated`, `metadataUpdated`,
`metadataPreviousMessageId`, `metadataProof` if only a subset of the metadata required.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  
<a name="IotaDocument+metadataCreated"></a>

### iotaDocument.metadataCreated() ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
Returns a copy of the timestamp of when the DID document was created.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  
<a name="IotaDocument+setMetadataCreated"></a>

### iotaDocument.setMetadataCreated(timestamp)
Sets the timestamp of when the DID document was created.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| timestamp | [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code> | 

<a name="IotaDocument+metadataUpdated"></a>

### iotaDocument.metadataUpdated() ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
Returns a copy of the timestamp of the last DID document update.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  
<a name="IotaDocument+setMetadataUpdated"></a>

### iotaDocument.setMetadataUpdated(timestamp)
Sets the timestamp of the last DID document update.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| timestamp | [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code> | 

<a name="IotaDocument+metadataDeactivated"></a>

### iotaDocument.metadataDeactivated() ⇒ <code>boolean</code> \| <code>undefined</code>
Returns a copy of the deactivated status of the DID document.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  
<a name="IotaDocument+setMetadataDeactivated"></a>

### iotaDocument.setMetadataDeactivated(deactivated)
Sets the deactivated status of the DID document.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| deactivated | <code>boolean</code> \| <code>undefined</code> | 

<a name="IotaDocument+metadataStateControllerAddress"></a>

### iotaDocument.metadataStateControllerAddress() ⇒ <code>string</code> \| <code>undefined</code>
Returns a copy of the Bech32-encoded state controller address, if present.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  
<a name="IotaDocument+metadataGovernorAddress"></a>

### iotaDocument.metadataGovernorAddress() ⇒ <code>string</code> \| <code>undefined</code>
Returns a copy of the Bech32-encoded governor address, if present.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  
<a name="IotaDocument+setMetadataPropertyUnchecked"></a>

### iotaDocument.setMetadataPropertyUnchecked(key, value)
Sets a custom property in the document metadata.
If the value is set to `null`, the custom property will be removed.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| key | <code>string</code> | 
| value | <code>any</code> | 

<a name="IotaDocument+revokeCredentials"></a>

### iotaDocument.revokeCredentials(serviceQuery, indices)
If the document has a `RevocationBitmap` service identified by `serviceQuery`,
revoke all specified `indices`.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| serviceQuery | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| indices | <code>number</code> \| <code>Array.&lt;number&gt;</code> | 

<a name="IotaDocument+unrevokeCredentials"></a>

### iotaDocument.unrevokeCredentials(serviceQuery, indices)
If the document has a `RevocationBitmap` service identified by `serviceQuery`,
unrevoke all specified `indices`.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| serviceQuery | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| indices | <code>number</code> \| <code>Array.&lt;number&gt;</code> | 

<a name="IotaDocument+clone"></a>

### iotaDocument.clone() ⇒ [<code>IotaDocument</code>](#IotaDocument)
Returns a deep clone of the `IotaDocument`.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  
<a name="IotaDocument+_shallowCloneInternal"></a>

### iotaDocument.\_shallowCloneInternal() ⇒ [<code>IotaDocument</code>](#IotaDocument)
### Warning
This is for internal use only. Do not rely on or call this method.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  
<a name="IotaDocument+toJSON"></a>

### iotaDocument.toJSON() ⇒ <code>any</code>
Serializes to a plain JS representation.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  
<a name="IotaDocument+toCoreDocument"></a>

### iotaDocument.toCoreDocument() ⇒ [<code>CoreDocument</code>](#CoreDocument)
Transforms the `IotaDocument` to its `CoreDocument` representation.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  
<a name="IotaDocument.newWithId"></a>

### IotaDocument.newWithId(id) ⇒ [<code>IotaDocument</code>](#IotaDocument)
Constructs an empty DID Document with the given identifier.

**Kind**: static method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| id | [<code>IotaDID</code>](#IotaDID) | 

<a name="IotaDocument.unpackFromOutput"></a>

### IotaDocument.unpackFromOutput(did, aliasOutput, allowEmpty, tokenSupply) ⇒ [<code>IotaDocument</code>](#IotaDocument)
Deserializes the document from an Alias Output.

If `allowEmpty` is true, this will return an empty DID document marked as `deactivated`
if `stateMetadata` is empty.

The `tokenSupply` must be equal to the token supply of the network the DID is associated with.

NOTE: `did` is required since it is omitted from the serialized DID Document and
cannot be inferred from the state metadata. It also indicates the network, which is not
encoded in the `AliasId` alone.

**Kind**: static method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| did | [<code>IotaDID</code>](#IotaDID) | 
| aliasOutput | <code>IAliasOutput</code> | 
| allowEmpty | <code>boolean</code> | 
| tokenSupply | <code>bigint</code> | 

<a name="IotaDocument.unpackFromBlock"></a>

### IotaDocument.unpackFromBlock(network, block, protocol_parameters) ⇒ [<code>Array.&lt;IotaDocument&gt;</code>](#IotaDocument)
Returns all DID documents of the Alias Outputs contained in the block's transaction payload
outputs, if any.

Errors if any Alias Output does not contain a valid or empty DID Document.

`protocolResponseJson` can be obtained from a `Client`.

**Kind**: static method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| network | <code>string</code> | 
| block | <code>IBlock</code> | 
| protocol_parameters | <code>INodeInfoProtocol</code> | 

<a name="IotaDocument.fromJSON"></a>

### IotaDocument.fromJSON(json) ⇒ [<code>IotaDocument</code>](#IotaDocument)
Deserializes an instance from a plain JS representation.

**Kind**: static method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="IotaDocumentMetadata"></a>

## IotaDocumentMetadata
Additional attributes related to an IOTA DID Document.

**Kind**: global class  

* [IotaDocumentMetadata](#IotaDocumentMetadata)
    * _instance_
        * [.created()](#IotaDocumentMetadata+created) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
        * [.updated()](#IotaDocumentMetadata+updated) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
        * [.deactivated()](#IotaDocumentMetadata+deactivated) ⇒ <code>boolean</code> \| <code>undefined</code>
        * [.stateControllerAddress()](#IotaDocumentMetadata+stateControllerAddress) ⇒ <code>string</code> \| <code>undefined</code>
        * [.governorAddress()](#IotaDocumentMetadata+governorAddress) ⇒ <code>string</code> \| <code>undefined</code>
        * [.properties()](#IotaDocumentMetadata+properties) ⇒ <code>Map.&lt;string, any&gt;</code>
        * [.toJSON()](#IotaDocumentMetadata+toJSON) ⇒ <code>any</code>
        * [.clone()](#IotaDocumentMetadata+clone) ⇒ [<code>IotaDocumentMetadata</code>](#IotaDocumentMetadata)
    * _static_
        * [.fromJSON(json)](#IotaDocumentMetadata.fromJSON) ⇒ [<code>IotaDocumentMetadata</code>](#IotaDocumentMetadata)

<a name="IotaDocumentMetadata+created"></a>

### iotaDocumentMetadata.created() ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
Returns a copy of the timestamp of when the DID document was created.

**Kind**: instance method of [<code>IotaDocumentMetadata</code>](#IotaDocumentMetadata)  
<a name="IotaDocumentMetadata+updated"></a>

### iotaDocumentMetadata.updated() ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
Returns a copy of the timestamp of the last DID document update.

**Kind**: instance method of [<code>IotaDocumentMetadata</code>](#IotaDocumentMetadata)  
<a name="IotaDocumentMetadata+deactivated"></a>

### iotaDocumentMetadata.deactivated() ⇒ <code>boolean</code> \| <code>undefined</code>
Returns a copy of the deactivated status of the DID document.

**Kind**: instance method of [<code>IotaDocumentMetadata</code>](#IotaDocumentMetadata)  
<a name="IotaDocumentMetadata+stateControllerAddress"></a>

### iotaDocumentMetadata.stateControllerAddress() ⇒ <code>string</code> \| <code>undefined</code>
Returns a copy of the Bech32-encoded state controller address, if present.

**Kind**: instance method of [<code>IotaDocumentMetadata</code>](#IotaDocumentMetadata)  
<a name="IotaDocumentMetadata+governorAddress"></a>

### iotaDocumentMetadata.governorAddress() ⇒ <code>string</code> \| <code>undefined</code>
Returns a copy of the Bech32-encoded governor address, if present.

**Kind**: instance method of [<code>IotaDocumentMetadata</code>](#IotaDocumentMetadata)  
<a name="IotaDocumentMetadata+properties"></a>

### iotaDocumentMetadata.properties() ⇒ <code>Map.&lt;string, any&gt;</code>
Returns a copy of the custom metadata properties.

**Kind**: instance method of [<code>IotaDocumentMetadata</code>](#IotaDocumentMetadata)  
<a name="IotaDocumentMetadata+toJSON"></a>

### iotaDocumentMetadata.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>IotaDocumentMetadata</code>](#IotaDocumentMetadata)  
<a name="IotaDocumentMetadata+clone"></a>

### iotaDocumentMetadata.clone() ⇒ [<code>IotaDocumentMetadata</code>](#IotaDocumentMetadata)
Deep clones the object.

**Kind**: instance method of [<code>IotaDocumentMetadata</code>](#IotaDocumentMetadata)  
<a name="IotaDocumentMetadata.fromJSON"></a>

### IotaDocumentMetadata.fromJSON(json) ⇒ [<code>IotaDocumentMetadata</code>](#IotaDocumentMetadata)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>IotaDocumentMetadata</code>](#IotaDocumentMetadata)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="IotaIdentityClientExt"></a>

## IotaIdentityClientExt
An extension interface that provides helper functions for publication
and resolution of DID documents in Alias Outputs.

**Kind**: global class  

* [IotaIdentityClientExt](#IotaIdentityClientExt)
    * [.newDidOutput(client, address, document, rentStructure)](#IotaIdentityClientExt.newDidOutput) ⇒ <code>Promise.&lt;IAliasOutput&gt;</code>
    * [.updateDidOutput(client, document)](#IotaIdentityClientExt.updateDidOutput) ⇒ <code>Promise.&lt;IAliasOutput&gt;</code>
    * [.deactivateDidOutput(client, did)](#IotaIdentityClientExt.deactivateDidOutput) ⇒ <code>Promise.&lt;IAliasOutput&gt;</code>
    * [.resolveDid(client, did)](#IotaIdentityClientExt.resolveDid) ⇒ [<code>Promise.&lt;IotaDocument&gt;</code>](#IotaDocument)
    * [.resolveDidOutput(client, did)](#IotaIdentityClientExt.resolveDidOutput) ⇒ <code>Promise.&lt;IAliasOutput&gt;</code>

<a name="IotaIdentityClientExt.newDidOutput"></a>

### IotaIdentityClientExt.newDidOutput(client, address, document, rentStructure) ⇒ <code>Promise.&lt;IAliasOutput&gt;</code>
Create a DID with a new Alias Output containing the given `document`.

The `address` will be set as the state controller and governor unlock conditions.
The minimum required token deposit amount will be set according to the given
`rent_structure`, which will be fetched from the node if not provided.
The returned Alias Output can be further customised before publication, if desired.

NOTE: this does *not* publish the Alias Output.

**Kind**: static method of [<code>IotaIdentityClientExt</code>](#IotaIdentityClientExt)  

| Param | Type |
| --- | --- |
| client | <code>IIotaIdentityClient</code> | 
| address | <code>AddressTypes</code> | 
| document | [<code>IotaDocument</code>](#IotaDocument) | 
| rentStructure | <code>IRent</code> \| <code>undefined</code> | 

<a name="IotaIdentityClientExt.updateDidOutput"></a>

### IotaIdentityClientExt.updateDidOutput(client, document) ⇒ <code>Promise.&lt;IAliasOutput&gt;</code>
Fetches the associated Alias Output and updates it with `document` in its state metadata.
The storage deposit on the output is left unchanged. If the size of the document increased,
the amount should be increased manually.

NOTE: this does *not* publish the updated Alias Output.

**Kind**: static method of [<code>IotaIdentityClientExt</code>](#IotaIdentityClientExt)  

| Param | Type |
| --- | --- |
| client | <code>IIotaIdentityClient</code> | 
| document | [<code>IotaDocument</code>](#IotaDocument) | 

<a name="IotaIdentityClientExt.deactivateDidOutput"></a>

### IotaIdentityClientExt.deactivateDidOutput(client, did) ⇒ <code>Promise.&lt;IAliasOutput&gt;</code>
Removes the DID document from the state metadata of its Alias Output,
effectively deactivating it. The storage deposit on the output is left unchanged,
and should be reallocated manually.

Deactivating does not destroy the output. Hence, it can be re-activated by publishing
an update containing a DID document.

NOTE: this does *not* publish the updated Alias Output.

**Kind**: static method of [<code>IotaIdentityClientExt</code>](#IotaIdentityClientExt)  

| Param | Type |
| --- | --- |
| client | <code>IIotaIdentityClient</code> | 
| did | [<code>IotaDID</code>](#IotaDID) | 

<a name="IotaIdentityClientExt.resolveDid"></a>

### IotaIdentityClientExt.resolveDid(client, did) ⇒ [<code>Promise.&lt;IotaDocument&gt;</code>](#IotaDocument)
Resolve a [IotaDocument](#IotaDocument). Returns an empty, deactivated document if the state metadata
of the Alias Output is empty.

**Kind**: static method of [<code>IotaIdentityClientExt</code>](#IotaIdentityClientExt)  

| Param | Type |
| --- | --- |
| client | <code>IIotaIdentityClient</code> | 
| did | [<code>IotaDID</code>](#IotaDID) | 

<a name="IotaIdentityClientExt.resolveDidOutput"></a>

### IotaIdentityClientExt.resolveDidOutput(client, did) ⇒ <code>Promise.&lt;IAliasOutput&gt;</code>
Fetches the `IAliasOutput` associated with the given DID.

**Kind**: static method of [<code>IotaIdentityClientExt</code>](#IotaIdentityClientExt)  

| Param | Type |
| --- | --- |
| client | <code>IIotaIdentityClient</code> | 
| did | [<code>IotaDID</code>](#IotaDID) | 

<a name="KeyPair"></a>

## KeyPair
**Kind**: global class  

* [KeyPair](#KeyPair)
    * [new KeyPair(type_)](#new_KeyPair_new)
    * _instance_
        * [.type()](#KeyPair+type) ⇒ <code>number</code>
        * [.public()](#KeyPair+public) ⇒ <code>Uint8Array</code>
        * [.private()](#KeyPair+private) ⇒ <code>Uint8Array</code>
        * [.toJSON()](#KeyPair+toJSON) ⇒ <code>any</code>
        * [.clone()](#KeyPair+clone) ⇒ [<code>KeyPair</code>](#KeyPair)
    * _static_
        * [.fromKeys(type_, public_key, private_key)](#KeyPair.fromKeys) ⇒ [<code>KeyPair</code>](#KeyPair)
        * [.tryFromPrivateKeyBytes(keyType, privateKeyBytes)](#KeyPair.tryFromPrivateKeyBytes) ⇒ [<code>KeyPair</code>](#KeyPair)
        * [.fromJSON(json)](#KeyPair.fromJSON) ⇒ [<code>KeyPair</code>](#KeyPair)

<a name="new_KeyPair_new"></a>

### new KeyPair(type_)
Generates a new `KeyPair` object.


| Param | Type |
| --- | --- |
| type_ | <code>number</code> | 

<a name="KeyPair+type"></a>

### keyPair.type() ⇒ <code>number</code>
Returns the `KeyType` of the `KeyPair` object.

**Kind**: instance method of [<code>KeyPair</code>](#KeyPair)  
<a name="KeyPair+public"></a>

### keyPair.public() ⇒ <code>Uint8Array</code>
Returns a copy of the public key as a `Uint8Array`.

**Kind**: instance method of [<code>KeyPair</code>](#KeyPair)  
<a name="KeyPair+private"></a>

### keyPair.private() ⇒ <code>Uint8Array</code>
Returns a copy of the private key as a `Uint8Array`.

**Kind**: instance method of [<code>KeyPair</code>](#KeyPair)  
<a name="KeyPair+toJSON"></a>

### keyPair.toJSON() ⇒ <code>any</code>
Serializes a `KeyPair` object as a JSON object.

**Kind**: instance method of [<code>KeyPair</code>](#KeyPair)  
<a name="KeyPair+clone"></a>

### keyPair.clone() ⇒ [<code>KeyPair</code>](#KeyPair)
Deep clones the object.

**Kind**: instance method of [<code>KeyPair</code>](#KeyPair)  
<a name="KeyPair.fromKeys"></a>

### KeyPair.fromKeys(type_, public_key, private_key) ⇒ [<code>KeyPair</code>](#KeyPair)
Parses a `KeyPair` object from the public/private keys.

**Kind**: static method of [<code>KeyPair</code>](#KeyPair)  

| Param | Type |
| --- | --- |
| type_ | <code>number</code> | 
| public_key | <code>Uint8Array</code> | 
| private_key | <code>Uint8Array</code> | 

<a name="KeyPair.tryFromPrivateKeyBytes"></a>

### KeyPair.tryFromPrivateKeyBytes(keyType, privateKeyBytes) ⇒ [<code>KeyPair</code>](#KeyPair)
Reconstructs a `KeyPair` from the bytes of a private key.

The private key for `Ed25519` must be a 32-byte seed in compliance
with [RFC 8032](https://datatracker.ietf.org/doc/html/rfc8032#section-3.2).
Other implementations often use another format. See [this blog post](https://blog.mozilla.org/warner/2011/11/29/ed25519-keys/) for further explanation.

**Kind**: static method of [<code>KeyPair</code>](#KeyPair)  

| Param | Type |
| --- | --- |
| keyType | <code>number</code> | 
| privateKeyBytes | <code>Uint8Array</code> | 

<a name="KeyPair.fromJSON"></a>

### KeyPair.fromJSON(json) ⇒ [<code>KeyPair</code>](#KeyPair)
Deserializes a `KeyPair` object from a JSON object.

**Kind**: static method of [<code>KeyPair</code>](#KeyPair)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="LinkedDomainService"></a>

## LinkedDomainService
**Kind**: global class  

* [LinkedDomainService](#LinkedDomainService)
    * [new LinkedDomainService(options)](#new_LinkedDomainService_new)
    * _instance_
        * [.domains()](#LinkedDomainService+domains) ⇒ <code>Array.&lt;string&gt;</code>
        * [.toService()](#LinkedDomainService+toService) ⇒ [<code>Service</code>](#Service)
        * [.clone()](#LinkedDomainService+clone) ⇒ [<code>LinkedDomainService</code>](#LinkedDomainService)
    * _static_
        * [.fromService(service)](#LinkedDomainService.fromService) ⇒ [<code>LinkedDomainService</code>](#LinkedDomainService)
        * [.isValid(service)](#LinkedDomainService.isValid) ⇒ <code>boolean</code>

<a name="new_LinkedDomainService_new"></a>

### new LinkedDomainService(options)
Constructs a new `LinkedDomainService` that wraps a spec compliant [Linked Domain Service Endpoint](https://identity.foundation/.well-known/resources/did-configuration/#linked-domain-service-endpoint)
Domain URLs must include the `https` scheme in order to pass the domain linkage validation.


| Param | Type |
| --- | --- |
| options | <code>ILinkedDomainService</code> | 

<a name="LinkedDomainService+domains"></a>

### linkedDomainService.domains() ⇒ <code>Array.&lt;string&gt;</code>
Returns the domains contained in the Linked Domain Service.

**Kind**: instance method of [<code>LinkedDomainService</code>](#LinkedDomainService)  
<a name="LinkedDomainService+toService"></a>

### linkedDomainService.toService() ⇒ [<code>Service</code>](#Service)
Returns the inner service which can be added to a DID Document.

**Kind**: instance method of [<code>LinkedDomainService</code>](#LinkedDomainService)  
<a name="LinkedDomainService+clone"></a>

### linkedDomainService.clone() ⇒ [<code>LinkedDomainService</code>](#LinkedDomainService)
Deep clones the object.

**Kind**: instance method of [<code>LinkedDomainService</code>](#LinkedDomainService)  
<a name="LinkedDomainService.fromService"></a>

### LinkedDomainService.fromService(service) ⇒ [<code>LinkedDomainService</code>](#LinkedDomainService)
Creates a new @link{LinkedDomainService} from a @link{Service}.
# Error
Errors if `service` is not a valid Linked Domain Service.

**Kind**: static method of [<code>LinkedDomainService</code>](#LinkedDomainService)  

| Param | Type |
| --- | --- |
| service | [<code>Service</code>](#Service) | 

<a name="LinkedDomainService.isValid"></a>

### LinkedDomainService.isValid(service) ⇒ <code>boolean</code>
Returns `true` if a @link{Service} is a valid Linked Domain Service.

**Kind**: static method of [<code>LinkedDomainService</code>](#LinkedDomainService)  

| Param | Type |
| --- | --- |
| service | [<code>Service</code>](#Service) | 

<a name="MethodData"></a>

## MethodData
Supported verification method data formats.

**Kind**: global class  

* [MethodData](#MethodData)
    * _instance_
        * [.tryDecode()](#MethodData+tryDecode) ⇒ <code>Uint8Array</code>
        * [.toJSON()](#MethodData+toJSON) ⇒ <code>any</code>
        * [.clone()](#MethodData+clone) ⇒ [<code>MethodData</code>](#MethodData)
    * _static_
        * [.newBase58(data)](#MethodData.newBase58) ⇒ [<code>MethodData</code>](#MethodData)
        * [.newMultibase(data)](#MethodData.newMultibase) ⇒ [<code>MethodData</code>](#MethodData)
        * [.fromJSON(json)](#MethodData.fromJSON) ⇒ [<code>MethodData</code>](#MethodData)

<a name="MethodData+tryDecode"></a>

### methodData.tryDecode() ⇒ <code>Uint8Array</code>
Returns a `Uint8Array` containing the decoded bytes of the `MethodData`.

This is generally a public key identified by a `MethodData` value.

### Errors
Decoding can fail if `MethodData` has invalid content or cannot be
represented as a vector of bytes.

**Kind**: instance method of [<code>MethodData</code>](#MethodData)  
<a name="MethodData+toJSON"></a>

### methodData.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>MethodData</code>](#MethodData)  
<a name="MethodData+clone"></a>

### methodData.clone() ⇒ [<code>MethodData</code>](#MethodData)
Deep clones the object.

**Kind**: instance method of [<code>MethodData</code>](#MethodData)  
<a name="MethodData.newBase58"></a>

### MethodData.newBase58(data) ⇒ [<code>MethodData</code>](#MethodData)
Creates a new `MethodData` variant with Base58-BTC encoded content.

**Kind**: static method of [<code>MethodData</code>](#MethodData)  

| Param | Type |
| --- | --- |
| data | <code>Uint8Array</code> | 

<a name="MethodData.newMultibase"></a>

### MethodData.newMultibase(data) ⇒ [<code>MethodData</code>](#MethodData)
Creates a new `MethodData` variant with Multibase-encoded content.

**Kind**: static method of [<code>MethodData</code>](#MethodData)  

| Param | Type |
| --- | --- |
| data | <code>Uint8Array</code> | 

<a name="MethodData.fromJSON"></a>

### MethodData.fromJSON(json) ⇒ [<code>MethodData</code>](#MethodData)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>MethodData</code>](#MethodData)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="MethodScope"></a>

## MethodScope
Supported verification method types.

**Kind**: global class  

* [MethodScope](#MethodScope)
    * _instance_
        * [.toString()](#MethodScope+toString) ⇒ <code>string</code>
        * [.toJSON()](#MethodScope+toJSON) ⇒ <code>any</code>
        * [.clone()](#MethodScope+clone) ⇒ [<code>MethodScope</code>](#MethodScope)
    * _static_
        * [.VerificationMethod()](#MethodScope.VerificationMethod) ⇒ [<code>MethodScope</code>](#MethodScope)
        * [.Authentication()](#MethodScope.Authentication) ⇒ [<code>MethodScope</code>](#MethodScope)
        * [.AssertionMethod()](#MethodScope.AssertionMethod) ⇒ [<code>MethodScope</code>](#MethodScope)
        * [.KeyAgreement()](#MethodScope.KeyAgreement) ⇒ [<code>MethodScope</code>](#MethodScope)
        * [.CapabilityDelegation()](#MethodScope.CapabilityDelegation) ⇒ [<code>MethodScope</code>](#MethodScope)
        * [.CapabilityInvocation()](#MethodScope.CapabilityInvocation) ⇒ [<code>MethodScope</code>](#MethodScope)
        * [.fromJSON(json)](#MethodScope.fromJSON) ⇒ [<code>MethodScope</code>](#MethodScope)

<a name="MethodScope+toString"></a>

### methodScope.toString() ⇒ <code>string</code>
Returns the `MethodScope` as a string.

**Kind**: instance method of [<code>MethodScope</code>](#MethodScope)  
<a name="MethodScope+toJSON"></a>

### methodScope.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>MethodScope</code>](#MethodScope)  
<a name="MethodScope+clone"></a>

### methodScope.clone() ⇒ [<code>MethodScope</code>](#MethodScope)
Deep clones the object.

**Kind**: instance method of [<code>MethodScope</code>](#MethodScope)  
<a name="MethodScope.VerificationMethod"></a>

### MethodScope.VerificationMethod() ⇒ [<code>MethodScope</code>](#MethodScope)
**Kind**: static method of [<code>MethodScope</code>](#MethodScope)  
<a name="MethodScope.Authentication"></a>

### MethodScope.Authentication() ⇒ [<code>MethodScope</code>](#MethodScope)
**Kind**: static method of [<code>MethodScope</code>](#MethodScope)  
<a name="MethodScope.AssertionMethod"></a>

### MethodScope.AssertionMethod() ⇒ [<code>MethodScope</code>](#MethodScope)
**Kind**: static method of [<code>MethodScope</code>](#MethodScope)  
<a name="MethodScope.KeyAgreement"></a>

### MethodScope.KeyAgreement() ⇒ [<code>MethodScope</code>](#MethodScope)
**Kind**: static method of [<code>MethodScope</code>](#MethodScope)  
<a name="MethodScope.CapabilityDelegation"></a>

### MethodScope.CapabilityDelegation() ⇒ [<code>MethodScope</code>](#MethodScope)
**Kind**: static method of [<code>MethodScope</code>](#MethodScope)  
<a name="MethodScope.CapabilityInvocation"></a>

### MethodScope.CapabilityInvocation() ⇒ [<code>MethodScope</code>](#MethodScope)
**Kind**: static method of [<code>MethodScope</code>](#MethodScope)  
<a name="MethodScope.fromJSON"></a>

### MethodScope.fromJSON(json) ⇒ [<code>MethodScope</code>](#MethodScope)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>MethodScope</code>](#MethodScope)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="MethodType"></a>

## MethodType
Supported verification method types.

**Kind**: global class  

* [MethodType](#MethodType)
    * _instance_
        * [.toString()](#MethodType+toString) ⇒ <code>string</code>
        * [.toJSON()](#MethodType+toJSON) ⇒ <code>any</code>
        * [.clone()](#MethodType+clone) ⇒ [<code>MethodType</code>](#MethodType)
    * _static_
        * [.Ed25519VerificationKey2018()](#MethodType.Ed25519VerificationKey2018) ⇒ [<code>MethodType</code>](#MethodType)
        * [.X25519KeyAgreementKey2019()](#MethodType.X25519KeyAgreementKey2019) ⇒ [<code>MethodType</code>](#MethodType)
        * [.fromJSON(json)](#MethodType.fromJSON) ⇒ [<code>MethodType</code>](#MethodType)

<a name="MethodType+toString"></a>

### methodType.toString() ⇒ <code>string</code>
Returns the `MethodType` as a string.

**Kind**: instance method of [<code>MethodType</code>](#MethodType)  
<a name="MethodType+toJSON"></a>

### methodType.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>MethodType</code>](#MethodType)  
<a name="MethodType+clone"></a>

### methodType.clone() ⇒ [<code>MethodType</code>](#MethodType)
Deep clones the object.

**Kind**: instance method of [<code>MethodType</code>](#MethodType)  
<a name="MethodType.Ed25519VerificationKey2018"></a>

### MethodType.Ed25519VerificationKey2018() ⇒ [<code>MethodType</code>](#MethodType)
**Kind**: static method of [<code>MethodType</code>](#MethodType)  
<a name="MethodType.X25519KeyAgreementKey2019"></a>

### MethodType.X25519KeyAgreementKey2019() ⇒ [<code>MethodType</code>](#MethodType)
**Kind**: static method of [<code>MethodType</code>](#MethodType)  
<a name="MethodType.fromJSON"></a>

### MethodType.fromJSON(json) ⇒ [<code>MethodType</code>](#MethodType)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>MethodType</code>](#MethodType)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="Presentation"></a>

## Presentation
**Kind**: global class  

* [Presentation](#Presentation)
    * [new Presentation(values)](#new_Presentation_new)
    * _instance_
        * [.context()](#Presentation+context) ⇒ <code>Array.&lt;(string\|Record.&lt;string, any&gt;)&gt;</code>
        * [.id()](#Presentation+id) ⇒ <code>string</code> \| <code>undefined</code>
        * [.type()](#Presentation+type) ⇒ <code>Array.&lt;string&gt;</code>
        * [.verifiableCredential()](#Presentation+verifiableCredential) ⇒ [<code>Array.&lt;Credential&gt;</code>](#Credential)
        * [.holder()](#Presentation+holder) ⇒ <code>string</code> \| <code>undefined</code>
        * [.refreshService()](#Presentation+refreshService) ⇒ <code>Array.&lt;RefreshService&gt;</code>
        * [.termsOfUse()](#Presentation+termsOfUse) ⇒ <code>Array.&lt;Policy&gt;</code>
        * [.proof()](#Presentation+proof) ⇒ [<code>Proof</code>](#Proof) \| <code>undefined</code>
        * [.properties()](#Presentation+properties) ⇒ <code>Map.&lt;string, any&gt;</code>
        * [.toJSON()](#Presentation+toJSON) ⇒ <code>any</code>
        * [.clone()](#Presentation+clone) ⇒ [<code>Presentation</code>](#Presentation)
    * _static_
        * [.BaseContext()](#Presentation.BaseContext) ⇒ <code>string</code>
        * [.BaseType()](#Presentation.BaseType) ⇒ <code>string</code>
        * [.fromJSON(json)](#Presentation.fromJSON) ⇒ [<code>Presentation</code>](#Presentation)

<a name="new_Presentation_new"></a>

### new Presentation(values)
Constructs a new `Presentation`.


| Param | Type |
| --- | --- |
| values | <code>IPresentation</code> | 

<a name="Presentation+context"></a>

### presentation.context() ⇒ <code>Array.&lt;(string\|Record.&lt;string, any&gt;)&gt;</code>
Returns a copy of the JSON-LD context(s) applicable to the `Presentation`.

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation+id"></a>

### presentation.id() ⇒ <code>string</code> \| <code>undefined</code>
Returns a copy of the unique `URI` identifying the `Presentation`.

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation+type"></a>

### presentation.type() ⇒ <code>Array.&lt;string&gt;</code>
Returns a copy of the URIs defining the type of the `Presentation`.

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation+verifiableCredential"></a>

### presentation.verifiableCredential() ⇒ [<code>Array.&lt;Credential&gt;</code>](#Credential)
Returns a copy of the [Credential](#Credential)(s) expressing the claims of the `Presentation`.

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation+holder"></a>

### presentation.holder() ⇒ <code>string</code> \| <code>undefined</code>
Returns a copy of the URI of the entity that generated the `Presentation`.

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation+refreshService"></a>

### presentation.refreshService() ⇒ <code>Array.&lt;RefreshService&gt;</code>
Returns a copy of the service(s) used to refresh an expired [Credential](#Credential) in the `Presentation`.

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation+termsOfUse"></a>

### presentation.termsOfUse() ⇒ <code>Array.&lt;Policy&gt;</code>
Returns a copy of the terms-of-use specified by the `Presentation` holder

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation+proof"></a>

### presentation.proof() ⇒ [<code>Proof</code>](#Proof) \| <code>undefined</code>
Returns a copy of the proof used to verify the `Presentation`.

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation+properties"></a>

### presentation.properties() ⇒ <code>Map.&lt;string, any&gt;</code>
Returns a copy of the miscellaneous properties on the `Presentation`.

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation+toJSON"></a>

### presentation.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation+clone"></a>

### presentation.clone() ⇒ [<code>Presentation</code>](#Presentation)
Deep clones the object.

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation.BaseContext"></a>

### Presentation.BaseContext() ⇒ <code>string</code>
Returns the base JSON-LD context.

**Kind**: static method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation.BaseType"></a>

### Presentation.BaseType() ⇒ <code>string</code>
Returns the base type.

**Kind**: static method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation.fromJSON"></a>

### Presentation.fromJSON(json) ⇒ [<code>Presentation</code>](#Presentation)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>Presentation</code>](#Presentation)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="PresentationValidationOptions"></a>

## PresentationValidationOptions
Options to declare validation criteria when validating presentation.

**Kind**: global class  

* [PresentationValidationOptions](#PresentationValidationOptions)
    * [new PresentationValidationOptions(options)](#new_PresentationValidationOptions_new)
    * _instance_
        * [.toJSON()](#PresentationValidationOptions+toJSON) ⇒ <code>any</code>
        * [.clone()](#PresentationValidationOptions+clone) ⇒ [<code>PresentationValidationOptions</code>](#PresentationValidationOptions)
    * _static_
        * [.default()](#PresentationValidationOptions.default) ⇒ [<code>PresentationValidationOptions</code>](#PresentationValidationOptions)
        * [.fromJSON(json)](#PresentationValidationOptions.fromJSON) ⇒ [<code>PresentationValidationOptions</code>](#PresentationValidationOptions)

<a name="new_PresentationValidationOptions_new"></a>

### new PresentationValidationOptions(options)
Creates a new `PresentationValidationOptions` from the given fields.

Throws an error if any of the options are invalid.


| Param | Type |
| --- | --- |
| options | <code>IPresentationValidationOptions</code> | 

<a name="PresentationValidationOptions+toJSON"></a>

### presentationValidationOptions.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>PresentationValidationOptions</code>](#PresentationValidationOptions)  
<a name="PresentationValidationOptions+clone"></a>

### presentationValidationOptions.clone() ⇒ [<code>PresentationValidationOptions</code>](#PresentationValidationOptions)
Deep clones the object.

**Kind**: instance method of [<code>PresentationValidationOptions</code>](#PresentationValidationOptions)  
<a name="PresentationValidationOptions.default"></a>

### PresentationValidationOptions.default() ⇒ [<code>PresentationValidationOptions</code>](#PresentationValidationOptions)
Creates a new `PresentationValidationOptions` with defaults.

**Kind**: static method of [<code>PresentationValidationOptions</code>](#PresentationValidationOptions)  
<a name="PresentationValidationOptions.fromJSON"></a>

### PresentationValidationOptions.fromJSON(json) ⇒ [<code>PresentationValidationOptions</code>](#PresentationValidationOptions)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>PresentationValidationOptions</code>](#PresentationValidationOptions)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="PresentationValidator"></a>

## PresentationValidator
**Kind**: global class  

* [PresentationValidator](#PresentationValidator)
    * [.validate(presentation, holder, issuers, options, fail_fast)](#PresentationValidator.validate)
    * [.verifyPresentationSignature(presentation, holder, options)](#PresentationValidator.verifyPresentationSignature)
    * [.checkStructure(presentation)](#PresentationValidator.checkStructure)
    * [.extractHolder(presentation)](#PresentationValidator.extractHolder) ⇒ [<code>CoreDID</code>](#CoreDID)

<a name="PresentationValidator.validate"></a>

### PresentationValidator.validate(presentation, holder, issuers, options, fail_fast)
Validate a `Presentation`.

The following properties are validated according to `options`:
- the semantic structure of the presentation,
- the holder's signature,
- the relationship between the holder and the credential subjects,
- the signatures and some properties of the constituent credentials (see
`CredentialValidator::validate`).

### Warning
The lack of an error returned from this method is in of itself not enough to conclude that the presentation can be
trusted. This section contains more information on additional checks that should be carried out before and after
calling this method.

#### The state of the supplied DID Documents.
The caller must ensure that the DID Documents in `holder` and `issuers` are up-to-date. The convenience methods
`Resolver::resolve_presentation_holder` and `Resolver::resolve_presentation_issuers`
can help extract the latest available states of these DID Documents.

#### Properties that are not validated
 There are many properties defined in [The Verifiable Credentials Data Model](https://www.w3.org/TR/vc-data-model/) that are **not** validated, such as:
`credentialStatus`, `type`, `credentialSchema`, `refreshService`, **and more**.
These should be manually checked after validation, according to your requirements.

### Errors
An error is returned whenever a validated condition is not satisfied.

**Kind**: static method of [<code>PresentationValidator</code>](#PresentationValidator)  

| Param | Type |
| --- | --- |
| presentation | [<code>Presentation</code>](#Presentation) | 
| holder | [<code>CoreDocument</code>](#CoreDocument) \| <code>IToCoreDocument</code> | 
| issuers | <code>Array.&lt;(CoreDocument\|IToCoreDocument)&gt;</code> | 
| options | [<code>PresentationValidationOptions</code>](#PresentationValidationOptions) | 
| fail_fast | <code>number</code> | 

<a name="PresentationValidator.verifyPresentationSignature"></a>

### PresentationValidator.verifyPresentationSignature(presentation, holder, options)
Verify the presentation's signature using the resolved document of the holder.

### Warning
The caller must ensure that the DID Document of the holder is up-to-date.

### Errors
Fails if the `holder` does not match the `presentation`'s holder property.
Fails if signature verification against the holder document fails.

**Kind**: static method of [<code>PresentationValidator</code>](#PresentationValidator)  

| Param | Type |
| --- | --- |
| presentation | [<code>Presentation</code>](#Presentation) | 
| holder | [<code>CoreDocument</code>](#CoreDocument) \| <code>IToCoreDocument</code> | 
| options | [<code>VerifierOptions</code>](#VerifierOptions) | 

<a name="PresentationValidator.checkStructure"></a>

### PresentationValidator.checkStructure(presentation)
Validates the semantic structure of the `Presentation`.

**Kind**: static method of [<code>PresentationValidator</code>](#PresentationValidator)  

| Param | Type |
| --- | --- |
| presentation | [<code>Presentation</code>](#Presentation) | 

<a name="PresentationValidator.extractHolder"></a>

### PresentationValidator.extractHolder(presentation) ⇒ [<code>CoreDID</code>](#CoreDID)
Utility for extracting the holder field of a `Presentation` as a DID.

### Errors

Fails if the holder field is missing or not a valid DID.

**Kind**: static method of [<code>PresentationValidator</code>](#PresentationValidator)  

| Param | Type |
| --- | --- |
| presentation | [<code>Presentation</code>](#Presentation) | 

<a name="Proof"></a>

## Proof
A digital signature.

For field definitions see: https://w3c-ccg.github.io/security-vocab/

**Kind**: global class  

* [Proof](#Proof)
    * _instance_
        * [.type()](#Proof+type) ⇒ <code>string</code>
        * [.value()](#Proof+value) ⇒ <code>string</code>
        * [.verificationMethod()](#Proof+verificationMethod) ⇒ <code>string</code>
        * [.created()](#Proof+created) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
        * [.expires()](#Proof+expires) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
        * [.challenge()](#Proof+challenge) ⇒ <code>string</code> \| <code>undefined</code>
        * [.domain()](#Proof+domain) ⇒ <code>string</code> \| <code>undefined</code>
        * [.purpose()](#Proof+purpose) ⇒ [<code>ProofPurpose</code>](#ProofPurpose) \| <code>undefined</code>
        * [.toJSON()](#Proof+toJSON) ⇒ <code>any</code>
        * [.clone()](#Proof+clone) ⇒ [<code>Proof</code>](#Proof)
    * _static_
        * [.fromJSON(json)](#Proof.fromJSON) ⇒ [<code>Proof</code>](#Proof)

<a name="Proof+type"></a>

### proof.type() ⇒ <code>string</code>
Returns a copy of the proof type.

**Kind**: instance method of [<code>Proof</code>](#Proof)  
<a name="Proof+value"></a>

### proof.value() ⇒ <code>string</code>
Returns a copy of the proof value string.

**Kind**: instance method of [<code>Proof</code>](#Proof)  
<a name="Proof+verificationMethod"></a>

### proof.verificationMethod() ⇒ <code>string</code>
Returns a copy of the identifier of the DID method used to create this proof.

**Kind**: instance method of [<code>Proof</code>](#Proof)  
<a name="Proof+created"></a>

### proof.created() ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
When the proof was generated.

**Kind**: instance method of [<code>Proof</code>](#Proof)  
<a name="Proof+expires"></a>

### proof.expires() ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
When the proof expires.

**Kind**: instance method of [<code>Proof</code>](#Proof)  
<a name="Proof+challenge"></a>

### proof.challenge() ⇒ <code>string</code> \| <code>undefined</code>
Challenge from a proof requester to mitigate replay attacks.

**Kind**: instance method of [<code>Proof</code>](#Proof)  
<a name="Proof+domain"></a>

### proof.domain() ⇒ <code>string</code> \| <code>undefined</code>
Domain for which a proof is valid to mitigate replay attacks.

**Kind**: instance method of [<code>Proof</code>](#Proof)  
<a name="Proof+purpose"></a>

### proof.purpose() ⇒ [<code>ProofPurpose</code>](#ProofPurpose) \| <code>undefined</code>
Purpose for which the proof was generated.

**Kind**: instance method of [<code>Proof</code>](#Proof)  
<a name="Proof+toJSON"></a>

### proof.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>Proof</code>](#Proof)  
<a name="Proof+clone"></a>

### proof.clone() ⇒ [<code>Proof</code>](#Proof)
Deep clones the object.

**Kind**: instance method of [<code>Proof</code>](#Proof)  
<a name="Proof.fromJSON"></a>

### Proof.fromJSON(json) ⇒ [<code>Proof</code>](#Proof)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>Proof</code>](#Proof)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="ProofOptions"></a>

## ProofOptions
Holds additional options for creating signatures.
See `IProofOptions`.

**Kind**: global class  

* [ProofOptions](#ProofOptions)
    * [new ProofOptions(options)](#new_ProofOptions_new)
    * _instance_
        * [.toJSON()](#ProofOptions+toJSON) ⇒ <code>any</code>
        * [.clone()](#ProofOptions+clone) ⇒ [<code>ProofOptions</code>](#ProofOptions)
    * _static_
        * [.default()](#ProofOptions.default) ⇒ [<code>ProofOptions</code>](#ProofOptions)
        * [.fromJSON(json)](#ProofOptions.fromJSON) ⇒ [<code>ProofOptions</code>](#ProofOptions)

<a name="new_ProofOptions_new"></a>

### new ProofOptions(options)
Creates a new `ProofOptions` from the given fields.

Throws an error if any of the options are invalid.


| Param | Type |
| --- | --- |
| options | <code>IProofOptions</code> | 

<a name="ProofOptions+toJSON"></a>

### proofOptions.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>ProofOptions</code>](#ProofOptions)  
<a name="ProofOptions+clone"></a>

### proofOptions.clone() ⇒ [<code>ProofOptions</code>](#ProofOptions)
Deep clones the object.

**Kind**: instance method of [<code>ProofOptions</code>](#ProofOptions)  
<a name="ProofOptions.default"></a>

### ProofOptions.default() ⇒ [<code>ProofOptions</code>](#ProofOptions)
Creates a new `ProofOptions` with default options.

**Kind**: static method of [<code>ProofOptions</code>](#ProofOptions)  
<a name="ProofOptions.fromJSON"></a>

### ProofOptions.fromJSON(json) ⇒ [<code>ProofOptions</code>](#ProofOptions)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>ProofOptions</code>](#ProofOptions)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="ProofPurpose"></a>

## ProofPurpose
Associates a purpose with a [Proof](#Proof).

See https://w3c-ccg.github.io/security-vocab/#proofPurpose

**Kind**: global class  

* [ProofPurpose](#ProofPurpose)
    * _instance_
        * [.toJSON()](#ProofPurpose+toJSON) ⇒ <code>any</code>
        * [.clone()](#ProofPurpose+clone) ⇒ [<code>ProofPurpose</code>](#ProofPurpose)
    * _static_
        * [.assertionMethod()](#ProofPurpose.assertionMethod) ⇒ [<code>ProofPurpose</code>](#ProofPurpose)
        * [.authentication()](#ProofPurpose.authentication) ⇒ [<code>ProofPurpose</code>](#ProofPurpose)
        * [.fromJSON(json)](#ProofPurpose.fromJSON) ⇒ [<code>ProofPurpose</code>](#ProofPurpose)

<a name="ProofPurpose+toJSON"></a>

### proofPurpose.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>ProofPurpose</code>](#ProofPurpose)  
<a name="ProofPurpose+clone"></a>

### proofPurpose.clone() ⇒ [<code>ProofPurpose</code>](#ProofPurpose)
Deep clones the object.

**Kind**: instance method of [<code>ProofPurpose</code>](#ProofPurpose)  
<a name="ProofPurpose.assertionMethod"></a>

### ProofPurpose.assertionMethod() ⇒ [<code>ProofPurpose</code>](#ProofPurpose)
Purpose is to assert a claim.
See https://www.w3.org/TR/did-core/#assertion

**Kind**: static method of [<code>ProofPurpose</code>](#ProofPurpose)  
<a name="ProofPurpose.authentication"></a>

### ProofPurpose.authentication() ⇒ [<code>ProofPurpose</code>](#ProofPurpose)
Purpose is to authenticate the signer.
See https://www.w3.org/TR/did-core/#authentication

**Kind**: static method of [<code>ProofPurpose</code>](#ProofPurpose)  
<a name="ProofPurpose.fromJSON"></a>

### ProofPurpose.fromJSON(json) ⇒ [<code>ProofPurpose</code>](#ProofPurpose)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>ProofPurpose</code>](#ProofPurpose)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="Resolver"></a>

## Resolver
Convenience type for resolving DID documents from different DID methods.

Also provides methods for resolving DID Documents associated with
verifiable `Credentials` and `Presentations`.

# Configuration
The resolver will only be able to resolve DID documents for methods it has been configured for in the constructor.

**Kind**: global class  

* [Resolver](#Resolver)
    * [new Resolver(config)](#new_Resolver_new)
    * [.resolvePresentationIssuers(presentation)](#Resolver+resolvePresentationIssuers) ⇒ <code>Promise.&lt;Array.&lt;(CoreDocument\|IToCoreDocument)&gt;&gt;</code>
    * [.resolvePresentationHolder(presentation)](#Resolver+resolvePresentationHolder) ⇒ <code>Promise.&lt;(CoreDocument\|IToCoreDocument)&gt;</code>
    * [.verifyPresentation(presentation, options, fail_fast, holder, issuers)](#Resolver+verifyPresentation) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.resolve(did)](#Resolver+resolve) ⇒ <code>Promise.&lt;(CoreDocument\|IToCoreDocument)&gt;</code>

<a name="new_Resolver_new"></a>

### new Resolver(config)
Constructs a new `Resolver`.

# Errors
If both a `client` is given and the `handlers` map contains the "iota" key the construction process
will throw an error because the handler for the "iota" method then becomes ambiguous.


| Param | Type |
| --- | --- |
| config | <code>ResolverConfig</code> | 

<a name="Resolver+resolvePresentationIssuers"></a>

### resolver.resolvePresentationIssuers(presentation) ⇒ <code>Promise.&lt;Array.&lt;(CoreDocument\|IToCoreDocument)&gt;&gt;</code>
Fetches all DID Documents of `Credential` issuers contained in a `Presentation`.
Issuer documents are returned in arbitrary order.

# Errors
Errors if any issuer URL cannot be parsed to a DID whose associated method is supported by this Resolver, or
resolution fails.

**Kind**: instance method of [<code>Resolver</code>](#Resolver)  

| Param | Type |
| --- | --- |
| presentation | [<code>Presentation</code>](#Presentation) | 

<a name="Resolver+resolvePresentationHolder"></a>

### resolver.resolvePresentationHolder(presentation) ⇒ <code>Promise.&lt;(CoreDocument\|IToCoreDocument)&gt;</code>
Fetches the DID Document of the holder of a `Presentation`.

# Errors
Errors if the holder URL is missing, cannot be parsed to a valid DID whose method is supported by the resolver, or
DID resolution fails.

**Kind**: instance method of [<code>Resolver</code>](#Resolver)  

| Param | Type |
| --- | --- |
| presentation | [<code>Presentation</code>](#Presentation) | 

<a name="Resolver+verifyPresentation"></a>

### resolver.verifyPresentation(presentation, options, fail_fast, holder, issuers) ⇒ <code>Promise.&lt;void&gt;</code>
Verifies a `Presentation`.

### Important
See `PresentationValidator::validate` for information about which properties get
validated and what is expected of the optional arguments `holder` and `issuer`.

### Resolution
The DID Documents for the `holder` and `issuers` are optionally resolved if not given.
If you already have up-to-date versions of these DID Documents, you may want
to use `PresentationValidator::validate`.
See also `Resolver::resolvePresentationIssuers` and `Resolver::resolvePresentationHolder`.

### Errors
Errors from resolving the holder and issuer DID Documents, if not provided, will be returned immediately.
Otherwise, errors from validating the presentation and its credentials will be returned
according to the `fail_fast` parameter.

**Kind**: instance method of [<code>Resolver</code>](#Resolver)  

| Param | Type |
| --- | --- |
| presentation | [<code>Presentation</code>](#Presentation) | 
| options | [<code>PresentationValidationOptions</code>](#PresentationValidationOptions) | 
| fail_fast | <code>number</code> | 
| holder | [<code>CoreDocument</code>](#CoreDocument) \| <code>IToCoreDocument</code> \| <code>undefined</code> | 
| issuers | <code>Array.&lt;(CoreDocument\|IToCoreDocument)&gt;</code> \| <code>undefined</code> | 

<a name="Resolver+resolve"></a>

### resolver.resolve(did) ⇒ <code>Promise.&lt;(CoreDocument\|IToCoreDocument)&gt;</code>
Fetches the DID Document of the given DID.

### Errors

Errors if the resolver has not been configured to handle the method
corresponding to the given DID or the resolution process itself fails.

**Kind**: instance method of [<code>Resolver</code>](#Resolver)  

| Param | Type |
| --- | --- |
| did | <code>string</code> | 

<a name="RevocationBitmap"></a>

## RevocationBitmap
A compressed bitmap for managing credential revocation.

**Kind**: global class  

* [RevocationBitmap](#RevocationBitmap)
    * [new RevocationBitmap()](#new_RevocationBitmap_new)
    * _instance_
        * [.isRevoked(index)](#RevocationBitmap+isRevoked) ⇒ <code>boolean</code>
        * [.revoke(index)](#RevocationBitmap+revoke) ⇒ <code>boolean</code>
        * [.unrevoke(index)](#RevocationBitmap+unrevoke) ⇒ <code>boolean</code>
        * [.len()](#RevocationBitmap+len) ⇒ <code>number</code>
        * [.toEndpoint()](#RevocationBitmap+toEndpoint) ⇒ <code>string</code> \| <code>Array.&lt;string&gt;</code> \| <code>Map.&lt;string, Array.&lt;string&gt;&gt;</code>
    * _static_
        * [.type()](#RevocationBitmap.type) ⇒ <code>string</code>
        * [.fromEndpoint(endpoint)](#RevocationBitmap.fromEndpoint) ⇒ [<code>RevocationBitmap</code>](#RevocationBitmap)

<a name="new_RevocationBitmap_new"></a>

### new RevocationBitmap()
Creates a new `RevocationBitmap` instance.

<a name="RevocationBitmap+isRevoked"></a>

### revocationBitmap.isRevoked(index) ⇒ <code>boolean</code>
Returns `true` if the credential at the given `index` is revoked.

**Kind**: instance method of [<code>RevocationBitmap</code>](#RevocationBitmap)  

| Param | Type |
| --- | --- |
| index | <code>number</code> | 

<a name="RevocationBitmap+revoke"></a>

### revocationBitmap.revoke(index) ⇒ <code>boolean</code>
Mark the given index as revoked.

Returns true if the index was absent from the set.

**Kind**: instance method of [<code>RevocationBitmap</code>](#RevocationBitmap)  

| Param | Type |
| --- | --- |
| index | <code>number</code> | 

<a name="RevocationBitmap+unrevoke"></a>

### revocationBitmap.unrevoke(index) ⇒ <code>boolean</code>
Mark the index as not revoked.

Returns true if the index was present in the set.

**Kind**: instance method of [<code>RevocationBitmap</code>](#RevocationBitmap)  

| Param | Type |
| --- | --- |
| index | <code>number</code> | 

<a name="RevocationBitmap+len"></a>

### revocationBitmap.len() ⇒ <code>number</code>
Returns the number of revoked credentials.

**Kind**: instance method of [<code>RevocationBitmap</code>](#RevocationBitmap)  
<a name="RevocationBitmap+toEndpoint"></a>

### revocationBitmap.toEndpoint() ⇒ <code>string</code> \| <code>Array.&lt;string&gt;</code> \| <code>Map.&lt;string, Array.&lt;string&gt;&gt;</code>
Return the bitmap as a data url embedded in a service endpoint.

**Kind**: instance method of [<code>RevocationBitmap</code>](#RevocationBitmap)  
<a name="RevocationBitmap.type"></a>

### RevocationBitmap.type() ⇒ <code>string</code>
The name of the service type.

**Kind**: static method of [<code>RevocationBitmap</code>](#RevocationBitmap)  
<a name="RevocationBitmap.fromEndpoint"></a>

### RevocationBitmap.fromEndpoint(endpoint) ⇒ [<code>RevocationBitmap</code>](#RevocationBitmap)
Construct a `RevocationBitmap` from a data `url`.

**Kind**: static method of [<code>RevocationBitmap</code>](#RevocationBitmap)  

| Param | Type |
| --- | --- |
| endpoint | <code>string</code> \| <code>Array.&lt;string&gt;</code> \| <code>Map.&lt;string, Array.&lt;string&gt;&gt;</code> | 

<a name="Service"></a>

## Service
A DID Document Service used to enable trusted interactions associated with a DID subject.

**Kind**: global class  

* [Service](#Service)
    * [new Service(service)](#new_Service_new)
    * _instance_
        * [.id()](#Service+id) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.type()](#Service+type) ⇒ <code>Array.&lt;string&gt;</code>
        * [.serviceEndpoint()](#Service+serviceEndpoint) ⇒ <code>string</code> \| <code>Array.&lt;string&gt;</code> \| <code>Map.&lt;string, Array.&lt;string&gt;&gt;</code>
        * [.properties()](#Service+properties) ⇒ <code>Map.&lt;string, any&gt;</code>
        * [.toJSON()](#Service+toJSON) ⇒ <code>any</code>
        * [.clone()](#Service+clone) ⇒ [<code>Service</code>](#Service)
    * _static_
        * [.fromJSON(json)](#Service.fromJSON) ⇒ [<code>Service</code>](#Service)

<a name="new_Service_new"></a>

### new Service(service)

| Param | Type |
| --- | --- |
| service | <code>IService</code> | 

<a name="Service+id"></a>

### service.id() ⇒ [<code>DIDUrl</code>](#DIDUrl)
Returns a copy of the `Service` id.

**Kind**: instance method of [<code>Service</code>](#Service)  
<a name="Service+type"></a>

### service.type() ⇒ <code>Array.&lt;string&gt;</code>
Returns a copy of the `Service` type.

**Kind**: instance method of [<code>Service</code>](#Service)  
<a name="Service+serviceEndpoint"></a>

### service.serviceEndpoint() ⇒ <code>string</code> \| <code>Array.&lt;string&gt;</code> \| <code>Map.&lt;string, Array.&lt;string&gt;&gt;</code>
Returns a copy of the `Service` endpoint.

**Kind**: instance method of [<code>Service</code>](#Service)  
<a name="Service+properties"></a>

### service.properties() ⇒ <code>Map.&lt;string, any&gt;</code>
Returns a copy of the custom properties on the `Service`.

**Kind**: instance method of [<code>Service</code>](#Service)  
<a name="Service+toJSON"></a>

### service.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>Service</code>](#Service)  
<a name="Service+clone"></a>

### service.clone() ⇒ [<code>Service</code>](#Service)
Deep clones the object.

**Kind**: instance method of [<code>Service</code>](#Service)  
<a name="Service.fromJSON"></a>

### Service.fromJSON(json) ⇒ [<code>Service</code>](#Service)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>Service</code>](#Service)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="Timestamp"></a>

## Timestamp
**Kind**: global class  

* [Timestamp](#Timestamp)
    * _instance_
        * [.toRFC3339()](#Timestamp+toRFC3339) ⇒ <code>string</code>
        * [.checkedAdd(duration)](#Timestamp+checkedAdd) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
        * [.checkedSub(duration)](#Timestamp+checkedSub) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
        * [.toJSON()](#Timestamp+toJSON) ⇒ <code>any</code>
    * _static_
        * [.parse(input)](#Timestamp.parse) ⇒ [<code>Timestamp</code>](#Timestamp)
        * [.nowUTC()](#Timestamp.nowUTC) ⇒ [<code>Timestamp</code>](#Timestamp)
        * [.fromJSON(json)](#Timestamp.fromJSON) ⇒ [<code>Timestamp</code>](#Timestamp)

<a name="Timestamp+toRFC3339"></a>

### timestamp.toRFC3339() ⇒ <code>string</code>
Returns the `Timestamp` as an RFC 3339 `String`.

**Kind**: instance method of [<code>Timestamp</code>](#Timestamp)  
<a name="Timestamp+checkedAdd"></a>

### timestamp.checkedAdd(duration) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
Computes `self + duration`

Returns `null` if the operation leads to a timestamp not in the valid range for [RFC 3339](https://tools.ietf.org/html/rfc3339).

**Kind**: instance method of [<code>Timestamp</code>](#Timestamp)  

| Param | Type |
| --- | --- |
| duration | [<code>Duration</code>](#Duration) | 

<a name="Timestamp+checkedSub"></a>

### timestamp.checkedSub(duration) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
Computes `self - duration`

Returns `null` if the operation leads to a timestamp not in the valid range for [RFC 3339](https://tools.ietf.org/html/rfc3339).

**Kind**: instance method of [<code>Timestamp</code>](#Timestamp)  

| Param | Type |
| --- | --- |
| duration | [<code>Duration</code>](#Duration) | 

<a name="Timestamp+toJSON"></a>

### timestamp.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>Timestamp</code>](#Timestamp)  
<a name="Timestamp.parse"></a>

### Timestamp.parse(input) ⇒ [<code>Timestamp</code>](#Timestamp)
Parses a `Timestamp` from the provided input string.

**Kind**: static method of [<code>Timestamp</code>](#Timestamp)  

| Param | Type |
| --- | --- |
| input | <code>string</code> | 

<a name="Timestamp.nowUTC"></a>

### Timestamp.nowUTC() ⇒ [<code>Timestamp</code>](#Timestamp)
Creates a new `Timestamp` with the current date and time.

**Kind**: static method of [<code>Timestamp</code>](#Timestamp)  
<a name="Timestamp.fromJSON"></a>

### Timestamp.fromJSON(json) ⇒ [<code>Timestamp</code>](#Timestamp)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>Timestamp</code>](#Timestamp)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="VerificationMethod"></a>

## VerificationMethod
A DID Document Verification Method.

**Kind**: global class  

* [VerificationMethod](#VerificationMethod)
    * [new VerificationMethod(did, keyType, publicKey, fragment)](#new_VerificationMethod_new)
    * _instance_
        * [.id()](#VerificationMethod+id) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.setId(id)](#VerificationMethod+setId)
        * [.controller()](#VerificationMethod+controller) ⇒ [<code>CoreDID</code>](#CoreDID)
        * [.setController(did)](#VerificationMethod+setController)
        * [.type()](#VerificationMethod+type) ⇒ [<code>MethodType</code>](#MethodType)
        * [.setType(type_)](#VerificationMethod+setType)
        * [.data()](#VerificationMethod+data) ⇒ [<code>MethodData</code>](#MethodData)
        * [.setData(data)](#VerificationMethod+setData)
        * [.properties()](#VerificationMethod+properties) ⇒ <code>Map.&lt;string, any&gt;</code>
        * [.setPropertyUnchecked(key, value)](#VerificationMethod+setPropertyUnchecked)
        * [.toJSON()](#VerificationMethod+toJSON) ⇒ <code>any</code>
        * [.clone()](#VerificationMethod+clone) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
    * _static_
        * [.fromJSON(json)](#VerificationMethod.fromJSON) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)

<a name="new_VerificationMethod_new"></a>

### new VerificationMethod(did, keyType, publicKey, fragment)
Creates a new `VerificationMethod` from the given `did` and public key.


| Param | Type |
| --- | --- |
| did | [<code>CoreDID</code>](#CoreDID) \| <code>IToCoreDID</code> | 
| keyType | <code>number</code> | 
| publicKey | <code>Uint8Array</code> | 
| fragment | <code>string</code> | 

<a name="VerificationMethod+id"></a>

### verificationMethod.id() ⇒ [<code>DIDUrl</code>](#DIDUrl)
Returns a copy of the `DIDUrl` of the `VerificationMethod`'s `id`.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod+setId"></a>

### verificationMethod.setId(id)
Sets the id of the `VerificationMethod`.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  

| Param | Type |
| --- | --- |
| id | [<code>DIDUrl</code>](#DIDUrl) | 

<a name="VerificationMethod+controller"></a>

### verificationMethod.controller() ⇒ [<code>CoreDID</code>](#CoreDID)
Returns a copy of the `controller` `DID` of the `VerificationMethod`.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod+setController"></a>

### verificationMethod.setController(did)
Sets the `controller` `DID` of the `VerificationMethod` object.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  

| Param | Type |
| --- | --- |
| did | [<code>CoreDID</code>](#CoreDID) | 

<a name="VerificationMethod+type"></a>

### verificationMethod.type() ⇒ [<code>MethodType</code>](#MethodType)
Returns a copy of the `VerificationMethod` type.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod+setType"></a>

### verificationMethod.setType(type_)
Sets the `VerificationMethod` type.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  

| Param | Type |
| --- | --- |
| type_ | [<code>MethodType</code>](#MethodType) | 

<a name="VerificationMethod+data"></a>

### verificationMethod.data() ⇒ [<code>MethodData</code>](#MethodData)
Returns a copy of the `VerificationMethod` public key data.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod+setData"></a>

### verificationMethod.setData(data)
Sets `VerificationMethod` public key data.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  

| Param | Type |
| --- | --- |
| data | [<code>MethodData</code>](#MethodData) | 

<a name="VerificationMethod+properties"></a>

### verificationMethod.properties() ⇒ <code>Map.&lt;string, any&gt;</code>
Get custom properties of the Verification Method.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod+setPropertyUnchecked"></a>

### verificationMethod.setPropertyUnchecked(key, value)
Adds a custom property to the Verification Method.
If the value is set to `null`, the custom property will be removed.

### WARNING
This method can overwrite existing properties like `id` and result
in an invalid Verification Method.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  

| Param | Type |
| --- | --- |
| key | <code>string</code> | 
| value | <code>any</code> | 

<a name="VerificationMethod+toJSON"></a>

### verificationMethod.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod+clone"></a>

### verificationMethod.clone() ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
Deep clones the object.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod.fromJSON"></a>

### VerificationMethod.fromJSON(json) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>VerificationMethod</code>](#VerificationMethod)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="VerifierOptions"></a>

## VerifierOptions
Holds additional proof verification options.
See `IVerifierOptions`.

**Kind**: global class  

* [VerifierOptions](#VerifierOptions)
    * [new VerifierOptions(options)](#new_VerifierOptions_new)
    * _instance_
        * [.toJSON()](#VerifierOptions+toJSON) ⇒ <code>any</code>
        * [.clone()](#VerifierOptions+clone) ⇒ [<code>VerifierOptions</code>](#VerifierOptions)
    * _static_
        * [.default()](#VerifierOptions.default) ⇒ [<code>VerifierOptions</code>](#VerifierOptions)
        * [.fromJSON(json)](#VerifierOptions.fromJSON) ⇒ [<code>VerifierOptions</code>](#VerifierOptions)

<a name="new_VerifierOptions_new"></a>

### new VerifierOptions(options)
Creates a new `VerifierOptions` from the given fields.

Throws an error if any of the options are invalid.


| Param | Type |
| --- | --- |
| options | <code>IVerifierOptions</code> | 

<a name="VerifierOptions+toJSON"></a>

### verifierOptions.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>VerifierOptions</code>](#VerifierOptions)  
<a name="VerifierOptions+clone"></a>

### verifierOptions.clone() ⇒ [<code>VerifierOptions</code>](#VerifierOptions)
Deep clones the object.

**Kind**: instance method of [<code>VerifierOptions</code>](#VerifierOptions)  
<a name="VerifierOptions.default"></a>

### VerifierOptions.default() ⇒ [<code>VerifierOptions</code>](#VerifierOptions)
Creates a new `VerifierOptions` with default options.

**Kind**: static method of [<code>VerifierOptions</code>](#VerifierOptions)  
<a name="VerifierOptions.fromJSON"></a>

### VerifierOptions.fromJSON(json) ⇒ [<code>VerifierOptions</code>](#VerifierOptions)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>VerifierOptions</code>](#VerifierOptions)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="X25519"></a>

## X25519
An implementation of `X25519` Elliptic-curve Diffie-Hellman (ECDH) cryptographic key exchange.

**Kind**: global class  

* [X25519](#X25519)
    * [.PRIVATE_KEY_LENGTH()](#X25519.PRIVATE_KEY_LENGTH) ⇒ <code>number</code>
    * [.PUBLIC_KEY_LENGTH()](#X25519.PUBLIC_KEY_LENGTH) ⇒ <code>number</code>
    * [.keyExchange(privateKey, publicKey)](#X25519.keyExchange) ⇒ <code>Uint8Array</code>
    * [.Ed25519toX25519Private(privateKey)](#X25519.Ed25519toX25519Private) ⇒ <code>Uint8Array</code>
    * [.Ed25519toX25519Public(publicKey)](#X25519.Ed25519toX25519Public) ⇒ <code>Uint8Array</code>

<a name="X25519.PRIVATE_KEY_LENGTH"></a>

### X25519.PRIVATE\_KEY\_LENGTH() ⇒ <code>number</code>
Length in bytes of an X25519 private key.

**Kind**: static method of [<code>X25519</code>](#X25519)  
<a name="X25519.PUBLIC_KEY_LENGTH"></a>

### X25519.PUBLIC\_KEY\_LENGTH() ⇒ <code>number</code>
Length in bytes of an X25519 public key.

**Kind**: static method of [<code>X25519</code>](#X25519)  
<a name="X25519.keyExchange"></a>

### X25519.keyExchange(privateKey, publicKey) ⇒ <code>Uint8Array</code>
Performs Diffie-Hellman key exchange using the private key of the first party with the
public key of the second party, resulting in a shared secret.

**Kind**: static method of [<code>X25519</code>](#X25519)  

| Param | Type |
| --- | --- |
| privateKey | <code>Uint8Array</code> | 
| publicKey | <code>Uint8Array</code> | 

<a name="X25519.Ed25519toX25519Private"></a>

### X25519.Ed25519toX25519Private(privateKey) ⇒ <code>Uint8Array</code>
Transforms an `Ed25519` private key to an `X25519` private key.

This is possible because Ed25519 is birationally equivalent to Curve25519 used by X25519.

**Kind**: static method of [<code>X25519</code>](#X25519)  

| Param | Type |
| --- | --- |
| privateKey | <code>Uint8Array</code> | 

<a name="X25519.Ed25519toX25519Public"></a>

### X25519.Ed25519toX25519Public(publicKey) ⇒ <code>Uint8Array</code>
Transforms an `Ed25519` public key to an `X25519` public key.

This is possible because Ed25519 is birationally equivalent to Curve25519 used by X25519.

**Kind**: static method of [<code>X25519</code>](#X25519)  

| Param | Type |
| --- | --- |
| publicKey | <code>Uint8Array</code> | 

<a name="KeyType"></a>

## KeyType
**Kind**: global variable  
<a name="MethodRelationship"></a>

## MethodRelationship
**Kind**: global variable  
<a name="StatusCheck"></a>

## StatusCheck
Controls validation behaviour when checking whether or not a credential has been revoked by its
[`credentialStatus`](https://www.w3.org/TR/vc-data-model/#status).

**Kind**: global variable  
<a name="Strict"></a>

## Strict
Validate the status if supported, reject any unsupported
[`credentialStatus`](https://www.w3.org/TR/vc-data-model/#status) types.

Only `RevocationBitmap2022` is currently supported.

This is the default.

**Kind**: global variable  
<a name="SkipUnsupported"></a>

## SkipUnsupported
Validate the status if supported, skip any unsupported
[`credentialStatus`](https://www.w3.org/TR/vc-data-model/#status) types.

**Kind**: global variable  
<a name="SkipAll"></a>

## SkipAll
Skip all status checks.

**Kind**: global variable  
<a name="SubjectHolderRelationship"></a>

## SubjectHolderRelationship
Declares how credential subjects must relate to the presentation holder during validation.
See `PresentationValidationOptions::subject_holder_relationship`.

See also the [Subject-Holder Relationship](https://www.w3.org/TR/vc-data-model/#subject-holder-relationships) section of the specification.

**Kind**: global variable  
<a name="AlwaysSubject"></a>

## AlwaysSubject
The holder must always match the subject on all credentials, regardless of their [`nonTransferable`](https://www.w3.org/TR/vc-data-model/#nontransferable-property) property.
This variant is the default used if no other variant is specified when constructing a new
`PresentationValidationOptions`.

**Kind**: global variable  
<a name="SubjectOnNonTransferable"></a>

## SubjectOnNonTransferable
The holder must match the subject only for credentials where the [`nonTransferable`](https://www.w3.org/TR/vc-data-model/#nontransferable-property) property is `true`.

**Kind**: global variable  
<a name="Any"></a>

## Any
The holder is not required to have any kind of relationship to any credential subject.

**Kind**: global variable  
<a name="FailFast"></a>

## FailFast
Declares when validation should return if an error occurs.

**Kind**: global variable  
<a name="AllErrors"></a>

## AllErrors
Return all errors that occur during validation.

**Kind**: global variable  
<a name="FirstError"></a>

## FirstError
Return after the first error occurs.

**Kind**: global variable  
<a name="StateMetadataEncoding"></a>

## StateMetadataEncoding
**Kind**: global variable  
<a name="start"></a>

## start()
Initializes the console error panic hook for better error messages

**Kind**: global function  

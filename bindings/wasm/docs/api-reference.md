## Classes

<dl>
<dt><a href="#Account">Account</a></dt>
<dd><p>An account manages one identity.</p>
<p>It handles private keys, writing to storage and
publishing to the Tangle.</p>
</dd>
<dt><a href="#AccountBuilder">AccountBuilder</a></dt>
<dd><p>An [<code>Account</code>] builder for easy account configuration.</p>
<p>To reduce memory usage, accounts created from the same builder share the same <code>Storage</code>
used to store identities, and the same <a href="#Client">Client</a> used to publish identities to the Tangle.</p>
<p>The configuration on the other hand is cloned, and therefore unique for each built account.
This means a builder can be reconfigured in-between account creations, without affecting
the configuration of previously built accounts.</p>
</dd>
<dt><a href="#AgreementInfo">AgreementInfo</a></dt>
<dd><p>Agreement information used as the input for the concat KDF.</p>
</dd>
<dt><a href="#AutoSave">AutoSave</a></dt>
<dd></dd>
<dt><a href="#CekAlgorithm">CekAlgorithm</a></dt>
<dd><p>Supported algorithms used to determine and potentially encrypt the content encryption key (CEK).</p>
</dd>
<dt><a href="#ChainState">ChainState</a></dt>
<dd></dd>
<dt><a href="#Client">Client</a></dt>
<dd></dd>
<dt><a href="#CoreDID">CoreDID</a></dt>
<dd><p>A Decentralized Identifier (DID).</p>
</dd>
<dt><a href="#CoreDocument">CoreDocument</a></dt>
<dd></dd>
<dt><a href="#Credential">Credential</a></dt>
<dd></dd>
<dt><a href="#CredentialValidationOptions">CredentialValidationOptions</a></dt>
<dd><p>Options to declare validation criteria when validating credentials.</p>
</dd>
<dt><a href="#CredentialValidator">CredentialValidator</a></dt>
<dd></dd>
<dt><a href="#DIDUrl">DIDUrl</a></dt>
<dd><p>A DID URL conforming to the IOTA DID method specification.</p>
</dd>
<dt><del><a href="#DiffChainHistory">DiffChainHistory</a></del></dt>
<dd></dd>
<dt><del><a href="#DiffMessage">DiffMessage</a></del></dt>
<dd><p>Defines the difference between two DID <code>Document</code>s&#39; JSON representations.</p>
</dd>
<dt><a href="#Document">Document</a></dt>
<dd></dd>
<dt><a href="#DocumentHistory">DocumentHistory</a></dt>
<dd><p>A DID Document&#39;s history and current state.</p>
</dd>
<dt><a href="#DocumentMetadata">DocumentMetadata</a></dt>
<dd><p>Additional attributes related to an IOTA DID Document.</p>
</dd>
<dt><a href="#Duration">Duration</a></dt>
<dd><p>A span of time.</p>
</dd>
<dt><a href="#Ed25519">Ed25519</a></dt>
<dd></dd>
<dt><a href="#EncryptedData">EncryptedData</a></dt>
<dd><p>The structure returned after encrypting data</p>
</dd>
<dt><a href="#EncryptionAlgorithm">EncryptionAlgorithm</a></dt>
<dd><p>Supported content encryption algorithms.</p>
</dd>
<dt><a href="#ExplorerUrl">ExplorerUrl</a></dt>
<dd></dd>
<dt><a href="#IntegrationChainHistory">IntegrationChainHistory</a></dt>
<dd></dd>
<dt><a href="#IotaDID">IotaDID</a></dt>
<dd><p>A DID conforming to the IOTA DID method specification.</p>
</dd>
<dt><a href="#KeyLocation">KeyLocation</a></dt>
<dd><p>The storage location of a verification method key.</p>
<p>A key is uniquely identified by the fragment and a hash of its public key.
Importantly, the fragment alone is insufficient to represent the storage location.
For example, when rotating a key, there will be two keys in storage for the
same identity with the same fragment. The <code>key_hash</code> disambiguates the keys in
situations like these.</p>
<p>The string representation of that location can be obtained via <code>canonicalRepr</code>.</p>
</dd>
<dt><a href="#KeyPair">KeyPair</a></dt>
<dd></dd>
<dt><a href="#MethodContent">MethodContent</a></dt>
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
<dt><a href="#MixedResolver">MixedResolver</a></dt>
<dd></dd>
<dt><a href="#Network">Network</a></dt>
<dd></dd>
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
<dt><a href="#Receipt">Receipt</a></dt>
<dd></dd>
<dt><a href="#ResolvedDocument">ResolvedDocument</a></dt>
<dd><p>An IOTA DID document resolved from the Tangle. Represents an integration chain message possibly
merged with one or more <code>DiffMessages</code>.</p>
</dd>
<dt><a href="#Resolver">Resolver</a></dt>
<dd></dd>
<dt><a href="#ResolverBuilder">ResolverBuilder</a></dt>
<dd><p>Builder for configuring [<code>Clients</code>][Client] when constructing a [<code>Resolver</code>].</p>
</dd>
<dt><a href="#RevocationBitmap">RevocationBitmap</a></dt>
<dd><p>A compressed bitmap for managing credential revocation.</p>
</dd>
<dt><a href="#Service">Service</a></dt>
<dd><p>A DID Document Service used to enable trusted interactions associated
with a DID subject.</p>
<p>See: <a href="https://www.w3.org/TR/did-core/#services">https://www.w3.org/TR/did-core/#services</a></p>
</dd>
<dt><a href="#Signature">Signature</a></dt>
<dd><p>A digital signature.</p>
</dd>
<dt><a href="#StardustDID">StardustDID</a></dt>
<dd><p>A DID conforming to the IOTA UTXO DID method specification.</p>
</dd>
<dt><a href="#StardustDIDUrl">StardustDIDUrl</a></dt>
<dd><p>A DID URL conforming to the IOTA Stardust UTXO DID method specification.</p>
</dd>
<dt><a href="#StardustDocument">StardustDocument</a></dt>
<dd></dd>
<dt><a href="#StardustDocumentMetadata">StardustDocumentMetadata</a></dt>
<dd><p>Additional attributes related to an IOTA DID Document.</p>
</dd>
<dt><a href="#StardustIdentityClientExt">StardustIdentityClientExt</a></dt>
<dd><p>An extension interface that provides helper functions for publication
and resolution of DID documents in Alias Outputs.</p>
</dd>
<dt><a href="#StardustService">StardustService</a></dt>
<dd><p>A <code>Service</code> adhering to the IOTA UTXO DID method specification.</p>
</dd>
<dt><a href="#StardustVerificationMethod">StardustVerificationMethod</a></dt>
<dd></dd>
<dt><a href="#StorageTestSuite">StorageTestSuite</a></dt>
<dd><p>A test suite for the <code>Storage</code> interface.</p>
<p>This module contains a set of tests that a correct storage implementation
should pass. Note that not every edge case is tested.</p>
<p>Tests usually rely on multiple interface methods being implemented, so they should only
be run on a fully implemented version. That&#39;s why there is not a single test case for every
interface method.</p>
</dd>
<dt><a href="#Timestamp">Timestamp</a></dt>
<dd></dd>
<dt><a href="#VerificationMethod">VerificationMethod</a></dt>
<dd></dd>
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
<dt><a href="#StateMetadataEncoding">StateMetadataEncoding</a></dt>
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
<dt><a href="#MethodRelationship">MethodRelationship</a></dt>
<dd></dd>
<dt><a href="#DIDType">DIDType</a></dt>
<dd><p>Supported types representing a DID that can be generated by the storage interface.</p>
</dd>
<dt><a href="#KeyType">KeyType</a></dt>
<dd></dd>
<dt><a href="#DIDMessageEncoding">DIDMessageEncoding</a></dt>
<dd></dd>
</dl>

## Functions

<dl>
<dt><a href="#start">start()</a></dt>
<dd><p>Initializes the console error panic hook for better error messages</p>
</dd>
</dl>

<a name="Account"></a>

## Account
An account manages one identity.

It handles private keys, writing to storage and
publishing to the Tangle.

**Kind**: global class  

* [Account](#Account)
    * [.createService(options)](#Account+createService) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.attachMethodRelationships(options)](#Account+attachMethodRelationships) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.createMethod(options)](#Account+createMethod) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.detachMethodRelationships(options)](#Account+detachMethodRelationships) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.did()](#Account+did) ⇒ [<code>IotaDID</code>](#IotaDID)
    * [.autopublish()](#Account+autopublish) ⇒ <code>boolean</code>
    * [.autosave()](#Account+autosave) ⇒ [<code>AutoSave</code>](#AutoSave)
    * [.document()](#Account+document) ⇒ [<code>Document</code>](#Document)
    * [.resolveIdentity()](#Account+resolveIdentity) ⇒ [<code>Promise.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
    * [.deleteIdentity()](#Account+deleteIdentity) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.publish(publish_options)](#Account+publish) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.createSignedCredential(fragment, credential, options)](#Account+createSignedCredential) ⇒ [<code>Promise.&lt;Credential&gt;</code>](#Credential)
    * [.createSignedDocument(fragment, document, options)](#Account+createSignedDocument) ⇒ [<code>Promise.&lt;Document&gt;</code>](#Document)
    * [.createSignedPresentation(fragment, presentation, options)](#Account+createSignedPresentation) ⇒ [<code>Promise.&lt;Presentation&gt;</code>](#Presentation)
    * [.createSignedData(fragment, data, options)](#Account+createSignedData) ⇒ <code>Promise.&lt;any&gt;</code>
    * [.updateDocumentUnchecked(document)](#Account+updateDocumentUnchecked) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.fetchDocument()](#Account+fetchDocument) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.revokeCredentials(fragment, indices)](#Account+revokeCredentials) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.unrevokeCredentials(fragment, indices)](#Account+unrevokeCredentials) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.encryptData(plaintext, associated_data, encryption_algorithm, cek_algorithm, public_key)](#Account+encryptData) ⇒ [<code>Promise.&lt;EncryptedData&gt;</code>](#EncryptedData)
    * [.decryptData(data, encryption_algorithm, cek_algorithm, fragment)](#Account+decryptData) ⇒ <code>Promise.&lt;Uint8Array&gt;</code>
    * [.deleteMethod(options)](#Account+deleteMethod) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.deleteService(options)](#Account+deleteService) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.setAlsoKnownAs(options)](#Account+setAlsoKnownAs) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.setController(options)](#Account+setController) ⇒ <code>Promise.&lt;void&gt;</code>

<a name="Account+createService"></a>

### account.createService(options) ⇒ <code>Promise.&lt;void&gt;</code>
Adds a new Service to the DID Document.

**Kind**: instance method of [<code>Account</code>](#Account)  

| Param | Type |
| --- | --- |
| options | <code>CreateServiceOptions</code> | 

<a name="Account+attachMethodRelationships"></a>

### account.attachMethodRelationships(options) ⇒ <code>Promise.&lt;void&gt;</code>
Attach one or more verification relationships to a method.

Note: the method must exist and be in the set of verification methods;
it cannot be an embedded method.

**Kind**: instance method of [<code>Account</code>](#Account)  

| Param | Type |
| --- | --- |
| options | <code>AttachMethodRelationshipOptions</code> | 

<a name="Account+createMethod"></a>

### account.createMethod(options) ⇒ <code>Promise.&lt;void&gt;</code>
Adds a new verification method to the DID document.

**Kind**: instance method of [<code>Account</code>](#Account)  

| Param | Type |
| --- | --- |
| options | <code>CreateMethodOptions</code> | 

<a name="Account+detachMethodRelationships"></a>

### account.detachMethodRelationships(options) ⇒ <code>Promise.&lt;void&gt;</code>
Detaches the given relationship from the given method, if the method exists.

**Kind**: instance method of [<code>Account</code>](#Account)  

| Param | Type |
| --- | --- |
| options | <code>DetachMethodRelationshipOptions</code> | 

<a name="Account+did"></a>

### account.did() ⇒ [<code>IotaDID</code>](#IotaDID)
Returns the [IotaDID](#IotaDID) of the managed identity.

**Kind**: instance method of [<code>Account</code>](#Account)  
<a name="Account+autopublish"></a>

### account.autopublish() ⇒ <code>boolean</code>
Returns whether auto-publish is enabled.

**Kind**: instance method of [<code>Account</code>](#Account)  
<a name="Account+autosave"></a>

### account.autosave() ⇒ [<code>AutoSave</code>](#AutoSave)
Returns the auto-save configuration value.

**Kind**: instance method of [<code>Account</code>](#Account)  
<a name="Account+document"></a>

### account.document() ⇒ [<code>Document</code>](#Document)
Returns a copy of the document managed by the `Account`.

Note: the returned document only has a valid signature after publishing an integration chain update.
In general, for use cases where the signature is required, it is advisable to resolve the
document from the Tangle.

**Kind**: instance method of [<code>Account</code>](#Account)  
<a name="Account+resolveIdentity"></a>

### account.resolveIdentity() ⇒ [<code>Promise.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
Resolves the DID Document associated with this `Account` from the Tangle.

**Kind**: instance method of [<code>Account</code>](#Account)  
<a name="Account+deleteIdentity"></a>

### account.deleteIdentity() ⇒ <code>Promise.&lt;void&gt;</code>
Removes the identity from the local storage entirely.

Note: This will remove all associated document updates and key material - recovery is NOT POSSIBLE!

**Kind**: instance method of [<code>Account</code>](#Account)  
<a name="Account+publish"></a>

### account.publish(publish_options) ⇒ <code>Promise.&lt;void&gt;</code>
Push all unpublished changes to the tangle in a single message.

**Kind**: instance method of [<code>Account</code>](#Account)  

| Param | Type |
| --- | --- |
| publish_options | <code>PublishOptions</code> \| <code>undefined</code> | 

<a name="Account+createSignedCredential"></a>

### account.createSignedCredential(fragment, credential, options) ⇒ [<code>Promise.&lt;Credential&gt;</code>](#Credential)
Signs a [Credential](#Credential) with the key specified by `fragment`.

**Kind**: instance method of [<code>Account</code>](#Account)  

| Param | Type |
| --- | --- |
| fragment | <code>string</code> | 
| credential | [<code>Credential</code>](#Credential) | 
| options | [<code>ProofOptions</code>](#ProofOptions) | 

<a name="Account+createSignedDocument"></a>

### account.createSignedDocument(fragment, document, options) ⇒ [<code>Promise.&lt;Document&gt;</code>](#Document)
Signs a [Document](#Document) with the key specified by `fragment`.

**Kind**: instance method of [<code>Account</code>](#Account)  

| Param | Type |
| --- | --- |
| fragment | <code>string</code> | 
| document | [<code>Document</code>](#Document) | 
| options | [<code>ProofOptions</code>](#ProofOptions) | 

<a name="Account+createSignedPresentation"></a>

### account.createSignedPresentation(fragment, presentation, options) ⇒ [<code>Promise.&lt;Presentation&gt;</code>](#Presentation)
Signs a [Presentation](#Presentation) the key specified by `fragment`.

**Kind**: instance method of [<code>Account</code>](#Account)  

| Param | Type |
| --- | --- |
| fragment | <code>string</code> | 
| presentation | [<code>Presentation</code>](#Presentation) | 
| options | [<code>ProofOptions</code>](#ProofOptions) | 

<a name="Account+createSignedData"></a>

### account.createSignedData(fragment, data, options) ⇒ <code>Promise.&lt;any&gt;</code>
Signs arbitrary `data` with the key specified by `fragment`.

**Kind**: instance method of [<code>Account</code>](#Account)  

| Param | Type |
| --- | --- |
| fragment | <code>string</code> | 
| data | <code>any</code> | 
| options | [<code>ProofOptions</code>](#ProofOptions) | 

<a name="Account+updateDocumentUnchecked"></a>

### account.updateDocumentUnchecked(document) ⇒ <code>Promise.&lt;void&gt;</code>
Overwrites the [Document](#Document) this account manages, **without doing any validation**.

### WARNING

This method is dangerous and can easily corrupt the internal state,
potentially making the identity unusable. Only call this if you fully
understand the implications!

**Kind**: instance method of [<code>Account</code>](#Account)  

| Param | Type |
| --- | --- |
| document | [<code>Document</code>](#Document) | 

<a name="Account+fetchDocument"></a>

### account.fetchDocument() ⇒ <code>Promise.&lt;void&gt;</code>
Fetches the latest changes from the tangle and **overwrites** the local document.

If a DID is managed from distributed accounts, this should be called before making changes
to the identity, to avoid publishing updates that would be ignored.

**Kind**: instance method of [<code>Account</code>](#Account)  
<a name="Account+revokeCredentials"></a>

### account.revokeCredentials(fragment, indices) ⇒ <code>Promise.&lt;void&gt;</code>
If the document has a `RevocationBitmap` service identified by `fragment`,
revoke all specified `indices`.

**Kind**: instance method of [<code>Account</code>](#Account)  

| Param | Type |
| --- | --- |
| fragment | <code>string</code> | 
| indices | <code>number</code> \| <code>Array.&lt;number&gt;</code> | 

<a name="Account+unrevokeCredentials"></a>

### account.unrevokeCredentials(fragment, indices) ⇒ <code>Promise.&lt;void&gt;</code>
If the document has a `RevocationBitmap` service identified by `fragment`,
unrevoke all specified `indices`.

**Kind**: instance method of [<code>Account</code>](#Account)  

| Param | Type |
| --- | --- |
| fragment | <code>string</code> | 
| indices | <code>number</code> \| <code>Array.&lt;number&gt;</code> | 

<a name="Account+encryptData"></a>

### account.encryptData(plaintext, associated_data, encryption_algorithm, cek_algorithm, public_key) ⇒ [<code>Promise.&lt;EncryptedData&gt;</code>](#EncryptedData)
Encrypts the given `plaintext` with the specified `encryption_algorithm` and `cek_algorithm`.

Returns an [`EncryptedData`] instance.

**Kind**: instance method of [<code>Account</code>](#Account)  

| Param | Type |
| --- | --- |
| plaintext | <code>Uint8Array</code> | 
| associated_data | <code>Uint8Array</code> | 
| encryption_algorithm | [<code>EncryptionAlgorithm</code>](#EncryptionAlgorithm) | 
| cek_algorithm | [<code>CekAlgorithm</code>](#CekAlgorithm) | 
| public_key | <code>Uint8Array</code> | 

<a name="Account+decryptData"></a>

### account.decryptData(data, encryption_algorithm, cek_algorithm, fragment) ⇒ <code>Promise.&lt;Uint8Array&gt;</code>
Decrypts the given `data` with the key identified by `fragment` using the given `encryption_algorithm` and
`cek_algorithm`.

Returns the decrypted text.

**Kind**: instance method of [<code>Account</code>](#Account)  

| Param | Type |
| --- | --- |
| data | [<code>EncryptedData</code>](#EncryptedData) | 
| encryption_algorithm | [<code>EncryptionAlgorithm</code>](#EncryptionAlgorithm) | 
| cek_algorithm | [<code>CekAlgorithm</code>](#CekAlgorithm) | 
| fragment | <code>string</code> | 

<a name="Account+deleteMethod"></a>

### account.deleteMethod(options) ⇒ <code>Promise.&lt;void&gt;</code>
Deletes a verification method if the method exists.

**Kind**: instance method of [<code>Account</code>](#Account)  

| Param | Type |
| --- | --- |
| options | <code>DeleteMethodOptions</code> | 

<a name="Account+deleteService"></a>

### account.deleteService(options) ⇒ <code>Promise.&lt;void&gt;</code>
Deletes a Service if it exists.

**Kind**: instance method of [<code>Account</code>](#Account)  

| Param | Type |
| --- | --- |
| options | <code>DeleteServiceOptions</code> | 

<a name="Account+setAlsoKnownAs"></a>

### account.setAlsoKnownAs(options) ⇒ <code>Promise.&lt;void&gt;</code>
Sets the `alsoKnownAs` property in the DID document.

**Kind**: instance method of [<code>Account</code>](#Account)  

| Param | Type |
| --- | --- |
| options | <code>SetAlsoKnownAsOptions</code> | 

<a name="Account+setController"></a>

### account.setController(options) ⇒ <code>Promise.&lt;void&gt;</code>
Sets the controllers of the DID document.

**Kind**: instance method of [<code>Account</code>](#Account)  

| Param | Type |
| --- | --- |
| options | <code>SetControllerOptions</code> | 

<a name="AccountBuilder"></a>

## AccountBuilder
An [`Account`] builder for easy account configuration.

To reduce memory usage, accounts created from the same builder share the same `Storage`
used to store identities, and the same [Client](#Client) used to publish identities to the Tangle.

The configuration on the other hand is cloned, and therefore unique for each built account.
This means a builder can be reconfigured in-between account creations, without affecting
the configuration of previously built accounts.

**Kind**: global class  

* [AccountBuilder](#AccountBuilder)
    * [new AccountBuilder(options)](#new_AccountBuilder_new)
    * [.loadIdentity(did)](#AccountBuilder+loadIdentity) ⇒ [<code>Promise.&lt;Account&gt;</code>](#Account)
    * [.createIdentity(identity_setup)](#AccountBuilder+createIdentity) ⇒ [<code>Promise.&lt;Account&gt;</code>](#Account)

<a name="new_AccountBuilder_new"></a>

### new AccountBuilder(options)
Creates a new `AccountBuilder`.


| Param | Type |
| --- | --- |
| options | <code>AccountBuilderOptions</code> \| <code>undefined</code> | 

<a name="AccountBuilder+loadIdentity"></a>

### accountBuilder.loadIdentity(did) ⇒ [<code>Promise.&lt;Account&gt;</code>](#Account)
Loads an existing identity with the specified `did` using the current builder configuration.
The identity must exist in the configured `Storage`.

**Kind**: instance method of [<code>AccountBuilder</code>](#AccountBuilder)  

| Param | Type |
| --- | --- |
| did | [<code>IotaDID</code>](#IotaDID) | 

<a name="AccountBuilder+createIdentity"></a>

### accountBuilder.createIdentity(identity_setup) ⇒ [<code>Promise.&lt;Account&gt;</code>](#Account)
Creates a new identity based on the builder configuration and returns
an [Account](#Account) object to manage it.

The identity is stored locally in the `Storage`. The DID network is automatically determined
by the [Client](#Client) used to publish it.

**Kind**: instance method of [<code>AccountBuilder</code>](#AccountBuilder)  

| Param | Type |
| --- | --- |
| identity_setup | <code>IdentitySetup</code> \| <code>undefined</code> | 

<a name="AgreementInfo"></a>

## AgreementInfo
Agreement information used as the input for the concat KDF.

**Kind**: global class  

* [AgreementInfo](#AgreementInfo)
    * [new AgreementInfo(apu, apv, pub_info, priv_info)](#new_AgreementInfo_new)
    * _instance_
        * [.apu()](#AgreementInfo+apu) ⇒ <code>Uint8Array</code>
        * [.apv()](#AgreementInfo+apv) ⇒ <code>Uint8Array</code>
        * [.pubInfo()](#AgreementInfo+pubInfo) ⇒ <code>Uint8Array</code>
        * [.privInfo()](#AgreementInfo+privInfo) ⇒ <code>Uint8Array</code>
        * [.toJSON()](#AgreementInfo+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#AgreementInfo.fromJSON) ⇒ [<code>AgreementInfo</code>](#AgreementInfo)

<a name="new_AgreementInfo_new"></a>

### new AgreementInfo(apu, apv, pub_info, priv_info)
Creates an `AgreementInfo` Object.


| Param | Type |
| --- | --- |
| apu | <code>Uint8Array</code> | 
| apv | <code>Uint8Array</code> | 
| pub_info | <code>Uint8Array</code> | 
| priv_info | <code>Uint8Array</code> | 

<a name="AgreementInfo+apu"></a>

### agreementInfo.apu() ⇒ <code>Uint8Array</code>
Returns a copy of `apu'

**Kind**: instance method of [<code>AgreementInfo</code>](#AgreementInfo)  
<a name="AgreementInfo+apv"></a>

### agreementInfo.apv() ⇒ <code>Uint8Array</code>
Returns a copy of `apv'

**Kind**: instance method of [<code>AgreementInfo</code>](#AgreementInfo)  
<a name="AgreementInfo+pubInfo"></a>

### agreementInfo.pubInfo() ⇒ <code>Uint8Array</code>
Returns a copy of `pubInfo'

**Kind**: instance method of [<code>AgreementInfo</code>](#AgreementInfo)  
<a name="AgreementInfo+privInfo"></a>

### agreementInfo.privInfo() ⇒ <code>Uint8Array</code>
Returns a copy of `privInfo'

**Kind**: instance method of [<code>AgreementInfo</code>](#AgreementInfo)  
<a name="AgreementInfo+toJSON"></a>

### agreementInfo.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>AgreementInfo</code>](#AgreementInfo)  
<a name="AgreementInfo.fromJSON"></a>

### AgreementInfo.fromJSON(json) ⇒ [<code>AgreementInfo</code>](#AgreementInfo)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>AgreementInfo</code>](#AgreementInfo)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="AutoSave"></a>

## AutoSave
**Kind**: global class  

* [AutoSave](#AutoSave)
    * _instance_
        * [.toJSON()](#AutoSave+toJSON) ⇒ <code>any</code>
    * _static_
        * [.never()](#AutoSave.never) ⇒ [<code>AutoSave</code>](#AutoSave)
        * [.every()](#AutoSave.every) ⇒ [<code>AutoSave</code>](#AutoSave)
        * [.batch(number_of_actions)](#AutoSave.batch) ⇒ [<code>AutoSave</code>](#AutoSave)
        * [.fromJSON(json)](#AutoSave.fromJSON) ⇒ [<code>AutoSave</code>](#AutoSave)

<a name="AutoSave+toJSON"></a>

### autoSave.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>AutoSave</code>](#AutoSave)  
<a name="AutoSave.never"></a>

### AutoSave.never() ⇒ [<code>AutoSave</code>](#AutoSave)
Never save.

**Kind**: static method of [<code>AutoSave</code>](#AutoSave)  
<a name="AutoSave.every"></a>

### AutoSave.every() ⇒ [<code>AutoSave</code>](#AutoSave)
Save after every action.

**Kind**: static method of [<code>AutoSave</code>](#AutoSave)  
<a name="AutoSave.batch"></a>

### AutoSave.batch(number_of_actions) ⇒ [<code>AutoSave</code>](#AutoSave)
Save after every N actions.

**Kind**: static method of [<code>AutoSave</code>](#AutoSave)  

| Param | Type |
| --- | --- |
| number_of_actions | <code>number</code> | 

<a name="AutoSave.fromJSON"></a>

### AutoSave.fromJSON(json) ⇒ [<code>AutoSave</code>](#AutoSave)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>AutoSave</code>](#AutoSave)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="CekAlgorithm"></a>

## CekAlgorithm
Supported algorithms used to determine and potentially encrypt the content encryption key (CEK).

**Kind**: global class  

* [CekAlgorithm](#CekAlgorithm)
    * _instance_
        * [.toJSON()](#CekAlgorithm+toJSON) ⇒ <code>any</code>
    * _static_
        * [.EcdhEs(agreement)](#CekAlgorithm.EcdhEs) ⇒ [<code>CekAlgorithm</code>](#CekAlgorithm)
        * [.EcdhEsA256Kw(agreement)](#CekAlgorithm.EcdhEsA256Kw) ⇒ [<code>CekAlgorithm</code>](#CekAlgorithm)
        * [.fromJSON(json)](#CekAlgorithm.fromJSON) ⇒ [<code>CekAlgorithm</code>](#CekAlgorithm)

<a name="CekAlgorithm+toJSON"></a>

### cekAlgorithm.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>CekAlgorithm</code>](#CekAlgorithm)  
<a name="CekAlgorithm.EcdhEs"></a>

### CekAlgorithm.EcdhEs(agreement) ⇒ [<code>CekAlgorithm</code>](#CekAlgorithm)
Elliptic Curve Diffie-Hellman Ephemeral Static key agreement using Concat KDF.

**Kind**: static method of [<code>CekAlgorithm</code>](#CekAlgorithm)  

| Param | Type |
| --- | --- |
| agreement | [<code>AgreementInfo</code>](#AgreementInfo) | 

<a name="CekAlgorithm.EcdhEsA256Kw"></a>

### CekAlgorithm.EcdhEsA256Kw(agreement) ⇒ [<code>CekAlgorithm</code>](#CekAlgorithm)
Elliptic Curve Diffie-Hellman Ephemeral Static key agreement using Concat KDF.

**Kind**: static method of [<code>CekAlgorithm</code>](#CekAlgorithm)  

| Param | Type |
| --- | --- |
| agreement | [<code>AgreementInfo</code>](#AgreementInfo) | 

<a name="CekAlgorithm.fromJSON"></a>

### CekAlgorithm.fromJSON(json) ⇒ [<code>CekAlgorithm</code>](#CekAlgorithm)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>CekAlgorithm</code>](#CekAlgorithm)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="ChainState"></a>

## ChainState
**Kind**: global class  

* [ChainState](#ChainState)
    * _instance_
        * [.toJSON()](#ChainState+toJSON) ⇒ <code>any</code>
        * [.clone()](#ChainState+clone) ⇒ [<code>ChainState</code>](#ChainState)
    * _static_
        * [.fromJSON(json)](#ChainState.fromJSON) ⇒ [<code>ChainState</code>](#ChainState)

<a name="ChainState+toJSON"></a>

### chainState.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>ChainState</code>](#ChainState)  
<a name="ChainState+clone"></a>

### chainState.clone() ⇒ [<code>ChainState</code>](#ChainState)
Deep clones the object.

**Kind**: instance method of [<code>ChainState</code>](#ChainState)  
<a name="ChainState.fromJSON"></a>

### ChainState.fromJSON(json) ⇒ [<code>ChainState</code>](#ChainState)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>ChainState</code>](#ChainState)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="Client"></a>

## Client
**Kind**: global class  

* [Client](#Client)
    * [new Client()](#new_Client_new)
    * _instance_
        * [.network()](#Client+network) ⇒ [<code>Network</code>](#Network)
        * [.publishDocument(document)](#Client+publishDocument) ⇒ [<code>Promise.&lt;Receipt&gt;</code>](#Receipt)
        * ~~[.publishDiff(message_id, diff)](#Client+publishDiff) ⇒ [<code>Promise.&lt;Receipt&gt;</code>](#Receipt)~~
        * [.publishJSON(index, data)](#Client+publishJSON) ⇒ [<code>Promise.&lt;Receipt&gt;</code>](#Receipt)
        * [.publishJsonWithRetry(index, data, interval, max_attempts)](#Client+publishJsonWithRetry) ⇒ <code>Promise.&lt;any&gt;</code>
        * [.isMessageIncluded(messageId)](#Client+isMessageIncluded) ⇒ <code>Promise.&lt;boolean&gt;</code>
        * [.resolve(did)](#Client+resolve) ⇒ [<code>Promise.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
        * [.resolveHistory(did)](#Client+resolveHistory) ⇒ [<code>Promise.&lt;DocumentHistory&gt;</code>](#DocumentHistory)
        * ~~[.resolveDiffHistory(document)](#Client+resolveDiffHistory) ⇒ [<code>Promise.&lt;DiffChainHistory&gt;</code>](#DiffChainHistory)~~
    * _static_
        * [.fromConfig(config)](#Client.fromConfig) ⇒ [<code>Promise.&lt;Client&gt;</code>](#Client)

<a name="new_Client_new"></a>

### new Client()
Creates a new `Client` with default settings.

<a name="Client+network"></a>

### client.network() ⇒ [<code>Network</code>](#Network)
Returns the `Client` Tangle network.

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+publishDocument"></a>

### client.publishDocument(document) ⇒ [<code>Promise.&lt;Receipt&gt;</code>](#Receipt)
Publishes a [Document](#Document) to the Tangle.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| document | [<code>Document</code>](#Document) | 

<a name="Client+publishDiff"></a>

### ~~client.publishDiff(message_id, diff) ⇒ [<code>Promise.&lt;Receipt&gt;</code>](#Receipt)~~
***Deprecated***

Publishes a `DiffMessage` to the Tangle.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| message_id | <code>string</code> | 
| diff | [<code>DiffMessage</code>](#DiffMessage) | 

<a name="Client+publishJSON"></a>

### client.publishJSON(index, data) ⇒ [<code>Promise.&lt;Receipt&gt;</code>](#Receipt)
Publishes arbitrary JSON data to the specified index on the Tangle.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| index | <code>string</code> | 
| data | <code>any</code> | 

<a name="Client+publishJsonWithRetry"></a>

### client.publishJsonWithRetry(index, data, interval, max_attempts) ⇒ <code>Promise.&lt;any&gt;</code>
Publishes arbitrary JSON data to the specified index on the Tangle.
Retries (promotes or reattaches) the message until it’s included (referenced by a milestone).
Default interval is 5 seconds and max attempts is 40.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| index | <code>string</code> | 
| data | <code>any</code> | 
| interval | <code>number</code> \| <code>undefined</code> | 
| max_attempts | <code>number</code> \| <code>undefined</code> | 

<a name="Client+isMessageIncluded"></a>

### client.isMessageIncluded(messageId) ⇒ <code>Promise.&lt;boolean&gt;</code>
Checks if a message is confirmed by a milestone.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| messageId | <code>string</code> | 

<a name="Client+resolve"></a>

### client.resolve(did) ⇒ [<code>Promise.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
Fetch the DID document specified by the given `DID`.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| did | [<code>IotaDID</code>](#IotaDID) \| <code>string</code> | 

<a name="Client+resolveHistory"></a>

### client.resolveHistory(did) ⇒ [<code>Promise.&lt;DocumentHistory&gt;</code>](#DocumentHistory)
Returns the message history of the given DID.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| did | [<code>IotaDID</code>](#IotaDID) \| <code>string</code> | 

<a name="Client+resolveDiffHistory"></a>

### ~~client.resolveDiffHistory(document) ⇒ [<code>Promise.&lt;DiffChainHistory&gt;</code>](#DiffChainHistory)~~
***Deprecated***

Returns the `DiffChainHistory` of a diff chain starting from a document on the
integration chain.

NOTE: the document must have been published to the tangle and have a valid message id and
capability invocation method.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| document | [<code>ResolvedDocument</code>](#ResolvedDocument) | 

<a name="Client.fromConfig"></a>

### Client.fromConfig(config) ⇒ [<code>Promise.&lt;Client&gt;</code>](#Client)
Creates a new `Client` with the given settings.

**Kind**: static method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| config | <code>IClientConfig</code> | 

<a name="CoreDID"></a>

## CoreDID
A Decentralized Identifier (DID).

**Kind**: global class  

* [CoreDID](#CoreDID)
    * _instance_
        * [.toString()](#CoreDID+toString) ⇒ <code>string</code>
        * [.toJSON()](#CoreDID+toJSON) ⇒ <code>any</code>
        * [.clone()](#CoreDID+clone) ⇒ [<code>CoreDID</code>](#CoreDID)
    * _static_
        * [.parse(input)](#CoreDID.parse) ⇒ [<code>CoreDID</code>](#CoreDID)
        * [.fromJSON(json)](#CoreDID.fromJSON) ⇒ [<code>CoreDID</code>](#CoreDID)

<a name="CoreDID+toString"></a>

### coreDID.toString() ⇒ <code>string</code>
Returns the `CoreDID` as a string.

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
Parses a [`CoreDID`] from the given `input`.

# Errors

Returns `Err` if the input is not a valid [`CoreDID`].

**Kind**: static method of [<code>CoreDID</code>](#CoreDID)  

| Param | Type |
| --- | --- |
| input | <code>string</code> | 

<a name="CoreDID.fromJSON"></a>

### CoreDID.fromJSON(json) ⇒ [<code>CoreDID</code>](#CoreDID)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>CoreDID</code>](#CoreDID)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="CoreDocument"></a>

## CoreDocument
**Kind**: global class  

* [CoreDocument](#CoreDocument)
    * _instance_
        * [.toJSON()](#CoreDocument+toJSON) ⇒ <code>any</code>
        * [.clone()](#CoreDocument+clone) ⇒ [<code>CoreDocument</code>](#CoreDocument)
    * _static_
        * [.fromJSON(json)](#CoreDocument.fromJSON) ⇒ [<code>CoreDocument</code>](#CoreDocument)

<a name="CoreDocument+toJSON"></a>

### coreDocument.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  
<a name="CoreDocument+clone"></a>

### coreDocument.clone() ⇒ [<code>CoreDocument</code>](#CoreDocument)
Deep clones the object.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  
<a name="CoreDocument.fromJSON"></a>

### CoreDocument.fromJSON(json) ⇒ [<code>CoreDocument</code>](#CoreDocument)
Deserializes an instance from a JSON object.

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
    * [.verifySignature(credential, trusted_issuers, options)](#CredentialValidator.verifySignature)
    * [.check_subject_holder_relationship(credential, holder_url, relationship)](#CredentialValidator.check_subject_holder_relationship)
    * [.checkStatus(credential, trustedIssuers, statusCheck)](#CredentialValidator.checkStatus)
    * [.extractIssuer(credential)](#CredentialValidator.extractIssuer) ⇒ [<code>IotaDID</code>](#IotaDID)

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
| issuer | [<code>Document</code>](#Document) \| [<code>ResolvedDocument</code>](#ResolvedDocument) | 
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

### CredentialValidator.verifySignature(credential, trusted_issuers, options)
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
| trusted_issuers | <code>Array.&lt;(Document\|ResolvedDocument)&gt;</code> | 
| options | [<code>VerifierOptions</code>](#VerifierOptions) | 

<a name="CredentialValidator.check_subject_holder_relationship"></a>

### CredentialValidator.check\_subject\_holder\_relationship(credential, holder_url, relationship)
Validate that the relationship between the `holder` and the credential subjects is in accordance with
`relationship`. The `holder_url` parameter is expected to be the URL of the holder.

**Kind**: static method of [<code>CredentialValidator</code>](#CredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| holder_url | <code>string</code> | 
| relationship | <code>number</code> | 

<a name="CredentialValidator.checkStatus"></a>

### CredentialValidator.checkStatus(credential, trustedIssuers, statusCheck)
Checks whether the credential status has been revoked.

Only supports `BitmapRevocation2022`.

**Kind**: static method of [<code>CredentialValidator</code>](#CredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| trustedIssuers | <code>Array.&lt;(Document\|ResolvedDocument)&gt;</code> | 
| statusCheck | <code>number</code> | 

<a name="CredentialValidator.extractIssuer"></a>

### CredentialValidator.extractIssuer(credential) ⇒ [<code>IotaDID</code>](#IotaDID)
Utility for extracting the issuer field of a `Credential` as a DID.

### Errors

Fails if the issuer field is not a valid DID.

**Kind**: static method of [<code>CredentialValidator</code>](#CredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 

<a name="DIDUrl"></a>

## DIDUrl
A DID URL conforming to the IOTA DID method specification.

**Kind**: global class  

* [DIDUrl](#DIDUrl)
    * _instance_
        * [.did()](#DIDUrl+did) ⇒ [<code>IotaDID</code>](#IotaDID)
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

### didUrl.did() ⇒ [<code>IotaDID</code>](#IotaDID)
Return a copy of the `DID` section of the `DIDUrl`.

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

<a name="DiffChainHistory"></a>

## ~~DiffChainHistory~~
***Deprecated***

**Kind**: global class  

* ~~[DiffChainHistory](#DiffChainHistory)~~
    * _instance_
        * [.chainData()](#DiffChainHistory+chainData) ⇒ [<code>Array.&lt;DiffMessage&gt;</code>](#DiffMessage)
        * [.spam()](#DiffChainHistory+spam) ⇒ <code>Array.&lt;string&gt;</code>
        * [.toJSON()](#DiffChainHistory+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#DiffChainHistory.fromJSON) ⇒ [<code>DiffChainHistory</code>](#DiffChainHistory)

<a name="DiffChainHistory+chainData"></a>

### diffChainHistory.chainData() ⇒ [<code>Array.&lt;DiffMessage&gt;</code>](#DiffMessage)
Returns an `Array` of the diff chain `DiffMessages`.

NOTE: this clones the field.

**Kind**: instance method of [<code>DiffChainHistory</code>](#DiffChainHistory)  
<a name="DiffChainHistory+spam"></a>

### diffChainHistory.spam() ⇒ <code>Array.&lt;string&gt;</code>
Returns an `Array` of `MessageIds` as strings.

NOTE: this clones the field.

**Kind**: instance method of [<code>DiffChainHistory</code>](#DiffChainHistory)  
<a name="DiffChainHistory+toJSON"></a>

### diffChainHistory.toJSON() ⇒ <code>any</code>
Serializes as a JSON object.

**Kind**: instance method of [<code>DiffChainHistory</code>](#DiffChainHistory)  
<a name="DiffChainHistory.fromJSON"></a>

### DiffChainHistory.fromJSON(json) ⇒ [<code>DiffChainHistory</code>](#DiffChainHistory)
Deserializes from a JSON object.

**Kind**: static method of [<code>DiffChainHistory</code>](#DiffChainHistory)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="DiffMessage"></a>

## ~~DiffMessage~~
***Deprecated***

Defines the difference between two DID `Document`s' JSON representations.

**Kind**: global class  

* ~~[DiffMessage](#DiffMessage)~~
    * _instance_
        * ~~[.id()](#DiffMessage+id) ⇒ [<code>IotaDID</code>](#IotaDID)~~
        * ~~[.did()](#DiffMessage+did) ⇒ [<code>IotaDID</code>](#IotaDID)~~
        * ~~[.diff()](#DiffMessage+diff) ⇒ <code>string</code>~~
        * ~~[.messageId()](#DiffMessage+messageId) ⇒ <code>string</code>~~
        * ~~[.setMessageId(message_id)](#DiffMessage+setMessageId)~~
        * ~~[.previousMessageId()](#DiffMessage+previousMessageId) ⇒ <code>string</code>~~
        * ~~[.setPreviousMessageId(message_id)](#DiffMessage+setPreviousMessageId)~~
        * ~~[.proof()](#DiffMessage+proof) ⇒ [<code>Proof</code>](#Proof) \| <code>undefined</code>~~
        * ~~[.merge(document)](#DiffMessage+merge) ⇒ [<code>Document</code>](#Document)~~
        * [.toJSON()](#DiffMessage+toJSON) ⇒ <code>any</code>
        * [.clone()](#DiffMessage+clone) ⇒ [<code>DiffMessage</code>](#DiffMessage)
    * _static_
        * [.fromJSON(json)](#DiffMessage.fromJSON) ⇒ [<code>DiffMessage</code>](#DiffMessage)

<a name="DiffMessage+id"></a>

### ~~diffMessage.id() ⇒ [<code>IotaDID</code>](#IotaDID)~~
***Deprecated***

Returns the DID of the associated DID Document.

NOTE: clones the data.

**Kind**: instance method of [<code>DiffMessage</code>](#DiffMessage)  
<a name="DiffMessage+did"></a>

### ~~diffMessage.did() ⇒ [<code>IotaDID</code>](#IotaDID)~~
***Deprecated***

Returns a copy of the DID of the associated DID Document.

**Kind**: instance method of [<code>DiffMessage</code>](#DiffMessage)  
<a name="DiffMessage+diff"></a>

### ~~diffMessage.diff() ⇒ <code>string</code>~~
***Deprecated***

Returns a copy of the raw contents of the DID Document diff as a JSON string.

**Kind**: instance method of [<code>DiffMessage</code>](#DiffMessage)  
<a name="DiffMessage+messageId"></a>

### ~~diffMessage.messageId() ⇒ <code>string</code>~~
***Deprecated***

Returns a copy of the message_id of the DID Document diff.

**Kind**: instance method of [<code>DiffMessage</code>](#DiffMessage)  
<a name="DiffMessage+setMessageId"></a>

### ~~diffMessage.setMessageId(message_id)~~
***Deprecated***

Sets the message_id of the DID Document diff.

**Kind**: instance method of [<code>DiffMessage</code>](#DiffMessage)  

| Param | Type |
| --- | --- |
| message_id | <code>string</code> | 

<a name="DiffMessage+previousMessageId"></a>

### ~~diffMessage.previousMessageId() ⇒ <code>string</code>~~
***Deprecated***

Returns a copy of the Tangle message id of the previous DID Document diff.

**Kind**: instance method of [<code>DiffMessage</code>](#DiffMessage)  
<a name="DiffMessage+setPreviousMessageId"></a>

### ~~diffMessage.setPreviousMessageId(message_id)~~
***Deprecated***

Sets the Tangle message id of the previous DID Document diff.

**Kind**: instance method of [<code>DiffMessage</code>](#DiffMessage)  

| Param | Type |
| --- | --- |
| message_id | <code>string</code> | 

<a name="DiffMessage+proof"></a>

### ~~diffMessage.proof() ⇒ [<code>Proof</code>](#Proof) \| <code>undefined</code>~~
***Deprecated***

Returns a copy of the proof.

**Kind**: instance method of [<code>DiffMessage</code>](#DiffMessage)  
<a name="DiffMessage+merge"></a>

### ~~diffMessage.merge(document) ⇒ [<code>Document</code>](#Document)~~
***Deprecated***

Returns a new DID Document which is the result of merging `self`
with the given Document.

**Kind**: instance method of [<code>DiffMessage</code>](#DiffMessage)  

| Param | Type |
| --- | --- |
| document | [<code>Document</code>](#Document) | 

<a name="DiffMessage+toJSON"></a>

### diffMessage.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>DiffMessage</code>](#DiffMessage)  
<a name="DiffMessage+clone"></a>

### diffMessage.clone() ⇒ [<code>DiffMessage</code>](#DiffMessage)
Deep clones the object.

**Kind**: instance method of [<code>DiffMessage</code>](#DiffMessage)  
<a name="DiffMessage.fromJSON"></a>

### DiffMessage.fromJSON(json) ⇒ [<code>DiffMessage</code>](#DiffMessage)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>DiffMessage</code>](#DiffMessage)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="Document"></a>

## Document
**Kind**: global class  

* [Document](#Document)
    * [new Document(keypair, network, fragment)](#new_Document_new)
    * _instance_
        * [.id()](#Document+id) ⇒ [<code>IotaDID</code>](#IotaDID)
        * [.setController(controllers)](#Document+setController)
        * [.controller()](#Document+controller) ⇒ [<code>Array.&lt;IotaDID&gt;</code>](#IotaDID)
        * [.setAlsoKnownAs(urls)](#Document+setAlsoKnownAs)
        * [.alsoKnownAs()](#Document+alsoKnownAs) ⇒ <code>Array.&lt;string&gt;</code>
        * [.setPropertyUnchecked(key, value)](#Document+setPropertyUnchecked)
        * [.properties()](#Document+properties) ⇒ <code>Map.&lt;string, any&gt;</code>
        * [.service()](#Document+service) ⇒ [<code>Array.&lt;Service&gt;</code>](#Service)
        * [.insertService(service)](#Document+insertService) ⇒ <code>boolean</code>
        * [.removeService(did)](#Document+removeService) ⇒ <code>boolean</code>
        * [.resolveService(query)](#Document+resolveService) ⇒ [<code>Service</code>](#Service) \| <code>undefined</code>
        * [.methods()](#Document+methods) ⇒ [<code>Array.&lt;VerificationMethod&gt;</code>](#VerificationMethod)
        * [.insertMethod(method, scope)](#Document+insertMethod)
        * [.removeMethod(did)](#Document+removeMethod)
        * [.defaultSigningMethod()](#Document+defaultSigningMethod) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
        * [.resolveMethod(query, scope)](#Document+resolveMethod) ⇒ [<code>VerificationMethod</code>](#VerificationMethod) \| <code>undefined</code>
        * [.resolveSigningMethod(query)](#Document+resolveSigningMethod) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
        * [.attachMethodRelationship(didUrl, relationship)](#Document+attachMethodRelationship) ⇒ <code>boolean</code>
        * [.detachMethodRelationship(didUrl, relationship)](#Document+detachMethodRelationship) ⇒ <code>boolean</code>
        * [.signSelf(key_pair, method_query)](#Document+signSelf)
        * [.signDocument(document, key_pair, method_query)](#Document+signDocument)
        * [.signCredential(credential, privateKey, methodQuery, options)](#Document+signCredential) ⇒ [<code>Credential</code>](#Credential)
        * [.signPresentation(presentation, privateKey, methodQuery, options)](#Document+signPresentation) ⇒ [<code>Presentation</code>](#Presentation)
        * [.signData(data, privateKey, methodQuery, options)](#Document+signData) ⇒ <code>any</code>
        * [.verifyData(data, options)](#Document+verifyData) ⇒ <code>boolean</code>
        * [.verifyDocument(signed)](#Document+verifyDocument)
        * ~~[.diff(other, message_id, key, method_query)](#Document+diff) ⇒ [<code>DiffMessage</code>](#DiffMessage)~~
        * ~~[.verifyDiff(diff)](#Document+verifyDiff)~~
        * ~~[.mergeDiff(diff)](#Document+mergeDiff)~~
        * [.integrationIndex()](#Document+integrationIndex) ⇒ <code>string</code>
        * [.metadata()](#Document+metadata) ⇒ [<code>DocumentMetadata</code>](#DocumentMetadata)
        * [.metadataCreated()](#Document+metadataCreated) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
        * [.setMetadataCreated(timestamp)](#Document+setMetadataCreated)
        * [.metadataUpdated()](#Document+metadataUpdated) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
        * [.setMetadataUpdated(timestamp)](#Document+setMetadataUpdated)
        * [.metadataPreviousMessageId()](#Document+metadataPreviousMessageId) ⇒ <code>string</code>
        * [.setMetadataPreviousMessageId(value)](#Document+setMetadataPreviousMessageId)
        * [.setMetadataPropertyUnchecked(key, value)](#Document+setMetadataPropertyUnchecked)
        * [.proof()](#Document+proof) ⇒ [<code>Proof</code>](#Proof) \| <code>undefined</code>
        * [.revokeCredentials(serviceQuery, indices)](#Document+revokeCredentials)
        * [.unrevokeCredentials(serviceQuery, indices)](#Document+unrevokeCredentials)
        * [.toJSON()](#Document+toJSON) ⇒ <code>any</code>
        * [.clone()](#Document+clone) ⇒ [<code>Document</code>](#Document)
    * _static_
        * [.fromVerificationMethod(method)](#Document.fromVerificationMethod) ⇒ [<code>Document</code>](#Document)
        * [.isSigningMethodType(method_type)](#Document.isSigningMethodType) ⇒ <code>boolean</code>
        * [.verifyRootDocument(document)](#Document.verifyRootDocument)
        * ~~[.diffIndex(message_id)](#Document.diffIndex) ⇒ <code>string</code>~~
        * [.fromJSON(json)](#Document.fromJSON) ⇒ [<code>Document</code>](#Document)

<a name="new_Document_new"></a>

### new Document(keypair, network, fragment)
Creates a new DID Document from the given `KeyPair`, network, and verification method
fragment name.

The DID Document will be pre-populated with a single verification method
derived from the provided `KeyPair` embedded as a capability invocation
verification relationship. This method will have the DID URL fragment
`#sign-0` by default and can be easily retrieved with `Document::defaultSigningMethod`.

NOTE: the generated document is unsigned, see `Document::signSelf`.

Arguments:

* keypair: the initial verification method is derived from the public key with this keypair.
* network: Tangle network to use for the DID, default `Network::mainnet`.
* fragment: name of the initial verification method, default "sign-0".


| Param | Type |
| --- | --- |
| keypair | [<code>KeyPair</code>](#KeyPair) | 
| network | <code>string</code> \| <code>undefined</code> | 
| fragment | <code>string</code> \| <code>undefined</code> | 

<a name="Document+id"></a>

### document.id() ⇒ [<code>IotaDID</code>](#IotaDID)
Returns a copy of the DID Document `id`.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+setController"></a>

### document.setController(controllers)
Sets the controllers of the DID Document.

Note: Duplicates will be ignored.
Use `null` to remove all controllers.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| controllers | [<code>IotaDID</code>](#IotaDID) \| [<code>Array.&lt;IotaDID&gt;</code>](#IotaDID) \| <code>null</code> | 

<a name="Document+controller"></a>

### document.controller() ⇒ [<code>Array.&lt;IotaDID&gt;</code>](#IotaDID)
Returns a copy of the list of document controllers.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+setAlsoKnownAs"></a>

### document.setAlsoKnownAs(urls)
Sets the `alsoKnownAs` property in the DID document.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| urls | <code>string</code> \| <code>Array.&lt;string&gt;</code> \| <code>null</code> | 

<a name="Document+alsoKnownAs"></a>

### document.alsoKnownAs() ⇒ <code>Array.&lt;string&gt;</code>
Returns a copy of the document's `alsoKnownAs` set.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+setPropertyUnchecked"></a>

### document.setPropertyUnchecked(key, value)
Adds a custom property to the DID Document.
If the value is set to `null`, the custom property will be removed.

### WARNING
This method can overwrite existing properties like `id` and result in an invalid document.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| key | <code>string</code> | 
| value | <code>any</code> | 

<a name="Document+properties"></a>

### document.properties() ⇒ <code>Map.&lt;string, any&gt;</code>
Returns a copy of the custom DID Document properties.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+service"></a>

### document.service() ⇒ [<code>Array.&lt;Service&gt;</code>](#Service)
Return a set of all [Services](#Service) in the document.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+insertService"></a>

### document.insertService(service) ⇒ <code>boolean</code>
Add a new [Service](#Service) to the document.

Returns `true` if the service was added.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| service | [<code>Service</code>](#Service) | 

<a name="Document+removeService"></a>

### document.removeService(did) ⇒ <code>boolean</code>
Remove a [Service](#Service) identified by the given [DIDUrl](#DIDUrl) from the document.

Returns `true` if a service was removed.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| did | [<code>DIDUrl</code>](#DIDUrl) | 

<a name="Document+resolveService"></a>

### document.resolveService(query) ⇒ [<code>Service</code>](#Service) \| <code>undefined</code>
Returns the first [Service](#Service) with an `id` property matching the provided `query`,
if present.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| query | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 

<a name="Document+methods"></a>

### document.methods() ⇒ [<code>Array.&lt;VerificationMethod&gt;</code>](#VerificationMethod)
Returns a list of all [VerificationMethod](#VerificationMethod) in the DID Document.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+insertMethod"></a>

### document.insertMethod(method, scope)
Adds a new `method` to the document in the given `scope`.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| method | [<code>VerificationMethod</code>](#VerificationMethod) | 
| scope | [<code>MethodScope</code>](#MethodScope) | 

<a name="Document+removeMethod"></a>

### document.removeMethod(did)
Removes all references to the specified Verification Method.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| did | [<code>DIDUrl</code>](#DIDUrl) | 

<a name="Document+defaultSigningMethod"></a>

### document.defaultSigningMethod() ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
Returns a copy of the first `VerificationMethod` with a capability invocation relationship
capable of signing this DID document.

Throws an error if no signing method is present.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+resolveMethod"></a>

### document.resolveMethod(query, scope) ⇒ [<code>VerificationMethod</code>](#VerificationMethod) \| <code>undefined</code>
Returns a copy of the first verification method with an `id` property
matching the provided `query` and the verification relationship
specified by `scope`, if present.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| query | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| scope | [<code>MethodScope</code>](#MethodScope) \| <code>undefined</code> | 

<a name="Document+resolveSigningMethod"></a>

### document.resolveSigningMethod(query) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
Attempts to resolve the given method query into a method capable of signing a document update.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| query | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 

<a name="Document+attachMethodRelationship"></a>

### document.attachMethodRelationship(didUrl, relationship) ⇒ <code>boolean</code>
Attaches the relationship to the given method, if the method exists.

Note: The method needs to be in the set of verification methods,
so it cannot be an embedded one.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| didUrl | [<code>DIDUrl</code>](#DIDUrl) | 
| relationship | <code>number</code> | 

<a name="Document+detachMethodRelationship"></a>

### document.detachMethodRelationship(didUrl, relationship) ⇒ <code>boolean</code>
Detaches the given relationship from the given method, if the method exists.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| didUrl | [<code>DIDUrl</code>](#DIDUrl) | 
| relationship | <code>number</code> | 

<a name="Document+signSelf"></a>

### document.signSelf(key_pair, method_query)
Signs the DID document with the verification method specified by `method_query`.
The `method_query` may be the full `DIDUrl` of the method or just its fragment,
e.g. "#sign-0".

NOTE: does not validate whether the private key of the given `key_pair` corresponds to the
verification method. See `Document::verifySelfSigned`.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| key_pair | [<code>KeyPair</code>](#KeyPair) | 
| method_query | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 

<a name="Document+signDocument"></a>

### document.signDocument(document, key_pair, method_query)
Signs another DID document using the verification method specified by `method_query`.
The `method_query` may be the full `DIDUrl` of the method or just its fragment,
e.g. "#sign-0".

`Document.signSelf` should be used in general, this throws an error if trying to operate
on the same document. This is intended for signing updates to a document where a sole
capability invocation method is rotated or replaced entirely.

NOTE: does not validate whether the private key of the given `key_pair` corresponds to the
verification method. See [Document.verifyDocument](#Document+verifyDocument).

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| document | [<code>Document</code>](#Document) | 
| key_pair | [<code>KeyPair</code>](#KeyPair) | 
| method_query | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 

<a name="Document+signCredential"></a>

### document.signCredential(credential, privateKey, methodQuery, options) ⇒ [<code>Credential</code>](#Credential)
Creates a signature for the given `Credential` with the specified DID Document
Verification Method.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| privateKey | <code>Uint8Array</code> | 
| methodQuery | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| options | [<code>ProofOptions</code>](#ProofOptions) | 

<a name="Document+signPresentation"></a>

### document.signPresentation(presentation, privateKey, methodQuery, options) ⇒ [<code>Presentation</code>](#Presentation)
Creates a signature for the given `Presentation` with the specified DID Document
Verification Method.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| presentation | [<code>Presentation</code>](#Presentation) | 
| privateKey | <code>Uint8Array</code> | 
| methodQuery | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| options | [<code>ProofOptions</code>](#ProofOptions) | 

<a name="Document+signData"></a>

### document.signData(data, privateKey, methodQuery, options) ⇒ <code>any</code>
Creates a signature for the given `data` with the specified DID Document
Verification Method.

NOTE: use `signSelf` or `signDocument` for DID Documents.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| data | <code>any</code> | 
| privateKey | <code>Uint8Array</code> | 
| methodQuery | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| options | [<code>ProofOptions</code>](#ProofOptions) | 

<a name="Document+verifyData"></a>

### document.verifyData(data, options) ⇒ <code>boolean</code>
Verifies the authenticity of `data` using the target verification method.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| data | <code>any</code> | 
| options | [<code>VerifierOptions</code>](#VerifierOptions) | 

<a name="Document+verifyDocument"></a>

### document.verifyDocument(signed)
Verifies that the signature on the DID document `signed` was generated by a valid method from
this DID document.

# Errors

Fails if:
- The signature proof section is missing in the `signed` document.
- The method is not found in this document.
- An unsupported verification method is used.
- The signature verification operation fails.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| signed | [<code>Document</code>](#Document) | 

<a name="Document+diff"></a>

### ~~document.diff(other, message_id, key, method_query) ⇒ [<code>DiffMessage</code>](#DiffMessage)~~
***Deprecated***

Generate a `DiffMessage` between two DID Documents and sign it using the specified
`key` and `method`.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| other | [<code>Document</code>](#Document) | 
| message_id | <code>string</code> | 
| key | [<code>KeyPair</code>](#KeyPair) | 
| method_query | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 

<a name="Document+verifyDiff"></a>

### ~~document.verifyDiff(diff)~~
***Deprecated***

Verifies the signature of the `diff` was created using a capability invocation method
in this DID Document.

# Errors

Fails if an unsupported verification method is used or the verification operation fails.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| diff | [<code>DiffMessage</code>](#DiffMessage) | 

<a name="Document+mergeDiff"></a>

### ~~document.mergeDiff(diff)~~
***Deprecated***

Verifies a `DiffMessage` signature and attempts to merge the changes into `self`.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| diff | [<code>DiffMessage</code>](#DiffMessage) | 

<a name="Document+integrationIndex"></a>

### document.integrationIndex() ⇒ <code>string</code>
Returns the Tangle index of the integration chain for this DID.

This is simply the tag segment of the `DID`.
E.g.
For a document with DID: did:iota:1234567890abcdefghijklmnopqrstuvxyzABCDEFGHI,
`doc.integration_index()` == "1234567890abcdefghijklmnopqrstuvxyzABCDEFGHI"

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+metadata"></a>

### document.metadata() ⇒ [<code>DocumentMetadata</code>](#DocumentMetadata)
Returns a copy of the metadata associated with this document.

NOTE: Copies all the metadata. See also `metadataCreated`, `metadataUpdated`,
`metadataPreviousMessageId`, `metadataProof` if only a subset of the metadata required.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+metadataCreated"></a>

### document.metadataCreated() ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
Returns a copy of the timestamp of when the DID document was created.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+setMetadataCreated"></a>

### document.setMetadataCreated(timestamp)
Sets the timestamp of when the DID document was created.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| timestamp | [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code> | 

<a name="Document+metadataUpdated"></a>

### document.metadataUpdated() ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
Returns a copy of the timestamp of the last DID document update.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+setMetadataUpdated"></a>

### document.setMetadataUpdated(timestamp)
Sets the timestamp of the last DID document update.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| timestamp | [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code> | 

<a name="Document+metadataPreviousMessageId"></a>

### document.metadataPreviousMessageId() ⇒ <code>string</code>
Returns a copy of the previous integration chain message id.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+setMetadataPreviousMessageId"></a>

### document.setMetadataPreviousMessageId(value)
Sets the previous integration chain message id.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="Document+setMetadataPropertyUnchecked"></a>

### document.setMetadataPropertyUnchecked(key, value)
Sets a custom property in the document metadata.
If the value is set to `null`, the custom property will be removed.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| key | <code>string</code> | 
| value | <code>any</code> | 

<a name="Document+proof"></a>

### document.proof() ⇒ [<code>Proof</code>](#Proof) \| <code>undefined</code>
Returns a copy of the proof.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+revokeCredentials"></a>

### document.revokeCredentials(serviceQuery, indices)
If the document has a `RevocationBitmap` service identified by `serviceQuery`,
revoke all specified `indices`.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| serviceQuery | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| indices | <code>number</code> \| <code>Array.&lt;number&gt;</code> | 

<a name="Document+unrevokeCredentials"></a>

### document.unrevokeCredentials(serviceQuery, indices)
If the document has a `RevocationBitmap` service identified by `serviceQuery`,
unrevoke all specified `indices`.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| serviceQuery | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| indices | <code>number</code> \| <code>Array.&lt;number&gt;</code> | 

<a name="Document+toJSON"></a>

### document.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+clone"></a>

### document.clone() ⇒ [<code>Document</code>](#Document)
Deep clones the object.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document.fromVerificationMethod"></a>

### Document.fromVerificationMethod(method) ⇒ [<code>Document</code>](#Document)
Creates a new DID Document from the given `VerificationMethod`.

NOTE: the generated document is unsigned, see `Document::signSelf`.

**Kind**: static method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| method | [<code>VerificationMethod</code>](#VerificationMethod) | 

<a name="Document.isSigningMethodType"></a>

### Document.isSigningMethodType(method_type) ⇒ <code>boolean</code>
Returns whether the given [MethodType](#MethodType) can be used to sign document updates.

**Kind**: static method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| method_type | [<code>MethodType</code>](#MethodType) | 

<a name="Document.verifyRootDocument"></a>

### Document.verifyRootDocument(document)
Verifies whether `document` is a valid root DID document according to the IOTA DID method
specification.

It must be signed using a verification method with a public key whose BLAKE2b-256 hash matches
the DID tag.

**Kind**: static method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| document | [<code>Document</code>](#Document) | 

<a name="Document.diffIndex"></a>

### ~~Document.diffIndex(message_id) ⇒ <code>string</code>~~
***Deprecated***

Returns the Tangle index of the DID diff chain. This should only be called on documents
published on the integration chain.

This is the Base58-btc encoded SHA-256 digest of the hex-encoded message id.

**Kind**: static method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| message_id | <code>string</code> | 

<a name="Document.fromJSON"></a>

### Document.fromJSON(json) ⇒ [<code>Document</code>](#Document)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="DocumentHistory"></a>

## DocumentHistory
A DID Document's history and current state.

**Kind**: global class  

* [DocumentHistory](#DocumentHistory)
    * _instance_
        * [.integrationChainData()](#DocumentHistory+integrationChainData) ⇒ [<code>Array.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
        * [.integrationChainSpam()](#DocumentHistory+integrationChainSpam) ⇒ <code>Array.&lt;string&gt;</code>
        * ~~[.diffChainData()](#DocumentHistory+diffChainData) ⇒ [<code>Array.&lt;DiffMessage&gt;</code>](#DiffMessage)~~
        * ~~[.diffChainSpam()](#DocumentHistory+diffChainSpam) ⇒ <code>Array.&lt;string&gt;</code>~~
        * [.toJSON()](#DocumentHistory+toJSON) ⇒ <code>any</code>
        * [.clone()](#DocumentHistory+clone) ⇒ [<code>DocumentHistory</code>](#DocumentHistory)
    * _static_
        * [.fromJSON(json)](#DocumentHistory.fromJSON) ⇒ [<code>DocumentHistory</code>](#DocumentHistory)

<a name="DocumentHistory+integrationChainData"></a>

### documentHistory.integrationChainData() ⇒ [<code>Array.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
Returns an `Array` of integration chain `Documents`.

NOTE: clones the data.

**Kind**: instance method of [<code>DocumentHistory</code>](#DocumentHistory)  
<a name="DocumentHistory+integrationChainSpam"></a>

### documentHistory.integrationChainSpam() ⇒ <code>Array.&lt;string&gt;</code>
Returns an `Array` of message id strings for "spam" messages on the same index
as the integration chain.

NOTE: clones the data.

**Kind**: instance method of [<code>DocumentHistory</code>](#DocumentHistory)  
<a name="DocumentHistory+diffChainData"></a>

### ~~documentHistory.diffChainData() ⇒ [<code>Array.&lt;DiffMessage&gt;</code>](#DiffMessage)~~
***Deprecated***

Returns an `Array` of diff chain `DiffMessages`.

NOTE: clones the data.

**Kind**: instance method of [<code>DocumentHistory</code>](#DocumentHistory)  
<a name="DocumentHistory+diffChainSpam"></a>

### ~~documentHistory.diffChainSpam() ⇒ <code>Array.&lt;string&gt;</code>~~
***Deprecated***

Returns an `Array` of message id strings for "spam" messages on the same index
as the diff chain.

NOTE: clones the data.

**Kind**: instance method of [<code>DocumentHistory</code>](#DocumentHistory)  
<a name="DocumentHistory+toJSON"></a>

### documentHistory.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>DocumentHistory</code>](#DocumentHistory)  
<a name="DocumentHistory+clone"></a>

### documentHistory.clone() ⇒ [<code>DocumentHistory</code>](#DocumentHistory)
Deep clones the object.

**Kind**: instance method of [<code>DocumentHistory</code>](#DocumentHistory)  
<a name="DocumentHistory.fromJSON"></a>

### DocumentHistory.fromJSON(json) ⇒ [<code>DocumentHistory</code>](#DocumentHistory)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>DocumentHistory</code>](#DocumentHistory)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="DocumentMetadata"></a>

## DocumentMetadata
Additional attributes related to an IOTA DID Document.

**Kind**: global class  

* [DocumentMetadata](#DocumentMetadata)
    * _instance_
        * [.created()](#DocumentMetadata+created) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
        * [.updated()](#DocumentMetadata+updated) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
        * [.previousMessageId()](#DocumentMetadata+previousMessageId) ⇒ <code>string</code>
        * [.properties()](#DocumentMetadata+properties) ⇒ <code>Map.&lt;string, any&gt;</code>
        * [.toJSON()](#DocumentMetadata+toJSON) ⇒ <code>any</code>
        * [.clone()](#DocumentMetadata+clone) ⇒ [<code>DocumentMetadata</code>](#DocumentMetadata)
    * _static_
        * [.fromJSON(json)](#DocumentMetadata.fromJSON) ⇒ [<code>DocumentMetadata</code>](#DocumentMetadata)

<a name="DocumentMetadata+created"></a>

### documentMetadata.created() ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
Returns a copy of the timestamp of when the DID document was created.

**Kind**: instance method of [<code>DocumentMetadata</code>](#DocumentMetadata)  
<a name="DocumentMetadata+updated"></a>

### documentMetadata.updated() ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
Returns a copy of the timestamp of the last DID document update.

**Kind**: instance method of [<code>DocumentMetadata</code>](#DocumentMetadata)  
<a name="DocumentMetadata+previousMessageId"></a>

### documentMetadata.previousMessageId() ⇒ <code>string</code>
Returns a copy of the previous message identifier.

**Kind**: instance method of [<code>DocumentMetadata</code>](#DocumentMetadata)  
<a name="DocumentMetadata+properties"></a>

### documentMetadata.properties() ⇒ <code>Map.&lt;string, any&gt;</code>
Returns a copy of the custom metadata properties.

**Kind**: instance method of [<code>DocumentMetadata</code>](#DocumentMetadata)  
<a name="DocumentMetadata+toJSON"></a>

### documentMetadata.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>DocumentMetadata</code>](#DocumentMetadata)  
<a name="DocumentMetadata+clone"></a>

### documentMetadata.clone() ⇒ [<code>DocumentMetadata</code>](#DocumentMetadata)
Deep clones the object.

**Kind**: instance method of [<code>DocumentMetadata</code>](#DocumentMetadata)  
<a name="DocumentMetadata.fromJSON"></a>

### DocumentMetadata.fromJSON(json) ⇒ [<code>DocumentMetadata</code>](#DocumentMetadata)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>DocumentMetadata</code>](#DocumentMetadata)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

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

<a name="EncryptedData"></a>

## EncryptedData
The structure returned after encrypting data

**Kind**: global class  

* [EncryptedData](#EncryptedData)
    * _instance_
        * [.nonce()](#EncryptedData+nonce) ⇒ <code>Uint8Array</code>
        * [.associatedData()](#EncryptedData+associatedData) ⇒ <code>Uint8Array</code>
        * [.ciphertext()](#EncryptedData+ciphertext) ⇒ <code>Uint8Array</code>
        * [.tag()](#EncryptedData+tag) ⇒ <code>Uint8Array</code>
        * [.toJSON()](#EncryptedData+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#EncryptedData.fromJSON) ⇒ [<code>EncryptedData</code>](#EncryptedData)

<a name="EncryptedData+nonce"></a>

### encryptedData.nonce() ⇒ <code>Uint8Array</code>
Returns a copy of the nonce

**Kind**: instance method of [<code>EncryptedData</code>](#EncryptedData)  
<a name="EncryptedData+associatedData"></a>

### encryptedData.associatedData() ⇒ <code>Uint8Array</code>
Returns a copy of the associated data

**Kind**: instance method of [<code>EncryptedData</code>](#EncryptedData)  
<a name="EncryptedData+ciphertext"></a>

### encryptedData.ciphertext() ⇒ <code>Uint8Array</code>
Returns a copy of the ciphertext

**Kind**: instance method of [<code>EncryptedData</code>](#EncryptedData)  
<a name="EncryptedData+tag"></a>

### encryptedData.tag() ⇒ <code>Uint8Array</code>
Returns a copy of the tag

**Kind**: instance method of [<code>EncryptedData</code>](#EncryptedData)  
<a name="EncryptedData+toJSON"></a>

### encryptedData.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>EncryptedData</code>](#EncryptedData)  
<a name="EncryptedData.fromJSON"></a>

### EncryptedData.fromJSON(json) ⇒ [<code>EncryptedData</code>](#EncryptedData)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>EncryptedData</code>](#EncryptedData)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="EncryptionAlgorithm"></a>

## EncryptionAlgorithm
Supported content encryption algorithms.

**Kind**: global class  

* [EncryptionAlgorithm](#EncryptionAlgorithm)
    * _instance_
        * [.keyLength()](#EncryptionAlgorithm+keyLength) ⇒ <code>number</code>
        * [.toJSON()](#EncryptionAlgorithm+toJSON) ⇒ <code>any</code>
    * _static_
        * [.A256GCM()](#EncryptionAlgorithm.A256GCM) ⇒ [<code>EncryptionAlgorithm</code>](#EncryptionAlgorithm)
        * [.fromJSON(json)](#EncryptionAlgorithm.fromJSON) ⇒ [<code>EncryptionAlgorithm</code>](#EncryptionAlgorithm)

<a name="EncryptionAlgorithm+keyLength"></a>

### encryptionAlgorithm.keyLength() ⇒ <code>number</code>
Returns the length of the cipher's key.

**Kind**: instance method of [<code>EncryptionAlgorithm</code>](#EncryptionAlgorithm)  
<a name="EncryptionAlgorithm+toJSON"></a>

### encryptionAlgorithm.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>EncryptionAlgorithm</code>](#EncryptionAlgorithm)  
<a name="EncryptionAlgorithm.A256GCM"></a>

### EncryptionAlgorithm.A256GCM() ⇒ [<code>EncryptionAlgorithm</code>](#EncryptionAlgorithm)
AES GCM using 256-bit key.

**Kind**: static method of [<code>EncryptionAlgorithm</code>](#EncryptionAlgorithm)  
<a name="EncryptionAlgorithm.fromJSON"></a>

### EncryptionAlgorithm.fromJSON(json) ⇒ [<code>EncryptionAlgorithm</code>](#EncryptionAlgorithm)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>EncryptionAlgorithm</code>](#EncryptionAlgorithm)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="ExplorerUrl"></a>

## ExplorerUrl
**Kind**: global class  

* [ExplorerUrl](#ExplorerUrl)
    * _instance_
        * [.messageUrl(message_id)](#ExplorerUrl+messageUrl) ⇒ <code>string</code>
        * [.resolverUrl(did)](#ExplorerUrl+resolverUrl) ⇒ <code>string</code>
        * [.toString()](#ExplorerUrl+toString) ⇒ <code>string</code>
        * [.toJSON()](#ExplorerUrl+toJSON) ⇒ <code>any</code>
    * _static_
        * [.parse(url)](#ExplorerUrl.parse) ⇒ [<code>ExplorerUrl</code>](#ExplorerUrl)
        * [.mainnet()](#ExplorerUrl.mainnet) ⇒ [<code>ExplorerUrl</code>](#ExplorerUrl)
        * [.devnet()](#ExplorerUrl.devnet) ⇒ [<code>ExplorerUrl</code>](#ExplorerUrl)
        * [.fromJSON(json)](#ExplorerUrl.fromJSON) ⇒ [<code>ExplorerUrl</code>](#ExplorerUrl)

<a name="ExplorerUrl+messageUrl"></a>

### explorerUrl.messageUrl(message_id) ⇒ <code>string</code>
Returns the web explorer URL of the given `message_id`.

E.g. https://explorer.iota.org/mainnet/message/{message_id}

**Kind**: instance method of [<code>ExplorerUrl</code>](#ExplorerUrl)  

| Param | Type |
| --- | --- |
| message_id | <code>string</code> | 

<a name="ExplorerUrl+resolverUrl"></a>

### explorerUrl.resolverUrl(did) ⇒ <code>string</code>
Returns the web identity resolver URL for the given DID.

E.g. https://explorer.iota.org/mainnet/identity-resolver/{did}

**Kind**: instance method of [<code>ExplorerUrl</code>](#ExplorerUrl)  

| Param | Type |
| --- | --- |
| did | [<code>IotaDID</code>](#IotaDID) \| <code>string</code> | 

<a name="ExplorerUrl+toString"></a>

### explorerUrl.toString() ⇒ <code>string</code>
**Kind**: instance method of [<code>ExplorerUrl</code>](#ExplorerUrl)  
<a name="ExplorerUrl+toJSON"></a>

### explorerUrl.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>ExplorerUrl</code>](#ExplorerUrl)  
<a name="ExplorerUrl.parse"></a>

### ExplorerUrl.parse(url) ⇒ [<code>ExplorerUrl</code>](#ExplorerUrl)
Constructs a new Tangle explorer URL from a string.

Use `ExplorerUrl::mainnet` or `ExplorerUrl::devnet` unless using a private Tangle
or local explorer.

**Kind**: static method of [<code>ExplorerUrl</code>](#ExplorerUrl)  

| Param | Type |
| --- | --- |
| url | <code>string</code> | 

<a name="ExplorerUrl.mainnet"></a>

### ExplorerUrl.mainnet() ⇒ [<code>ExplorerUrl</code>](#ExplorerUrl)
Returns the Tangle explorer URL for the mainnet.

**Kind**: static method of [<code>ExplorerUrl</code>](#ExplorerUrl)  
<a name="ExplorerUrl.devnet"></a>

### ExplorerUrl.devnet() ⇒ [<code>ExplorerUrl</code>](#ExplorerUrl)
Returns the Tangle explorer URL for the devnet.

**Kind**: static method of [<code>ExplorerUrl</code>](#ExplorerUrl)  
<a name="ExplorerUrl.fromJSON"></a>

### ExplorerUrl.fromJSON(json) ⇒ [<code>ExplorerUrl</code>](#ExplorerUrl)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>ExplorerUrl</code>](#ExplorerUrl)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="IntegrationChainHistory"></a>

## IntegrationChainHistory
**Kind**: global class  

* [IntegrationChainHistory](#IntegrationChainHistory)
    * _instance_
        * [.chainData()](#IntegrationChainHistory+chainData) ⇒ [<code>Array.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
        * [.spam()](#IntegrationChainHistory+spam) ⇒ <code>Array.&lt;string&gt;</code>
        * [.toJSON()](#IntegrationChainHistory+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#IntegrationChainHistory.fromJSON) ⇒ [<code>IntegrationChainHistory</code>](#IntegrationChainHistory)

<a name="IntegrationChainHistory+chainData"></a>

### integrationChainHistory.chainData() ⇒ [<code>Array.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
Returns an `Array` of the integration chain `Documents`.

NOTE: this clones the field.

**Kind**: instance method of [<code>IntegrationChainHistory</code>](#IntegrationChainHistory)  
<a name="IntegrationChainHistory+spam"></a>

### integrationChainHistory.spam() ⇒ <code>Array.&lt;string&gt;</code>
Returns an `Array` of `MessageIds` as strings.

NOTE: this clones the field.

**Kind**: instance method of [<code>IntegrationChainHistory</code>](#IntegrationChainHistory)  
<a name="IntegrationChainHistory+toJSON"></a>

### integrationChainHistory.toJSON() ⇒ <code>any</code>
Serializes as a JSON object.

**Kind**: instance method of [<code>IntegrationChainHistory</code>](#IntegrationChainHistory)  
<a name="IntegrationChainHistory.fromJSON"></a>

### IntegrationChainHistory.fromJSON(json) ⇒ [<code>IntegrationChainHistory</code>](#IntegrationChainHistory)
Deserializes from a JSON object.

**Kind**: static method of [<code>IntegrationChainHistory</code>](#IntegrationChainHistory)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="IotaDID"></a>

## IotaDID
A DID conforming to the IOTA DID method specification.

**Kind**: global class  

* [IotaDID](#IotaDID)
    * [new IotaDID(public_key, network)](#new_IotaDID_new)
    * _instance_
        * [.network()](#IotaDID+network) ⇒ [<code>Network</code>](#Network)
        * [.networkStr()](#IotaDID+networkStr) ⇒ <code>string</code>
        * [.tag()](#IotaDID+tag) ⇒ <code>string</code>
        * [.scheme()](#IotaDID+scheme) ⇒ <code>string</code>
        * [.authority()](#IotaDID+authority) ⇒ <code>string</code>
        * [.method()](#IotaDID+method) ⇒ <code>string</code>
        * [.methodId()](#IotaDID+methodId) ⇒ <code>string</code>
        * [.join(segment)](#IotaDID+join) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.toUrl()](#IotaDID+toUrl) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.intoUrl()](#IotaDID+intoUrl) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.toString()](#IotaDID+toString) ⇒ <code>string</code>
        * [.toJSON()](#IotaDID+toJSON) ⇒ <code>any</code>
        * [.clone()](#IotaDID+clone) ⇒ [<code>IotaDID</code>](#IotaDID)
    * _static_
        * [.METHOD](#IotaDID.METHOD) ⇒ <code>string</code>
        * [.DEFAULT_NETWORK](#IotaDID.DEFAULT_NETWORK) ⇒ <code>string</code>
        * [.parse(input)](#IotaDID.parse) ⇒ [<code>IotaDID</code>](#IotaDID)
        * [.fromJSON(json)](#IotaDID.fromJSON) ⇒ [<code>IotaDID</code>](#IotaDID)

<a name="new_IotaDID_new"></a>

### new IotaDID(public_key, network)
Creates a new `DID` from a public key.


| Param | Type |
| --- | --- |
| public_key | <code>Uint8Array</code> | 
| network | <code>string</code> \| <code>undefined</code> | 

<a name="IotaDID+network"></a>

### iotaDID.network() ⇒ [<code>Network</code>](#Network)
Returns the Tangle network of the `IotaDID`.

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+networkStr"></a>

### iotaDID.networkStr() ⇒ <code>string</code>
Returns the Tangle network name of the `IotaDID`.

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+tag"></a>

### iotaDID.tag() ⇒ <code>string</code>
Returns a copy of the unique tag of the `IotaDID`.

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+scheme"></a>

### iotaDID.scheme() ⇒ <code>string</code>
Returns the `DID` scheme.

E.g.
- `"did:example:12345678" -> "did"`
- `"did:iota:main:12345678" -> "did"`

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+authority"></a>

### iotaDID.authority() ⇒ <code>string</code>
Returns the `DID` authority: the method name and method-id.

E.g.
- `"did:example:12345678" -> "example:12345678"`
- `"did:iota:main:12345678" -> "iota:main:12345678"`

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+method"></a>

### iotaDID.method() ⇒ <code>string</code>
Returns the `DID` method name.

E.g.
- `"did:example:12345678" -> "example"`
- `"did:iota:main:12345678" -> "iota"`

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+methodId"></a>

### iotaDID.methodId() ⇒ <code>string</code>
Returns the `DID` method-specific ID.

E.g.
- `"did:example:12345678" -> "12345678"`
- `"did:iota:main:12345678" -> "main:12345678"`

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+join"></a>

### iotaDID.join(segment) ⇒ [<code>DIDUrl</code>](#DIDUrl)
Construct a new `DIDUrl` by joining with a relative DID Url string.

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  

| Param | Type |
| --- | --- |
| segment | <code>string</code> | 

<a name="IotaDID+toUrl"></a>

### iotaDID.toUrl() ⇒ [<code>DIDUrl</code>](#DIDUrl)
Clones the `IotaDID` into a `DIDUrl`.

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+intoUrl"></a>

### iotaDID.intoUrl() ⇒ [<code>DIDUrl</code>](#DIDUrl)
Converts the `IotaDID` into a `DIDUrl`, consuming it.

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+toString"></a>

### iotaDID.toString() ⇒ <code>string</code>
Returns the `IotaDID` as a string.

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+toJSON"></a>

### iotaDID.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+clone"></a>

### iotaDID.clone() ⇒ [<code>IotaDID</code>](#IotaDID)
Deep clones the object.

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID.METHOD"></a>

### IotaDID.METHOD ⇒ <code>string</code>
The IOTA DID method name (`"iota"`).

**Kind**: static property of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID.DEFAULT_NETWORK"></a>

### IotaDID.DEFAULT\_NETWORK ⇒ <code>string</code>
The default Tangle network (`"main"`).

**Kind**: static property of [<code>IotaDID</code>](#IotaDID)  
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

<a name="KeyLocation"></a>

## KeyLocation
The storage location of a verification method key.

A key is uniquely identified by the fragment and a hash of its public key.
Importantly, the fragment alone is insufficient to represent the storage location.
For example, when rotating a key, there will be two keys in storage for the
same identity with the same fragment. The `key_hash` disambiguates the keys in
situations like these.

The string representation of that location can be obtained via `canonicalRepr`.

**Kind**: global class  

* [KeyLocation](#KeyLocation)
    * [new KeyLocation(keyType, fragment, publicKey)](#new_KeyLocation_new)
    * _instance_
        * [.canonical()](#KeyLocation+canonical) ⇒ <code>string</code>
        * [.keyType()](#KeyLocation+keyType) ⇒ <code>number</code>
        * [.toString()](#KeyLocation+toString) ⇒ <code>string</code>
        * [.toJSON()](#KeyLocation+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromVerificationMethod(method)](#KeyLocation.fromVerificationMethod) ⇒ [<code>KeyLocation</code>](#KeyLocation)
        * [.fromJSON(json)](#KeyLocation.fromJSON) ⇒ [<code>KeyLocation</code>](#KeyLocation)

<a name="new_KeyLocation_new"></a>

### new KeyLocation(keyType, fragment, publicKey)
Create a location from a `KeyType`, the fragment of a verification method
and the bytes of a public key.


| Param | Type |
| --- | --- |
| keyType | <code>number</code> | 
| fragment | <code>string</code> | 
| publicKey | <code>Uint8Array</code> | 

<a name="KeyLocation+canonical"></a>

### keyLocation.canonical() ⇒ <code>string</code>
Returns the canonical string representation of the location.

This should be used as the representation for storage keys.

**Kind**: instance method of [<code>KeyLocation</code>](#KeyLocation)  
<a name="KeyLocation+keyType"></a>

### keyLocation.keyType() ⇒ <code>number</code>
Returns a copy of the key type of the key location.

**Kind**: instance method of [<code>KeyLocation</code>](#KeyLocation)  
<a name="KeyLocation+toString"></a>

### keyLocation.toString() ⇒ <code>string</code>
**Kind**: instance method of [<code>KeyLocation</code>](#KeyLocation)  
<a name="KeyLocation+toJSON"></a>

### keyLocation.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>KeyLocation</code>](#KeyLocation)  
<a name="KeyLocation.fromVerificationMethod"></a>

### KeyLocation.fromVerificationMethod(method) ⇒ [<code>KeyLocation</code>](#KeyLocation)
Obtain the location of a verification method's key in storage.

**Kind**: static method of [<code>KeyLocation</code>](#KeyLocation)  

| Param | Type |
| --- | --- |
| method | [<code>VerificationMethod</code>](#VerificationMethod) | 

<a name="KeyLocation.fromJSON"></a>

### KeyLocation.fromJSON(json) ⇒ [<code>KeyLocation</code>](#KeyLocation)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>KeyLocation</code>](#KeyLocation)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

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

<a name="MethodContent"></a>

## MethodContent
**Kind**: global class  

* [MethodContent](#MethodContent)
    * _instance_
        * [.toJSON()](#MethodContent+toJSON) ⇒ <code>any</code>
    * _static_
        * [.GenerateEd25519()](#MethodContent.GenerateEd25519) ⇒ [<code>MethodContent</code>](#MethodContent)
        * [.PrivateEd25519(privateKey)](#MethodContent.PrivateEd25519) ⇒ [<code>MethodContent</code>](#MethodContent)
        * [.PublicEd25519(publicKey)](#MethodContent.PublicEd25519) ⇒ [<code>MethodContent</code>](#MethodContent)
        * [.GenerateX25519()](#MethodContent.GenerateX25519) ⇒ [<code>MethodContent</code>](#MethodContent)
        * [.PrivateX25519(privateKey)](#MethodContent.PrivateX25519) ⇒ [<code>MethodContent</code>](#MethodContent)
        * [.PublicX25519(publicKey)](#MethodContent.PublicX25519) ⇒ [<code>MethodContent</code>](#MethodContent)
        * [.fromJSON(json)](#MethodContent.fromJSON) ⇒ [<code>MethodContent</code>](#MethodContent)

<a name="MethodContent+toJSON"></a>

### methodContent.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>MethodContent</code>](#MethodContent)  
<a name="MethodContent.GenerateEd25519"></a>

### MethodContent.GenerateEd25519() ⇒ [<code>MethodContent</code>](#MethodContent)
Generate and store a new Ed25519 keypair for a new `Ed25519VerificationKey2018` method.

**Kind**: static method of [<code>MethodContent</code>](#MethodContent)  
<a name="MethodContent.PrivateEd25519"></a>

### MethodContent.PrivateEd25519(privateKey) ⇒ [<code>MethodContent</code>](#MethodContent)
Store an existing Ed25519 private key and derive a public key from it for a new
`Ed25519VerificationKey2018` method.

**Kind**: static method of [<code>MethodContent</code>](#MethodContent)  

| Param | Type |
| --- | --- |
| privateKey | <code>Uint8Array</code> | 

<a name="MethodContent.PublicEd25519"></a>

### MethodContent.PublicEd25519(publicKey) ⇒ [<code>MethodContent</code>](#MethodContent)
Insert an existing Ed25519 public key into a new `Ed25519VerificationKey2018` method,
without generating or storing a private key.

NOTE: the method will be unable to be used to sign anything without a private key.

**Kind**: static method of [<code>MethodContent</code>](#MethodContent)  

| Param | Type |
| --- | --- |
| publicKey | <code>Uint8Array</code> | 

<a name="MethodContent.GenerateX25519"></a>

### MethodContent.GenerateX25519() ⇒ [<code>MethodContent</code>](#MethodContent)
Generate and store a new X25519 keypair for a new `X25519KeyAgreementKey2019` method.

**Kind**: static method of [<code>MethodContent</code>](#MethodContent)  
<a name="MethodContent.PrivateX25519"></a>

### MethodContent.PrivateX25519(privateKey) ⇒ [<code>MethodContent</code>](#MethodContent)
Store an existing X25519 private key and derive a public key from it for a new
`X25519KeyAgreementKey2019` method.

**Kind**: static method of [<code>MethodContent</code>](#MethodContent)  

| Param | Type |
| --- | --- |
| privateKey | <code>Uint8Array</code> | 

<a name="MethodContent.PublicX25519"></a>

### MethodContent.PublicX25519(publicKey) ⇒ [<code>MethodContent</code>](#MethodContent)
Insert an existing X25519 public key into a new `X25519KeyAgreementKey2019` method,
without generating or storing a private key.

NOTE: the method will be unable to be used for key exchange without a private key.

**Kind**: static method of [<code>MethodContent</code>](#MethodContent)  

| Param | Type |
| --- | --- |
| publicKey | <code>Uint8Array</code> | 

<a name="MethodContent.fromJSON"></a>

### MethodContent.fromJSON(json) ⇒ [<code>MethodContent</code>](#MethodContent)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>MethodContent</code>](#MethodContent)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

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

<a name="MixedResolver"></a>

## MixedResolver
**Kind**: global class  

* [MixedResolver](#MixedResolver)
    * [new MixedResolver()](#new_MixedResolver_new)
    * [.attachHandler(method, handler)](#MixedResolver+attachHandler)
    * [.resolvePresentationIssuers(presentation)](#MixedResolver+resolvePresentationIssuers) ⇒ <code>Promise.&lt;Array.&lt;(StardustDocument\|CoreDocument)&gt;&gt;</code>
    * [.resolve(did)](#MixedResolver+resolve) ⇒ <code>Promise.&lt;(StardustDocument\|CoreDocument)&gt;</code>

<a name="new_MixedResolver_new"></a>

### new MixedResolver()
Constructs a new [`MixedResolver`].

<a name="MixedResolver+attachHandler"></a>

### mixedResolver.attachHandler(method, handler)
**Kind**: instance method of [<code>MixedResolver</code>](#MixedResolver)  

| Param | Type |
| --- | --- |
| method | <code>string</code> | 
| handler | <code>function</code> | 

<a name="MixedResolver+resolvePresentationIssuers"></a>

### mixedResolver.resolvePresentationIssuers(presentation) ⇒ <code>Promise.&lt;Array.&lt;(StardustDocument\|CoreDocument)&gt;&gt;</code>
Fetches all DID Documents of [`Credential`] issuers contained in a [`Presentation`].
Issuer documents are returned in arbitrary order.

# Errors

Errors if any issuer URL cannot be parsed to a DID whose associated method is supported by this Resolver, or
resolution fails.

**Kind**: instance method of [<code>MixedResolver</code>](#MixedResolver)  

| Param | Type |
| --- | --- |
| presentation | [<code>Presentation</code>](#Presentation) | 

<a name="MixedResolver+resolve"></a>

### mixedResolver.resolve(did) ⇒ <code>Promise.&lt;(StardustDocument\|CoreDocument)&gt;</code>
**Kind**: instance method of [<code>MixedResolver</code>](#MixedResolver)  

| Param | Type |
| --- | --- |
| did | <code>string</code> | 

<a name="Network"></a>

## Network
**Kind**: global class  

* [Network](#Network)
    * _instance_
        * [.name()](#Network+name) ⇒ <code>string</code>
        * [.defaultNodeURL()](#Network+defaultNodeURL) ⇒ <code>string</code> \| <code>undefined</code>
        * [.toString()](#Network+toString) ⇒ <code>string</code>
        * [.toJSON()](#Network+toJSON) ⇒ <code>any</code>
        * [.clone()](#Network+clone) ⇒ [<code>Network</code>](#Network)
    * _static_
        * [.tryFromName(name)](#Network.tryFromName) ⇒ [<code>Network</code>](#Network)
        * [.mainnet()](#Network.mainnet) ⇒ [<code>Network</code>](#Network)
        * [.devnet()](#Network.devnet) ⇒ [<code>Network</code>](#Network)
        * [.fromJSON(json)](#Network.fromJSON) ⇒ [<code>Network</code>](#Network)

<a name="Network+name"></a>

### network.name() ⇒ <code>string</code>
Returns a copy of the network name.

**Kind**: instance method of [<code>Network</code>](#Network)  
<a name="Network+defaultNodeURL"></a>

### network.defaultNodeURL() ⇒ <code>string</code> \| <code>undefined</code>
Returns a copy of the node URL of the Tangle network.

**Kind**: instance method of [<code>Network</code>](#Network)  
<a name="Network+toString"></a>

### network.toString() ⇒ <code>string</code>
**Kind**: instance method of [<code>Network</code>](#Network)  
<a name="Network+toJSON"></a>

### network.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>Network</code>](#Network)  
<a name="Network+clone"></a>

### network.clone() ⇒ [<code>Network</code>](#Network)
Deep clones the object.

**Kind**: instance method of [<code>Network</code>](#Network)  
<a name="Network.tryFromName"></a>

### Network.tryFromName(name) ⇒ [<code>Network</code>](#Network)
Parses the provided string to a `Network`.

Errors if the name is invalid.

**Kind**: static method of [<code>Network</code>](#Network)  

| Param | Type |
| --- | --- |
| name | <code>string</code> | 

<a name="Network.mainnet"></a>

### Network.mainnet() ⇒ [<code>Network</code>](#Network)
**Kind**: static method of [<code>Network</code>](#Network)  
<a name="Network.devnet"></a>

### Network.devnet() ⇒ [<code>Network</code>](#Network)
**Kind**: static method of [<code>Network</code>](#Network)  
<a name="Network.fromJSON"></a>

### Network.fromJSON(json) ⇒ [<code>Network</code>](#Network)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>Network</code>](#Network)  

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
    * [.extractHolder(presentation)](#PresentationValidator.extractHolder) ⇒ [<code>IotaDID</code>](#IotaDID)

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
| holder | [<code>Document</code>](#Document) \| [<code>ResolvedDocument</code>](#ResolvedDocument) | 
| issuers | <code>Array.&lt;(Document\|ResolvedDocument)&gt;</code> | 
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
| holder | [<code>Document</code>](#Document) \| [<code>ResolvedDocument</code>](#ResolvedDocument) | 
| options | [<code>VerifierOptions</code>](#VerifierOptions) | 

<a name="PresentationValidator.checkStructure"></a>

### PresentationValidator.checkStructure(presentation)
Validates the semantic structure of the `Presentation`.

**Kind**: static method of [<code>PresentationValidator</code>](#PresentationValidator)  

| Param | Type |
| --- | --- |
| presentation | [<code>Presentation</code>](#Presentation) | 

<a name="PresentationValidator.extractHolder"></a>

### PresentationValidator.extractHolder(presentation) ⇒ [<code>IotaDID</code>](#IotaDID)
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

<a name="Receipt"></a>

## Receipt
**Kind**: global class  

* [Receipt](#Receipt)
    * _instance_
        * [.network()](#Receipt+network) ⇒ [<code>Network</code>](#Network)
        * [.messageId()](#Receipt+messageId) ⇒ <code>string</code>
        * [.networkId()](#Receipt+networkId) ⇒ <code>string</code>
        * [.nonce()](#Receipt+nonce) ⇒ <code>string</code>
        * [.toJSON()](#Receipt+toJSON) ⇒ <code>any</code>
        * [.clone()](#Receipt+clone) ⇒ [<code>Receipt</code>](#Receipt)
    * _static_
        * [.fromJSON(json)](#Receipt.fromJSON) ⇒ [<code>Receipt</code>](#Receipt)

<a name="Receipt+network"></a>

### receipt.network() ⇒ [<code>Network</code>](#Network)
Returns a copy of the associated IOTA Tangle `Network`.

**Kind**: instance method of [<code>Receipt</code>](#Receipt)  
<a name="Receipt+messageId"></a>

### receipt.messageId() ⇒ <code>string</code>
Returns a copy of the message `id`.

**Kind**: instance method of [<code>Receipt</code>](#Receipt)  
<a name="Receipt+networkId"></a>

### receipt.networkId() ⇒ <code>string</code>
Returns a copy of the message `network_id`.

**Kind**: instance method of [<code>Receipt</code>](#Receipt)  
<a name="Receipt+nonce"></a>

### receipt.nonce() ⇒ <code>string</code>
Returns a copy of the message `nonce`.

**Kind**: instance method of [<code>Receipt</code>](#Receipt)  
<a name="Receipt+toJSON"></a>

### receipt.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>Receipt</code>](#Receipt)  
<a name="Receipt+clone"></a>

### receipt.clone() ⇒ [<code>Receipt</code>](#Receipt)
Deep clones the object.

**Kind**: instance method of [<code>Receipt</code>](#Receipt)  
<a name="Receipt.fromJSON"></a>

### Receipt.fromJSON(json) ⇒ [<code>Receipt</code>](#Receipt)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>Receipt</code>](#Receipt)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="ResolvedDocument"></a>

## ResolvedDocument
An IOTA DID document resolved from the Tangle. Represents an integration chain message possibly
merged with one or more `DiffMessages`.

**Kind**: global class  

* [ResolvedDocument](#ResolvedDocument)
    * _instance_
        * ~~[.mergeDiffMessage(diff_message)](#ResolvedDocument+mergeDiffMessage)~~
        * [.document()](#ResolvedDocument+document) ⇒ [<code>Document</code>](#Document)
        * [.intoDocument()](#ResolvedDocument+intoDocument) ⇒ [<code>Document</code>](#Document)
        * ~~[.diffMessageId()](#ResolvedDocument+diffMessageId) ⇒ <code>string</code>~~
        * ~~[.setDiffMessageId(value)](#ResolvedDocument+setDiffMessageId)~~
        * [.integrationMessageId()](#ResolvedDocument+integrationMessageId) ⇒ <code>string</code>
        * [.setIntegrationMessageId(value)](#ResolvedDocument+setIntegrationMessageId)
        * [.toJSON()](#ResolvedDocument+toJSON) ⇒ <code>any</code>
        * [.clone()](#ResolvedDocument+clone) ⇒ [<code>ResolvedDocument</code>](#ResolvedDocument)
    * _static_
        * [.fromJSON(json)](#ResolvedDocument.fromJSON) ⇒ [<code>ResolvedDocument</code>](#ResolvedDocument)

<a name="ResolvedDocument+mergeDiffMessage"></a>

### ~~resolvedDocument.mergeDiffMessage(diff_message)~~
***Deprecated***

Attempts to merge changes from a `DiffMessage` into this document and
updates the `ResolvedDocument::diffMessageId`.

If merging fails the document remains unmodified, otherwise this represents
the merged document state.

See `Document::mergeDiff`.

# Errors

Fails if the merge operation or signature verification on the diff fails.

**Kind**: instance method of [<code>ResolvedDocument</code>](#ResolvedDocument)  

| Param | Type |
| --- | --- |
| diff_message | [<code>DiffMessage</code>](#DiffMessage) | 

<a name="ResolvedDocument+document"></a>

### resolvedDocument.document() ⇒ [<code>Document</code>](#Document)
Returns a copy of the inner DID document.

NOTE: If the `ResolvedDocument` is no longer needed after calling this method
then consider using `intoDocument()` for efficiency.

**Kind**: instance method of [<code>ResolvedDocument</code>](#ResolvedDocument)  
<a name="ResolvedDocument+intoDocument"></a>

### resolvedDocument.intoDocument() ⇒ [<code>Document</code>](#Document)
Consumes this object and returns the inner DID document.

NOTE: trying to use the `ResolvedDocument` after calling this will throw an error.

**Kind**: instance method of [<code>ResolvedDocument</code>](#ResolvedDocument)  
<a name="ResolvedDocument+diffMessageId"></a>

### ~~resolvedDocument.diffMessageId() ⇒ <code>string</code>~~
***Deprecated***

Returns a copy of the diff chain message id.

**Kind**: instance method of [<code>ResolvedDocument</code>](#ResolvedDocument)  
<a name="ResolvedDocument+setDiffMessageId"></a>

### ~~resolvedDocument.setDiffMessageId(value)~~
***Deprecated***

Sets the diff chain message id.

**Kind**: instance method of [<code>ResolvedDocument</code>](#ResolvedDocument)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="ResolvedDocument+integrationMessageId"></a>

### resolvedDocument.integrationMessageId() ⇒ <code>string</code>
Returns a copy of the integration chain message id.

**Kind**: instance method of [<code>ResolvedDocument</code>](#ResolvedDocument)  
<a name="ResolvedDocument+setIntegrationMessageId"></a>

### resolvedDocument.setIntegrationMessageId(value)
Sets the integration chain message id.

**Kind**: instance method of [<code>ResolvedDocument</code>](#ResolvedDocument)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="ResolvedDocument+toJSON"></a>

### resolvedDocument.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>ResolvedDocument</code>](#ResolvedDocument)  
<a name="ResolvedDocument+clone"></a>

### resolvedDocument.clone() ⇒ [<code>ResolvedDocument</code>](#ResolvedDocument)
Deep clones the object.

**Kind**: instance method of [<code>ResolvedDocument</code>](#ResolvedDocument)  
<a name="ResolvedDocument.fromJSON"></a>

### ResolvedDocument.fromJSON(json) ⇒ [<code>ResolvedDocument</code>](#ResolvedDocument)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>ResolvedDocument</code>](#ResolvedDocument)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="Resolver"></a>

## Resolver
**Kind**: global class  

* [Resolver](#Resolver)
    * [new Resolver()](#new_Resolver_new)
    * _instance_
        * [.getClient(network_name)](#Resolver+getClient) ⇒ [<code>Client</code>](#Client) \| <code>undefined</code>
        * [.resolve(did)](#Resolver+resolve) ⇒ [<code>Promise.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
        * [.resolveHistory(did)](#Resolver+resolveHistory) ⇒ [<code>Promise.&lt;DocumentHistory&gt;</code>](#DocumentHistory)
        * ~~[.resolveDiffHistory(document)](#Resolver+resolveDiffHistory) ⇒ [<code>Promise.&lt;DiffChainHistory&gt;</code>](#DiffChainHistory)~~
        * [.resolveCredentialIssuer(credential)](#Resolver+resolveCredentialIssuer) ⇒ [<code>Promise.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
        * [.resolvePresentationIssuers(presentation)](#Resolver+resolvePresentationIssuers) ⇒ <code>Promise.&lt;Array.&lt;ResolvedDocument&gt;&gt;</code>
        * [.resolvePresentationHolder(presentation)](#Resolver+resolvePresentationHolder) ⇒ [<code>Promise.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
        * [.verifyPresentation(presentation, options, fail_fast, holder, issuers)](#Resolver+verifyPresentation) ⇒ <code>Promise.&lt;void&gt;</code>
    * _static_
        * [.builder()](#Resolver.builder) ⇒ [<code>ResolverBuilder</code>](#ResolverBuilder)

<a name="new_Resolver_new"></a>

### new Resolver()
Constructs a new `Resolver` with a default `Client` for
the `Mainnet`.

<a name="Resolver+getClient"></a>

### resolver.getClient(network_name) ⇒ [<code>Client</code>](#Client) \| <code>undefined</code>
Returns the `Client` corresponding to the given network name if one exists.

**Kind**: instance method of [<code>Resolver</code>](#Resolver)  

| Param | Type |
| --- | --- |
| network_name | <code>string</code> | 

<a name="Resolver+resolve"></a>

### resolver.resolve(did) ⇒ [<code>Promise.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
Fetches the `Document` of the given `DID`.

**Kind**: instance method of [<code>Resolver</code>](#Resolver)  

| Param | Type |
| --- | --- |
| did | [<code>IotaDID</code>](#IotaDID) \| <code>string</code> | 

<a name="Resolver+resolveHistory"></a>

### resolver.resolveHistory(did) ⇒ [<code>Promise.&lt;DocumentHistory&gt;</code>](#DocumentHistory)
Fetches the `DocumentHistory` of the given `DID`.

**Kind**: instance method of [<code>Resolver</code>](#Resolver)  

| Param | Type |
| --- | --- |
| did | [<code>IotaDID</code>](#IotaDID) \| <code>string</code> | 

<a name="Resolver+resolveDiffHistory"></a>

### ~~resolver.resolveDiffHistory(document) ⇒ [<code>Promise.&lt;DiffChainHistory&gt;</code>](#DiffChainHistory)~~
***Deprecated***

Returns the `DiffChainHistory` of a diff chain starting from a `Document` on the
integration chain.

NOTE: the document must have been published to the Tangle and have a valid message id.

**Kind**: instance method of [<code>Resolver</code>](#Resolver)  

| Param | Type |
| --- | --- |
| document | [<code>ResolvedDocument</code>](#ResolvedDocument) | 

<a name="Resolver+resolveCredentialIssuer"></a>

### resolver.resolveCredentialIssuer(credential) ⇒ [<code>Promise.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
Fetches the DID Document of the issuer on a `Credential`.

### Errors

Errors if the issuer URL is not a valid `DID` or document resolution fails.

**Kind**: instance method of [<code>Resolver</code>](#Resolver)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 

<a name="Resolver+resolvePresentationIssuers"></a>

### resolver.resolvePresentationIssuers(presentation) ⇒ <code>Promise.&lt;Array.&lt;ResolvedDocument&gt;&gt;</code>
Fetches all DID Documents of `Credential` issuers contained in a `Presentation`.
Issuer documents are returned in arbitrary order.

### Errors

Errors if any issuer URL is not a valid `DID` or document resolution fails.

**Kind**: instance method of [<code>Resolver</code>](#Resolver)  

| Param | Type |
| --- | --- |
| presentation | [<code>Presentation</code>](#Presentation) | 

<a name="Resolver+resolvePresentationHolder"></a>

### resolver.resolvePresentationHolder(presentation) ⇒ [<code>Promise.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
Fetches the DID Document of the holder of a `Presentation`.

### Errors

Errors if the holder URL is missing, is not a valid `DID`, or document resolution fails.

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
| holder | [<code>Document</code>](#Document) \| [<code>ResolvedDocument</code>](#ResolvedDocument) \| <code>undefined</code> | 
| issuers | <code>Array.&lt;(Document\|ResolvedDocument)&gt;</code> \| <code>undefined</code> | 

<a name="Resolver.builder"></a>

### Resolver.builder() ⇒ [<code>ResolverBuilder</code>](#ResolverBuilder)
Returns a [ResolverBuilder](#ResolverBuilder) to construct a new `Resolver`.

**Kind**: static method of [<code>Resolver</code>](#Resolver)  
<a name="ResolverBuilder"></a>

## ResolverBuilder
Builder for configuring [`Clients`][Client] when constructing a [`Resolver`].

**Kind**: global class  

* [ResolverBuilder](#ResolverBuilder)
    * [new ResolverBuilder()](#new_ResolverBuilder_new)
    * [.client(client)](#ResolverBuilder+client) ⇒ [<code>ResolverBuilder</code>](#ResolverBuilder)
    * [.clientConfig(config)](#ResolverBuilder+clientConfig) ⇒ [<code>ResolverBuilder</code>](#ResolverBuilder)
    * [.build()](#ResolverBuilder+build) ⇒ [<code>Promise.&lt;Resolver&gt;</code>](#Resolver)

<a name="new_ResolverBuilder_new"></a>

### new ResolverBuilder()
Constructs a new `ResolverBuilder` with no `Clients` configured.

<a name="ResolverBuilder+client"></a>

### resolverBuilder.client(client) ⇒ [<code>ResolverBuilder</code>](#ResolverBuilder)
Inserts a `Client`.

NOTE: replaces any previous `Client` or `Config` with the same network name.

**Kind**: instance method of [<code>ResolverBuilder</code>](#ResolverBuilder)  

| Param | Type |
| --- | --- |
| client | [<code>Client</code>](#Client) | 

<a name="ResolverBuilder+clientConfig"></a>

### resolverBuilder.clientConfig(config) ⇒ [<code>ResolverBuilder</code>](#ResolverBuilder)
Inserts a `Config` used to create a `Client`.

NOTE: replaces any previous `Client` or `Config` with the same network name.

**Kind**: instance method of [<code>ResolverBuilder</code>](#ResolverBuilder)  

| Param | Type |
| --- | --- |
| config | <code>IClientConfig</code> | 

<a name="ResolverBuilder+build"></a>

### resolverBuilder.build() ⇒ [<code>Promise.&lt;Resolver&gt;</code>](#Resolver)
Constructs a new [`Resolver`] based on the builder configuration.

**Kind**: instance method of [<code>ResolverBuilder</code>](#ResolverBuilder)  
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
A DID Document Service used to enable trusted interactions associated
with a DID subject.

See: https://www.w3.org/TR/did-core/#services

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
| service | <code>IIotaService</code> | 

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

<a name="Signature"></a>

## Signature
A digital signature.

**Kind**: global class  

* [Signature](#Signature)
    * [new Signature(data)](#new_Signature_new)
    * _instance_
        * [.asBytes()](#Signature+asBytes) ⇒ <code>Uint8Array</code>
        * [.toJSON()](#Signature+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#Signature.fromJSON) ⇒ [<code>Signature</code>](#Signature)

<a name="new_Signature_new"></a>

### new Signature(data)
Creates a new `Signature`.


| Param | Type |
| --- | --- |
| data | <code>Uint8Array</code> | 

<a name="Signature+asBytes"></a>

### signature.asBytes() ⇒ <code>Uint8Array</code>
Returns a copy of the signature as a `UInt8Array`.

**Kind**: instance method of [<code>Signature</code>](#Signature)  
<a name="Signature+toJSON"></a>

### signature.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>Signature</code>](#Signature)  
<a name="Signature.fromJSON"></a>

### Signature.fromJSON(json) ⇒ [<code>Signature</code>](#Signature)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>Signature</code>](#Signature)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="StardustDID"></a>

## StardustDID
A DID conforming to the IOTA UTXO DID method specification.

**Kind**: global class  

* [StardustDID](#StardustDID)
    * [new StardustDID(bytes, network)](#new_StardustDID_new)
    * _instance_
        * [.networkStr()](#StardustDID+networkStr) ⇒ <code>string</code>
        * [.tag()](#StardustDID+tag) ⇒ <code>string</code>
        * [.scheme()](#StardustDID+scheme) ⇒ <code>string</code>
        * [.authority()](#StardustDID+authority) ⇒ <code>string</code>
        * [.method()](#StardustDID+method) ⇒ <code>string</code>
        * [.methodId()](#StardustDID+methodId) ⇒ <code>string</code>
        * [.join(segment)](#StardustDID+join) ⇒ [<code>StardustDIDUrl</code>](#StardustDIDUrl)
        * [.toUrl()](#StardustDID+toUrl) ⇒ [<code>StardustDIDUrl</code>](#StardustDIDUrl)
        * [.intoUrl()](#StardustDID+intoUrl) ⇒ [<code>StardustDIDUrl</code>](#StardustDIDUrl)
        * [.toString()](#StardustDID+toString) ⇒ <code>string</code>
        * [.toJSON()](#StardustDID+toJSON) ⇒ <code>any</code>
        * [.clone()](#StardustDID+clone) ⇒ [<code>StardustDID</code>](#StardustDID)
    * _static_
        * [.METHOD](#StardustDID.METHOD) ⇒ <code>string</code>
        * [.DEFAULT_NETWORK](#StardustDID.DEFAULT_NETWORK) ⇒ <code>string</code>
        * [.placeholder(network)](#StardustDID.placeholder) ⇒ [<code>StardustDID</code>](#StardustDID)
        * [.parse(input)](#StardustDID.parse) ⇒ [<code>StardustDID</code>](#StardustDID)
        * [.fromJSON(json)](#StardustDID.fromJSON) ⇒ [<code>StardustDID</code>](#StardustDID)

<a name="new_StardustDID_new"></a>

### new StardustDID(bytes, network)
Constructs a new `StardustDID` from a byte representation of the tag and the given
network name.

See also [placeholder](#StardustDID.placeholder).


| Param | Type |
| --- | --- |
| bytes | <code>Uint8Array</code> | 
| network | <code>string</code> | 

<a name="StardustDID+networkStr"></a>

### did.networkStr() ⇒ <code>string</code>
Returns the Tangle network name of the `StardustDID`.

**Kind**: instance method of [<code>StardustDID</code>](#StardustDID)  
<a name="StardustDID+tag"></a>

### did.tag() ⇒ <code>string</code>
Returns a copy of the unique tag of the `StardustDID`.

**Kind**: instance method of [<code>StardustDID</code>](#StardustDID)  
<a name="StardustDID+scheme"></a>

### did.scheme() ⇒ <code>string</code>
Returns the `DID` scheme.

E.g.
- `"did:example:12345678" -> "did"`
- `"did:iota:main:12345678" -> "did"`

**Kind**: instance method of [<code>StardustDID</code>](#StardustDID)  
<a name="StardustDID+authority"></a>

### did.authority() ⇒ <code>string</code>
Returns the `DID` authority: the method name and method-id.

E.g.
- `"did:example:12345678" -> "example:12345678"`
- `"did:iota:main:12345678" -> "iota:main:12345678"`

**Kind**: instance method of [<code>StardustDID</code>](#StardustDID)  
<a name="StardustDID+method"></a>

### did.method() ⇒ <code>string</code>
Returns the `DID` method name.

E.g.
- `"did:example:12345678" -> "example"`
- `"did:iota:main:12345678" -> "iota"`

**Kind**: instance method of [<code>StardustDID</code>](#StardustDID)  
<a name="StardustDID+methodId"></a>

### did.methodId() ⇒ <code>string</code>
Returns the `DID` method-specific ID.

E.g.
- `"did:example:12345678" -> "12345678"`
- `"did:iota:main:12345678" -> "main:12345678"`

**Kind**: instance method of [<code>StardustDID</code>](#StardustDID)  
<a name="StardustDID+join"></a>

### did.join(segment) ⇒ [<code>StardustDIDUrl</code>](#StardustDIDUrl)
Construct a new `DIDUrl` by joining with a relative DID Url string.

**Kind**: instance method of [<code>StardustDID</code>](#StardustDID)  

| Param | Type |
| --- | --- |
| segment | <code>string</code> | 

<a name="StardustDID+toUrl"></a>

### did.toUrl() ⇒ [<code>StardustDIDUrl</code>](#StardustDIDUrl)
Clones the `DID` into a `DIDUrl`.

**Kind**: instance method of [<code>StardustDID</code>](#StardustDID)  
<a name="StardustDID+intoUrl"></a>

### did.intoUrl() ⇒ [<code>StardustDIDUrl</code>](#StardustDIDUrl)
Converts the `DID` into a `DIDUrl`, consuming it.

**Kind**: instance method of [<code>StardustDID</code>](#StardustDID)  
<a name="StardustDID+toString"></a>

### did.toString() ⇒ <code>string</code>
Returns the `DID` as a string.

**Kind**: instance method of [<code>StardustDID</code>](#StardustDID)  
<a name="StardustDID+toJSON"></a>

### did.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>StardustDID</code>](#StardustDID)  
<a name="StardustDID+clone"></a>

### did.clone() ⇒ [<code>StardustDID</code>](#StardustDID)
Deep clones the object.

**Kind**: instance method of [<code>StardustDID</code>](#StardustDID)  
<a name="StardustDID.METHOD"></a>

### StardustDID.METHOD ⇒ <code>string</code>
The IOTA UTXO DID method name (`"stardust"`).

**Kind**: static property of [<code>StardustDID</code>](#StardustDID)  
<a name="StardustDID.DEFAULT_NETWORK"></a>

### StardustDID.DEFAULT\_NETWORK ⇒ <code>string</code>
The default Tangle network (`"main"`).

**Kind**: static property of [<code>StardustDID</code>](#StardustDID)  
<a name="StardustDID.placeholder"></a>

### StardustDID.placeholder(network) ⇒ [<code>StardustDID</code>](#StardustDID)
Creates a new placeholder [`StardustDID`] with the given network name.

E.g. `did:stardust:smr:0x0000000000000000000000000000000000000000000000000000000000000000`.

**Kind**: static method of [<code>StardustDID</code>](#StardustDID)  

| Param | Type |
| --- | --- |
| network | <code>string</code> | 

<a name="StardustDID.parse"></a>

### StardustDID.parse(input) ⇒ [<code>StardustDID</code>](#StardustDID)
Parses a `StardustDID` from the input string.

**Kind**: static method of [<code>StardustDID</code>](#StardustDID)  

| Param | Type |
| --- | --- |
| input | <code>string</code> | 

<a name="StardustDID.fromJSON"></a>

### StardustDID.fromJSON(json) ⇒ [<code>StardustDID</code>](#StardustDID)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>StardustDID</code>](#StardustDID)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="StardustDIDUrl"></a>

## StardustDIDUrl
A DID URL conforming to the IOTA Stardust UTXO DID method specification.

**Kind**: global class  

* [StardustDIDUrl](#StardustDIDUrl)
    * _instance_
        * [.did()](#StardustDIDUrl+did) ⇒ [<code>StardustDID</code>](#StardustDID)
        * [.urlStr()](#StardustDIDUrl+urlStr) ⇒ <code>string</code>
        * [.fragment()](#StardustDIDUrl+fragment) ⇒ <code>string</code> \| <code>undefined</code>
        * [.setFragment(value)](#StardustDIDUrl+setFragment)
        * [.path()](#StardustDIDUrl+path) ⇒ <code>string</code> \| <code>undefined</code>
        * [.setPath(value)](#StardustDIDUrl+setPath)
        * [.query()](#StardustDIDUrl+query) ⇒ <code>string</code> \| <code>undefined</code>
        * [.setQuery(value)](#StardustDIDUrl+setQuery)
        * [.join(segment)](#StardustDIDUrl+join) ⇒ [<code>StardustDIDUrl</code>](#StardustDIDUrl)
        * [.toString()](#StardustDIDUrl+toString) ⇒ <code>string</code>
        * [.toJSON()](#StardustDIDUrl+toJSON) ⇒ <code>any</code>
        * [.clone()](#StardustDIDUrl+clone) ⇒ [<code>StardustDIDUrl</code>](#StardustDIDUrl)
    * _static_
        * [.parse(input)](#StardustDIDUrl.parse) ⇒ [<code>StardustDIDUrl</code>](#StardustDIDUrl)
        * [.fromJSON(json)](#StardustDIDUrl.fromJSON) ⇒ [<code>StardustDIDUrl</code>](#StardustDIDUrl)

<a name="StardustDIDUrl+did"></a>

### stardustDIDUrl.did() ⇒ [<code>StardustDID</code>](#StardustDID)
Return a copy of the `StardustDID` section of the `StardustDIDUrl`.

**Kind**: instance method of [<code>StardustDIDUrl</code>](#StardustDIDUrl)  
<a name="StardustDIDUrl+urlStr"></a>

### stardustDIDUrl.urlStr() ⇒ <code>string</code>
Return a copy of the relative DID Url as a string, including only the path, query, and fragment.

**Kind**: instance method of [<code>StardustDIDUrl</code>](#StardustDIDUrl)  
<a name="StardustDIDUrl+fragment"></a>

### stardustDIDUrl.fragment() ⇒ <code>string</code> \| <code>undefined</code>
Returns a copy of the `StardustDIDUrl` method fragment, if any. Excludes the leading '#'.

**Kind**: instance method of [<code>StardustDIDUrl</code>](#StardustDIDUrl)  
<a name="StardustDIDUrl+setFragment"></a>

### stardustDIDUrl.setFragment(value)
Sets the `fragment` component of the `StardustDIDUrl`.

**Kind**: instance method of [<code>StardustDIDUrl</code>](#StardustDIDUrl)  

| Param | Type |
| --- | --- |
| value | <code>string</code> \| <code>undefined</code> | 

<a name="StardustDIDUrl+path"></a>

### stardustDIDUrl.path() ⇒ <code>string</code> \| <code>undefined</code>
Returns a copy of the `StardustDIDUrl` path.

**Kind**: instance method of [<code>StardustDIDUrl</code>](#StardustDIDUrl)  
<a name="StardustDIDUrl+setPath"></a>

### stardustDIDUrl.setPath(value)
Sets the `path` component of the `StardustDIDUrl`.

**Kind**: instance method of [<code>StardustDIDUrl</code>](#StardustDIDUrl)  

| Param | Type |
| --- | --- |
| value | <code>string</code> \| <code>undefined</code> | 

<a name="StardustDIDUrl+query"></a>

### stardustDIDUrl.query() ⇒ <code>string</code> \| <code>undefined</code>
Returns a copy of the `StardustDIDUrl` method query, if any. Excludes the leading '?'.

**Kind**: instance method of [<code>StardustDIDUrl</code>](#StardustDIDUrl)  
<a name="StardustDIDUrl+setQuery"></a>

### stardustDIDUrl.setQuery(value)
Sets the `query` component of the `StardustDIDUrl`.

**Kind**: instance method of [<code>StardustDIDUrl</code>](#StardustDIDUrl)  

| Param | Type |
| --- | --- |
| value | <code>string</code> \| <code>undefined</code> | 

<a name="StardustDIDUrl+join"></a>

### stardustDIDUrl.join(segment) ⇒ [<code>StardustDIDUrl</code>](#StardustDIDUrl)
Append a string representing a path, query, and/or fragment, returning a new `StardustDIDUrl`.

Must begin with a valid delimiter character: '/', '?', '#'. Overwrites the existing URL
segment and any following segments in order of path, query, then fragment.

I.e.
- joining a path will clear the query and fragment.
- joining a query will clear the fragment.
- joining a fragment will only overwrite the fragment.

**Kind**: instance method of [<code>StardustDIDUrl</code>](#StardustDIDUrl)  

| Param | Type |
| --- | --- |
| segment | <code>string</code> | 

<a name="StardustDIDUrl+toString"></a>

### stardustDIDUrl.toString() ⇒ <code>string</code>
Returns the `StardustDIDUrl` as a string.

**Kind**: instance method of [<code>StardustDIDUrl</code>](#StardustDIDUrl)  
<a name="StardustDIDUrl+toJSON"></a>

### stardustDIDUrl.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>StardustDIDUrl</code>](#StardustDIDUrl)  
<a name="StardustDIDUrl+clone"></a>

### stardustDIDUrl.clone() ⇒ [<code>StardustDIDUrl</code>](#StardustDIDUrl)
Deep clones the object.

**Kind**: instance method of [<code>StardustDIDUrl</code>](#StardustDIDUrl)  
<a name="StardustDIDUrl.parse"></a>

### StardustDIDUrl.parse(input) ⇒ [<code>StardustDIDUrl</code>](#StardustDIDUrl)
Parses a `StardustDIDUrl` from the input string.

**Kind**: static method of [<code>StardustDIDUrl</code>](#StardustDIDUrl)  

| Param | Type |
| --- | --- |
| input | <code>string</code> | 

<a name="StardustDIDUrl.fromJSON"></a>

### StardustDIDUrl.fromJSON(json) ⇒ [<code>StardustDIDUrl</code>](#StardustDIDUrl)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>StardustDIDUrl</code>](#StardustDIDUrl)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="StardustDocument"></a>

## StardustDocument
**Kind**: global class  

* [StardustDocument](#StardustDocument)
    * [new StardustDocument(network)](#new_StardustDocument_new)
    * _instance_
        * [.id()](#StardustDocument+id) ⇒ [<code>StardustDID</code>](#StardustDID)
        * [.controller()](#StardustDocument+controller) ⇒ [<code>Array.&lt;StardustDID&gt;</code>](#StardustDID)
        * [.alsoKnownAs()](#StardustDocument+alsoKnownAs) ⇒ <code>Array.&lt;string&gt;</code>
        * [.setAlsoKnownAs(urls)](#StardustDocument+setAlsoKnownAs)
        * [.properties()](#StardustDocument+properties) ⇒ <code>Map.&lt;string, any&gt;</code>
        * [.setPropertyUnchecked(key, value)](#StardustDocument+setPropertyUnchecked)
        * [.service()](#StardustDocument+service) ⇒ [<code>Array.&lt;StardustService&gt;</code>](#StardustService)
        * [.insertService(service)](#StardustDocument+insertService) ⇒ <code>boolean</code>
        * [.removeService(did)](#StardustDocument+removeService) ⇒ <code>boolean</code>
        * [.resolveService(query)](#StardustDocument+resolveService) ⇒ [<code>StardustService</code>](#StardustService) \| <code>undefined</code>
        * [.methods()](#StardustDocument+methods) ⇒ [<code>Array.&lt;StardustVerificationMethod&gt;</code>](#StardustVerificationMethod)
        * [.insertMethod(method, scope)](#StardustDocument+insertMethod)
        * [.removeMethod(did)](#StardustDocument+removeMethod)
        * [.resolveMethod(query, scope)](#StardustDocument+resolveMethod) ⇒ [<code>StardustVerificationMethod</code>](#StardustVerificationMethod) \| <code>undefined</code>
        * [.attachMethodRelationship(didUrl, relationship)](#StardustDocument+attachMethodRelationship) ⇒ <code>boolean</code>
        * [.detachMethodRelationship(didUrl, relationship)](#StardustDocument+detachMethodRelationship) ⇒ <code>boolean</code>
        * [.signCredential(credential, privateKey, methodQuery, options)](#StardustDocument+signCredential) ⇒ [<code>Credential</code>](#Credential)
        * [.signPresentation(presentation, privateKey, methodQuery, options)](#StardustDocument+signPresentation) ⇒ [<code>Presentation</code>](#Presentation)
        * [.signData(data, privateKey, methodQuery, options)](#StardustDocument+signData) ⇒ <code>any</code>
        * [.verifyData(data, options)](#StardustDocument+verifyData) ⇒ <code>boolean</code>
        * [.pack()](#StardustDocument+pack) ⇒ <code>Uint8Array</code>
        * [.packWithEncoding(encoding)](#StardustDocument+packWithEncoding) ⇒ <code>Uint8Array</code>
        * [.metadata()](#StardustDocument+metadata) ⇒ [<code>StardustDocumentMetadata</code>](#StardustDocumentMetadata)
        * [.metadataCreated()](#StardustDocument+metadataCreated) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
        * [.setMetadataCreated(timestamp)](#StardustDocument+setMetadataCreated)
        * [.metadataUpdated()](#StardustDocument+metadataUpdated) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
        * [.setMetadataUpdated(timestamp)](#StardustDocument+setMetadataUpdated)
        * [.metadataDeactivated()](#StardustDocument+metadataDeactivated) ⇒ <code>boolean</code> \| <code>undefined</code>
        * [.setMetadataDeactivated(deactivated)](#StardustDocument+setMetadataDeactivated)
        * [.setMetadataPropertyUnchecked(key, value)](#StardustDocument+setMetadataPropertyUnchecked)
        * [.revokeCredentials(serviceQuery, indices)](#StardustDocument+revokeCredentials)
        * [.unrevokeCredentials(serviceQuery, indices)](#StardustDocument+unrevokeCredentials)
        * [.toJSON()](#StardustDocument+toJSON) ⇒ <code>any</code>
        * [.clone()](#StardustDocument+clone) ⇒ [<code>StardustDocument</code>](#StardustDocument)
    * _static_
        * [.newWithId(id)](#StardustDocument.newWithId) ⇒ [<code>StardustDocument</code>](#StardustDocument)
        * [.unpack(did, stateMetadata, allowEmpty)](#StardustDocument.unpack) ⇒ [<code>StardustDocument</code>](#StardustDocument)
        * [.unpackFromBlock(network, block)](#StardustDocument.unpackFromBlock) ⇒ [<code>Array.&lt;StardustDocument&gt;</code>](#StardustDocument)
        * [.fromJSON(json)](#StardustDocument.fromJSON) ⇒ [<code>StardustDocument</code>](#StardustDocument)

<a name="new_StardustDocument_new"></a>

### new StardustDocument(network)
Constructs an empty DID Document with a [placeholder](#StardustDID.placeholder) identifier
for the given `network`.


| Param | Type |
| --- | --- |
| network | <code>string</code> | 

<a name="StardustDocument+id"></a>

### stardustDocument.id() ⇒ [<code>StardustDID</code>](#StardustDID)
Returns a copy of the DID Document `id`.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  
<a name="StardustDocument+controller"></a>

### stardustDocument.controller() ⇒ [<code>Array.&lt;StardustDID&gt;</code>](#StardustDID)
Returns a copy of the list of document controllers.

NOTE: controllers are determined by the `state_controller` unlock condition of the output
during resolution and are omitted when publishing.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  
<a name="StardustDocument+alsoKnownAs"></a>

### stardustDocument.alsoKnownAs() ⇒ <code>Array.&lt;string&gt;</code>
Returns a copy of the document's `alsoKnownAs` set.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  
<a name="StardustDocument+setAlsoKnownAs"></a>

### stardustDocument.setAlsoKnownAs(urls)
Sets the `alsoKnownAs` property in the DID document.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| urls | <code>string</code> \| <code>Array.&lt;string&gt;</code> \| <code>null</code> | 

<a name="StardustDocument+properties"></a>

### stardustDocument.properties() ⇒ <code>Map.&lt;string, any&gt;</code>
Returns a copy of the custom DID Document properties.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  
<a name="StardustDocument+setPropertyUnchecked"></a>

### stardustDocument.setPropertyUnchecked(key, value)
Sets a custom property in the DID Document.
If the value is set to `null`, the custom property will be removed.

### WARNING
This method can overwrite existing properties like `id` and result in an invalid document.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| key | <code>string</code> | 
| value | <code>any</code> | 

<a name="StardustDocument+service"></a>

### stardustDocument.service() ⇒ [<code>Array.&lt;StardustService&gt;</code>](#StardustService)
Return a set of all [StardustService](#StardustService) in the document.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  
<a name="StardustDocument+insertService"></a>

### stardustDocument.insertService(service) ⇒ <code>boolean</code>
Add a new [StardustService](#StardustService) to the document.

Returns `true` if the service was added.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| service | [<code>StardustService</code>](#StardustService) | 

<a name="StardustDocument+removeService"></a>

### stardustDocument.removeService(did) ⇒ <code>boolean</code>
Remove a [StardustService](#StardustService) identified by the given [DIDUrl](#DIDUrl) from the document.

Returns `true` if a service was removed.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| did | [<code>StardustDIDUrl</code>](#StardustDIDUrl) | 

<a name="StardustDocument+resolveService"></a>

### stardustDocument.resolveService(query) ⇒ [<code>StardustService</code>](#StardustService) \| <code>undefined</code>
Returns the first [StardustService](#StardustService) with an `id` property matching the provided `query`,
if present.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| query | [<code>StardustDIDUrl</code>](#StardustDIDUrl) \| <code>string</code> | 

<a name="StardustDocument+methods"></a>

### stardustDocument.methods() ⇒ [<code>Array.&lt;StardustVerificationMethod&gt;</code>](#StardustVerificationMethod)
Returns a list of all [StardustVerificationMethod](#StardustVerificationMethod) in the DID Document.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  
<a name="StardustDocument+insertMethod"></a>

### stardustDocument.insertMethod(method, scope)
Adds a new `method` to the document in the given `scope`.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| method | [<code>StardustVerificationMethod</code>](#StardustVerificationMethod) | 
| scope | [<code>MethodScope</code>](#MethodScope) | 

<a name="StardustDocument+removeMethod"></a>

### stardustDocument.removeMethod(did)
Removes all references to the specified Verification Method.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| did | [<code>StardustDIDUrl</code>](#StardustDIDUrl) | 

<a name="StardustDocument+resolveMethod"></a>

### stardustDocument.resolveMethod(query, scope) ⇒ [<code>StardustVerificationMethod</code>](#StardustVerificationMethod) \| <code>undefined</code>
Returns a copy of the first verification method with an `id` property
matching the provided `query` and the verification relationship
specified by `scope`, if present.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| query | [<code>StardustDIDUrl</code>](#StardustDIDUrl) \| <code>string</code> | 
| scope | [<code>MethodScope</code>](#MethodScope) \| <code>undefined</code> | 

<a name="StardustDocument+attachMethodRelationship"></a>

### stardustDocument.attachMethodRelationship(didUrl, relationship) ⇒ <code>boolean</code>
Attaches the relationship to the given method, if the method exists.

Note: The method needs to be in the set of verification methods,
so it cannot be an embedded one.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| didUrl | [<code>StardustDIDUrl</code>](#StardustDIDUrl) | 
| relationship | <code>number</code> | 

<a name="StardustDocument+detachMethodRelationship"></a>

### stardustDocument.detachMethodRelationship(didUrl, relationship) ⇒ <code>boolean</code>
Detaches the given relationship from the given method, if the method exists.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| didUrl | [<code>StardustDIDUrl</code>](#StardustDIDUrl) | 
| relationship | <code>number</code> | 

<a name="StardustDocument+signCredential"></a>

### stardustDocument.signCredential(credential, privateKey, methodQuery, options) ⇒ [<code>Credential</code>](#Credential)
Creates a signature for the given `Credential` with the specified DID Document
Verification Method.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| privateKey | <code>Uint8Array</code> | 
| methodQuery | [<code>StardustDIDUrl</code>](#StardustDIDUrl) \| <code>string</code> | 
| options | [<code>ProofOptions</code>](#ProofOptions) | 

<a name="StardustDocument+signPresentation"></a>

### stardustDocument.signPresentation(presentation, privateKey, methodQuery, options) ⇒ [<code>Presentation</code>](#Presentation)
Creates a signature for the given `Presentation` with the specified DID Document
Verification Method.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| presentation | [<code>Presentation</code>](#Presentation) | 
| privateKey | <code>Uint8Array</code> | 
| methodQuery | [<code>StardustDIDUrl</code>](#StardustDIDUrl) \| <code>string</code> | 
| options | [<code>ProofOptions</code>](#ProofOptions) | 

<a name="StardustDocument+signData"></a>

### stardustDocument.signData(data, privateKey, methodQuery, options) ⇒ <code>any</code>
Creates a signature for the given `data` with the specified DID Document
Verification Method.

NOTE: use `signSelf` or `signDocument` for DID Documents.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| data | <code>any</code> | 
| privateKey | <code>Uint8Array</code> | 
| methodQuery | [<code>StardustDIDUrl</code>](#StardustDIDUrl) \| <code>string</code> | 
| options | [<code>ProofOptions</code>](#ProofOptions) | 

<a name="StardustDocument+verifyData"></a>

### stardustDocument.verifyData(data, options) ⇒ <code>boolean</code>
Verifies the authenticity of `data` using the target verification method.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| data | <code>any</code> | 
| options | [<code>VerifierOptions</code>](#VerifierOptions) | 

<a name="StardustDocument+pack"></a>

### stardustDocument.pack() ⇒ <code>Uint8Array</code>
Serializes the document for inclusion in an Alias Output's state metadata
with the default [StateMetadataEncoding](#StateMetadataEncoding).

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  
<a name="StardustDocument+packWithEncoding"></a>

### stardustDocument.packWithEncoding(encoding) ⇒ <code>Uint8Array</code>
Serializes the document for inclusion in an Alias Output's state metadata.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| encoding | <code>number</code> | 

<a name="StardustDocument+metadata"></a>

### stardustDocument.metadata() ⇒ [<code>StardustDocumentMetadata</code>](#StardustDocumentMetadata)
Returns a copy of the metadata associated with this document.

NOTE: Copies all the metadata. See also `metadataCreated`, `metadataUpdated`,
`metadataPreviousMessageId`, `metadataProof` if only a subset of the metadata required.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  
<a name="StardustDocument+metadataCreated"></a>

### stardustDocument.metadataCreated() ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
Returns a copy of the timestamp of when the DID document was created.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  
<a name="StardustDocument+setMetadataCreated"></a>

### stardustDocument.setMetadataCreated(timestamp)
Sets the timestamp of when the DID document was created.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| timestamp | [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code> | 

<a name="StardustDocument+metadataUpdated"></a>

### stardustDocument.metadataUpdated() ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
Returns a copy of the timestamp of the last DID document update.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  
<a name="StardustDocument+setMetadataUpdated"></a>

### stardustDocument.setMetadataUpdated(timestamp)
Sets the timestamp of the last DID document update.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| timestamp | [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code> | 

<a name="StardustDocument+metadataDeactivated"></a>

### stardustDocument.metadataDeactivated() ⇒ <code>boolean</code> \| <code>undefined</code>
Returns a copy of the deactivated status of the DID document.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  
<a name="StardustDocument+setMetadataDeactivated"></a>

### stardustDocument.setMetadataDeactivated(deactivated)
Sets the deactivated status of the DID document.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| deactivated | <code>boolean</code> \| <code>undefined</code> | 

<a name="StardustDocument+setMetadataPropertyUnchecked"></a>

### stardustDocument.setMetadataPropertyUnchecked(key, value)
Sets a custom property in the document metadata.
If the value is set to `null`, the custom property will be removed.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| key | <code>string</code> | 
| value | <code>any</code> | 

<a name="StardustDocument+revokeCredentials"></a>

### stardustDocument.revokeCredentials(serviceQuery, indices)
If the document has a `RevocationBitmap` service identified by `serviceQuery`,
revoke all specified `indices`.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| serviceQuery | [<code>StardustDIDUrl</code>](#StardustDIDUrl) \| <code>string</code> | 
| indices | <code>number</code> \| <code>Array.&lt;number&gt;</code> | 

<a name="StardustDocument+unrevokeCredentials"></a>

### stardustDocument.unrevokeCredentials(serviceQuery, indices)
If the document has a `RevocationBitmap` service identified by `serviceQuery`,
unrevoke all specified `indices`.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| serviceQuery | [<code>StardustDIDUrl</code>](#StardustDIDUrl) \| <code>string</code> | 
| indices | <code>number</code> \| <code>Array.&lt;number&gt;</code> | 

<a name="StardustDocument+toJSON"></a>

### stardustDocument.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  
<a name="StardustDocument+clone"></a>

### stardustDocument.clone() ⇒ [<code>StardustDocument</code>](#StardustDocument)
Deep clones the object.

**Kind**: instance method of [<code>StardustDocument</code>](#StardustDocument)  
<a name="StardustDocument.newWithId"></a>

### StardustDocument.newWithId(id) ⇒ [<code>StardustDocument</code>](#StardustDocument)
Constructs an empty DID Document with the given identifier.

**Kind**: static method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| id | [<code>StardustDID</code>](#StardustDID) | 

<a name="StardustDocument.unpack"></a>

### StardustDocument.unpack(did, stateMetadata, allowEmpty) ⇒ [<code>StardustDocument</code>](#StardustDocument)
Deserializes the document from the state metadata bytes of an Alias Output.

If `allowEmpty` is true, this will return an empty DID document marked as `deactivated`
if `stateMetadata` is empty.

NOTE: `did` is required since it is omitted from the serialized DID Document and
cannot be inferred from the state metadata. It also indicates the network, which is not
encoded in the `AliasId` alone.

**Kind**: static method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| did | [<code>StardustDID</code>](#StardustDID) | 
| stateMetadata | <code>Uint8Array</code> | 
| allowEmpty | <code>boolean</code> | 

<a name="StardustDocument.unpackFromBlock"></a>

### StardustDocument.unpackFromBlock(network, block) ⇒ [<code>Array.&lt;StardustDocument&gt;</code>](#StardustDocument)
Returns all DID documents of the Alias Outputs contained in the block's transaction payload
outputs, if any.

Errors if any Alias Output does not contain a valid or empty DID Document.

**Kind**: static method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| network | <code>string</code> | 
| block | <code>IBlock</code> | 

<a name="StardustDocument.fromJSON"></a>

### StardustDocument.fromJSON(json) ⇒ [<code>StardustDocument</code>](#StardustDocument)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>StardustDocument</code>](#StardustDocument)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="StardustDocumentMetadata"></a>

## StardustDocumentMetadata
Additional attributes related to an IOTA DID Document.

**Kind**: global class  

* [StardustDocumentMetadata](#StardustDocumentMetadata)
    * _instance_
        * [.created()](#StardustDocumentMetadata+created) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
        * [.updated()](#StardustDocumentMetadata+updated) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
        * [.deactivated()](#StardustDocumentMetadata+deactivated) ⇒ <code>boolean</code> \| <code>undefined</code>
        * [.properties()](#StardustDocumentMetadata+properties) ⇒ <code>Map.&lt;string, any&gt;</code>
        * [.toJSON()](#StardustDocumentMetadata+toJSON) ⇒ <code>any</code>
        * [.clone()](#StardustDocumentMetadata+clone) ⇒ [<code>StardustDocumentMetadata</code>](#StardustDocumentMetadata)
    * _static_
        * [.fromJSON(json)](#StardustDocumentMetadata.fromJSON) ⇒ [<code>StardustDocumentMetadata</code>](#StardustDocumentMetadata)

<a name="StardustDocumentMetadata+created"></a>

### stardustDocumentMetadata.created() ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
Returns a copy of the timestamp of when the DID document was created.

**Kind**: instance method of [<code>StardustDocumentMetadata</code>](#StardustDocumentMetadata)  
<a name="StardustDocumentMetadata+updated"></a>

### stardustDocumentMetadata.updated() ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
Returns a copy of the timestamp of the last DID document update.

**Kind**: instance method of [<code>StardustDocumentMetadata</code>](#StardustDocumentMetadata)  
<a name="StardustDocumentMetadata+deactivated"></a>

### stardustDocumentMetadata.deactivated() ⇒ <code>boolean</code> \| <code>undefined</code>
Returns a copy of the deactivated status of the DID document.

**Kind**: instance method of [<code>StardustDocumentMetadata</code>](#StardustDocumentMetadata)  
<a name="StardustDocumentMetadata+properties"></a>

### stardustDocumentMetadata.properties() ⇒ <code>Map.&lt;string, any&gt;</code>
Returns a copy of the custom metadata properties.

**Kind**: instance method of [<code>StardustDocumentMetadata</code>](#StardustDocumentMetadata)  
<a name="StardustDocumentMetadata+toJSON"></a>

### stardustDocumentMetadata.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>StardustDocumentMetadata</code>](#StardustDocumentMetadata)  
<a name="StardustDocumentMetadata+clone"></a>

### stardustDocumentMetadata.clone() ⇒ [<code>StardustDocumentMetadata</code>](#StardustDocumentMetadata)
Deep clones the object.

**Kind**: instance method of [<code>StardustDocumentMetadata</code>](#StardustDocumentMetadata)  
<a name="StardustDocumentMetadata.fromJSON"></a>

### StardustDocumentMetadata.fromJSON(json) ⇒ [<code>StardustDocumentMetadata</code>](#StardustDocumentMetadata)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>StardustDocumentMetadata</code>](#StardustDocumentMetadata)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="StardustIdentityClientExt"></a>

## StardustIdentityClientExt
An extension interface that provides helper functions for publication
and resolution of DID documents in Alias Outputs.

**Kind**: global class  

* [StardustIdentityClientExt](#StardustIdentityClientExt)
    * [.newDidOutput(client, address, document, rentStructure)](#StardustIdentityClientExt.newDidOutput) ⇒ <code>Promise.&lt;IAliasOutput&gt;</code>
    * [.updateDidOutput(client, document)](#StardustIdentityClientExt.updateDidOutput) ⇒ <code>Promise.&lt;IAliasOutput&gt;</code>
    * [.deactivateDidOutput(client, did)](#StardustIdentityClientExt.deactivateDidOutput) ⇒ <code>Promise.&lt;IAliasOutput&gt;</code>
    * [.resolveDid(client, did)](#StardustIdentityClientExt.resolveDid) ⇒ [<code>Promise.&lt;StardustDocument&gt;</code>](#StardustDocument)
    * [.resolveDidOutput(client, did)](#StardustIdentityClientExt.resolveDidOutput) ⇒ <code>Promise.&lt;IAliasOutput&gt;</code>

<a name="StardustIdentityClientExt.newDidOutput"></a>

### StardustIdentityClientExt.newDidOutput(client, address, document, rentStructure) ⇒ <code>Promise.&lt;IAliasOutput&gt;</code>
Create a DID with a new Alias Output containing the given `document`.

The `address` will be set as the state controller and governor unlock conditions.
The minimum required token deposit amount will be set according to the given
`rent_structure`, which will be fetched from the node if not provided.
The returned Alias Output can be further customised before publication, if desired.

NOTE: this does *not* publish the Alias Output.

**Kind**: static method of [<code>StardustIdentityClientExt</code>](#StardustIdentityClientExt)  

| Param | Type |
| --- | --- |
| client | <code>IStardustIdentityClient</code> | 
| address | <code>AddressTypes</code> | 
| document | [<code>StardustDocument</code>](#StardustDocument) | 
| rentStructure | <code>IRent</code> \| <code>undefined</code> | 

<a name="StardustIdentityClientExt.updateDidOutput"></a>

### StardustIdentityClientExt.updateDidOutput(client, document) ⇒ <code>Promise.&lt;IAliasOutput&gt;</code>
Fetches the associated Alias Output and updates it with `document` in its state metadata.
The storage deposit on the output is left unchanged. If the size of the document increased,
the amount should be increased manually.

NOTE: this does *not* publish the updated Alias Output.

**Kind**: static method of [<code>StardustIdentityClientExt</code>](#StardustIdentityClientExt)  

| Param | Type |
| --- | --- |
| client | <code>IStardustIdentityClient</code> | 
| document | [<code>StardustDocument</code>](#StardustDocument) | 

<a name="StardustIdentityClientExt.deactivateDidOutput"></a>

### StardustIdentityClientExt.deactivateDidOutput(client, did) ⇒ <code>Promise.&lt;IAliasOutput&gt;</code>
Removes the DID document from the state metadata of its Alias Output,
effectively deactivating it. The storage deposit on the output is left unchanged,
and should be reallocated manually.

Deactivating does not destroy the output. Hence, it can be re-activated by publishing
an update containing a DID document.

NOTE: this does *not* publish the updated Alias Output.

**Kind**: static method of [<code>StardustIdentityClientExt</code>](#StardustIdentityClientExt)  

| Param | Type |
| --- | --- |
| client | <code>IStardustIdentityClient</code> | 
| did | [<code>StardustDID</code>](#StardustDID) | 

<a name="StardustIdentityClientExt.resolveDid"></a>

### StardustIdentityClientExt.resolveDid(client, did) ⇒ [<code>Promise.&lt;StardustDocument&gt;</code>](#StardustDocument)
Resolve a [StardustDocument](#StardustDocument). Returns an empty, deactivated document if the state metadata
of the Alias Output is empty.

**Kind**: static method of [<code>StardustIdentityClientExt</code>](#StardustIdentityClientExt)  

| Param | Type |
| --- | --- |
| client | <code>IStardustIdentityClient</code> | 
| did | [<code>StardustDID</code>](#StardustDID) | 

<a name="StardustIdentityClientExt.resolveDidOutput"></a>

### StardustIdentityClientExt.resolveDidOutput(client, did) ⇒ <code>Promise.&lt;IAliasOutput&gt;</code>
Fetches the `IAliasOutput` associated with the given DID.

**Kind**: static method of [<code>StardustIdentityClientExt</code>](#StardustIdentityClientExt)  

| Param | Type |
| --- | --- |
| client | <code>IStardustIdentityClient</code> | 
| did | [<code>StardustDID</code>](#StardustDID) | 

<a name="StardustService"></a>

## StardustService
A `Service` adhering to the IOTA UTXO DID method specification.

**Kind**: global class  

* [StardustService](#StardustService)
    * [new StardustService(service)](#new_StardustService_new)
    * _instance_
        * [.id()](#StardustService+id) ⇒ [<code>StardustDIDUrl</code>](#StardustDIDUrl)
        * [.type()](#StardustService+type) ⇒ <code>Array.&lt;string&gt;</code>
        * [.serviceEndpoint()](#StardustService+serviceEndpoint) ⇒ <code>string</code> \| <code>Array.&lt;string&gt;</code> \| <code>Map.&lt;string, Array.&lt;string&gt;&gt;</code>
        * [.properties()](#StardustService+properties) ⇒ <code>Map.&lt;string, any&gt;</code>
        * [.toJSON()](#StardustService+toJSON) ⇒ <code>any</code>
        * [.clone()](#StardustService+clone) ⇒ [<code>StardustService</code>](#StardustService)
    * _static_
        * [.fromJSON(json)](#StardustService.fromJSON) ⇒ [<code>StardustService</code>](#StardustService)

<a name="new_StardustService_new"></a>

### new StardustService(service)

| Param | Type |
| --- | --- |
| service | <code>IStardustService</code> | 

<a name="StardustService+id"></a>

### stardustService.id() ⇒ [<code>StardustDIDUrl</code>](#StardustDIDUrl)
Returns a copy of the `Service` id.

**Kind**: instance method of [<code>StardustService</code>](#StardustService)  
<a name="StardustService+type"></a>

### stardustService.type() ⇒ <code>Array.&lt;string&gt;</code>
Returns a copy of the `Service` type.

**Kind**: instance method of [<code>StardustService</code>](#StardustService)  
<a name="StardustService+serviceEndpoint"></a>

### stardustService.serviceEndpoint() ⇒ <code>string</code> \| <code>Array.&lt;string&gt;</code> \| <code>Map.&lt;string, Array.&lt;string&gt;&gt;</code>
Returns a copy of the `Service` endpoint.

**Kind**: instance method of [<code>StardustService</code>](#StardustService)  
<a name="StardustService+properties"></a>

### stardustService.properties() ⇒ <code>Map.&lt;string, any&gt;</code>
Returns a copy of the custom properties on the `Service`.

**Kind**: instance method of [<code>StardustService</code>](#StardustService)  
<a name="StardustService+toJSON"></a>

### stardustService.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>StardustService</code>](#StardustService)  
<a name="StardustService+clone"></a>

### stardustService.clone() ⇒ [<code>StardustService</code>](#StardustService)
Deep clones the object.

**Kind**: instance method of [<code>StardustService</code>](#StardustService)  
<a name="StardustService.fromJSON"></a>

### StardustService.fromJSON(json) ⇒ [<code>StardustService</code>](#StardustService)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>StardustService</code>](#StardustService)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="StardustVerificationMethod"></a>

## StardustVerificationMethod
**Kind**: global class  

* [StardustVerificationMethod](#StardustVerificationMethod)
    * [new StardustVerificationMethod(did, keyType, publicKey, fragment)](#new_StardustVerificationMethod_new)
    * _instance_
        * [.id()](#StardustVerificationMethod+id) ⇒ [<code>StardustDIDUrl</code>](#StardustDIDUrl)
        * [.controller()](#StardustVerificationMethod+controller) ⇒ [<code>StardustDID</code>](#StardustDID)
        * [.setController(did)](#StardustVerificationMethod+setController)
        * [.type()](#StardustVerificationMethod+type) ⇒ [<code>MethodType</code>](#MethodType)
        * [.data()](#StardustVerificationMethod+data) ⇒ [<code>MethodData</code>](#MethodData)
        * [.toJSON()](#StardustVerificationMethod+toJSON) ⇒ <code>any</code>
        * [.clone()](#StardustVerificationMethod+clone) ⇒ [<code>StardustVerificationMethod</code>](#StardustVerificationMethod)
    * _static_
        * [.fromJSON(json)](#StardustVerificationMethod.fromJSON) ⇒ [<code>StardustVerificationMethod</code>](#StardustVerificationMethod)

<a name="new_StardustVerificationMethod_new"></a>

### new StardustVerificationMethod(did, keyType, publicKey, fragment)
Creates a new `StardustVerificationMethod` from the given `did` and public key.


| Param | Type |
| --- | --- |
| did | [<code>StardustDID</code>](#StardustDID) | 
| keyType | <code>number</code> | 
| publicKey | <code>Uint8Array</code> | 
| fragment | <code>string</code> | 

<a name="StardustVerificationMethod+id"></a>

### stardustVerificationMethod.id() ⇒ [<code>StardustDIDUrl</code>](#StardustDIDUrl)
Returns a reference to the `StardustVerificationMethod` id.

**Kind**: instance method of [<code>StardustVerificationMethod</code>](#StardustVerificationMethod)  
<a name="StardustVerificationMethod+controller"></a>

### stardustVerificationMethod.controller() ⇒ [<code>StardustDID</code>](#StardustDID)
Returns a copy of the `controller` `DID` of the `StardustVerificationMethod`.

**Kind**: instance method of [<code>StardustVerificationMethod</code>](#StardustVerificationMethod)  
<a name="StardustVerificationMethod+setController"></a>

### stardustVerificationMethod.setController(did)
Sets the `controller` `DID` of the `StardustVerificationMethod`.

**Kind**: instance method of [<code>StardustVerificationMethod</code>](#StardustVerificationMethod)  

| Param | Type |
| --- | --- |
| did | [<code>StardustDID</code>](#StardustDID) | 

<a name="StardustVerificationMethod+type"></a>

### stardustVerificationMethod.type() ⇒ [<code>MethodType</code>](#MethodType)
Returns a copy of the `StardustVerificationMethod` type.

**Kind**: instance method of [<code>StardustVerificationMethod</code>](#StardustVerificationMethod)  
<a name="StardustVerificationMethod+data"></a>

### stardustVerificationMethod.data() ⇒ [<code>MethodData</code>](#MethodData)
Returns a copy of the `StardustVerificationMethod` public key data.

**Kind**: instance method of [<code>StardustVerificationMethod</code>](#StardustVerificationMethod)  
<a name="StardustVerificationMethod+toJSON"></a>

### stardustVerificationMethod.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>StardustVerificationMethod</code>](#StardustVerificationMethod)  
<a name="StardustVerificationMethod+clone"></a>

### stardustVerificationMethod.clone() ⇒ [<code>StardustVerificationMethod</code>](#StardustVerificationMethod)
Deep clones the object.

**Kind**: instance method of [<code>StardustVerificationMethod</code>](#StardustVerificationMethod)  
<a name="StardustVerificationMethod.fromJSON"></a>

### StardustVerificationMethod.fromJSON(json) ⇒ [<code>StardustVerificationMethod</code>](#StardustVerificationMethod)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>StardustVerificationMethod</code>](#StardustVerificationMethod)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="StorageTestSuite"></a>

## StorageTestSuite
A test suite for the `Storage` interface.

This module contains a set of tests that a correct storage implementation
should pass. Note that not every edge case is tested.

Tests usually rely on multiple interface methods being implemented, so they should only
be run on a fully implemented version. That's why there is not a single test case for every
interface method.

**Kind**: global class  

* [StorageTestSuite](#StorageTestSuite)
    * [.didCreateGenerateKeyTest(storage)](#StorageTestSuite.didCreateGenerateKeyTest) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.didCreatePrivateKeyTest(storage)](#StorageTestSuite.didCreatePrivateKeyTest) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.didListTest(storage)](#StorageTestSuite.didListTest) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.didPurgeTest(storage)](#StorageTestSuite.didPurgeTest) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.keyGenerateTest(storage)](#StorageTestSuite.keyGenerateTest) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.keyDeleteTest(storage)](#StorageTestSuite.keyDeleteTest) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.keyInsertTest(storage)](#StorageTestSuite.keyInsertTest) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.keySignEd25519Test(storage)](#StorageTestSuite.keySignEd25519Test) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.encryptionTest(alice_storage, bob_storage)](#StorageTestSuite.encryptionTest) ⇒ <code>Promise.&lt;void&gt;</code>

<a name="StorageTestSuite.didCreateGenerateKeyTest"></a>

### StorageTestSuite.didCreateGenerateKeyTest(storage) ⇒ <code>Promise.&lt;void&gt;</code>
**Kind**: static method of [<code>StorageTestSuite</code>](#StorageTestSuite)  

| Param | Type |
| --- | --- |
| storage | <code>Storage</code> | 

<a name="StorageTestSuite.didCreatePrivateKeyTest"></a>

### StorageTestSuite.didCreatePrivateKeyTest(storage) ⇒ <code>Promise.&lt;void&gt;</code>
**Kind**: static method of [<code>StorageTestSuite</code>](#StorageTestSuite)  

| Param | Type |
| --- | --- |
| storage | <code>Storage</code> | 

<a name="StorageTestSuite.didListTest"></a>

### StorageTestSuite.didListTest(storage) ⇒ <code>Promise.&lt;void&gt;</code>
**Kind**: static method of [<code>StorageTestSuite</code>](#StorageTestSuite)  

| Param | Type |
| --- | --- |
| storage | <code>Storage</code> | 

<a name="StorageTestSuite.didPurgeTest"></a>

### StorageTestSuite.didPurgeTest(storage) ⇒ <code>Promise.&lt;void&gt;</code>
**Kind**: static method of [<code>StorageTestSuite</code>](#StorageTestSuite)  

| Param | Type |
| --- | --- |
| storage | <code>Storage</code> | 

<a name="StorageTestSuite.keyGenerateTest"></a>

### StorageTestSuite.keyGenerateTest(storage) ⇒ <code>Promise.&lt;void&gt;</code>
**Kind**: static method of [<code>StorageTestSuite</code>](#StorageTestSuite)  

| Param | Type |
| --- | --- |
| storage | <code>Storage</code> | 

<a name="StorageTestSuite.keyDeleteTest"></a>

### StorageTestSuite.keyDeleteTest(storage) ⇒ <code>Promise.&lt;void&gt;</code>
**Kind**: static method of [<code>StorageTestSuite</code>](#StorageTestSuite)  

| Param | Type |
| --- | --- |
| storage | <code>Storage</code> | 

<a name="StorageTestSuite.keyInsertTest"></a>

### StorageTestSuite.keyInsertTest(storage) ⇒ <code>Promise.&lt;void&gt;</code>
**Kind**: static method of [<code>StorageTestSuite</code>](#StorageTestSuite)  

| Param | Type |
| --- | --- |
| storage | <code>Storage</code> | 

<a name="StorageTestSuite.keySignEd25519Test"></a>

### StorageTestSuite.keySignEd25519Test(storage) ⇒ <code>Promise.&lt;void&gt;</code>
**Kind**: static method of [<code>StorageTestSuite</code>](#StorageTestSuite)  

| Param | Type |
| --- | --- |
| storage | <code>Storage</code> | 

<a name="StorageTestSuite.encryptionTest"></a>

### StorageTestSuite.encryptionTest(alice_storage, bob_storage) ⇒ <code>Promise.&lt;void&gt;</code>
**Kind**: static method of [<code>StorageTestSuite</code>](#StorageTestSuite)  

| Param | Type |
| --- | --- |
| alice_storage | <code>Storage</code> | 
| bob_storage | <code>Storage</code> | 

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
**Kind**: global class  

* [VerificationMethod](#VerificationMethod)
    * [new VerificationMethod(did, keyType, publicKey, fragment)](#new_VerificationMethod_new)
    * _instance_
        * [.id()](#VerificationMethod+id) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.controller()](#VerificationMethod+controller) ⇒ [<code>IotaDID</code>](#IotaDID)
        * [.setController(did)](#VerificationMethod+setController)
        * [.type()](#VerificationMethod+type) ⇒ [<code>MethodType</code>](#MethodType)
        * [.data()](#VerificationMethod+data) ⇒ [<code>MethodData</code>](#MethodData)
        * [.toJSON()](#VerificationMethod+toJSON) ⇒ <code>any</code>
        * [.clone()](#VerificationMethod+clone) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
    * _static_
        * [.fromJSON(json)](#VerificationMethod.fromJSON) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)

<a name="new_VerificationMethod_new"></a>

### new VerificationMethod(did, keyType, publicKey, fragment)
Creates a new `VerificationMethod` from the given `did` and public key.


| Param | Type |
| --- | --- |
| did | [<code>IotaDID</code>](#IotaDID) | 
| keyType | <code>number</code> | 
| publicKey | <code>Uint8Array</code> | 
| fragment | <code>string</code> | 

<a name="VerificationMethod+id"></a>

### verificationMethod.id() ⇒ [<code>DIDUrl</code>](#DIDUrl)
Returns a copy of the `id` `DIDUrl` of the `VerificationMethod`.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod+controller"></a>

### verificationMethod.controller() ⇒ [<code>IotaDID</code>](#IotaDID)
Returns a copy of the `controller` `DID` of the `VerificationMethod`.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod+setController"></a>

### verificationMethod.setController(did)
Sets the `controller` `DID` of the `VerificationMethod` object.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  

| Param | Type |
| --- | --- |
| did | [<code>IotaDID</code>](#IotaDID) | 

<a name="VerificationMethod+type"></a>

### verificationMethod.type() ⇒ [<code>MethodType</code>](#MethodType)
Returns a copy of the `VerificationMethod` type.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod+data"></a>

### verificationMethod.data() ⇒ [<code>MethodData</code>](#MethodData)
Returns a copy of the `VerificationMethod` public key data.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  
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

<a name="StateMetadataEncoding"></a>

## StateMetadataEncoding
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
<a name="MethodRelationship"></a>

## MethodRelationship
**Kind**: global variable  
<a name="DIDType"></a>

## DIDType
Supported types representing a DID that can be generated by the storage interface.

**Kind**: global variable  
<a name="KeyType"></a>

## KeyType
**Kind**: global variable  
<a name="DIDMessageEncoding"></a>

## DIDMessageEncoding
**Kind**: global variable  
<a name="start"></a>

## start()
Initializes the console error panic hook for better error messages

**Kind**: global function  

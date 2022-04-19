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
<dt><a href="#AutoSave">AutoSave</a></dt>
<dd></dd>
<dt><a href="#ChainState">ChainState</a></dt>
<dd></dd>
<dt><a href="#Client">Client</a></dt>
<dd></dd>
<dt><a href="#Credential">Credential</a></dt>
<dd></dd>
<dt><a href="#CredentialValidationOptions">CredentialValidationOptions</a></dt>
<dd><p>Options to declare validation criteria when validating credentials.</p>
</dd>
<dt><a href="#CredentialValidator">CredentialValidator</a></dt>
<dd></dd>
<dt><a href="#DID">DID</a></dt>
<dd></dd>
<dt><a href="#DIDUrl">DIDUrl</a></dt>
<dd></dd>
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
<dt><a href="#ExplorerUrl">ExplorerUrl</a></dt>
<dd></dd>
<dt><a href="#IntegrationChainHistory">IntegrationChainHistory</a></dt>
<dd></dd>
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
<dt><a href="#Service">Service</a></dt>
<dd><p>A DID Document Service used to enable trusted interactions associated
with a DID subject.</p>
<p>See: <a href="https://www.w3.org/TR/did-core/#services">https://www.w3.org/TR/did-core/#services</a></p>
</dd>
<dt><a href="#Signature">Signature</a></dt>
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
<dt><a href="#DIDMessageEncoding">DIDMessageEncoding</a></dt>
<dd></dd>
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
<dt><a href="#KeyType">KeyType</a></dt>
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
    * [.attachMethodRelationships(options)](#Account+attachMethodRelationships) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.createMethod(options)](#Account+createMethod) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.detachMethodRelationships(options)](#Account+detachMethodRelationships) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.setAlsoKnownAs(options)](#Account+setAlsoKnownAs) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.did()](#Account+did) ⇒ [<code>DID</code>](#DID)
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
    * [.deleteMethod(options)](#Account+deleteMethod) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.deleteService(options)](#Account+deleteService) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.setController(options)](#Account+setController) ⇒ <code>Promise.&lt;void&gt;</code>
    * [.createService(options)](#Account+createService) ⇒ <code>Promise.&lt;void&gt;</code>

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

<a name="Account+setAlsoKnownAs"></a>

### account.setAlsoKnownAs(options) ⇒ <code>Promise.&lt;void&gt;</code>
Sets the `alsoKnownAs` property in the DID document.

**Kind**: instance method of [<code>Account</code>](#Account)  

| Param | Type |
| --- | --- |
| options | <code>SetAlsoKnownAsOptions</code> | 

<a name="Account+did"></a>

### account.did() ⇒ [<code>DID</code>](#DID)
Returns the [DID](#DID) of the managed identity.

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

<a name="Account+setController"></a>

### account.setController(options) ⇒ <code>Promise.&lt;void&gt;</code>
Sets the controllers of the DID document.

**Kind**: instance method of [<code>Account</code>](#Account)  

| Param | Type |
| --- | --- |
| options | <code>SetControllerOptions</code> | 

<a name="Account+createService"></a>

### account.createService(options) ⇒ <code>Promise.&lt;void&gt;</code>
Adds a new Service to the DID Document.

**Kind**: instance method of [<code>Account</code>](#Account)  

| Param | Type |
| --- | --- |
| options | <code>CreateServiceOptions</code> | 

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
| did | [<code>DID</code>](#DID) | 

<a name="AccountBuilder+createIdentity"></a>

### accountBuilder.createIdentity(identity_setup) ⇒ [<code>Promise.&lt;Account&gt;</code>](#Account)
Creates a new identity based on the builder configuration and returns
an [Account](#Account) object to manage it.

The identity is stored locally in the `Storage`. The DID network is automatically determined
by the [Client](#Client) used to publish it.

**Kind**: instance method of [<code>AccountBuilder</code>](#AccountBuilder)  
**See**: [IdentitySetup](IdentitySetup) to customize the identity creation.  

| Param | Type |
| --- | --- |
| identity_setup | <code>IdentitySetup</code> \| <code>undefined</code> | 

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
        * [.fromJSON(json_value)](#AutoSave.fromJSON) ⇒ [<code>AutoSave</code>](#AutoSave)

<a name="AutoSave+toJSON"></a>

### autoSave.toJSON() ⇒ <code>any</code>
Serializes `AutoSave` as a JSON object.

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

### AutoSave.fromJSON(json_value) ⇒ [<code>AutoSave</code>](#AutoSave)
Deserializes `AutoSave` from a JSON object.

**Kind**: static method of [<code>AutoSave</code>](#AutoSave)  

| Param | Type |
| --- | --- |
| json_value | <code>any</code> | 

<a name="ChainState"></a>

## ChainState
**Kind**: global class  

* [ChainState](#ChainState)
    * _instance_
        * [.toJSON()](#ChainState+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json_value)](#ChainState.fromJSON) ⇒ [<code>ChainState</code>](#ChainState)

<a name="ChainState+toJSON"></a>

### chainState.toJSON() ⇒ <code>any</code>
Serializes a `ChainState` object as a JSON object.

**Kind**: instance method of [<code>ChainState</code>](#ChainState)  
<a name="ChainState.fromJSON"></a>

### ChainState.fromJSON(json_value) ⇒ [<code>ChainState</code>](#ChainState)
Deserializes a JSON object as `ChainState`.

**Kind**: static method of [<code>ChainState</code>](#ChainState)  

| Param | Type |
| --- | --- |
| json_value | <code>any</code> | 

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
Publishes an `IotaDocument` to the Tangle.

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

<a name="Client+resolve"></a>

### client.resolve(did) ⇒ [<code>Promise.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
Fetch the DID document specified by the given `DID`.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| did | [<code>DID</code>](#DID) \| <code>string</code> | 

<a name="Client+resolveHistory"></a>

### client.resolveHistory(did) ⇒ [<code>Promise.&lt;DocumentHistory&gt;</code>](#DocumentHistory)
Returns the message history of the given DID.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| did | [<code>DID</code>](#DID) \| <code>string</code> | 

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

<a name="Credential"></a>

## Credential
**Kind**: global class  

* [Credential](#Credential)
    * _instance_
        * [.toJSON()](#Credential+toJSON) ⇒ <code>any</code>
        * [.clone()](#Credential+clone) ⇒ [<code>Credential</code>](#Credential)
    * _static_
        * [.extend(value)](#Credential.extend) ⇒ [<code>Credential</code>](#Credential)
        * [.issue(issuer_doc, subject_data, credential_type, credential_id)](#Credential.issue) ⇒ [<code>Credential</code>](#Credential)
        * [.fromJSON(json)](#Credential.fromJSON) ⇒ [<code>Credential</code>](#Credential)

<a name="Credential+toJSON"></a>

### credential.toJSON() ⇒ <code>any</code>
Serializes a `Credential` object as a JSON object.

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+clone"></a>

### credential.clone() ⇒ [<code>Credential</code>](#Credential)
Deep clones the object.

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential.extend"></a>

### Credential.extend(value) ⇒ [<code>Credential</code>](#Credential)
**Kind**: static method of [<code>Credential</code>](#Credential)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="Credential.issue"></a>

### Credential.issue(issuer_doc, subject_data, credential_type, credential_id) ⇒ [<code>Credential</code>](#Credential)
**Kind**: static method of [<code>Credential</code>](#Credential)  

| Param | Type |
| --- | --- |
| issuer_doc | [<code>Document</code>](#Document) | 
| subject_data | <code>any</code> | 
| credential_type | <code>string</code> \| <code>undefined</code> | 
| credential_id | <code>string</code> \| <code>undefined</code> | 

<a name="Credential.fromJSON"></a>

### Credential.fromJSON(json) ⇒ [<code>Credential</code>](#Credential)
Deserializes a `Credential` object from a JSON object.

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
Serializes a `CredentialValidationOptions` as a JSON object.

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
Deserializes a `CredentialValidationOptions` from a JSON object.

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
| trusted_issuers | [<code>Array.&lt;Document&gt;</code>](#Document) \| [<code>Array.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument) | 
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

<a name="DID"></a>

## DID
**Kind**: global class  

* [DID](#DID)
    * [new DID(public_key, network)](#new_DID_new)
    * _instance_
        * [.networkName](#DID+networkName) ⇒ <code>string</code>
        * [.network()](#DID+network) ⇒ [<code>Network</code>](#Network)
        * [.tag()](#DID+tag) ⇒ <code>string</code>
        * [.join(segment)](#DID+join) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.toUrl()](#DID+toUrl) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.intoUrl()](#DID+intoUrl) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.toString()](#DID+toString) ⇒ <code>string</code>
        * [.toJSON()](#DID+toJSON) ⇒ <code>any</code>
        * [.clone()](#DID+clone) ⇒ [<code>DID</code>](#DID)
    * _static_
        * [.parse(input)](#DID.parse) ⇒ [<code>DID</code>](#DID)
        * [.fromJSON(json_value)](#DID.fromJSON) ⇒ [<code>DID</code>](#DID)

<a name="new_DID_new"></a>

### new DID(public_key, network)
Creates a new `DID` from a public key.


| Param | Type |
| --- | --- |
| public_key | <code>Uint8Array</code> | 
| network | <code>string</code> \| <code>undefined</code> | 

<a name="DID+networkName"></a>

### did.networkName ⇒ <code>string</code>
Returns the IOTA tangle network of the `DID`.

**Kind**: instance property of [<code>DID</code>](#DID)  
<a name="DID+network"></a>

### did.network() ⇒ [<code>Network</code>](#Network)
Returns the IOTA tangle network of the `DID`.

**Kind**: instance method of [<code>DID</code>](#DID)  
<a name="DID+tag"></a>

### did.tag() ⇒ <code>string</code>
Returns a copy of the unique tag of the `DID`.

**Kind**: instance method of [<code>DID</code>](#DID)  
<a name="DID+join"></a>

### did.join(segment) ⇒ [<code>DIDUrl</code>](#DIDUrl)
Construct a new `DIDUrl` by joining with a relative DID Url string.

**Kind**: instance method of [<code>DID</code>](#DID)  

| Param | Type |
| --- | --- |
| segment | <code>string</code> | 

<a name="DID+toUrl"></a>

### did.toUrl() ⇒ [<code>DIDUrl</code>](#DIDUrl)
Clones the `DID` into a `DIDUrl`.

**Kind**: instance method of [<code>DID</code>](#DID)  
<a name="DID+intoUrl"></a>

### did.intoUrl() ⇒ [<code>DIDUrl</code>](#DIDUrl)
Converts the `DID` into a `DIDUrl`.

**Kind**: instance method of [<code>DID</code>](#DID)  
<a name="DID+toString"></a>

### did.toString() ⇒ <code>string</code>
Returns the `DID` as a string.

**Kind**: instance method of [<code>DID</code>](#DID)  
<a name="DID+toJSON"></a>

### did.toJSON() ⇒ <code>any</code>
Serializes a `DID` as a JSON object.

**Kind**: instance method of [<code>DID</code>](#DID)  
<a name="DID+clone"></a>

### did.clone() ⇒ [<code>DID</code>](#DID)
Deep clones the object.

**Kind**: instance method of [<code>DID</code>](#DID)  
<a name="DID.parse"></a>

### DID.parse(input) ⇒ [<code>DID</code>](#DID)
Parses a `DID` from the input string.

**Kind**: static method of [<code>DID</code>](#DID)  

| Param | Type |
| --- | --- |
| input | <code>string</code> | 

<a name="DID.fromJSON"></a>

### DID.fromJSON(json_value) ⇒ [<code>DID</code>](#DID)
Deserializes a JSON object as `DID`.

**Kind**: static method of [<code>DID</code>](#DID)  

| Param | Type |
| --- | --- |
| json_value | <code>any</code> | 

<a name="DIDUrl"></a>

## DIDUrl
**Kind**: global class  

* [DIDUrl](#DIDUrl)
    * _instance_
        * [.did()](#DIDUrl+did) ⇒ [<code>DID</code>](#DID)
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

<a name="DIDUrl+did"></a>

### didUrl.did() ⇒ [<code>DID</code>](#DID)
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
Append a string representing a path, query, and/or fragment to this `DIDUrl`.

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
Serializes a `DIDUrl` as a JSON object.

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
        * ~~[.id()](#DiffMessage+id) ⇒ [<code>DID</code>](#DID)~~
        * ~~[.did()](#DiffMessage+did) ⇒ [<code>DID</code>](#DID)~~
        * ~~[.diff()](#DiffMessage+diff) ⇒ <code>string</code>~~
        * ~~[.messageId()](#DiffMessage+messageId) ⇒ <code>string</code>~~
        * ~~[.setMessageId(message_id)](#DiffMessage+setMessageId)~~
        * ~~[.previousMessageId()](#DiffMessage+previousMessageId) ⇒ <code>string</code>~~
        * ~~[.setPreviousMessageId(message_id)](#DiffMessage+setPreviousMessageId)~~
        * ~~[.proof()](#DiffMessage+proof) ⇒ [<code>Proof</code>](#Proof) \| <code>undefined</code>~~
        * ~~[.merge(document)](#DiffMessage+merge) ⇒ [<code>Document</code>](#Document)~~
        * ~~[.toJSON()](#DiffMessage+toJSON) ⇒ <code>any</code>~~
        * [.clone()](#DiffMessage+clone) ⇒ [<code>DiffMessage</code>](#DiffMessage)
    * _static_
        * ~~[.fromJSON(json)](#DiffMessage.fromJSON) ⇒ [<code>DiffMessage</code>](#DiffMessage)~~

<a name="DiffMessage+id"></a>

### ~~diffMessage.id() ⇒ [<code>DID</code>](#DID)~~
***Deprecated***

Returns the DID of the associated DID Document.

NOTE: clones the data.

**Kind**: instance method of [<code>DiffMessage</code>](#DiffMessage)  
<a name="DiffMessage+did"></a>

### ~~diffMessage.did() ⇒ [<code>DID</code>](#DID)~~
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

### ~~diffMessage.toJSON() ⇒ <code>any</code>~~
***Deprecated***

Serializes a `DiffMessage` as a JSON object.

**Kind**: instance method of [<code>DiffMessage</code>](#DiffMessage)  
<a name="DiffMessage+clone"></a>

### diffMessage.clone() ⇒ [<code>DiffMessage</code>](#DiffMessage)
Deep clones the object.

**Kind**: instance method of [<code>DiffMessage</code>](#DiffMessage)  
<a name="DiffMessage.fromJSON"></a>

### ~~DiffMessage.fromJSON(json) ⇒ [<code>DiffMessage</code>](#DiffMessage)~~
***Deprecated***

Deserializes a `DiffMessage` from a JSON object.

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
        * [.id()](#Document+id) ⇒ [<code>DID</code>](#DID)
        * [.setController(controllers)](#Document+setController)
        * [.controller()](#Document+controller) ⇒ [<code>Array.&lt;DID&gt;</code>](#DID)
        * [.setAlsoKnownAs(urls)](#Document+setAlsoKnownAs)
        * [.alsoKnownAs()](#Document+alsoKnownAs) ⇒ <code>Array.&lt;string&gt;</code>
        * [.setPropertyUnchecked(key, value)](#Document+setPropertyUnchecked)
        * [.properties()](#Document+properties) ⇒ <code>Map.&lt;string, any&gt;</code>
        * [.service()](#Document+service) ⇒ [<code>Array.&lt;Service&gt;</code>](#Service)
        * [.insertService(service)](#Document+insertService) ⇒ <code>boolean</code>
        * [.removeService(did)](#Document+removeService)
        * [.methods()](#Document+methods) ⇒ [<code>Array.&lt;VerificationMethod&gt;</code>](#VerificationMethod)
        * [.insertMethod(method, scope)](#Document+insertMethod)
        * [.removeMethod(did)](#Document+removeMethod)
        * [.defaultSigningMethod()](#Document+defaultSigningMethod) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
        * [.resolveMethod(query, scope)](#Document+resolveMethod) ⇒ [<code>VerificationMethod</code>](#VerificationMethod) \| <code>undefined</code>
        * [.resolveSigningMethod(query)](#Document+resolveSigningMethod) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
        * [.attachMethodRelationship(did_url, relationship)](#Document+attachMethodRelationship) ⇒ <code>boolean</code>
        * [.detachMethodRelationship(did_url, relationship)](#Document+detachMethodRelationship) ⇒ <code>boolean</code>
        * [.signSelf(key_pair, method_query)](#Document+signSelf)
        * [.signDocument(document, key_pair, method_query)](#Document+signDocument)
        * [.signCredential(data, privateKey, methodQuery, options)](#Document+signCredential) ⇒ [<code>Credential</code>](#Credential)
        * [.signPresentation(data, privateKey, methodQuery, options)](#Document+signPresentation) ⇒ [<code>Presentation</code>](#Presentation)
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
        * [.proof()](#Document+proof) ⇒ [<code>Proof</code>](#Proof) \| <code>undefined</code>
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

### document.id() ⇒ [<code>DID</code>](#DID)
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
| controllers | [<code>DID</code>](#DID) \| [<code>Array.&lt;DID&gt;</code>](#DID) \| <code>null</code> | 

<a name="Document+controller"></a>

### document.controller() ⇒ [<code>Array.&lt;DID&gt;</code>](#DID)
Returns a list of document controllers.

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
Returns a set of the document's `alsoKnownAs`.

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

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| service | [<code>Service</code>](#Service) | 

<a name="Document+removeService"></a>

### document.removeService(did)
Remove a [Service](#Service) identified by the given [DIDUrl](#DIDUrl) from the document.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| did | [<code>DIDUrl</code>](#DIDUrl) | 

<a name="Document+methods"></a>

### document.methods() ⇒ [<code>Array.&lt;VerificationMethod&gt;</code>](#VerificationMethod)
Returns a list of all [VerificationMethod](#VerificationMethod) in the DID Document.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+insertMethod"></a>

### document.insertMethod(method, scope)
Adds a new Verification Method to the DID Document.

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
Returns a copy of the first `VerificationMethod` with an `id` property
matching the provided `query`.

Throws an error if the method is not found.

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

### document.attachMethodRelationship(did_url, relationship) ⇒ <code>boolean</code>
Attaches the relationship to the given method, if the method exists.

Note: The method needs to be in the set of verification methods,
so it cannot be an embedded one.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| did_url | [<code>DIDUrl</code>](#DIDUrl) | 
| relationship | <code>number</code> | 

<a name="Document+detachMethodRelationship"></a>

### document.detachMethodRelationship(did_url, relationship) ⇒ <code>boolean</code>
Detaches the given relationship from the given method, if the method exists.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| did_url | [<code>DIDUrl</code>](#DIDUrl) | 
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
verification method. See [Document.verifyDocument](Document.verifyDocument).

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| document | [<code>Document</code>](#Document) | 
| key_pair | [<code>KeyPair</code>](#KeyPair) | 
| method_query | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 

<a name="Document+signCredential"></a>

### document.signCredential(data, privateKey, methodQuery, options) ⇒ [<code>Credential</code>](#Credential)
Creates a signature for the given `Credential` with the specified DID Document
Verification Method.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| data | <code>any</code> | 
| privateKey | <code>Uint8Array</code> | 
| methodQuery | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| options | [<code>ProofOptions</code>](#ProofOptions) | 

<a name="Document+signPresentation"></a>

### document.signPresentation(data, privateKey, methodQuery, options) ⇒ [<code>Presentation</code>](#Presentation)
Creates a signature for the given `Presentation` with the specified DID Document
Verification Method.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| data | <code>any</code> | 
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

<a name="Document+proof"></a>

### document.proof() ⇒ [<code>Proof</code>](#Proof) \| <code>undefined</code>
Returns a copy of the proof.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+toJSON"></a>

### document.toJSON() ⇒ <code>any</code>
Serializes a `Document` as a JSON object.

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
Deserializes a `Document` from a JSON object.

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
Serializes `DocumentHistory` as a JSON object.

**Kind**: instance method of [<code>DocumentHistory</code>](#DocumentHistory)  
<a name="DocumentHistory+clone"></a>

### documentHistory.clone() ⇒ [<code>DocumentHistory</code>](#DocumentHistory)
Deep clones the object.

**Kind**: instance method of [<code>DocumentHistory</code>](#DocumentHistory)  
<a name="DocumentHistory.fromJSON"></a>

### DocumentHistory.fromJSON(json) ⇒ [<code>DocumentHistory</code>](#DocumentHistory)
Deserializes `DocumentHistory` from a JSON object.

**Kind**: static method of [<code>DocumentHistory</code>](#DocumentHistory)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="DocumentMetadata"></a>

## DocumentMetadata
Additional attributes related to an IOTA DID Document.

**Kind**: global class  

* [DocumentMetadata](#DocumentMetadata)
    * [.previousMessageId](#DocumentMetadata+previousMessageId) ⇒ <code>string</code>
    * [.created()](#DocumentMetadata+created) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
    * [.updated()](#DocumentMetadata+updated) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
    * [.clone()](#DocumentMetadata+clone) ⇒ [<code>DocumentMetadata</code>](#DocumentMetadata)

<a name="DocumentMetadata+previousMessageId"></a>

### documentMetadata.previousMessageId ⇒ <code>string</code>
**Kind**: instance property of [<code>DocumentMetadata</code>](#DocumentMetadata)  
<a name="DocumentMetadata+created"></a>

### documentMetadata.created() ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
Returns a copy of the timestamp of when the DID document was created.

**Kind**: instance method of [<code>DocumentMetadata</code>](#DocumentMetadata)  
<a name="DocumentMetadata+updated"></a>

### documentMetadata.updated() ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
Returns a copy of the timestamp of the last DID document update.

**Kind**: instance method of [<code>DocumentMetadata</code>](#DocumentMetadata)  
<a name="DocumentMetadata+clone"></a>

### documentMetadata.clone() ⇒ [<code>DocumentMetadata</code>](#DocumentMetadata)
Deep clones the object.

**Kind**: instance method of [<code>DocumentMetadata</code>](#DocumentMetadata)  
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
Serializes a `Duration` as a JSON object.

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
Deserializes a `Duration` from a JSON object.

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

NOTE: this differs from [Document.signData](Document.signData) which uses JCS
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

NOTE: this differs from [Document.verifyData](Document.verifyData) which uses JCS
to canonicalize JSON messages.

**Kind**: static method of [<code>Ed25519</code>](#Ed25519)  

| Param | Type |
| --- | --- |
| message | <code>Uint8Array</code> | 
| signature | <code>Uint8Array</code> | 
| publicKey | <code>Uint8Array</code> | 

<a name="ExplorerUrl"></a>

## ExplorerUrl
**Kind**: global class  

* [ExplorerUrl](#ExplorerUrl)
    * _instance_
        * [.messageUrl(message_id)](#ExplorerUrl+messageUrl) ⇒ <code>string</code>
        * [.resolverUrl(did)](#ExplorerUrl+resolverUrl) ⇒ <code>string</code>
        * [.toString()](#ExplorerUrl+toString) ⇒ <code>string</code>
    * _static_
        * [.parse(url)](#ExplorerUrl.parse) ⇒ [<code>ExplorerUrl</code>](#ExplorerUrl)
        * [.mainnet()](#ExplorerUrl.mainnet) ⇒ [<code>ExplorerUrl</code>](#ExplorerUrl)
        * [.devnet()](#ExplorerUrl.devnet) ⇒ [<code>ExplorerUrl</code>](#ExplorerUrl)

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
| did | [<code>DID</code>](#DID) \| <code>string</code> | 

<a name="ExplorerUrl+toString"></a>

### explorerUrl.toString() ⇒ <code>string</code>
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
        * [.toJSON()](#KeyLocation+toJSON) ⇒ <code>any</code>
        * [.toString()](#KeyLocation+toString) ⇒ <code>string</code>
    * _static_
        * [.fromVerificationMethod(method)](#KeyLocation.fromVerificationMethod) ⇒ [<code>KeyLocation</code>](#KeyLocation)
        * [.fromJSON(json_value)](#KeyLocation.fromJSON) ⇒ [<code>KeyLocation</code>](#KeyLocation)

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
<a name="KeyLocation+toJSON"></a>

### keyLocation.toJSON() ⇒ <code>any</code>
Serializes `KeyLocation` as a JSON object.

**Kind**: instance method of [<code>KeyLocation</code>](#KeyLocation)  
<a name="KeyLocation+toString"></a>

### keyLocation.toString() ⇒ <code>string</code>
**Kind**: instance method of [<code>KeyLocation</code>](#KeyLocation)  
<a name="KeyLocation.fromVerificationMethod"></a>

### KeyLocation.fromVerificationMethod(method) ⇒ [<code>KeyLocation</code>](#KeyLocation)
Obtain the location of a verification method's key in storage.

**Kind**: static method of [<code>KeyLocation</code>](#KeyLocation)  

| Param | Type |
| --- | --- |
| method | [<code>VerificationMethod</code>](#VerificationMethod) | 

<a name="KeyLocation.fromJSON"></a>

### KeyLocation.fromJSON(json_value) ⇒ [<code>KeyLocation</code>](#KeyLocation)
Deserializes a JSON object into a `KeyLocation`.

**Kind**: static method of [<code>KeyLocation</code>](#KeyLocation)  

| Param | Type |
| --- | --- |
| json_value | <code>any</code> | 

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
        * [.fromJSON(json_value)](#MethodContent.fromJSON) ⇒ [<code>MethodContent</code>](#MethodContent)

<a name="MethodContent+toJSON"></a>

### methodContent.toJSON() ⇒ <code>any</code>
Serializes `MethodContent` as a JSON object.

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

### MethodContent.fromJSON(json_value) ⇒ [<code>MethodContent</code>](#MethodContent)
Deserializes `MethodContent` from a JSON object.

**Kind**: static method of [<code>MethodContent</code>](#MethodContent)  

| Param | Type |
| --- | --- |
| json_value | <code>any</code> | 

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
Serializes a `MethodData` object as a JSON object.

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
Deserializes a `MethodData` object from a JSON object.

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
Serializes a `MethodScope` object as a JSON object.

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
Deserializes a `MethodScope` object from a JSON object.

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
        * [.toJSON()](#MethodType+toJSON) ⇒ <code>any</code>
        * [.toString()](#MethodType+toString) ⇒ <code>string</code>
        * [.clone()](#MethodType+clone) ⇒ [<code>MethodType</code>](#MethodType)
    * _static_
        * [.Ed25519VerificationKey2018()](#MethodType.Ed25519VerificationKey2018) ⇒ [<code>MethodType</code>](#MethodType)
        * [.X25519KeyAgreementKey2019()](#MethodType.X25519KeyAgreementKey2019) ⇒ [<code>MethodType</code>](#MethodType)
        * [.fromJSON(json)](#MethodType.fromJSON) ⇒ [<code>MethodType</code>](#MethodType)

<a name="MethodType+toJSON"></a>

### methodType.toJSON() ⇒ <code>any</code>
Serializes a `MethodType` object as a JSON object.

**Kind**: instance method of [<code>MethodType</code>](#MethodType)  
<a name="MethodType+toString"></a>

### methodType.toString() ⇒ <code>string</code>
Returns the `MethodType` as a string.

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
Deserializes a `MethodType` object from a JSON object.

**Kind**: static method of [<code>MethodType</code>](#MethodType)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

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
Serializes a `Network` as a JSON object.

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
Deserializes a `Network` from a JSON object.

**Kind**: static method of [<code>Network</code>](#Network)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="Presentation"></a>

## Presentation
**Kind**: global class  

* [Presentation](#Presentation)
    * [new Presentation(holder_doc, credential_data, presentation_type, presentation_id)](#new_Presentation_new)
    * _instance_
        * [.toJSON()](#Presentation+toJSON) ⇒ <code>any</code>
        * [.verifiableCredential()](#Presentation+verifiableCredential) ⇒ [<code>Array.&lt;Credential&gt;</code>](#Credential)
        * [.clone()](#Presentation+clone) ⇒ [<code>Presentation</code>](#Presentation)
    * _static_
        * [.fromJSON(json)](#Presentation.fromJSON) ⇒ [<code>Presentation</code>](#Presentation)

<a name="new_Presentation_new"></a>

### new Presentation(holder_doc, credential_data, presentation_type, presentation_id)

| Param | Type |
| --- | --- |
| holder_doc | [<code>Document</code>](#Document) | 
| credential_data | <code>any</code> | 
| presentation_type | <code>string</code> \| <code>undefined</code> | 
| presentation_id | <code>string</code> \| <code>undefined</code> | 

<a name="Presentation+toJSON"></a>

### presentation.toJSON() ⇒ <code>any</code>
Serializes a `Presentation` object as a JSON object.

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation+verifiableCredential"></a>

### presentation.verifiableCredential() ⇒ [<code>Array.&lt;Credential&gt;</code>](#Credential)
Returns a copy of the credentials contained in the presentation.

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation+clone"></a>

### presentation.clone() ⇒ [<code>Presentation</code>](#Presentation)
Deep clones the object.

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation.fromJSON"></a>

### Presentation.fromJSON(json) ⇒ [<code>Presentation</code>](#Presentation)
Deserializes a `Presentation` object from a JSON object.

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
Serializes a `PresentationValidationOptions` as a JSON object.

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
Deserializes a `PresentationValidationOptions` from a JSON object.

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
| issuers | [<code>Array.&lt;Document&gt;</code>](#Document) \| [<code>Array.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument) | 
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
Serializes a `Proof` to a JSON object.

**Kind**: instance method of [<code>Proof</code>](#Proof)  
<a name="Proof+clone"></a>

### proof.clone() ⇒ [<code>Proof</code>](#Proof)
Deep clones the object.

**Kind**: instance method of [<code>Proof</code>](#Proof)  
<a name="Proof.fromJSON"></a>

### Proof.fromJSON(json) ⇒ [<code>Proof</code>](#Proof)
Deserializes a `Proof` from a JSON object.

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
        * [.clone()](#ProofOptions+clone) ⇒ [<code>ProofOptions</code>](#ProofOptions)
    * _static_
        * [.default()](#ProofOptions.default) ⇒ [<code>ProofOptions</code>](#ProofOptions)

<a name="new_ProofOptions_new"></a>

### new ProofOptions(options)
Creates a new `ProofOptions` from the given fields.

Throws an error if any of the options are invalid.


| Param | Type |
| --- | --- |
| options | <code>IProofOptions</code> | 

<a name="ProofOptions+clone"></a>

### proofOptions.clone() ⇒ [<code>ProofOptions</code>](#ProofOptions)
Deep clones the object.

**Kind**: instance method of [<code>ProofOptions</code>](#ProofOptions)  
<a name="ProofOptions.default"></a>

### ProofOptions.default() ⇒ [<code>ProofOptions</code>](#ProofOptions)
Creates a new `ProofOptions` with default options.

**Kind**: static method of [<code>ProofOptions</code>](#ProofOptions)  
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
Serializes a `ProofPurpose` to a JSON object.

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
Deserializes a `ProofPurpose` from a JSON object.

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
Serializes a `Receipt` as a JSON object.

**Kind**: instance method of [<code>Receipt</code>](#Receipt)  
<a name="Receipt+clone"></a>

### receipt.clone() ⇒ [<code>Receipt</code>](#Receipt)
Deep clones the object.

**Kind**: instance method of [<code>Receipt</code>](#Receipt)  
<a name="Receipt.fromJSON"></a>

### Receipt.fromJSON(json) ⇒ [<code>Receipt</code>](#Receipt)
Deserializes a `Receipt` from a JSON object.

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
Serializes a `Document` object as a JSON object.

**Kind**: instance method of [<code>ResolvedDocument</code>](#ResolvedDocument)  
<a name="ResolvedDocument+clone"></a>

### resolvedDocument.clone() ⇒ [<code>ResolvedDocument</code>](#ResolvedDocument)
Deep clones the object.

**Kind**: instance method of [<code>ResolvedDocument</code>](#ResolvedDocument)  
<a name="ResolvedDocument.fromJSON"></a>

### ResolvedDocument.fromJSON(json) ⇒ [<code>ResolvedDocument</code>](#ResolvedDocument)
Deserializes a `Document` object from a JSON object.

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
| did | [<code>DID</code>](#DID) \| <code>string</code> | 

<a name="Resolver+resolveHistory"></a>

### resolver.resolveHistory(did) ⇒ [<code>Promise.&lt;DocumentHistory&gt;</code>](#DocumentHistory)
Fetches the `DocumentHistory` of the given `DID`.

**Kind**: instance method of [<code>Resolver</code>](#Resolver)  

| Param | Type |
| --- | --- |
| did | [<code>DID</code>](#DID) \| <code>string</code> | 

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
| holder | [<code>ResolvedDocument</code>](#ResolvedDocument) \| <code>undefined</code> | 
| issuers | [<code>Array.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument) \| <code>undefined</code> | 

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
        * [.type()](#Service+type) ⇒ <code>string</code>
        * [.serviceEndpoint()](#Service+serviceEndpoint) ⇒ <code>string</code> \| <code>Array.&lt;string&gt;</code> \| <code>Map.&lt;string, Array.&lt;string&gt;&gt;</code>
        * [.properties()](#Service+properties) ⇒ <code>Map.&lt;string, any&gt;</code>
        * [.toJSON()](#Service+toJSON) ⇒ <code>any</code>
        * [.clone()](#Service+clone) ⇒ [<code>Service</code>](#Service)
    * _static_
        * [.fromJSON(value)](#Service.fromJSON) ⇒ [<code>Service</code>](#Service)

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

### service.type() ⇒ <code>string</code>
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
Serializes a `Service` object as a JSON object.

**Kind**: instance method of [<code>Service</code>](#Service)  
<a name="Service+clone"></a>

### service.clone() ⇒ [<code>Service</code>](#Service)
Deep clones the object.

**Kind**: instance method of [<code>Service</code>](#Service)  
<a name="Service.fromJSON"></a>

### Service.fromJSON(value) ⇒ [<code>Service</code>](#Service)
Deserializes a `Service` object from a JSON object.

**Kind**: static method of [<code>Service</code>](#Service)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="Signature"></a>

## Signature
**Kind**: global class  

* [Signature](#Signature)
    * [new Signature(data)](#new_Signature_new)
    * _instance_
        * [.asBytes()](#Signature+asBytes) ⇒ <code>Uint8Array</code>
        * [.toJSON()](#Signature+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json_value)](#Signature.fromJSON) ⇒ [<code>Signature</code>](#Signature)

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
Serializes a `Signature` as a JSON object.

**Kind**: instance method of [<code>Signature</code>](#Signature)  
<a name="Signature.fromJSON"></a>

### Signature.fromJSON(json_value) ⇒ [<code>Signature</code>](#Signature)
Deserializes a JSON object as `Signature`.

**Kind**: static method of [<code>Signature</code>](#Signature)  

| Param | Type |
| --- | --- |
| json_value | <code>any</code> | 

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
Serializes a `Timestamp` as a JSON object.

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
Deserializes a `Timestamp` from a JSON object.

**Kind**: static method of [<code>Timestamp</code>](#Timestamp)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="VerificationMethod"></a>

## VerificationMethod
**Kind**: global class  

* [VerificationMethod](#VerificationMethod)
    * [new VerificationMethod(did, key_type, public_key, fragment)](#new_VerificationMethod_new)
    * _instance_
        * [.id()](#VerificationMethod+id) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.controller()](#VerificationMethod+controller) ⇒ [<code>DID</code>](#DID)
        * [.SetController(did)](#VerificationMethod+SetController)
        * [.type()](#VerificationMethod+type) ⇒ [<code>MethodType</code>](#MethodType)
        * [.data()](#VerificationMethod+data) ⇒ [<code>MethodData</code>](#MethodData)
        * [.toJSON()](#VerificationMethod+toJSON) ⇒ <code>any</code>
        * [.clone()](#VerificationMethod+clone) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
    * _static_
        * [.fromJSON(value)](#VerificationMethod.fromJSON) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)

<a name="new_VerificationMethod_new"></a>

### new VerificationMethod(did, key_type, public_key, fragment)
Creates a new `VerificationMethod` object from the given `did` and public key.


| Param | Type |
| --- | --- |
| did | [<code>DID</code>](#DID) | 
| key_type | <code>number</code> | 
| public_key | <code>Uint8Array</code> | 
| fragment | <code>string</code> | 

<a name="VerificationMethod+id"></a>

### verificationMethod.id() ⇒ [<code>DIDUrl</code>](#DIDUrl)
Returns a copy of the `id` `DIDUrl` of the `VerificationMethod` object.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod+controller"></a>

### verificationMethod.controller() ⇒ [<code>DID</code>](#DID)
Returns a copy of the `controller` `DID` of the `VerificationMethod` object.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod+SetController"></a>

### verificationMethod.SetController(did)
Sets the `controller` `DID` of the `VerificationMethod` object.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  

| Param | Type |
| --- | --- |
| did | [<code>DID</code>](#DID) | 

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
Serializes a `VerificationMethod` object as a JSON object.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod+clone"></a>

### verificationMethod.clone() ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
Deep clones the object.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod.fromJSON"></a>

### VerificationMethod.fromJSON(value) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
Deserializes a `VerificationMethod` object from a JSON object.

**Kind**: static method of [<code>VerificationMethod</code>](#VerificationMethod)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

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
Serializes a `VerifierOptions` as a JSON object.

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
Deserializes a `VerifierOptions` from a JSON object.

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

<a name="DIDMessageEncoding"></a>

## DIDMessageEncoding
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
<a name="KeyType"></a>

## KeyType
**Kind**: global variable  
<a name="start"></a>

## start()
Initializes the console error panic hook for better error messages

**Kind**: global function  

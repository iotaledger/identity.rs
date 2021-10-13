## Classes

<dl>
<dt><a href="#AuthenticationRequest">AuthenticationRequest</a></dt>
<dd></dd>
<dt><a href="#AuthenticationResponse">AuthenticationResponse</a></dt>
<dd></dd>
<dt><a href="#Client">Client</a></dt>
<dd></dd>
<dt><a href="#Config">Config</a></dt>
<dd></dd>
<dt><a href="#CredentialIssuance">CredentialIssuance</a></dt>
<dd></dd>
<dt><a href="#CredentialOptionRequest">CredentialOptionRequest</a></dt>
<dd></dd>
<dt><a href="#CredentialOptionResponse">CredentialOptionResponse</a></dt>
<dd></dd>
<dt><a href="#CredentialRevocation">CredentialRevocation</a></dt>
<dd></dd>
<dt><a href="#CredentialSelection">CredentialSelection</a></dt>
<dd></dd>
<dt><a href="#DID">DID</a></dt>
<dd></dd>
<dt><a href="#DidRequest">DidRequest</a></dt>
<dd></dd>
<dt><a href="#DidResponse">DidResponse</a></dt>
<dd></dd>
<dt><a href="#DiffChainHistory">DiffChainHistory</a></dt>
<dd></dd>
<dt><a href="#Document">Document</a></dt>
<dd></dd>
<dt><a href="#DocumentDiff">DocumentDiff</a></dt>
<dd><p>Defines the difference between two DID [<code>Document</code>]s&#39; JSON representations.</p>
</dd>
<dt><a href="#DocumentHistory">DocumentHistory</a></dt>
<dd><p>A DID Document&#39;s history and current state.</p>
</dd>
<dt><a href="#FeaturesRequest">FeaturesRequest</a></dt>
<dd></dd>
<dt><a href="#FeaturesResponse">FeaturesResponse</a></dt>
<dd></dd>
<dt><a href="#IntegrationChainHistory">IntegrationChainHistory</a></dt>
<dd></dd>
<dt><a href="#Introduction">Introduction</a></dt>
<dd></dd>
<dt><a href="#IntroductionProposal">IntroductionProposal</a></dt>
<dd></dd>
<dt><a href="#IntroductionResponse">IntroductionResponse</a></dt>
<dd></dd>
<dt><a href="#KeyCollection">KeyCollection</a></dt>
<dd></dd>
<dt><a href="#KeyPair">KeyPair</a></dt>
<dd></dd>
<dt><a href="#Network">Network</a></dt>
<dd></dd>
<dt><a href="#PresentationRequest">PresentationRequest</a></dt>
<dd></dd>
<dt><a href="#PresentationResponse">PresentationResponse</a></dt>
<dd></dd>
<dt><a href="#ResolutionRequest">ResolutionRequest</a></dt>
<dd></dd>
<dt><a href="#ResolutionResponse">ResolutionResponse</a></dt>
<dd></dd>
<dt><a href="#Service">Service</a></dt>
<dd></dd>
<dt><a href="#Timestamp">Timestamp</a></dt>
<dd></dd>
<dt><a href="#Timing">Timing</a></dt>
<dd></dd>
<dt><a href="#TrustPing">TrustPing</a></dt>
<dd></dd>
<dt><a href="#TrustedIssuer">TrustedIssuer</a></dt>
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
</dl>

<a name="AuthenticationRequest"></a>

## AuthenticationRequest
**Kind**: global class  

* [AuthenticationRequest](#AuthenticationRequest)
    * _instance_
        * [.toJSON()](#AuthenticationRequest+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(value)](#AuthenticationRequest.fromJSON) ⇒ [<code>AuthenticationRequest</code>](#AuthenticationRequest)

<a name="AuthenticationRequest+toJSON"></a>

### authenticationRequest.toJSON() ⇒ <code>any</code>
**Kind**: instance method of [<code>AuthenticationRequest</code>](#AuthenticationRequest)  
<a name="AuthenticationRequest.fromJSON"></a>

### AuthenticationRequest.fromJSON(value) ⇒ [<code>AuthenticationRequest</code>](#AuthenticationRequest)
**Kind**: static method of [<code>AuthenticationRequest</code>](#AuthenticationRequest)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="AuthenticationResponse"></a>

## AuthenticationResponse
**Kind**: global class  

* [AuthenticationResponse](#AuthenticationResponse)
    * _instance_
        * [.toJSON()](#AuthenticationResponse+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(value)](#AuthenticationResponse.fromJSON) ⇒ [<code>AuthenticationResponse</code>](#AuthenticationResponse)

<a name="AuthenticationResponse+toJSON"></a>

### authenticationResponse.toJSON() ⇒ <code>any</code>
**Kind**: instance method of [<code>AuthenticationResponse</code>](#AuthenticationResponse)  
<a name="AuthenticationResponse.fromJSON"></a>

### AuthenticationResponse.fromJSON(value) ⇒ [<code>AuthenticationResponse</code>](#AuthenticationResponse)
**Kind**: static method of [<code>AuthenticationResponse</code>](#AuthenticationResponse)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="Client"></a>

## Client
**Kind**: global class  

* [Client](#Client)
    * [new Client()](#new_Client_new)
    * _instance_
        * [.network()](#Client+network) ⇒ [<code>Network</code>](#Network)
        * [.publishDocument(document)](#Client+publishDocument) ⇒ <code>Promise.&lt;any&gt;</code>
        * [.publishDiff(message_id, diff)](#Client+publishDiff) ⇒ <code>Promise.&lt;any&gt;</code>
        * [.publishJSON(index, data)](#Client+publishJSON) ⇒ <code>Promise.&lt;any&gt;</code>
        * [.resolve(did)](#Client+resolve) ⇒ <code>Promise.&lt;any&gt;</code>
        * [.resolveHistory(did)](#Client+resolveHistory) ⇒ <code>Promise.&lt;any&gt;</code>
        * [.resolveDiffHistory(document)](#Client+resolveDiffHistory) ⇒ <code>Promise.&lt;any&gt;</code>
        * [.checkCredential(data)](#Client+checkCredential) ⇒ <code>Promise.&lt;any&gt;</code>
        * [.checkPresentation(data)](#Client+checkPresentation) ⇒ <code>Promise.&lt;any&gt;</code>
    * _static_
        * [.fromConfig(config)](#Client.fromConfig) ⇒ [<code>Client</code>](#Client)
        * [.fromNetwork(network)](#Client.fromNetwork) ⇒ [<code>Client</code>](#Client)

<a name="new_Client_new"></a>

### new Client()
Creates a new `Client` with default settings.

<a name="Client+network"></a>

### client.network() ⇒ [<code>Network</code>](#Network)
Returns the `Client` Tangle network.

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+publishDocument"></a>

### client.publishDocument(document) ⇒ <code>Promise.&lt;any&gt;</code>
Publishes an `IotaDocument` to the Tangle.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| document | <code>any</code> | 

<a name="Client+publishDiff"></a>

### client.publishDiff(message_id, diff) ⇒ <code>Promise.&lt;any&gt;</code>
Publishes a `DocumentDiff` to the Tangle.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| message_id | <code>string</code> | 
| diff | [<code>DocumentDiff</code>](#DocumentDiff) | 

<a name="Client+publishJSON"></a>

### client.publishJSON(index, data) ⇒ <code>Promise.&lt;any&gt;</code>
Publishes arbitrary JSON data to the specified index on the Tangle.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| index | <code>string</code> | 
| data | <code>any</code> | 

<a name="Client+resolve"></a>

### client.resolve(did) ⇒ <code>Promise.&lt;any&gt;</code>
**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| did | <code>string</code> | 

<a name="Client+resolveHistory"></a>

### client.resolveHistory(did) ⇒ <code>Promise.&lt;any&gt;</code>
Returns the message history of the given DID.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| did | <code>string</code> | 

<a name="Client+resolveDiffHistory"></a>

### client.resolveDiffHistory(document) ⇒ <code>Promise.&lt;any&gt;</code>
Returns the [`DiffChainHistory`] of a diff chain starting from a document on the
integration chain.

NOTE: the document must have been published to the tangle and have a valid message id and
authentication method.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| document | [<code>Document</code>](#Document) | 

<a name="Client+checkCredential"></a>

### client.checkCredential(data) ⇒ <code>Promise.&lt;any&gt;</code>
Validates a credential with the DID Document from the Tangle.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| data | <code>string</code> | 

<a name="Client+checkPresentation"></a>

### client.checkPresentation(data) ⇒ <code>Promise.&lt;any&gt;</code>
Validates a presentation with the DID Document from the Tangle.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| data | <code>string</code> | 

<a name="Client.fromConfig"></a>

### Client.fromConfig(config) ⇒ [<code>Client</code>](#Client)
Creates a new `Client` with settings from the given `Config`.

**Kind**: static method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| config | [<code>Config</code>](#Config) | 

<a name="Client.fromNetwork"></a>

### Client.fromNetwork(network) ⇒ [<code>Client</code>](#Client)
Creates a new `Client` with default settings for the given `Network`.

**Kind**: static method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| network | [<code>Network</code>](#Network) | 

<a name="Config"></a>

## Config
**Kind**: global class  

* [Config](#Config)
    * _instance_
        * [.setNetwork(network)](#Config+setNetwork)
        * [.setNode(url)](#Config+setNode)
        * [.setPrimaryNode(url, jwt, username, password)](#Config+setPrimaryNode)
        * [.setPrimaryPoWNode(url, jwt, username, password)](#Config+setPrimaryPoWNode)
        * [.setPermanode(url, jwt, username, password)](#Config+setPermanode)
        * [.setNodeAuth(url, jwt, username, password)](#Config+setNodeAuth)
        * [.setNodeSyncInterval(value)](#Config+setNodeSyncInterval)
        * [.setNodeSyncDisabled()](#Config+setNodeSyncDisabled)
        * [.setQuorum(value)](#Config+setQuorum)
        * [.setQuorumSize(value)](#Config+setQuorumSize)
        * [.setQuorumThreshold(value)](#Config+setQuorumThreshold)
        * [.setLocalPoW(value)](#Config+setLocalPoW)
        * [.setTipsInterval(value)](#Config+setTipsInterval)
        * [.setRequestTimeout(value)](#Config+setRequestTimeout)
    * _static_
        * [.fromNetwork(network)](#Config.fromNetwork) ⇒ [<code>Config</code>](#Config)

<a name="Config+setNetwork"></a>

### config.setNetwork(network)
**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| network | [<code>Network</code>](#Network) | 

<a name="Config+setNode"></a>

### config.setNode(url)
**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| url | <code>string</code> | 

<a name="Config+setPrimaryNode"></a>

### config.setPrimaryNode(url, jwt, username, password)
**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| url | <code>string</code> | 
| jwt | <code>string</code> \| <code>undefined</code> | 
| username | <code>string</code> \| <code>undefined</code> | 
| password | <code>string</code> \| <code>undefined</code> | 

<a name="Config+setPrimaryPoWNode"></a>

### config.setPrimaryPoWNode(url, jwt, username, password)
**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| url | <code>string</code> | 
| jwt | <code>string</code> \| <code>undefined</code> | 
| username | <code>string</code> \| <code>undefined</code> | 
| password | <code>string</code> \| <code>undefined</code> | 

<a name="Config+setPermanode"></a>

### config.setPermanode(url, jwt, username, password)
**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| url | <code>string</code> | 
| jwt | <code>string</code> \| <code>undefined</code> | 
| username | <code>string</code> \| <code>undefined</code> | 
| password | <code>string</code> \| <code>undefined</code> | 

<a name="Config+setNodeAuth"></a>

### config.setNodeAuth(url, jwt, username, password)
**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| url | <code>string</code> | 
| jwt | <code>string</code> \| <code>undefined</code> | 
| username | <code>string</code> \| <code>undefined</code> | 
| password | <code>string</code> \| <code>undefined</code> | 

<a name="Config+setNodeSyncInterval"></a>

### config.setNodeSyncInterval(value)
**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| value | <code>number</code> | 

<a name="Config+setNodeSyncDisabled"></a>

### config.setNodeSyncDisabled()
**Kind**: instance method of [<code>Config</code>](#Config)  
<a name="Config+setQuorum"></a>

### config.setQuorum(value)
**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| value | <code>boolean</code> | 

<a name="Config+setQuorumSize"></a>

### config.setQuorumSize(value)
**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| value | <code>number</code> | 

<a name="Config+setQuorumThreshold"></a>

### config.setQuorumThreshold(value)
**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| value | <code>number</code> | 

<a name="Config+setLocalPoW"></a>

### config.setLocalPoW(value)
**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| value | <code>boolean</code> | 

<a name="Config+setTipsInterval"></a>

### config.setTipsInterval(value)
**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| value | <code>number</code> | 

<a name="Config+setRequestTimeout"></a>

### config.setRequestTimeout(value)
**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| value | <code>number</code> | 

<a name="Config.fromNetwork"></a>

### Config.fromNetwork(network) ⇒ [<code>Config</code>](#Config)
**Kind**: static method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| network | [<code>Network</code>](#Network) | 

<a name="CredentialIssuance"></a>

## CredentialIssuance
**Kind**: global class  

* [CredentialIssuance](#CredentialIssuance)
    * _instance_
        * [.toJSON()](#CredentialIssuance+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(value)](#CredentialIssuance.fromJSON) ⇒ [<code>CredentialIssuance</code>](#CredentialIssuance)

<a name="CredentialIssuance+toJSON"></a>

### credentialIssuance.toJSON() ⇒ <code>any</code>
**Kind**: instance method of [<code>CredentialIssuance</code>](#CredentialIssuance)  
<a name="CredentialIssuance.fromJSON"></a>

### CredentialIssuance.fromJSON(value) ⇒ [<code>CredentialIssuance</code>](#CredentialIssuance)
**Kind**: static method of [<code>CredentialIssuance</code>](#CredentialIssuance)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="CredentialOptionRequest"></a>

## CredentialOptionRequest
**Kind**: global class  

* [CredentialOptionRequest](#CredentialOptionRequest)
    * _instance_
        * [.toJSON()](#CredentialOptionRequest+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(value)](#CredentialOptionRequest.fromJSON) ⇒ [<code>CredentialOptionRequest</code>](#CredentialOptionRequest)

<a name="CredentialOptionRequest+toJSON"></a>

### credentialOptionRequest.toJSON() ⇒ <code>any</code>
**Kind**: instance method of [<code>CredentialOptionRequest</code>](#CredentialOptionRequest)  
<a name="CredentialOptionRequest.fromJSON"></a>

### CredentialOptionRequest.fromJSON(value) ⇒ [<code>CredentialOptionRequest</code>](#CredentialOptionRequest)
**Kind**: static method of [<code>CredentialOptionRequest</code>](#CredentialOptionRequest)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="CredentialOptionResponse"></a>

## CredentialOptionResponse
**Kind**: global class  

* [CredentialOptionResponse](#CredentialOptionResponse)
    * _instance_
        * [.toJSON()](#CredentialOptionResponse+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(value)](#CredentialOptionResponse.fromJSON) ⇒ [<code>CredentialOptionResponse</code>](#CredentialOptionResponse)

<a name="CredentialOptionResponse+toJSON"></a>

### credentialOptionResponse.toJSON() ⇒ <code>any</code>
**Kind**: instance method of [<code>CredentialOptionResponse</code>](#CredentialOptionResponse)  
<a name="CredentialOptionResponse.fromJSON"></a>

### CredentialOptionResponse.fromJSON(value) ⇒ [<code>CredentialOptionResponse</code>](#CredentialOptionResponse)
**Kind**: static method of [<code>CredentialOptionResponse</code>](#CredentialOptionResponse)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="CredentialRevocation"></a>

## CredentialRevocation
**Kind**: global class  

* [CredentialRevocation](#CredentialRevocation)
    * _instance_
        * [.toJSON()](#CredentialRevocation+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(value)](#CredentialRevocation.fromJSON) ⇒ [<code>CredentialRevocation</code>](#CredentialRevocation)

<a name="CredentialRevocation+toJSON"></a>

### credentialRevocation.toJSON() ⇒ <code>any</code>
**Kind**: instance method of [<code>CredentialRevocation</code>](#CredentialRevocation)  
<a name="CredentialRevocation.fromJSON"></a>

### CredentialRevocation.fromJSON(value) ⇒ [<code>CredentialRevocation</code>](#CredentialRevocation)
**Kind**: static method of [<code>CredentialRevocation</code>](#CredentialRevocation)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="CredentialSelection"></a>

## CredentialSelection
**Kind**: global class  

* [CredentialSelection](#CredentialSelection)
    * _instance_
        * [.toJSON()](#CredentialSelection+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(value)](#CredentialSelection.fromJSON) ⇒ [<code>CredentialSelection</code>](#CredentialSelection)

<a name="CredentialSelection+toJSON"></a>

### credentialSelection.toJSON() ⇒ <code>any</code>
**Kind**: instance method of [<code>CredentialSelection</code>](#CredentialSelection)  
<a name="CredentialSelection.fromJSON"></a>

### CredentialSelection.fromJSON(value) ⇒ [<code>CredentialSelection</code>](#CredentialSelection)
**Kind**: static method of [<code>CredentialSelection</code>](#CredentialSelection)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="DID"></a>

## DID
**Kind**: global class  

* [DID](#DID)
    * [new DID(key, network)](#new_DID_new)
    * _instance_
        * [.network](#DID+network) ⇒ [<code>Network</code>](#Network)
        * [.networkName](#DID+networkName) ⇒ <code>string</code>
        * [.tag](#DID+tag) ⇒ <code>string</code>
        * [.toString()](#DID+toString) ⇒ <code>string</code>
    * _static_
        * [.fromBase58(key, network)](#DID.fromBase58) ⇒ [<code>DID</code>](#DID)
        * [.parse(input)](#DID.parse) ⇒ [<code>DID</code>](#DID)

<a name="new_DID_new"></a>

### new DID(key, network)
Creates a new `DID` from a `KeyPair` object.


| Param | Type |
| --- | --- |
| key | [<code>KeyPair</code>](#KeyPair) | 
| network | <code>string</code> \| <code>undefined</code> | 

<a name="DID+network"></a>

### did.network ⇒ [<code>Network</code>](#Network)
Returns the IOTA tangle network of the `DID`.

**Kind**: instance property of [<code>DID</code>](#DID)  
<a name="DID+networkName"></a>

### did.networkName ⇒ <code>string</code>
Returns the IOTA tangle network of the `DID`.

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

### DID.fromBase58(key, network) ⇒ [<code>DID</code>](#DID)
Creates a new `DID` from a base58-encoded public key.

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

<a name="DidRequest"></a>

## DidRequest
**Kind**: global class  

* [DidRequest](#DidRequest)
    * _instance_
        * [.toJSON()](#DidRequest+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(value)](#DidRequest.fromJSON) ⇒ [<code>DidRequest</code>](#DidRequest)

<a name="DidRequest+toJSON"></a>

### didRequest.toJSON() ⇒ <code>any</code>
**Kind**: instance method of [<code>DidRequest</code>](#DidRequest)  
<a name="DidRequest.fromJSON"></a>

### DidRequest.fromJSON(value) ⇒ [<code>DidRequest</code>](#DidRequest)
**Kind**: static method of [<code>DidRequest</code>](#DidRequest)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="DidResponse"></a>

## DidResponse
**Kind**: global class  

* [DidResponse](#DidResponse)
    * _instance_
        * [.toJSON()](#DidResponse+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(value)](#DidResponse.fromJSON) ⇒ [<code>DidResponse</code>](#DidResponse)

<a name="DidResponse+toJSON"></a>

### didResponse.toJSON() ⇒ <code>any</code>
**Kind**: instance method of [<code>DidResponse</code>](#DidResponse)  
<a name="DidResponse.fromJSON"></a>

### DidResponse.fromJSON(value) ⇒ [<code>DidResponse</code>](#DidResponse)
**Kind**: static method of [<code>DidResponse</code>](#DidResponse)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="DiffChainHistory"></a>

## DiffChainHistory
**Kind**: global class  

* [DiffChainHistory](#DiffChainHistory)
    * _instance_
        * [.chainData()](#DiffChainHistory+chainData) ⇒ <code>Array.&lt;any&gt;</code>
        * [.spam()](#DiffChainHistory+spam) ⇒ <code>Array.&lt;any&gt;</code>
        * [.toJSON()](#DiffChainHistory+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#DiffChainHistory.fromJSON) ⇒ [<code>DiffChainHistory</code>](#DiffChainHistory)

<a name="DiffChainHistory+chainData"></a>

### diffChainHistory.chainData() ⇒ <code>Array.&lt;any&gt;</code>
Returns a [`js_sys::Array`] of `$wasm_ty` as strings.

NOTE: this clones the field.

**Kind**: instance method of [<code>DiffChainHistory</code>](#DiffChainHistory)  
<a name="DiffChainHistory+spam"></a>

### diffChainHistory.spam() ⇒ <code>Array.&lt;any&gt;</code>
Returns a [`js_sys::Array`] of [`MessageIds`][MessageId] as strings.

NOTE: this clones the field.

**Kind**: instance method of [<code>DiffChainHistory</code>](#DiffChainHistory)  
<a name="DiffChainHistory+toJSON"></a>

### diffChainHistory.toJSON() ⇒ <code>any</code>
Serializes a `$ident` object as a JSON object.

**Kind**: instance method of [<code>DiffChainHistory</code>](#DiffChainHistory)  
<a name="DiffChainHistory.fromJSON"></a>

### DiffChainHistory.fromJSON(json) ⇒ [<code>DiffChainHistory</code>](#DiffChainHistory)
Deserializes a `$ident` object from a JSON object.

**Kind**: static method of [<code>DiffChainHistory</code>](#DiffChainHistory)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="Document"></a>

## Document
**Kind**: global class  

* [Document](#Document)
    * [new Document(keypair, network, fragment)](#new_Document_new)
    * _instance_
        * [.id](#Document+id) ⇒ [<code>DID</code>](#DID)
        * [.created](#Document+created) ⇒ [<code>Timestamp</code>](#Timestamp)
        * [.created](#Document+created)
        * [.updated](#Document+updated) ⇒ [<code>Timestamp</code>](#Timestamp)
        * [.updated](#Document+updated)
        * [.proof](#Document+proof) ⇒ <code>any</code>
        * [.messageId](#Document+messageId) ⇒ <code>string</code>
        * [.messageId](#Document+messageId)
        * [.previousMessageId](#Document+previousMessageId) ⇒ <code>string</code>
        * [.previousMessageId](#Document+previousMessageId)
        * [.authentication()](#Document+authentication) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
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
        * [.diff(other, message, key)](#Document+diff) ⇒ [<code>DocumentDiff</code>](#DocumentDiff)
        * [.merge(diff)](#Document+merge)
        * [.integrationIndex()](#Document+integrationIndex) ⇒ <code>string</code>
        * [.toJSON()](#Document+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromAuthentication(method)](#Document.fromAuthentication) ⇒ [<code>Document</code>](#Document)
        * [.diffIndex(message_id)](#Document.diffIndex) ⇒ <code>string</code>
        * [.fromJSON(json)](#Document.fromJSON) ⇒ [<code>Document</code>](#Document)

<a name="new_Document_new"></a>

### new Document(keypair, network, fragment)
Creates a new DID Document from the given `KeyPair`, network, and verification method
fragment name.

The DID Document will be pre-populated with a single verification method
derived from the provided `KeyPair`, with an attached authentication relationship.
This method will have the DID URL fragment `#authentication` by default and can be easily
retrieved with `Document::authentication`.

NOTE: the generated document is unsigned, see `Document::sign`.

Arguments:

* keypair: the initial verification method is derived from the public key with this keypair.
* network: Tangle network to use for the DID, default `Network::mainnet`.
* fragment: name of the initial verification method, default "authentication".


| Param | Type |
| --- | --- |
| keypair | [<code>KeyPair</code>](#KeyPair) | 
| network | <code>string</code> \| <code>undefined</code> | 
| fragment | <code>string</code> \| <code>undefined</code> | 

<a name="Document+id"></a>

### document.id ⇒ [<code>DID</code>](#DID)
Returns the DID Document `id`.

**Kind**: instance property of [<code>Document</code>](#Document)  
<a name="Document+created"></a>

### document.created ⇒ [<code>Timestamp</code>](#Timestamp)
Returns the timestamp of when the DID document was created.

**Kind**: instance property of [<code>Document</code>](#Document)  
<a name="Document+created"></a>

### document.created
Sets the timestamp of when the DID document was created.

**Kind**: instance property of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| timestamp | [<code>Timestamp</code>](#Timestamp) | 

<a name="Document+updated"></a>

### document.updated ⇒ [<code>Timestamp</code>](#Timestamp)
Returns the timestamp of the last DID document update.

**Kind**: instance property of [<code>Document</code>](#Document)  
<a name="Document+updated"></a>

### document.updated
Sets the timestamp of the last DID document update.

**Kind**: instance property of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| timestamp | [<code>Timestamp</code>](#Timestamp) | 

<a name="Document+proof"></a>

### document.proof ⇒ <code>any</code>
Returns the DID Document `proof` object.

**Kind**: instance property of [<code>Document</code>](#Document)  
<a name="Document+messageId"></a>

### document.messageId ⇒ <code>string</code>
Get the message_id of the DID Document.

**Kind**: instance property of [<code>Document</code>](#Document)  
<a name="Document+messageId"></a>

### document.messageId
Set the message_id of the DID Document.

**Kind**: instance property of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| message_id | <code>string</code> | 

<a name="Document+previousMessageId"></a>

### document.previousMessageId ⇒ <code>string</code>
**Kind**: instance property of [<code>Document</code>](#Document)  
<a name="Document+previousMessageId"></a>

### document.previousMessageId
**Kind**: instance property of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="Document+authentication"></a>

### document.authentication() ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
Returns the default Verification Method of the DID Document.

**Kind**: instance method of [<code>Document</code>](#Document)  
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

### document.diff(other, message, key) ⇒ [<code>DocumentDiff</code>](#DocumentDiff)
Generate the difference between two DID Documents and sign it

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| other | [<code>Document</code>](#Document) | 
| message | <code>string</code> | 
| key | [<code>KeyPair</code>](#KeyPair) | 

<a name="Document+merge"></a>

### document.merge(diff)
Verifies a `DocumentDiff` signature and merges the changes into `self`.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| diff | [<code>DocumentDiff</code>](#DocumentDiff) | 

<a name="Document+integrationIndex"></a>

### document.integrationIndex() ⇒ <code>string</code>
Returns the Tangle index of the integration chain for this DID.

This is simply the tag segment of the [`IotaDID`].
E.g.
For an [`IotaDocument`] `doc` with DID: did:iota:1234567890abcdefghijklmnopqrstuvxyzABCDEFGHI,
`doc.integration_index()` == "1234567890abcdefghijklmnopqrstuvxyzABCDEFGHI"

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+toJSON"></a>

### document.toJSON() ⇒ <code>any</code>
Serializes a `Document` object as a JSON object.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document.fromAuthentication"></a>

### Document.fromAuthentication(method) ⇒ [<code>Document</code>](#Document)
Creates a new DID Document from the given `VerificationMethod`.

NOTE: the generated document is unsigned, see Document::sign.

**Kind**: static method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| method | [<code>VerificationMethod</code>](#VerificationMethod) | 

<a name="Document.diffIndex"></a>

### Document.diffIndex(message_id) ⇒ <code>string</code>
Returns the Tangle index of the DID diff chain. This should only be called on documents
published on the integration chain.

This is the Base58-btc encoded SHA-256 digest of the hex-encoded message id.

**Kind**: static method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| message_id | <code>string</code> | 

<a name="Document.fromJSON"></a>

### Document.fromJSON(json) ⇒ [<code>Document</code>](#Document)
Deserializes a `Document` object from a JSON object.

**Kind**: static method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="DocumentDiff"></a>

## DocumentDiff
Defines the difference between two DID [`Document`]s' JSON representations.

**Kind**: global class  

* [DocumentDiff](#DocumentDiff)
    * [.did](#DocumentDiff+did) ⇒ [<code>DID</code>](#DID)
    * [.diff](#DocumentDiff+diff) ⇒ <code>string</code>
    * [.messageId](#DocumentDiff+messageId) ⇒ <code>string</code>
    * [.messageId](#DocumentDiff+messageId)
    * [.previousMessageId](#DocumentDiff+previousMessageId) ⇒ <code>string</code>
    * [.previousMessageId](#DocumentDiff+previousMessageId)
    * [.proof](#DocumentDiff+proof) ⇒ <code>any</code>
    * [.id()](#DocumentDiff+id) ⇒ [<code>DID</code>](#DID)
    * [.merge(document)](#DocumentDiff+merge) ⇒ [<code>Document</code>](#Document)

<a name="DocumentDiff+did"></a>

### documentDiff.did ⇒ [<code>DID</code>](#DID)
Returns the DID of the associated DID Document.

**Kind**: instance property of [<code>DocumentDiff</code>](#DocumentDiff)  
<a name="DocumentDiff+diff"></a>

### documentDiff.diff ⇒ <code>string</code>
Returns the raw contents of the DID Document diff.

NOTE: clones the data.

**Kind**: instance property of [<code>DocumentDiff</code>](#DocumentDiff)  
<a name="DocumentDiff+messageId"></a>

### documentDiff.messageId ⇒ <code>string</code>
Returns the message_id of the DID Document diff.

**Kind**: instance property of [<code>DocumentDiff</code>](#DocumentDiff)  
<a name="DocumentDiff+messageId"></a>

### documentDiff.messageId
Sets the message_id of the DID Document diff.

**Kind**: instance property of [<code>DocumentDiff</code>](#DocumentDiff)  

| Param | Type |
| --- | --- |
| message_id | <code>string</code> | 

<a name="DocumentDiff+previousMessageId"></a>

### documentDiff.previousMessageId ⇒ <code>string</code>
Returns the Tangle message id of the previous DID Document diff.

**Kind**: instance property of [<code>DocumentDiff</code>](#DocumentDiff)  
<a name="DocumentDiff+previousMessageId"></a>

### documentDiff.previousMessageId
Sets the Tangle message id of the previous DID Document diff.

**Kind**: instance property of [<code>DocumentDiff</code>](#DocumentDiff)  

| Param | Type |
| --- | --- |
| message_id | <code>string</code> | 

<a name="DocumentDiff+proof"></a>

### documentDiff.proof ⇒ <code>any</code>
Returns the `proof` object.

**Kind**: instance property of [<code>DocumentDiff</code>](#DocumentDiff)  
<a name="DocumentDiff+id"></a>

### documentDiff.id() ⇒ [<code>DID</code>](#DID)
Returns the DID of the associated DID Document.

NOTE: clones the data.

**Kind**: instance method of [<code>DocumentDiff</code>](#DocumentDiff)  
<a name="DocumentDiff+merge"></a>

### documentDiff.merge(document) ⇒ [<code>Document</code>](#Document)
Returns a new DID Document which is the result of merging `self`
with the given Document.

**Kind**: instance method of [<code>DocumentDiff</code>](#DocumentDiff)  

| Param | Type |
| --- | --- |
| document | [<code>Document</code>](#Document) | 

<a name="DocumentHistory"></a>

## DocumentHistory
A DID Document's history and current state.

**Kind**: global class  

* [DocumentHistory](#DocumentHistory)
    * _instance_
        * [.integrationChainData()](#DocumentHistory+integrationChainData) ⇒ <code>Array.&lt;any&gt;</code>
        * [.integrationChainSpam()](#DocumentHistory+integrationChainSpam) ⇒ <code>Array.&lt;any&gt;</code>
        * [.diffChainData()](#DocumentHistory+diffChainData) ⇒ <code>Array.&lt;any&gt;</code>
        * [.diffChainSpam()](#DocumentHistory+diffChainSpam) ⇒ <code>Array.&lt;any&gt;</code>
        * [.toJSON()](#DocumentHistory+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#DocumentHistory.fromJSON) ⇒ [<code>DocumentHistory</code>](#DocumentHistory)

<a name="DocumentHistory+integrationChainData"></a>

### documentHistory.integrationChainData() ⇒ <code>Array.&lt;any&gt;</code>
Returns a [`js_sys::Array`] of integration chain [`WasmDocuments`](WasmDocument).

NOTE: clones the data.

**Kind**: instance method of [<code>DocumentHistory</code>](#DocumentHistory)  
<a name="DocumentHistory+integrationChainSpam"></a>

### documentHistory.integrationChainSpam() ⇒ <code>Array.&lt;any&gt;</code>
Returns a [`js_sys::Array`] of message id strings for "spam" messages on the same index
as the integration chain.

NOTE: clones the data.

**Kind**: instance method of [<code>DocumentHistory</code>](#DocumentHistory)  
<a name="DocumentHistory+diffChainData"></a>

### documentHistory.diffChainData() ⇒ <code>Array.&lt;any&gt;</code>
Returns a [`js_sys::Array`] of diff chain [`WasmDocumentDiffs`](WasmDocumentDiff).

NOTE: clones the data.

**Kind**: instance method of [<code>DocumentHistory</code>](#DocumentHistory)  
<a name="DocumentHistory+diffChainSpam"></a>

### documentHistory.diffChainSpam() ⇒ <code>Array.&lt;any&gt;</code>
Returns a [`js_sys::Array`] of message id strings for "spam" messages on the same index
as the diff chain.

NOTE: clones the data.

**Kind**: instance method of [<code>DocumentHistory</code>](#DocumentHistory)  
<a name="DocumentHistory+toJSON"></a>

### documentHistory.toJSON() ⇒ <code>any</code>
Serializes a [`WasmDocumentHistory`] object as a JSON object.

**Kind**: instance method of [<code>DocumentHistory</code>](#DocumentHistory)  
<a name="DocumentHistory.fromJSON"></a>

### DocumentHistory.fromJSON(json) ⇒ [<code>DocumentHistory</code>](#DocumentHistory)
Deserializes a [`WasmDocumentHistory`] object from a JSON object.

**Kind**: static method of [<code>DocumentHistory</code>](#DocumentHistory)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="FeaturesRequest"></a>

## FeaturesRequest
**Kind**: global class  

* [FeaturesRequest](#FeaturesRequest)
    * _instance_
        * [.toJSON()](#FeaturesRequest+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(value)](#FeaturesRequest.fromJSON) ⇒ [<code>FeaturesRequest</code>](#FeaturesRequest)

<a name="FeaturesRequest+toJSON"></a>

### featuresRequest.toJSON() ⇒ <code>any</code>
**Kind**: instance method of [<code>FeaturesRequest</code>](#FeaturesRequest)  
<a name="FeaturesRequest.fromJSON"></a>

### FeaturesRequest.fromJSON(value) ⇒ [<code>FeaturesRequest</code>](#FeaturesRequest)
**Kind**: static method of [<code>FeaturesRequest</code>](#FeaturesRequest)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="FeaturesResponse"></a>

## FeaturesResponse
**Kind**: global class  

* [FeaturesResponse](#FeaturesResponse)
    * _instance_
        * [.toJSON()](#FeaturesResponse+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(value)](#FeaturesResponse.fromJSON) ⇒ [<code>FeaturesResponse</code>](#FeaturesResponse)

<a name="FeaturesResponse+toJSON"></a>

### featuresResponse.toJSON() ⇒ <code>any</code>
**Kind**: instance method of [<code>FeaturesResponse</code>](#FeaturesResponse)  
<a name="FeaturesResponse.fromJSON"></a>

### FeaturesResponse.fromJSON(value) ⇒ [<code>FeaturesResponse</code>](#FeaturesResponse)
**Kind**: static method of [<code>FeaturesResponse</code>](#FeaturesResponse)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="IntegrationChainHistory"></a>

## IntegrationChainHistory
**Kind**: global class  

* [IntegrationChainHistory](#IntegrationChainHistory)
    * _instance_
        * [.chainData()](#IntegrationChainHistory+chainData) ⇒ <code>Array.&lt;any&gt;</code>
        * [.spam()](#IntegrationChainHistory+spam) ⇒ <code>Array.&lt;any&gt;</code>
        * [.toJSON()](#IntegrationChainHistory+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#IntegrationChainHistory.fromJSON) ⇒ [<code>IntegrationChainHistory</code>](#IntegrationChainHistory)

<a name="IntegrationChainHistory+chainData"></a>

### integrationChainHistory.chainData() ⇒ <code>Array.&lt;any&gt;</code>
Returns a [`js_sys::Array`] of `$wasm_ty` as strings.

NOTE: this clones the field.

**Kind**: instance method of [<code>IntegrationChainHistory</code>](#IntegrationChainHistory)  
<a name="IntegrationChainHistory+spam"></a>

### integrationChainHistory.spam() ⇒ <code>Array.&lt;any&gt;</code>
Returns a [`js_sys::Array`] of [`MessageIds`][MessageId] as strings.

NOTE: this clones the field.

**Kind**: instance method of [<code>IntegrationChainHistory</code>](#IntegrationChainHistory)  
<a name="IntegrationChainHistory+toJSON"></a>

### integrationChainHistory.toJSON() ⇒ <code>any</code>
Serializes a `$ident` object as a JSON object.

**Kind**: instance method of [<code>IntegrationChainHistory</code>](#IntegrationChainHistory)  
<a name="IntegrationChainHistory.fromJSON"></a>

### IntegrationChainHistory.fromJSON(json) ⇒ [<code>IntegrationChainHistory</code>](#IntegrationChainHistory)
Deserializes a `$ident` object from a JSON object.

**Kind**: static method of [<code>IntegrationChainHistory</code>](#IntegrationChainHistory)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="Introduction"></a>

## Introduction
**Kind**: global class  

* [Introduction](#Introduction)
    * _instance_
        * [.toJSON()](#Introduction+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(value)](#Introduction.fromJSON) ⇒ [<code>Introduction</code>](#Introduction)

<a name="Introduction+toJSON"></a>

### introduction.toJSON() ⇒ <code>any</code>
**Kind**: instance method of [<code>Introduction</code>](#Introduction)  
<a name="Introduction.fromJSON"></a>

### Introduction.fromJSON(value) ⇒ [<code>Introduction</code>](#Introduction)
**Kind**: static method of [<code>Introduction</code>](#Introduction)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="IntroductionProposal"></a>

## IntroductionProposal
**Kind**: global class  

* [IntroductionProposal](#IntroductionProposal)
    * _instance_
        * [.toJSON()](#IntroductionProposal+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(value)](#IntroductionProposal.fromJSON) ⇒ [<code>IntroductionProposal</code>](#IntroductionProposal)

<a name="IntroductionProposal+toJSON"></a>

### introductionProposal.toJSON() ⇒ <code>any</code>
**Kind**: instance method of [<code>IntroductionProposal</code>](#IntroductionProposal)  
<a name="IntroductionProposal.fromJSON"></a>

### IntroductionProposal.fromJSON(value) ⇒ [<code>IntroductionProposal</code>](#IntroductionProposal)
**Kind**: static method of [<code>IntroductionProposal</code>](#IntroductionProposal)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="IntroductionResponse"></a>

## IntroductionResponse
**Kind**: global class  

* [IntroductionResponse](#IntroductionResponse)
    * _instance_
        * [.toJSON()](#IntroductionResponse+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(value)](#IntroductionResponse.fromJSON) ⇒ [<code>IntroductionResponse</code>](#IntroductionResponse)

<a name="IntroductionResponse+toJSON"></a>

### introductionResponse.toJSON() ⇒ <code>any</code>
**Kind**: instance method of [<code>IntroductionResponse</code>](#IntroductionResponse)  
<a name="IntroductionResponse.fromJSON"></a>

### IntroductionResponse.fromJSON(value) ⇒ [<code>IntroductionResponse</code>](#IntroductionResponse)
**Kind**: static method of [<code>IntroductionResponse</code>](#IntroductionResponse)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

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
        * [.private(index)](#KeyCollection+private) ⇒ <code>string</code> \| <code>undefined</code>
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

<a name="KeyCollection+private"></a>

### keyCollection.private(index) ⇒ <code>string</code> \| <code>undefined</code>
Returns the private key at the specified `index` as a base58-encoded string.

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
        * [.private](#KeyPair+private) ⇒ <code>string</code>
        * [.toJSON()](#KeyPair+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromBase58(type_, public_key, private_key)](#KeyPair.fromBase58) ⇒ [<code>KeyPair</code>](#KeyPair)
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
<a name="KeyPair+private"></a>

### keyPair.private ⇒ <code>string</code>
Returns the private key as a base58-encoded string.

**Kind**: instance property of [<code>KeyPair</code>](#KeyPair)  
<a name="KeyPair+toJSON"></a>

### keyPair.toJSON() ⇒ <code>any</code>
Serializes a `KeyPair` object as a JSON object.

**Kind**: instance method of [<code>KeyPair</code>](#KeyPair)  
<a name="KeyPair.fromBase58"></a>

### KeyPair.fromBase58(type_, public_key, private_key) ⇒ [<code>KeyPair</code>](#KeyPair)
Parses a `KeyPair` object from base58-encoded public/private keys.

**Kind**: static method of [<code>KeyPair</code>](#KeyPair)  

| Param | Type |
| --- | --- |
| type_ | <code>number</code> | 
| public_key | <code>string</code> | 
| private_key | <code>string</code> | 

<a name="KeyPair.fromJSON"></a>

### KeyPair.fromJSON(json) ⇒ [<code>KeyPair</code>](#KeyPair)
Deserializes a `KeyPair` object from a JSON object.

**Kind**: static method of [<code>KeyPair</code>](#KeyPair)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="Network"></a>

## Network
**Kind**: global class  

* [Network](#Network)
    * _instance_
        * [.defaultNodeURL](#Network+defaultNodeURL) ⇒ <code>string</code> \| <code>undefined</code>
        * [.explorerURL](#Network+explorerURL) ⇒ <code>string</code> \| <code>undefined</code>
        * [.messageURL(message_id)](#Network+messageURL) ⇒ <code>string</code>
        * [.toString()](#Network+toString) ⇒ <code>string</code>
    * _static_
        * [.try_from_name(name)](#Network.try_from_name) ⇒ [<code>Network</code>](#Network)
        * [.mainnet()](#Network.mainnet) ⇒ [<code>Network</code>](#Network)
        * [.devnet()](#Network.devnet) ⇒ [<code>Network</code>](#Network)

<a name="Network+defaultNodeURL"></a>

### network.defaultNodeURL ⇒ <code>string</code> \| <code>undefined</code>
Returns the node URL of the Tangle network.

**Kind**: instance property of [<code>Network</code>](#Network)  
<a name="Network+explorerURL"></a>

### network.explorerURL ⇒ <code>string</code> \| <code>undefined</code>
Returns the web explorer URL of the Tangle network.

**Kind**: instance property of [<code>Network</code>](#Network)  
<a name="Network+messageURL"></a>

### network.messageURL(message_id) ⇒ <code>string</code>
Returns the web explorer URL of the given `message`.

**Kind**: instance method of [<code>Network</code>](#Network)  

| Param | Type |
| --- | --- |
| message_id | <code>string</code> | 

<a name="Network+toString"></a>

### network.toString() ⇒ <code>string</code>
**Kind**: instance method of [<code>Network</code>](#Network)  
<a name="Network.try_from_name"></a>

### Network.try\_from\_name(name) ⇒ [<code>Network</code>](#Network)
Parses the provided string to a [`WasmNetwork`].

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
<a name="PresentationRequest"></a>

## PresentationRequest
**Kind**: global class  

* [PresentationRequest](#PresentationRequest)
    * _instance_
        * [.toJSON()](#PresentationRequest+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(value)](#PresentationRequest.fromJSON) ⇒ [<code>PresentationRequest</code>](#PresentationRequest)

<a name="PresentationRequest+toJSON"></a>

### presentationRequest.toJSON() ⇒ <code>any</code>
**Kind**: instance method of [<code>PresentationRequest</code>](#PresentationRequest)  
<a name="PresentationRequest.fromJSON"></a>

### PresentationRequest.fromJSON(value) ⇒ [<code>PresentationRequest</code>](#PresentationRequest)
**Kind**: static method of [<code>PresentationRequest</code>](#PresentationRequest)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="PresentationResponse"></a>

## PresentationResponse
**Kind**: global class  

* [PresentationResponse](#PresentationResponse)
    * _instance_
        * [.toJSON()](#PresentationResponse+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(value)](#PresentationResponse.fromJSON) ⇒ [<code>PresentationResponse</code>](#PresentationResponse)

<a name="PresentationResponse+toJSON"></a>

### presentationResponse.toJSON() ⇒ <code>any</code>
**Kind**: instance method of [<code>PresentationResponse</code>](#PresentationResponse)  
<a name="PresentationResponse.fromJSON"></a>

### PresentationResponse.fromJSON(value) ⇒ [<code>PresentationResponse</code>](#PresentationResponse)
**Kind**: static method of [<code>PresentationResponse</code>](#PresentationResponse)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="ResolutionRequest"></a>

## ResolutionRequest
**Kind**: global class  

* [ResolutionRequest](#ResolutionRequest)
    * _instance_
        * [.toJSON()](#ResolutionRequest+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(value)](#ResolutionRequest.fromJSON) ⇒ [<code>ResolutionRequest</code>](#ResolutionRequest)

<a name="ResolutionRequest+toJSON"></a>

### resolutionRequest.toJSON() ⇒ <code>any</code>
**Kind**: instance method of [<code>ResolutionRequest</code>](#ResolutionRequest)  
<a name="ResolutionRequest.fromJSON"></a>

### ResolutionRequest.fromJSON(value) ⇒ [<code>ResolutionRequest</code>](#ResolutionRequest)
**Kind**: static method of [<code>ResolutionRequest</code>](#ResolutionRequest)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="ResolutionResponse"></a>

## ResolutionResponse
**Kind**: global class  

* [ResolutionResponse](#ResolutionResponse)
    * _instance_
        * [.toJSON()](#ResolutionResponse+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(value)](#ResolutionResponse.fromJSON) ⇒ [<code>ResolutionResponse</code>](#ResolutionResponse)

<a name="ResolutionResponse+toJSON"></a>

### resolutionResponse.toJSON() ⇒ <code>any</code>
**Kind**: instance method of [<code>ResolutionResponse</code>](#ResolutionResponse)  
<a name="ResolutionResponse.fromJSON"></a>

### ResolutionResponse.fromJSON(value) ⇒ [<code>ResolutionResponse</code>](#ResolutionResponse)
**Kind**: static method of [<code>ResolutionResponse</code>](#ResolutionResponse)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

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
Deserializes a `Service` object from a JSON object.

**Kind**: static method of [<code>Service</code>](#Service)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="Timestamp"></a>

## Timestamp
**Kind**: global class  

* [Timestamp](#Timestamp)
    * _instance_
        * [.toRFC3339()](#Timestamp+toRFC3339) ⇒ <code>string</code>
    * _static_
        * [.parse(input)](#Timestamp.parse) ⇒ [<code>Timestamp</code>](#Timestamp)
        * [.nowUTC()](#Timestamp.nowUTC) ⇒ [<code>Timestamp</code>](#Timestamp)

<a name="Timestamp+toRFC3339"></a>

### timestamp.toRFC3339() ⇒ <code>string</code>
Returns the `Timestamp` as an RFC 3339 `String`.

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
<a name="Timing"></a>

## Timing
**Kind**: global class  

* [Timing](#Timing)
    * _instance_
        * [.outTime](#Timing+outTime) ⇒ <code>string</code> \| <code>undefined</code>
        * [.outTime](#Timing+outTime)
        * [.inTime](#Timing+inTime) ⇒ <code>string</code> \| <code>undefined</code>
        * [.inTime](#Timing+inTime)
        * [.staleTime](#Timing+staleTime) ⇒ <code>string</code> \| <code>undefined</code>
        * [.staleTime](#Timing+staleTime)
        * [.expiresTime](#Timing+expiresTime) ⇒ <code>string</code> \| <code>undefined</code>
        * [.expiresTime](#Timing+expiresTime)
        * [.waitUntilTime](#Timing+waitUntilTime) ⇒ <code>string</code> \| <code>undefined</code>
        * [.waitUntilTime](#Timing+waitUntilTime)
        * [.delayMilli](#Timing+delayMilli) ⇒ <code>number</code> \| <code>undefined</code>
        * [.delayMilli](#Timing+delayMilli)
        * [.toJSON()](#Timing+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(value)](#Timing.fromJSON) ⇒ [<code>Timing</code>](#Timing)

<a name="Timing+outTime"></a>

### timing.outTime ⇒ <code>string</code> \| <code>undefined</code>
**Kind**: instance property of [<code>Timing</code>](#Timing)  
<a name="Timing+outTime"></a>

### timing.outTime
**Kind**: instance property of [<code>Timing</code>](#Timing)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="Timing+inTime"></a>

### timing.inTime ⇒ <code>string</code> \| <code>undefined</code>
**Kind**: instance property of [<code>Timing</code>](#Timing)  
<a name="Timing+inTime"></a>

### timing.inTime
**Kind**: instance property of [<code>Timing</code>](#Timing)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="Timing+staleTime"></a>

### timing.staleTime ⇒ <code>string</code> \| <code>undefined</code>
**Kind**: instance property of [<code>Timing</code>](#Timing)  
<a name="Timing+staleTime"></a>

### timing.staleTime
**Kind**: instance property of [<code>Timing</code>](#Timing)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="Timing+expiresTime"></a>

### timing.expiresTime ⇒ <code>string</code> \| <code>undefined</code>
**Kind**: instance property of [<code>Timing</code>](#Timing)  
<a name="Timing+expiresTime"></a>

### timing.expiresTime
**Kind**: instance property of [<code>Timing</code>](#Timing)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="Timing+waitUntilTime"></a>

### timing.waitUntilTime ⇒ <code>string</code> \| <code>undefined</code>
**Kind**: instance property of [<code>Timing</code>](#Timing)  
<a name="Timing+waitUntilTime"></a>

### timing.waitUntilTime
**Kind**: instance property of [<code>Timing</code>](#Timing)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="Timing+delayMilli"></a>

### timing.delayMilli ⇒ <code>number</code> \| <code>undefined</code>
**Kind**: instance property of [<code>Timing</code>](#Timing)  
<a name="Timing+delayMilli"></a>

### timing.delayMilli
**Kind**: instance property of [<code>Timing</code>](#Timing)  

| Param | Type |
| --- | --- |
| value | <code>number</code> | 

<a name="Timing+toJSON"></a>

### timing.toJSON() ⇒ <code>any</code>
**Kind**: instance method of [<code>Timing</code>](#Timing)  
<a name="Timing.fromJSON"></a>

### Timing.fromJSON(value) ⇒ [<code>Timing</code>](#Timing)
**Kind**: static method of [<code>Timing</code>](#Timing)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="TrustPing"></a>

## TrustPing
**Kind**: global class  

* [TrustPing](#TrustPing)
    * _instance_
        * [.toJSON()](#TrustPing+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(value)](#TrustPing.fromJSON) ⇒ [<code>TrustPing</code>](#TrustPing)

<a name="TrustPing+toJSON"></a>

### trustPing.toJSON() ⇒ <code>any</code>
**Kind**: instance method of [<code>TrustPing</code>](#TrustPing)  
<a name="TrustPing.fromJSON"></a>

### TrustPing.fromJSON(value) ⇒ [<code>TrustPing</code>](#TrustPing)
**Kind**: static method of [<code>TrustPing</code>](#TrustPing)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="TrustedIssuer"></a>

## TrustedIssuer
**Kind**: global class  

* [TrustedIssuer](#TrustedIssuer)
    * _instance_
        * [.toJSON()](#TrustedIssuer+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(value)](#TrustedIssuer.fromJSON) ⇒ [<code>TrustedIssuer</code>](#TrustedIssuer)

<a name="TrustedIssuer+toJSON"></a>

### trustedIssuer.toJSON() ⇒ <code>any</code>
**Kind**: instance method of [<code>TrustedIssuer</code>](#TrustedIssuer)  
<a name="TrustedIssuer.fromJSON"></a>

### TrustedIssuer.fromJSON(value) ⇒ [<code>TrustedIssuer</code>](#TrustedIssuer)
**Kind**: static method of [<code>TrustedIssuer</code>](#TrustedIssuer)  

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

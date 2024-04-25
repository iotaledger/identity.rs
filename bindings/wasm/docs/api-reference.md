## Classes

<dl>
<dt><a href="#CoreDID">CoreDID</a></dt>
<dd><p>A method-agnostic Decentralized Identifier (DID).</p>
</dd>
<dt><a href="#CoreDocument">CoreDocument</a></dt>
<dd><p>A method-agnostic DID Document.</p>
<p>Note: All methods that involve reading from this class may potentially raise an error
if the object is being concurrently modified.</p>
</dd>
<dt><a href="#Credential">Credential</a></dt>
<dd></dd>
<dt><a href="#CustomMethodData">CustomMethodData</a></dt>
<dd><p>A custom verification method data format.</p>
</dd>
<dt><a href="#DIDUrl">DIDUrl</a></dt>
<dd><p>A method agnostic DID Url.</p>
</dd>
<dt><a href="#DecodedJptCredential">DecodedJptCredential</a></dt>
<dd></dd>
<dt><a href="#DecodedJptPresentation">DecodedJptPresentation</a></dt>
<dd></dd>
<dt><a href="#DecodedJws">DecodedJws</a></dt>
<dd><p>A cryptographically verified decoded token from a JWS.</p>
<p>Contains the decoded headers and the raw claims.</p>
</dd>
<dt><a href="#DecodedJwtCredential">DecodedJwtCredential</a></dt>
<dd><p>A cryptographically verified and decoded Credential.</p>
<p>Note that having an instance of this type only means the JWS it was constructed from was verified.
It does not imply anything about a potentially present proof property on the credential itself.</p>
</dd>
<dt><a href="#DecodedJwtPresentation">DecodedJwtPresentation</a></dt>
<dd><p>A cryptographically verified and decoded presentation.</p>
<p>Note that having an instance of this type only means the JWS it was constructed from was verified.
It does not imply anything about a potentially present proof property on the presentation itself.</p>
</dd>
<dt><a href="#Disclosure">Disclosure</a></dt>
<dd><p>Represents an elements constructing a disclosure.
Object properties and array elements disclosures are supported.</p>
<p>See: <a href="https://www.ietf.org/archive/id/draft-ietf-oauth-selective-disclosure-jwt-07.html#name-disclosures">https://www.ietf.org/archive/id/draft-ietf-oauth-selective-disclosure-jwt-07.html#name-disclosures</a></p>
</dd>
<dt><a href="#DomainLinkageConfiguration">DomainLinkageConfiguration</a></dt>
<dd><p>DID Configuration Resource which contains Domain Linkage Credentials.
It can be placed in an origin&#39;s <code>.well-known</code> directory to prove linkage between the origin and a DID.
See: <a href="https://identity.foundation/.well-known/resources/did-configuration/#did-configuration-resource">https://identity.foundation/.well-known/resources/did-configuration/#did-configuration-resource</a></p>
<p>Note:</p>
<ul>
<li>Only the <a href="https://identity.foundation/.well-known/resources/did-configuration/#json-web-token-proof-format">JSON Web Token Proof Format</a></li>
</ul>
</dd>
<dt><a href="#Duration">Duration</a></dt>
<dd><p>A span of time.</p>
</dd>
<dt><a href="#EdDSAJwsVerifier">EdDSAJwsVerifier</a></dt>
<dd><p>An implementor of <code>IJwsVerifier</code> that can handle the
<code>EdDSA</code> algorithm.</p>
</dd>
<dt><a href="#IotaDID">IotaDID</a></dt>
<dd><p>A DID conforming to the IOTA DID method specification.</p>
</dd>
<dt><a href="#IotaDocument">IotaDocument</a></dt>
<dd><p>A DID Document adhering to the IOTA DID method specification.</p>
<p>Note: All methods that involve reading from this class may potentially raise an error
if the object is being concurrently modified.</p>
</dd>
<dt><a href="#IotaDocumentMetadata">IotaDocumentMetadata</a></dt>
<dd><p>Additional attributes related to an IOTA DID Document.</p>
</dd>
<dt><a href="#IotaIdentityClientExt">IotaIdentityClientExt</a></dt>
<dd><p>An extension interface that provides helper functions for publication
and resolution of DID documents in Alias Outputs.</p>
</dd>
<dt><a href="#IssuerProtectedHeader">IssuerProtectedHeader</a></dt>
<dd></dd>
<dt><a href="#Jpt">Jpt</a></dt>
<dd><p>A JSON Proof Token (JPT).</p>
</dd>
<dt><a href="#JptCredentialValidationOptions">JptCredentialValidationOptions</a></dt>
<dd><p>Options to declare validation criteria for <a href="#Jpt">Jpt</a>.</p>
</dd>
<dt><a href="#JptCredentialValidator">JptCredentialValidator</a></dt>
<dd></dd>
<dt><a href="#JptCredentialValidatorUtils">JptCredentialValidatorUtils</a></dt>
<dd><p>Utility functions for validating JPT credentials.</p>
</dd>
<dt><a href="#JptPresentationValidationOptions">JptPresentationValidationOptions</a></dt>
<dd><p>Options to declare validation criteria for a <a href="#Jpt">Jpt</a> presentation.</p>
</dd>
<dt><a href="#JptPresentationValidator">JptPresentationValidator</a></dt>
<dd></dd>
<dt><a href="#JptPresentationValidatorUtils">JptPresentationValidatorUtils</a></dt>
<dd><p>Utility functions for verifying JPT presentations.</p>
</dd>
<dt><a href="#Jwk">Jwk</a></dt>
<dd></dd>
<dt><a href="#JwkGenOutput">JwkGenOutput</a></dt>
<dd><p>The result of a key generation in <code>JwkStorage</code>.</p>
</dd>
<dt><a href="#JwpCredentialOptions">JwpCredentialOptions</a></dt>
<dd></dd>
<dt><a href="#JwpIssued">JwpIssued</a></dt>
<dd></dd>
<dt><a href="#JwpPresentationOptions">JwpPresentationOptions</a></dt>
<dd><p>Options to be set in the JWT claims of a verifiable presentation.</p>
</dd>
<dt><a href="#JwpVerificationOptions">JwpVerificationOptions</a></dt>
<dd></dd>
<dt><a href="#Jws">Jws</a></dt>
<dd><p>A wrapper around a JSON Web Signature (JWS).</p>
</dd>
<dt><a href="#JwsHeader">JwsHeader</a></dt>
<dd></dd>
<dt><a href="#JwsSignatureOptions">JwsSignatureOptions</a></dt>
<dd></dd>
<dt><a href="#JwsVerificationOptions">JwsVerificationOptions</a></dt>
<dd></dd>
<dt><a href="#Jwt">Jwt</a></dt>
<dd><p>A wrapper around a JSON Web Token (JWK).</p>
</dd>
<dt><a href="#JwtCredentialValidationOptions">JwtCredentialValidationOptions</a></dt>
<dd><p>Options to declare validation criteria when validating credentials.</p>
</dd>
<dt><a href="#JwtCredentialValidator">JwtCredentialValidator</a></dt>
<dd><p>A type for decoding and validating <a href="#Credential">Credential</a>.</p>
</dd>
<dt><a href="#JwtDomainLinkageValidator">JwtDomainLinkageValidator</a></dt>
<dd><p>A validator for a Domain Linkage Configuration and Credentials.</p>
</dd>
<dt><a href="#JwtPresentationOptions">JwtPresentationOptions</a></dt>
<dd></dd>
<dt><a href="#JwtPresentationValidationOptions">JwtPresentationValidationOptions</a></dt>
<dd><p>Options to declare validation criteria when validating presentation.</p>
</dd>
<dt><a href="#JwtPresentationValidator">JwtPresentationValidator</a></dt>
<dd></dd>
<dt><a href="#KeyBindingJWTValidationOptions">KeyBindingJWTValidationOptions</a></dt>
<dd><p>Options to declare validation criteria when validating credentials.</p>
</dd>
<dt><a href="#KeyBindingJwtClaims">KeyBindingJwtClaims</a></dt>
<dd><p>Claims set for key binding JWT.</p>
</dd>
<dt><a href="#LinkedDomainService">LinkedDomainService</a></dt>
<dd></dd>
<dt><a href="#MethodData">MethodData</a></dt>
<dd><p>Supported verification method data formats.</p>
</dd>
<dt><a href="#MethodDigest">MethodDigest</a></dt>
<dd><p>Unique identifier of a <a href="#VerificationMethod">VerificationMethod</a>.</p>
<p>NOTE:
This class does not have a JSON representation,
use the methods <code>pack</code> and <code>unpack</code> instead.</p>
</dd>
<dt><a href="#MethodScope">MethodScope</a></dt>
<dd><p>Supported verification method types.</p>
</dd>
<dt><a href="#MethodType">MethodType</a></dt>
<dd><p>Supported verification method types.</p>
</dd>
<dt><a href="#PayloadEntry">PayloadEntry</a></dt>
<dd></dd>
<dt><a href="#Payloads">Payloads</a></dt>
<dd></dd>
<dt><a href="#Presentation">Presentation</a></dt>
<dd></dd>
<dt><a href="#PresentationProtectedHeader">PresentationProtectedHeader</a></dt>
<dd></dd>
<dt><a href="#Proof">Proof</a></dt>
<dd><p>Represents a cryptographic proof that can be used to validate verifiable credentials and
presentations.</p>
<p>This representation does not inherently implement any standard; instead, it
can be utilized to implement standards or user-defined proofs. The presence of the
<code>type</code> field is necessary to accommodate different types of cryptographic proofs.</p>
<p>Note that this proof is not related to JWT and can be used in combination or as an alternative
to it.</p>
</dd>
<dt><a href="#ProofUpdateCtx">ProofUpdateCtx</a></dt>
<dd></dd>
<dt><a href="#Resolver">Resolver</a></dt>
<dd><p>Convenience type for resolving DID documents from different DID methods.</p>
<p>Also provides methods for resolving DID Documents associated with
verifiable <a href="#Credential">Credential</a>s and <a href="#Presentation">Presentation</a>s.</p>
<h1 id="configuration">Configuration</h1>
<p>The resolver will only be able to resolve DID documents for methods it has been configured for in the constructor.</p>
</dd>
<dt><a href="#RevocationBitmap">RevocationBitmap</a></dt>
<dd><p>A compressed bitmap for managing credential revocation.</p>
</dd>
<dt><a href="#RevocationTimeframeStatus">RevocationTimeframeStatus</a></dt>
<dd><p>Information used to determine the current status of a <a href="#Credential">Credential</a>.</p>
</dd>
<dt><a href="#SdJwt">SdJwt</a></dt>
<dd><p>Representation of an SD-JWT of the format
<code>&lt;Issuer-signed JWT&gt;~&lt;Disclosure 1&gt;~&lt;Disclosure 2&gt;~...~&lt;Disclosure N&gt;~&lt;optional KB-JWT&gt;</code>.</p>
</dd>
<dt><a href="#SdJwtCredentialValidator">SdJwtCredentialValidator</a></dt>
<dd><p>A type for decoding and validating <a href="#Credential">Credential</a>.</p>
</dd>
<dt><a href="#SdObjectDecoder">SdObjectDecoder</a></dt>
<dd><p>Substitutes digests in an SD-JWT object by their corresponding plaintext values provided by disclosures.</p>
</dd>
<dt><a href="#SdObjectEncoder">SdObjectEncoder</a></dt>
<dd><p>Transforms a JSON object into an SD-JWT object by substituting selected values
with their corresponding disclosure digests.</p>
<p>Note: digests are created using the sha-256 algorithm.</p>
</dd>
<dt><a href="#SelectiveDisclosurePresentation">SelectiveDisclosurePresentation</a></dt>
<dd><p>Used to construct a JwpPresentedBuilder and handle the selective disclosure of attributes</p>
<ul>
<li>@context MUST NOT be blinded</li>
<li>id MUST be blinded</li>
<li>type MUST NOT be blinded</li>
<li>issuer MUST NOT be blinded</li>
<li>issuanceDate MUST be blinded (if Timeframe Revocation mechanism is used)</li>
<li>expirationDate MUST be blinded (if Timeframe Revocation mechanism is used)</li>
<li>credentialSubject (User have to choose which attribute must be blinded)</li>
<li>credentialSchema MUST NOT be blinded</li>
<li>credentialStatus MUST NOT be blinded</li>
<li>refreshService MUST NOT be blinded (probably will be used for Timeslot Revocation mechanism)</li>
<li>termsOfUse NO reason to use it in ZK VC (will be in any case blinded)</li>
<li>evidence (User have to choose which attribute must be blinded)</li>
</ul>
</dd>
<dt><a href="#Service">Service</a></dt>
<dd><p>A DID Document Service used to enable trusted interactions associated with a DID subject.</p>
</dd>
<dt><a href="#StatusList2021">StatusList2021</a></dt>
<dd><p>StatusList2021 data structure as described in <a href="https://www.w3.org/TR/2023/WD-vc-status-list-20230427/">W3C&#39;s VC status list 2021</a>.</p>
</dd>
<dt><a href="#StatusList2021Credential">StatusList2021Credential</a></dt>
<dd><p>A parsed <a href="https://www.w3.org/TR/2023/WD-vc-status-list-20230427/#statuslist2021credential">StatusList2021Credential</a>.</p>
</dd>
<dt><a href="#StatusList2021CredentialBuilder">StatusList2021CredentialBuilder</a></dt>
<dd><p>Builder type to construct valid <a href="#StatusList2021Credential">StatusList2021Credential</a> istances.</p>
</dd>
<dt><a href="#StatusList2021Entry">StatusList2021Entry</a></dt>
<dd><p><a href="https://www.w3.org/TR/2023/WD-vc-status-list-20230427/#statuslist2021entry">StatusList2021Entry</a> implementation.</p>
</dd>
<dt><a href="#Storage">Storage</a></dt>
<dd><p>A type wrapping a <code>JwkStorage</code> and <code>KeyIdStorage</code> that should always be used together when
working with storage backed DID documents.</p>
</dd>
<dt><a href="#Timestamp">Timestamp</a></dt>
<dd></dd>
<dt><a href="#UnknownCredential">UnknownCredential</a></dt>
<dd></dd>
<dt><a href="#VerificationMethod">VerificationMethod</a></dt>
<dd><p>A DID Document Verification Method.</p>
</dd>
</dl>

## Members

<dl>
<dt><a href="#PresentationProofAlgorithm">PresentationProofAlgorithm</a></dt>
<dd></dd>
<dt><a href="#ProofAlgorithm">ProofAlgorithm</a></dt>
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
<dt><a href="#SerializationType">SerializationType</a></dt>
<dd></dd>
<dt><a href="#MethodRelationship">MethodRelationship</a></dt>
<dd></dd>
<dt><a href="#SubjectHolderRelationship">SubjectHolderRelationship</a></dt>
<dd><p>Declares how credential subjects must relate to the presentation holder.</p>
<p>See also the <a href="https://www.w3.org/TR/vc-data-model/#subject-holder-relationships">Subject-Holder Relationship</a> section of the specification.</p>
</dd>
<dt><a href="#AlwaysSubject">AlwaysSubject</a></dt>
<dd><p>The holder must always match the subject on all credentials, regardless of their <a href="https://www.w3.org/TR/vc-data-model/#nontransferable-property"><code>nonTransferable</code></a> property.
This variant is the default.</p>
</dd>
<dt><a href="#SubjectOnNonTransferable">SubjectOnNonTransferable</a></dt>
<dd><p>The holder must match the subject only for credentials where the <a href="https://www.w3.org/TR/vc-data-model/#nontransferable-property"><code>nonTransferable</code></a> property is <code>true</code>.</p>
</dd>
<dt><a href="#Any">Any</a></dt>
<dd><p>The holder is not required to have any kind of relationship to any credential subject.</p>
</dd>
<dt><a href="#CredentialStatus">CredentialStatus</a></dt>
<dd></dd>
<dt><a href="#StatusPurpose">StatusPurpose</a></dt>
<dd><p>Purpose of a <a href="#StatusList2021">StatusList2021</a>.</p>
</dd>
<dt><a href="#StateMetadataEncoding">StateMetadataEncoding</a></dt>
<dd></dd>
<dt><a href="#FailFast">FailFast</a></dt>
<dd><p>Declares when validation should return if an error occurs.</p>
</dd>
<dt><a href="#AllErrors">AllErrors</a></dt>
<dd><p>Return all errors that occur during validation.</p>
</dd>
<dt><a href="#FirstError">FirstError</a></dt>
<dd><p>Return after the first error occurs.</p>
</dd>
<dt><a href="#PayloadType">PayloadType</a></dt>
<dd></dd>
<dt><a href="#MethodRelationship">MethodRelationship</a></dt>
<dd></dd>
<dt><a href="#CredentialStatus">CredentialStatus</a></dt>
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
</dl>

## Functions

<dl>
<dt><a href="#verifyEd25519">verifyEd25519(alg, signingInput, decodedSignature, publicKey)</a></dt>
<dd><p>Verify a JWS signature secured with the <code>EdDSA</code> algorithm and curve <code>Ed25519</code>.</p>
<p>This function is useful when one is composing a <code>IJwsVerifier</code> that delegates
<code>EdDSA</code> verification with curve <code>Ed25519</code> to this function.</p>
<h1 id="warning">Warning</h1>
<p>This function does not check whether <code>alg = EdDSA</code> in the protected header. Callers are expected to assert this
prior to calling the function.</p>
</dd>
<dt><a href="#start">start()</a></dt>
<dd><p>Initializes the console error panic hook for better error messages</p>
</dd>
<dt><a href="#encodeB64">encodeB64(data)</a> ⇒ <code>string</code></dt>
<dd><p>Encode the given bytes in url-safe base64.</p>
</dd>
<dt><a href="#decodeB64">decodeB64(data)</a> ⇒ <code>Uint8Array</code></dt>
<dd><p>Decode the given url-safe base64-encoded slice into its raw bytes.</p>
</dd>
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
Set the method name of the [CoreDID](#CoreDID).

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
Returns the [CoreDID](#CoreDID) scheme.

E.g.
- `"did:example:12345678" -> "did"`
- `"did:iota:smr:12345678" -> "did"`

**Kind**: instance method of [<code>CoreDID</code>](#CoreDID)  
<a name="CoreDID+authority"></a>

### coreDID.authority() ⇒ <code>string</code>
Returns the [CoreDID](#CoreDID) authority: the method name and method-id.

E.g.
- `"did:example:12345678" -> "example:12345678"`
- `"did:iota:smr:12345678" -> "iota:smr:12345678"`

**Kind**: instance method of [<code>CoreDID</code>](#CoreDID)  
<a name="CoreDID+method"></a>

### coreDID.method() ⇒ <code>string</code>
Returns the [CoreDID](#CoreDID) method name.

E.g.
- `"did:example:12345678" -> "example"`
- `"did:iota:smr:12345678" -> "iota"`

**Kind**: instance method of [<code>CoreDID</code>](#CoreDID)  
<a name="CoreDID+methodId"></a>

### coreDID.methodId() ⇒ <code>string</code>
Returns the [CoreDID](#CoreDID) method-specific ID.

E.g.
- `"did:example:12345678" -> "12345678"`
- `"did:iota:smr:12345678" -> "smr:12345678"`

**Kind**: instance method of [<code>CoreDID</code>](#CoreDID)  
<a name="CoreDID+join"></a>

### coreDID.join(segment) ⇒ [<code>DIDUrl</code>](#DIDUrl)
Construct a new [DIDUrl](#DIDUrl) by joining with a relative DID Url string.

**Kind**: instance method of [<code>CoreDID</code>](#CoreDID)  

| Param | Type |
| --- | --- |
| segment | <code>string</code> | 

<a name="CoreDID+toUrl"></a>

### coreDID.toUrl() ⇒ [<code>DIDUrl</code>](#DIDUrl)
Clones the [CoreDID](#CoreDID) into a [DIDUrl](#DIDUrl).

**Kind**: instance method of [<code>CoreDID</code>](#CoreDID)  
<a name="CoreDID+intoUrl"></a>

### coreDID.intoUrl() ⇒ [<code>DIDUrl</code>](#DIDUrl)
Converts the [CoreDID](#CoreDID) into a [DIDUrl](#DIDUrl), consuming it.

**Kind**: instance method of [<code>CoreDID</code>](#CoreDID)  
<a name="CoreDID+toString"></a>

### coreDID.toString() ⇒ <code>string</code>
Returns the [CoreDID](#CoreDID) as a string.

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
Parses a [CoreDID](#CoreDID) from the given `input`.

### Errors

Throws an error if the input is not a valid [CoreDID](#CoreDID).

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

Note: All methods that involve reading from this class may potentially raise an error
if the object is being concurrently modified.

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
        * [.methods([scope])](#CoreDocument+methods) ⇒ [<code>Array.&lt;VerificationMethod&gt;</code>](#VerificationMethod)
        * [.verificationRelationships()](#CoreDocument+verificationRelationships) ⇒ <code>Array.&lt;(DIDUrl\|VerificationMethod)&gt;</code>
        * [.insertMethod(method, scope)](#CoreDocument+insertMethod)
        * [.removeMethod(did)](#CoreDocument+removeMethod) ⇒ [<code>VerificationMethod</code>](#VerificationMethod) \| <code>undefined</code>
        * [.resolveMethod(query, [scope])](#CoreDocument+resolveMethod) ⇒ [<code>VerificationMethod</code>](#VerificationMethod) \| <code>undefined</code>
        * [.attachMethodRelationship(didUrl, relationship)](#CoreDocument+attachMethodRelationship) ⇒ <code>boolean</code>
        * [.detachMethodRelationship(didUrl, relationship)](#CoreDocument+detachMethodRelationship) ⇒ <code>boolean</code>
        * [.verifyJws(jws, options, signatureVerifier, [detachedPayload])](#CoreDocument+verifyJws) ⇒ [<code>DecodedJws</code>](#DecodedJws)
        * [.revokeCredentials(serviceQuery, indices)](#CoreDocument+revokeCredentials)
        * [.unrevokeCredentials(serviceQuery, indices)](#CoreDocument+unrevokeCredentials)
        * [.clone()](#CoreDocument+clone) ⇒ [<code>CoreDocument</code>](#CoreDocument)
        * [._shallowCloneInternal()](#CoreDocument+_shallowCloneInternal) ⇒ [<code>CoreDocument</code>](#CoreDocument)
        * [._strongCountInternal()](#CoreDocument+_strongCountInternal) ⇒ <code>number</code>
        * [.toJSON()](#CoreDocument+toJSON) ⇒ <code>any</code>
        * [.generateMethod(storage, keyType, alg, fragment, scope)](#CoreDocument+generateMethod) ⇒ <code>Promise.&lt;string&gt;</code>
        * [.purgeMethod(storage, id)](#CoreDocument+purgeMethod) ⇒ <code>Promise.&lt;void&gt;</code>
        * [.createJws(storage, fragment, payload, options)](#CoreDocument+createJws) ⇒ [<code>Promise.&lt;Jws&gt;</code>](#Jws)
        * [.createCredentialJwt(storage, fragment, credential, options, [custom_claims])](#CoreDocument+createCredentialJwt) ⇒ [<code>Promise.&lt;Jwt&gt;</code>](#Jwt)
        * [.createPresentationJwt(storage, fragment, presentation, signature_options, presentation_options)](#CoreDocument+createPresentationJwt) ⇒ [<code>Promise.&lt;Jwt&gt;</code>](#Jwt)
    * _static_
        * [.fromJSON(json)](#CoreDocument.fromJSON) ⇒ [<code>CoreDocument</code>](#CoreDocument)

<a name="new_CoreDocument_new"></a>

### new CoreDocument(values)
Creates a new [CoreDocument](#CoreDocument) with the given properties.


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
`resolve_method`, `resolve_service` and the related
[DID URL dereferencing](https://w3c-ccg.github.io/did-resolution/#dereferencing) algorithm.

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

### coreDocument.methods([scope]) ⇒ [<code>Array.&lt;VerificationMethod&gt;</code>](#VerificationMethod)
Returns a list of all [VerificationMethod](#VerificationMethod) in the DID Document,
whose verification relationship matches `scope`.

If `scope` is not set, a list over the **embedded** methods is returned.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| [scope] | [<code>MethodScope</code>](#MethodScope) \| <code>undefined</code> | 

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

### coreDocument.resolveMethod(query, [scope]) ⇒ [<code>VerificationMethod</code>](#VerificationMethod) \| <code>undefined</code>
Returns a copy of the first verification method with an `id` property
matching the provided `query` and the verification relationship
specified by `scope`, if present.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| query | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| [scope] | [<code>MethodScope</code>](#MethodScope) \| <code>undefined</code> | 

<a name="CoreDocument+attachMethodRelationship"></a>

### coreDocument.attachMethodRelationship(didUrl, relationship) ⇒ <code>boolean</code>
Attaches the relationship to the given method, if the method exists.

Note: The method needs to be in the set of verification methods,
so it cannot be an embedded one.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| didUrl | [<code>DIDUrl</code>](#DIDUrl) | 
| relationship | [<code>MethodRelationship</code>](#MethodRelationship) | 

<a name="CoreDocument+detachMethodRelationship"></a>

### coreDocument.detachMethodRelationship(didUrl, relationship) ⇒ <code>boolean</code>
Detaches the given relationship from the given method, if the method exists.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| didUrl | [<code>DIDUrl</code>](#DIDUrl) | 
| relationship | [<code>MethodRelationship</code>](#MethodRelationship) | 

<a name="CoreDocument+verifyJws"></a>

### coreDocument.verifyJws(jws, options, signatureVerifier, [detachedPayload]) ⇒ [<code>DecodedJws</code>](#DecodedJws)
Decodes and verifies the provided JWS according to the passed `options` and `signatureVerifier`.
 If no `signatureVerifier` argument is provided a default verifier will be used that is (only) capable of
verifying EdDSA signatures.

Regardless of which options are passed the following conditions must be met in order for a verification attempt to
take place.
- The JWS must be encoded according to the JWS compact serialization.
- The `kid` value in the protected header must be an identifier of a verification method in this DID document,
or set explicitly in the `options`.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| jws | [<code>Jws</code>](#Jws) | 
| options | [<code>JwsVerificationOptions</code>](#JwsVerificationOptions) | 
| signatureVerifier | <code>IJwsVerifier</code> | 
| [detachedPayload] | <code>string</code> \| <code>undefined</code> | 

<a name="CoreDocument+revokeCredentials"></a>

### coreDocument.revokeCredentials(serviceQuery, indices)
If the document has a [RevocationBitmap](#RevocationBitmap) service identified by `serviceQuery`,
revoke all specified `indices`.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| serviceQuery | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| indices | <code>number</code> \| <code>Array.&lt;number&gt;</code> | 

<a name="CoreDocument+unrevokeCredentials"></a>

### coreDocument.unrevokeCredentials(serviceQuery, indices)
If the document has a [RevocationBitmap](#RevocationBitmap) service identified by `serviceQuery`,
unrevoke all specified `indices`.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| serviceQuery | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| indices | <code>number</code> \| <code>Array.&lt;number&gt;</code> | 

<a name="CoreDocument+clone"></a>

### coreDocument.clone() ⇒ [<code>CoreDocument</code>](#CoreDocument)
Deep clones the [CoreDocument](#CoreDocument).

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  
<a name="CoreDocument+_shallowCloneInternal"></a>

### coreDocument.\_shallowCloneInternal() ⇒ [<code>CoreDocument</code>](#CoreDocument)
### Warning
This is for internal use only. Do not rely on or call this method.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  
<a name="CoreDocument+_strongCountInternal"></a>

### coreDocument.\_strongCountInternal() ⇒ <code>number</code>
### Warning
This is for internal use only. Do not rely on or call this method.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  
<a name="CoreDocument+toJSON"></a>

### coreDocument.toJSON() ⇒ <code>any</code>
Serializes to a plain JS representation.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  
<a name="CoreDocument+generateMethod"></a>

### coreDocument.generateMethod(storage, keyType, alg, fragment, scope) ⇒ <code>Promise.&lt;string&gt;</code>
Generate new key material in the given `storage` and insert a new verification method with the corresponding
public key material into the DID document.

- If no fragment is given the `kid` of the generated JWK is used, if it is set, otherwise an error is returned.
- The `keyType` must be compatible with the given `storage`. `Storage`s are expected to export key type constants
for that use case.

The fragment of the generated method is returned.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| storage | [<code>Storage</code>](#Storage) | 
| keyType | <code>string</code> | 
| alg | <code>JwsAlgorithm</code> | 
| fragment | <code>string</code> \| <code>undefined</code> | 
| scope | [<code>MethodScope</code>](#MethodScope) | 

<a name="CoreDocument+purgeMethod"></a>

### coreDocument.purgeMethod(storage, id) ⇒ <code>Promise.&lt;void&gt;</code>
Remove the method identified by the `fragment` from the document and delete the corresponding key material in
the `storage`.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| storage | [<code>Storage</code>](#Storage) | 
| id | [<code>DIDUrl</code>](#DIDUrl) | 

<a name="CoreDocument+createJws"></a>

### coreDocument.createJws(storage, fragment, payload, options) ⇒ [<code>Promise.&lt;Jws&gt;</code>](#Jws)
Sign the `payload` according to `options` with the storage backed private key corresponding to the public key
material in the verification method identified by the given `fragment.

Upon success a string representing a JWS encoded according to the Compact JWS Serialization format is returned.
See [RFC7515 section 3.1](https://www.rfc-editor.org/rfc/rfc7515#section-3.1).

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| storage | [<code>Storage</code>](#Storage) | 
| fragment | <code>string</code> | 
| payload | <code>string</code> | 
| options | [<code>JwsSignatureOptions</code>](#JwsSignatureOptions) | 

<a name="CoreDocument+createCredentialJwt"></a>

### coreDocument.createCredentialJwt(storage, fragment, credential, options, [custom_claims]) ⇒ [<code>Promise.&lt;Jwt&gt;</code>](#Jwt)
Produces a JWT where the payload is produced from the given `credential`
in accordance with [VC Data Model v1.1](https://www.w3.org/TR/vc-data-model/#json-web-token).

Unless the `kid` is explicitly set in the options, the `kid` in the protected header is the `id`
of the method identified by `fragment` and the JWS signature will be produced by the corresponding
private key backed by the `storage` in accordance with the passed `options`.

The `custom_claims` can be used to set additional claims on the resulting JWT.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| storage | [<code>Storage</code>](#Storage) | 
| fragment | <code>string</code> | 
| credential | [<code>Credential</code>](#Credential) | 
| options | [<code>JwsSignatureOptions</code>](#JwsSignatureOptions) | 
| [custom_claims] | <code>Record.&lt;string, any&gt;</code> \| <code>undefined</code> | 

<a name="CoreDocument+createPresentationJwt"></a>

### coreDocument.createPresentationJwt(storage, fragment, presentation, signature_options, presentation_options) ⇒ [<code>Promise.&lt;Jwt&gt;</code>](#Jwt)
Produces a JWT where the payload is produced from the given presentation.
in accordance with [VC Data Model v1.1](https://www.w3.org/TR/vc-data-model/#json-web-token).

Unless the `kid` is explicitly set in the options, the `kid` in the protected header is the `id`
of the method identified by `fragment` and the JWS signature will be produced by the corresponding
private key backed by the `storage` in accordance with the passed `options`.

**Kind**: instance method of [<code>CoreDocument</code>](#CoreDocument)  

| Param | Type |
| --- | --- |
| storage | [<code>Storage</code>](#Storage) | 
| fragment | <code>string</code> | 
| presentation | [<code>Presentation</code>](#Presentation) | 
| signature_options | [<code>JwsSignatureOptions</code>](#JwsSignatureOptions) | 
| presentation_options | [<code>JwtPresentationOptions</code>](#JwtPresentationOptions) | 

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
        * [.setProof([proof])](#Credential+setProof)
        * [.toJwtClaims([custom_claims])](#Credential+toJwtClaims) ⇒ <code>Record.&lt;string, any&gt;</code>
        * [.toJSON()](#Credential+toJSON) ⇒ <code>any</code>
        * [.clone()](#Credential+clone) ⇒ [<code>Credential</code>](#Credential)
    * _static_
        * [.BaseContext()](#Credential.BaseContext) ⇒ <code>string</code>
        * [.BaseType()](#Credential.BaseType) ⇒ <code>string</code>
        * [.createDomainLinkageCredential(values)](#Credential.createDomainLinkageCredential) ⇒ [<code>Credential</code>](#Credential)
        * [.fromJSON(json)](#Credential.fromJSON) ⇒ [<code>Credential</code>](#Credential)

<a name="new_Credential_new"></a>

### new Credential(values)
Constructs a new [Credential](#Credential).


| Param | Type |
| --- | --- |
| values | <code>ICredential</code> | 

<a name="Credential+context"></a>

### credential.context() ⇒ <code>Array.&lt;(string\|Record.&lt;string, any&gt;)&gt;</code>
Returns a copy of the JSON-LD context(s) applicable to the [Credential](#Credential).

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+id"></a>

### credential.id() ⇒ <code>string</code> \| <code>undefined</code>
Returns a copy of the unique `URI` identifying the [Credential](#Credential) .

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+type"></a>

### credential.type() ⇒ <code>Array.&lt;string&gt;</code>
Returns a copy of the URIs defining the type of the [Credential](#Credential).

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+credentialSubject"></a>

### credential.credentialSubject() ⇒ <code>Array.&lt;Subject&gt;</code>
Returns a copy of the [Credential](#Credential) subject(s).

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+issuer"></a>

### credential.issuer() ⇒ <code>string</code> \| <code>Issuer</code>
Returns a copy of the issuer of the [Credential](#Credential).

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+issuanceDate"></a>

### credential.issuanceDate() ⇒ [<code>Timestamp</code>](#Timestamp)
Returns a copy of the timestamp of when the [Credential](#Credential) becomes valid.

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+expirationDate"></a>

### credential.expirationDate() ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
Returns a copy of the timestamp of when the [Credential](#Credential) should no longer be considered valid.

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+credentialStatus"></a>

### credential.credentialStatus() ⇒ <code>Array.&lt;Status&gt;</code>
Returns a copy of the information used to determine the current status of the [Credential](#Credential).

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+credentialSchema"></a>

### credential.credentialSchema() ⇒ <code>Array.&lt;Schema&gt;</code>
Returns a copy of the information used to assist in the enforcement of a specific [Credential](#Credential) structure.

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+refreshService"></a>

### credential.refreshService() ⇒ <code>Array.&lt;RefreshService&gt;</code>
Returns a copy of the service(s) used to refresh an expired [Credential](#Credential).

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+termsOfUse"></a>

### credential.termsOfUse() ⇒ <code>Array.&lt;Policy&gt;</code>
Returns a copy of the terms-of-use specified by the [Credential](#Credential) issuer.

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+evidence"></a>

### credential.evidence() ⇒ <code>Array.&lt;Evidence&gt;</code>
Returns a copy of the human-readable evidence used to support the claims within the [Credential](#Credential).

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+nonTransferable"></a>

### credential.nonTransferable() ⇒ <code>boolean</code> \| <code>undefined</code>
Returns whether or not the [Credential](#Credential) must only be contained within a  [Presentation](#Presentation)
with a proof issued from the [Credential](#Credential) subject.

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+proof"></a>

### credential.proof() ⇒ [<code>Proof</code>](#Proof) \| <code>undefined</code>
Optional cryptographic proof, unrelated to JWT.

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+properties"></a>

### credential.properties() ⇒ <code>Map.&lt;string, any&gt;</code>
Returns a copy of the miscellaneous properties on the [Credential](#Credential).

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential+setProof"></a>

### credential.setProof([proof])
Sets the `proof` property of the [Credential](#Credential).

Note that this proof is not related to JWT.

**Kind**: instance method of [<code>Credential</code>](#Credential)  

| Param | Type |
| --- | --- |
| [proof] | [<code>Proof</code>](#Proof) \| <code>undefined</code> | 

<a name="Credential+toJwtClaims"></a>

### credential.toJwtClaims([custom_claims]) ⇒ <code>Record.&lt;string, any&gt;</code>
Serializes the `Credential` as a JWT claims set
in accordance with [VC Data Model v1.1](https://www.w3.org/TR/vc-data-model/#json-web-token).

The resulting object can be used as the payload of a JWS when issuing the credential.

**Kind**: instance method of [<code>Credential</code>](#Credential)  

| Param | Type |
| --- | --- |
| [custom_claims] | <code>Record.&lt;string, any&gt;</code> \| <code>undefined</code> | 

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

<a name="CustomMethodData"></a>

## CustomMethodData
A custom verification method data format.

**Kind**: global class  

* [CustomMethodData](#CustomMethodData)
    * [new CustomMethodData(name, data)](#new_CustomMethodData_new)
    * _instance_
        * [.clone()](#CustomMethodData+clone) ⇒ [<code>CustomMethodData</code>](#CustomMethodData)
        * [.toJSON()](#CustomMethodData+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#CustomMethodData.fromJSON) ⇒ [<code>CustomMethodData</code>](#CustomMethodData)

<a name="new_CustomMethodData_new"></a>

### new CustomMethodData(name, data)

| Param | Type |
| --- | --- |
| name | <code>string</code> | 
| data | <code>any</code> | 

<a name="CustomMethodData+clone"></a>

### customMethodData.clone() ⇒ [<code>CustomMethodData</code>](#CustomMethodData)
Deep clones the object.

**Kind**: instance method of [<code>CustomMethodData</code>](#CustomMethodData)  
<a name="CustomMethodData+toJSON"></a>

### customMethodData.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>CustomMethodData</code>](#CustomMethodData)  
<a name="CustomMethodData.fromJSON"></a>

### CustomMethodData.fromJSON(json) ⇒ [<code>CustomMethodData</code>](#CustomMethodData)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>CustomMethodData</code>](#CustomMethodData)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="DIDUrl"></a>

## DIDUrl
A method agnostic DID Url.

**Kind**: global class  

* [DIDUrl](#DIDUrl)
    * _instance_
        * [.did()](#DIDUrl+did) ⇒ [<code>CoreDID</code>](#CoreDID)
        * [.urlStr()](#DIDUrl+urlStr) ⇒ <code>string</code>
        * [.fragment()](#DIDUrl+fragment) ⇒ <code>string</code> \| <code>undefined</code>
        * [.setFragment([value])](#DIDUrl+setFragment)
        * [.path()](#DIDUrl+path) ⇒ <code>string</code> \| <code>undefined</code>
        * [.setPath([value])](#DIDUrl+setPath)
        * [.query()](#DIDUrl+query) ⇒ <code>string</code> \| <code>undefined</code>
        * [.setQuery([value])](#DIDUrl+setQuery)
        * [.join(segment)](#DIDUrl+join) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.toString()](#DIDUrl+toString) ⇒ <code>string</code>
        * [.toJSON()](#DIDUrl+toJSON) ⇒ <code>any</code>
        * [.clone()](#DIDUrl+clone) ⇒ [<code>DIDUrl</code>](#DIDUrl)
    * _static_
        * [.parse(input)](#DIDUrl.parse) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.fromJSON(json)](#DIDUrl.fromJSON) ⇒ [<code>DIDUrl</code>](#DIDUrl)

<a name="DIDUrl+did"></a>

### didUrl.did() ⇒ [<code>CoreDID</code>](#CoreDID)
Return a copy of the [CoreDID](#CoreDID) section of the [DIDUrl](#DIDUrl).

**Kind**: instance method of [<code>DIDUrl</code>](#DIDUrl)  
<a name="DIDUrl+urlStr"></a>

### didUrl.urlStr() ⇒ <code>string</code>
Return a copy of the relative DID Url as a string, including only the path, query, and fragment.

**Kind**: instance method of [<code>DIDUrl</code>](#DIDUrl)  
<a name="DIDUrl+fragment"></a>

### didUrl.fragment() ⇒ <code>string</code> \| <code>undefined</code>
Returns a copy of the [DIDUrl](#DIDUrl) method fragment, if any. Excludes the leading '#'.

**Kind**: instance method of [<code>DIDUrl</code>](#DIDUrl)  
<a name="DIDUrl+setFragment"></a>

### didUrl.setFragment([value])
Sets the `fragment` component of the [DIDUrl](#DIDUrl).

**Kind**: instance method of [<code>DIDUrl</code>](#DIDUrl)  

| Param | Type |
| --- | --- |
| [value] | <code>string</code> \| <code>undefined</code> | 

<a name="DIDUrl+path"></a>

### didUrl.path() ⇒ <code>string</code> \| <code>undefined</code>
Returns a copy of the [DIDUrl](#DIDUrl) path.

**Kind**: instance method of [<code>DIDUrl</code>](#DIDUrl)  
<a name="DIDUrl+setPath"></a>

### didUrl.setPath([value])
Sets the `path` component of the [DIDUrl](#DIDUrl).

**Kind**: instance method of [<code>DIDUrl</code>](#DIDUrl)  

| Param | Type |
| --- | --- |
| [value] | <code>string</code> \| <code>undefined</code> | 

<a name="DIDUrl+query"></a>

### didUrl.query() ⇒ <code>string</code> \| <code>undefined</code>
Returns a copy of the [DIDUrl](#DIDUrl) method query, if any. Excludes the leading '?'.

**Kind**: instance method of [<code>DIDUrl</code>](#DIDUrl)  
<a name="DIDUrl+setQuery"></a>

### didUrl.setQuery([value])
Sets the `query` component of the [DIDUrl](#DIDUrl).

**Kind**: instance method of [<code>DIDUrl</code>](#DIDUrl)  

| Param | Type |
| --- | --- |
| [value] | <code>string</code> \| <code>undefined</code> | 

<a name="DIDUrl+join"></a>

### didUrl.join(segment) ⇒ [<code>DIDUrl</code>](#DIDUrl)
Append a string representing a path, query, and/or fragment, returning a new [DIDUrl](#DIDUrl).

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
Returns the [DIDUrl](#DIDUrl) as a string.

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
Parses a [DIDUrl](#DIDUrl) from the input string.

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

<a name="DecodedJptCredential"></a>

## DecodedJptCredential
**Kind**: global class  

* [DecodedJptCredential](#DecodedJptCredential)
    * [.clone()](#DecodedJptCredential+clone) ⇒ [<code>DecodedJptCredential</code>](#DecodedJptCredential)
    * [.credential()](#DecodedJptCredential+credential) ⇒ [<code>Credential</code>](#Credential)
    * [.customClaims()](#DecodedJptCredential+customClaims) ⇒ <code>Map.&lt;string, any&gt;</code>
    * [.decodedJwp()](#DecodedJptCredential+decodedJwp) ⇒ [<code>JwpIssued</code>](#JwpIssued)

<a name="DecodedJptCredential+clone"></a>

### decodedJptCredential.clone() ⇒ [<code>DecodedJptCredential</code>](#DecodedJptCredential)
Deep clones the object.

**Kind**: instance method of [<code>DecodedJptCredential</code>](#DecodedJptCredential)  
<a name="DecodedJptCredential+credential"></a>

### decodedJptCredential.credential() ⇒ [<code>Credential</code>](#Credential)
Returns the [Credential](#Credential) embedded into this JPT.

**Kind**: instance method of [<code>DecodedJptCredential</code>](#DecodedJptCredential)  
<a name="DecodedJptCredential+customClaims"></a>

### decodedJptCredential.customClaims() ⇒ <code>Map.&lt;string, any&gt;</code>
Returns the custom claims parsed from the JPT.

**Kind**: instance method of [<code>DecodedJptCredential</code>](#DecodedJptCredential)  
<a name="DecodedJptCredential+decodedJwp"></a>

### decodedJptCredential.decodedJwp() ⇒ [<code>JwpIssued</code>](#JwpIssued)
**Kind**: instance method of [<code>DecodedJptCredential</code>](#DecodedJptCredential)  
<a name="DecodedJptPresentation"></a>

## DecodedJptPresentation
**Kind**: global class  

* [DecodedJptPresentation](#DecodedJptPresentation)
    * [.clone()](#DecodedJptPresentation+clone) ⇒ [<code>DecodedJptPresentation</code>](#DecodedJptPresentation)
    * [.credential()](#DecodedJptPresentation+credential) ⇒ [<code>Credential</code>](#Credential)
    * [.customClaims()](#DecodedJptPresentation+customClaims) ⇒ <code>Map.&lt;string, any&gt;</code>
    * [.aud()](#DecodedJptPresentation+aud) ⇒ <code>string</code> \| <code>undefined</code>

<a name="DecodedJptPresentation+clone"></a>

### decodedJptPresentation.clone() ⇒ [<code>DecodedJptPresentation</code>](#DecodedJptPresentation)
Deep clones the object.

**Kind**: instance method of [<code>DecodedJptPresentation</code>](#DecodedJptPresentation)  
<a name="DecodedJptPresentation+credential"></a>

### decodedJptPresentation.credential() ⇒ [<code>Credential</code>](#Credential)
Returns the [Credential](#Credential) embedded into this JPT.

**Kind**: instance method of [<code>DecodedJptPresentation</code>](#DecodedJptPresentation)  
<a name="DecodedJptPresentation+customClaims"></a>

### decodedJptPresentation.customClaims() ⇒ <code>Map.&lt;string, any&gt;</code>
Returns the custom claims parsed from the JPT.

**Kind**: instance method of [<code>DecodedJptPresentation</code>](#DecodedJptPresentation)  
<a name="DecodedJptPresentation+aud"></a>

### decodedJptPresentation.aud() ⇒ <code>string</code> \| <code>undefined</code>
Returns the `aud` property parsed from the JWT claims.

**Kind**: instance method of [<code>DecodedJptPresentation</code>](#DecodedJptPresentation)  
<a name="DecodedJws"></a>

## DecodedJws
A cryptographically verified decoded token from a JWS.

Contains the decoded headers and the raw claims.

**Kind**: global class  

* [DecodedJws](#DecodedJws)
    * [.claims()](#DecodedJws+claims) ⇒ <code>string</code>
    * [.claimsBytes()](#DecodedJws+claimsBytes) ⇒ <code>Uint8Array</code>
    * [.protectedHeader()](#DecodedJws+protectedHeader) ⇒ [<code>JwsHeader</code>](#JwsHeader)
    * [.clone()](#DecodedJws+clone) ⇒ [<code>DecodedJws</code>](#DecodedJws)
    * [.toJSON()](#DecodedJws+toJSON) ⇒ <code>any</code>

<a name="DecodedJws+claims"></a>

### decodedJws.claims() ⇒ <code>string</code>
Returns a copy of the parsed claims represented as a string.

# Errors
An error is thrown if the claims cannot be represented as a string.

This error can only occur if the Token was decoded from a detached payload.

**Kind**: instance method of [<code>DecodedJws</code>](#DecodedJws)  
<a name="DecodedJws+claimsBytes"></a>

### decodedJws.claimsBytes() ⇒ <code>Uint8Array</code>
Return a copy of the parsed claims represented as an array of bytes.

**Kind**: instance method of [<code>DecodedJws</code>](#DecodedJws)  
<a name="DecodedJws+protectedHeader"></a>

### decodedJws.protectedHeader() ⇒ [<code>JwsHeader</code>](#JwsHeader)
Returns a copy of the protected header.

**Kind**: instance method of [<code>DecodedJws</code>](#DecodedJws)  
<a name="DecodedJws+clone"></a>

### decodedJws.clone() ⇒ [<code>DecodedJws</code>](#DecodedJws)
Deep clones the object.

**Kind**: instance method of [<code>DecodedJws</code>](#DecodedJws)  
<a name="DecodedJws+toJSON"></a>

### decodedJws.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>DecodedJws</code>](#DecodedJws)  
<a name="DecodedJwtCredential"></a>

## DecodedJwtCredential
A cryptographically verified and decoded Credential.

Note that having an instance of this type only means the JWS it was constructed from was verified.
It does not imply anything about a potentially present proof property on the credential itself.

**Kind**: global class  

* [DecodedJwtCredential](#DecodedJwtCredential)
    * [.credential()](#DecodedJwtCredential+credential) ⇒ [<code>Credential</code>](#Credential)
    * [.protectedHeader()](#DecodedJwtCredential+protectedHeader) ⇒ [<code>JwsHeader</code>](#JwsHeader)
    * [.customClaims()](#DecodedJwtCredential+customClaims) ⇒ <code>Record.&lt;string, any&gt;</code> \| <code>undefined</code>
    * [.intoCredential()](#DecodedJwtCredential+intoCredential) ⇒ [<code>Credential</code>](#Credential)

<a name="DecodedJwtCredential+credential"></a>

### decodedJwtCredential.credential() ⇒ [<code>Credential</code>](#Credential)
Returns a copy of the credential parsed to the [Verifiable Credentials Data model](https://www.w3.org/TR/vc-data-model/).

**Kind**: instance method of [<code>DecodedJwtCredential</code>](#DecodedJwtCredential)  
<a name="DecodedJwtCredential+protectedHeader"></a>

### decodedJwtCredential.protectedHeader() ⇒ [<code>JwsHeader</code>](#JwsHeader)
Returns a copy of the protected header parsed from the decoded JWS.

**Kind**: instance method of [<code>DecodedJwtCredential</code>](#DecodedJwtCredential)  
<a name="DecodedJwtCredential+customClaims"></a>

### decodedJwtCredential.customClaims() ⇒ <code>Record.&lt;string, any&gt;</code> \| <code>undefined</code>
The custom claims parsed from the JWT.

**Kind**: instance method of [<code>DecodedJwtCredential</code>](#DecodedJwtCredential)  
<a name="DecodedJwtCredential+intoCredential"></a>

### decodedJwtCredential.intoCredential() ⇒ [<code>Credential</code>](#Credential)
Consumes the object and returns the decoded credential.

### Warning

This destroys the [DecodedJwtCredential](#DecodedJwtCredential) object.

**Kind**: instance method of [<code>DecodedJwtCredential</code>](#DecodedJwtCredential)  
<a name="DecodedJwtPresentation"></a>

## DecodedJwtPresentation
A cryptographically verified and decoded presentation.

Note that having an instance of this type only means the JWS it was constructed from was verified.
It does not imply anything about a potentially present proof property on the presentation itself.

**Kind**: global class  

* [DecodedJwtPresentation](#DecodedJwtPresentation)
    * [.presentation()](#DecodedJwtPresentation+presentation) ⇒ [<code>Presentation</code>](#Presentation)
    * [.protectedHeader()](#DecodedJwtPresentation+protectedHeader) ⇒ [<code>JwsHeader</code>](#JwsHeader)
    * [.intoPresentation()](#DecodedJwtPresentation+intoPresentation) ⇒ [<code>Presentation</code>](#Presentation)
    * [.expirationDate()](#DecodedJwtPresentation+expirationDate) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
    * [.issuanceDate()](#DecodedJwtPresentation+issuanceDate) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
    * [.audience()](#DecodedJwtPresentation+audience) ⇒ <code>string</code> \| <code>undefined</code>
    * [.customClaims()](#DecodedJwtPresentation+customClaims) ⇒ <code>Record.&lt;string, any&gt;</code> \| <code>undefined</code>

<a name="DecodedJwtPresentation+presentation"></a>

### decodedJwtPresentation.presentation() ⇒ [<code>Presentation</code>](#Presentation)
**Kind**: instance method of [<code>DecodedJwtPresentation</code>](#DecodedJwtPresentation)  
<a name="DecodedJwtPresentation+protectedHeader"></a>

### decodedJwtPresentation.protectedHeader() ⇒ [<code>JwsHeader</code>](#JwsHeader)
Returns a copy of the protected header parsed from the decoded JWS.

**Kind**: instance method of [<code>DecodedJwtPresentation</code>](#DecodedJwtPresentation)  
<a name="DecodedJwtPresentation+intoPresentation"></a>

### decodedJwtPresentation.intoPresentation() ⇒ [<code>Presentation</code>](#Presentation)
Consumes the object and returns the decoded presentation.

### Warning
This destroys the [DecodedJwtPresentation](#DecodedJwtPresentation) object.

**Kind**: instance method of [<code>DecodedJwtPresentation</code>](#DecodedJwtPresentation)  
<a name="DecodedJwtPresentation+expirationDate"></a>

### decodedJwtPresentation.expirationDate() ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
The expiration date parsed from the JWT claims.

**Kind**: instance method of [<code>DecodedJwtPresentation</code>](#DecodedJwtPresentation)  
<a name="DecodedJwtPresentation+issuanceDate"></a>

### decodedJwtPresentation.issuanceDate() ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
The issuance date parsed from the JWT claims.

**Kind**: instance method of [<code>DecodedJwtPresentation</code>](#DecodedJwtPresentation)  
<a name="DecodedJwtPresentation+audience"></a>

### decodedJwtPresentation.audience() ⇒ <code>string</code> \| <code>undefined</code>
The `aud` property parsed from JWT claims.

**Kind**: instance method of [<code>DecodedJwtPresentation</code>](#DecodedJwtPresentation)  
<a name="DecodedJwtPresentation+customClaims"></a>

### decodedJwtPresentation.customClaims() ⇒ <code>Record.&lt;string, any&gt;</code> \| <code>undefined</code>
The custom claims parsed from the JWT.

**Kind**: instance method of [<code>DecodedJwtPresentation</code>](#DecodedJwtPresentation)  
<a name="Disclosure"></a>

## Disclosure
Represents an elements constructing a disclosure.
Object properties and array elements disclosures are supported.

See: https://www.ietf.org/archive/id/draft-ietf-oauth-selective-disclosure-jwt-07.html#name-disclosures

**Kind**: global class  

* [Disclosure](#Disclosure)
    * [new Disclosure(salt, claim_name, claim_value)](#new_Disclosure_new)
    * _instance_
        * [.disclosure()](#Disclosure+disclosure) ⇒ <code>string</code>
        * [.toEncodedString()](#Disclosure+toEncodedString) ⇒ <code>string</code>
        * [.toString()](#Disclosure+toString) ⇒ <code>string</code>
        * [.salt()](#Disclosure+salt) ⇒ <code>string</code>
        * [.claimName()](#Disclosure+claimName) ⇒ <code>string</code> \| <code>undefined</code>
        * [.claimValue()](#Disclosure+claimValue) ⇒ <code>any</code>
        * [.toJSON()](#Disclosure+toJSON) ⇒ <code>any</code>
    * _static_
        * [.parse(disclosure)](#Disclosure.parse) ⇒ [<code>Disclosure</code>](#Disclosure)
        * [.fromJSON(json)](#Disclosure.fromJSON) ⇒ [<code>Disclosure</code>](#Disclosure)

<a name="new_Disclosure_new"></a>

### new Disclosure(salt, claim_name, claim_value)

| Param | Type |
| --- | --- |
| salt | <code>string</code> | 
| claim_name | <code>string</code> \| <code>undefined</code> | 
| claim_value | <code>any</code> | 

<a name="Disclosure+disclosure"></a>

### disclosure.disclosure() ⇒ <code>string</code>
Returns a copy of the base64url-encoded string.

**Kind**: instance method of [<code>Disclosure</code>](#Disclosure)  
<a name="Disclosure+toEncodedString"></a>

### disclosure.toEncodedString() ⇒ <code>string</code>
Returns a copy of the base64url-encoded string.

**Kind**: instance method of [<code>Disclosure</code>](#Disclosure)  
<a name="Disclosure+toString"></a>

### disclosure.toString() ⇒ <code>string</code>
Returns a copy of the base64url-encoded string.

**Kind**: instance method of [<code>Disclosure</code>](#Disclosure)  
<a name="Disclosure+salt"></a>

### disclosure.salt() ⇒ <code>string</code>
Returns a copy of the salt value.

**Kind**: instance method of [<code>Disclosure</code>](#Disclosure)  
<a name="Disclosure+claimName"></a>

### disclosure.claimName() ⇒ <code>string</code> \| <code>undefined</code>
Returns a copy of the claim name, optional for array elements.

**Kind**: instance method of [<code>Disclosure</code>](#Disclosure)  
<a name="Disclosure+claimValue"></a>

### disclosure.claimValue() ⇒ <code>any</code>
Returns a copy of the claim Value which can be of any type.

**Kind**: instance method of [<code>Disclosure</code>](#Disclosure)  
<a name="Disclosure+toJSON"></a>

### disclosure.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>Disclosure</code>](#Disclosure)  
<a name="Disclosure.parse"></a>

### Disclosure.parse(disclosure) ⇒ [<code>Disclosure</code>](#Disclosure)
Parses a Base64 encoded disclosure into a `Disclosure`.

## Error

Returns an `InvalidDisclosure` if input is not a valid disclosure.

**Kind**: static method of [<code>Disclosure</code>](#Disclosure)  

| Param | Type |
| --- | --- |
| disclosure | <code>string</code> | 

<a name="Disclosure.fromJSON"></a>

### Disclosure.fromJSON(json) ⇒ [<code>Disclosure</code>](#Disclosure)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>Disclosure</code>](#Disclosure)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="DomainLinkageConfiguration"></a>

## DomainLinkageConfiguration
DID Configuration Resource which contains Domain Linkage Credentials.
It can be placed in an origin's `.well-known` directory to prove linkage between the origin and a DID.
See: <https://identity.foundation/.well-known/resources/did-configuration/#did-configuration-resource>

Note:
- Only the [JSON Web Token Proof Format](https://identity.foundation/.well-known/resources/did-configuration/#json-web-token-proof-format)

**Kind**: global class  

* [DomainLinkageConfiguration](#DomainLinkageConfiguration)
    * [new DomainLinkageConfiguration(linkedDids)](#new_DomainLinkageConfiguration_new)
    * _instance_
        * [.linkedDids()](#DomainLinkageConfiguration+linkedDids) ⇒ [<code>Array.&lt;Jwt&gt;</code>](#Jwt)
        * [.issuers()](#DomainLinkageConfiguration+issuers) ⇒ [<code>Array.&lt;CoreDID&gt;</code>](#CoreDID)
        * [.toJSON()](#DomainLinkageConfiguration+toJSON) ⇒ <code>any</code>
        * [.clone()](#DomainLinkageConfiguration+clone) ⇒ [<code>DomainLinkageConfiguration</code>](#DomainLinkageConfiguration)
    * _static_
        * [.fromJSON(json)](#DomainLinkageConfiguration.fromJSON) ⇒ [<code>DomainLinkageConfiguration</code>](#DomainLinkageConfiguration)

<a name="new_DomainLinkageConfiguration_new"></a>

### new DomainLinkageConfiguration(linkedDids)
Constructs a new [DomainLinkageConfiguration](#DomainLinkageConfiguration).


| Param | Type |
| --- | --- |
| linkedDids | [<code>Array.&lt;Jwt&gt;</code>](#Jwt) | 

<a name="DomainLinkageConfiguration+linkedDids"></a>

### domainLinkageConfiguration.linkedDids() ⇒ [<code>Array.&lt;Jwt&gt;</code>](#Jwt)
List of the Domain Linkage Credentials.

**Kind**: instance method of [<code>DomainLinkageConfiguration</code>](#DomainLinkageConfiguration)  
<a name="DomainLinkageConfiguration+issuers"></a>

### domainLinkageConfiguration.issuers() ⇒ [<code>Array.&lt;CoreDID&gt;</code>](#CoreDID)
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
Create a new [Duration](#Duration) with the given number of seconds.

**Kind**: static method of [<code>Duration</code>](#Duration)  

| Param | Type |
| --- | --- |
| seconds | <code>number</code> | 

<a name="Duration.minutes"></a>

### Duration.minutes(minutes) ⇒ [<code>Duration</code>](#Duration)
Create a new [Duration](#Duration) with the given number of minutes.

**Kind**: static method of [<code>Duration</code>](#Duration)  

| Param | Type |
| --- | --- |
| minutes | <code>number</code> | 

<a name="Duration.hours"></a>

### Duration.hours(hours) ⇒ [<code>Duration</code>](#Duration)
Create a new [Duration](#Duration) with the given number of hours.

**Kind**: static method of [<code>Duration</code>](#Duration)  

| Param | Type |
| --- | --- |
| hours | <code>number</code> | 

<a name="Duration.days"></a>

### Duration.days(days) ⇒ [<code>Duration</code>](#Duration)
Create a new [Duration](#Duration) with the given number of days.

**Kind**: static method of [<code>Duration</code>](#Duration)  

| Param | Type |
| --- | --- |
| days | <code>number</code> | 

<a name="Duration.weeks"></a>

### Duration.weeks(weeks) ⇒ [<code>Duration</code>](#Duration)
Create a new [Duration](#Duration) with the given number of weeks.

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

<a name="EdDSAJwsVerifier"></a>

## EdDSAJwsVerifier
An implementor of `IJwsVerifier` that can handle the
`EdDSA` algorithm.

**Kind**: global class  

* [EdDSAJwsVerifier](#EdDSAJwsVerifier)
    * [new EdDSAJwsVerifier()](#new_EdDSAJwsVerifier_new)
    * [.verify(alg, signingInput, decodedSignature, publicKey)](#EdDSAJwsVerifier+verify)

<a name="new_EdDSAJwsVerifier_new"></a>

### new EdDSAJwsVerifier()
Constructs an EdDSAJwsVerifier.

<a name="EdDSAJwsVerifier+verify"></a>

### edDSAJwsVerifier.verify(alg, signingInput, decodedSignature, publicKey)
Verify a JWS signature secured with the `EdDSA` algorithm.
Only the `Ed25519` curve is supported for now.

This function is useful when one is building an `IJwsVerifier` that extends the default provided by
the IOTA Identity Framework.

# Warning

This function does not check whether `alg = EdDSA` in the protected header. Callers are expected to assert this
prior to calling the function.

**Kind**: instance method of [<code>EdDSAJwsVerifier</code>](#EdDSAJwsVerifier)  

| Param | Type |
| --- | --- |
| alg | <code>JwsAlgorithm</code> | 
| signingInput | <code>Uint8Array</code> | 
| decodedSignature | <code>Uint8Array</code> | 
| publicKey | [<code>Jwk</code>](#Jwk) | 

<a name="IotaDID"></a>

## IotaDID
A DID conforming to the IOTA DID method specification.

**Kind**: global class  

* [IotaDID](#IotaDID)
    * [new IotaDID(bytes, network)](#new_IotaDID_new)
    * _instance_
        * [.network()](#IotaDID+network) ⇒ <code>string</code>
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
Constructs a new [IotaDID](#IotaDID) from a byte representation of the tag and the given
network name.

See also [placeholder](#IotaDID.placeholder).


| Param | Type |
| --- | --- |
| bytes | <code>Uint8Array</code> | 
| network | <code>string</code> | 

<a name="IotaDID+network"></a>

### did.network() ⇒ <code>string</code>
Returns the Tangle network name of the [IotaDID](#IotaDID).

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+tag"></a>

### did.tag() ⇒ <code>string</code>
Returns a copy of the unique tag of the [IotaDID](#IotaDID).

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+toCoreDid"></a>

### did.toCoreDid() ⇒ [<code>CoreDID</code>](#CoreDID)
Returns the DID represented as a [CoreDID](#CoreDID).

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
Construct a new [DIDUrl](#DIDUrl) by joining with a relative DID Url string.

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  

| Param | Type |
| --- | --- |
| segment | <code>string</code> | 

<a name="IotaDID+toUrl"></a>

### did.toUrl() ⇒ [<code>DIDUrl</code>](#DIDUrl)
Clones the `DID` into a [DIDUrl](#DIDUrl).

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+toAliasId"></a>

### did.toAliasId() ⇒ <code>string</code>
Returns the hex-encoded AliasId with a '0x' prefix, from the DID tag.

**Kind**: instance method of [<code>IotaDID</code>](#IotaDID)  
<a name="IotaDID+intoUrl"></a>

### did.intoUrl() ⇒ [<code>DIDUrl</code>](#DIDUrl)
Converts the `DID` into a [DIDUrl](#DIDUrl), consuming it.

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
Constructs a new [IotaDID](#IotaDID) from a hex representation of an Alias Id and the given
network name.

**Kind**: static method of [<code>IotaDID</code>](#IotaDID)  

| Param | Type |
| --- | --- |
| aliasId | <code>string</code> | 
| network | <code>string</code> | 

<a name="IotaDID.placeholder"></a>

### IotaDID.placeholder(network) ⇒ [<code>IotaDID</code>](#IotaDID)
Creates a new placeholder [IotaDID](#IotaDID) with the given network name.

E.g. `did:iota:smr:0x0000000000000000000000000000000000000000000000000000000000000000`.

**Kind**: static method of [<code>IotaDID</code>](#IotaDID)  

| Param | Type |
| --- | --- |
| network | <code>string</code> | 

<a name="IotaDID.parse"></a>

### IotaDID.parse(input) ⇒ [<code>IotaDID</code>](#IotaDID)
Parses a [IotaDID](#IotaDID) from the input string.

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
A DID Document adhering to the IOTA DID method specification.

Note: All methods that involve reading from this class may potentially raise an error
if the object is being concurrently modified.

**Kind**: global class  

* [IotaDocument](#IotaDocument)
    * [new IotaDocument(network)](#new_IotaDocument_new)
    * _instance_
        * [.id()](#IotaDocument+id) ⇒ [<code>IotaDID</code>](#IotaDID)
        * [.controller()](#IotaDocument+controller) ⇒ [<code>Array.&lt;IotaDID&gt;</code>](#IotaDID)
        * [.setController(controller)](#IotaDocument+setController)
        * [.alsoKnownAs()](#IotaDocument+alsoKnownAs) ⇒ <code>Array.&lt;string&gt;</code>
        * [.setAlsoKnownAs(urls)](#IotaDocument+setAlsoKnownAs)
        * [.properties()](#IotaDocument+properties) ⇒ <code>Map.&lt;string, any&gt;</code>
        * [.setPropertyUnchecked(key, value)](#IotaDocument+setPropertyUnchecked)
        * [.service()](#IotaDocument+service) ⇒ [<code>Array.&lt;Service&gt;</code>](#Service)
        * [.insertService(service)](#IotaDocument+insertService)
        * [.removeService(did)](#IotaDocument+removeService) ⇒ [<code>Service</code>](#Service) \| <code>undefined</code>
        * [.resolveService(query)](#IotaDocument+resolveService) ⇒ [<code>Service</code>](#Service) \| <code>undefined</code>
        * [.methods([scope])](#IotaDocument+methods) ⇒ [<code>Array.&lt;VerificationMethod&gt;</code>](#VerificationMethod)
        * [.insertMethod(method, scope)](#IotaDocument+insertMethod)
        * [.removeMethod(did)](#IotaDocument+removeMethod) ⇒ [<code>VerificationMethod</code>](#VerificationMethod) \| <code>undefined</code>
        * [.resolveMethod(query, [scope])](#IotaDocument+resolveMethod) ⇒ [<code>VerificationMethod</code>](#VerificationMethod) \| <code>undefined</code>
        * [.attachMethodRelationship(didUrl, relationship)](#IotaDocument+attachMethodRelationship) ⇒ <code>boolean</code>
        * [.detachMethodRelationship(didUrl, relationship)](#IotaDocument+detachMethodRelationship) ⇒ <code>boolean</code>
        * [.verifyJws(jws, options, signatureVerifier, [detachedPayload])](#IotaDocument+verifyJws) ⇒ [<code>DecodedJws</code>](#DecodedJws)
        * [.pack()](#IotaDocument+pack) ⇒ <code>Uint8Array</code>
        * [.packWithEncoding(encoding)](#IotaDocument+packWithEncoding) ⇒ <code>Uint8Array</code>
        * [.metadata()](#IotaDocument+metadata) ⇒ [<code>IotaDocumentMetadata</code>](#IotaDocumentMetadata)
        * [.metadataCreated()](#IotaDocument+metadataCreated) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
        * [.setMetadataCreated(timestamp)](#IotaDocument+setMetadataCreated)
        * [.metadataUpdated()](#IotaDocument+metadataUpdated) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
        * [.setMetadataUpdated(timestamp)](#IotaDocument+setMetadataUpdated)
        * [.metadataDeactivated()](#IotaDocument+metadataDeactivated) ⇒ <code>boolean</code> \| <code>undefined</code>
        * [.setMetadataDeactivated([deactivated])](#IotaDocument+setMetadataDeactivated)
        * [.metadataStateControllerAddress()](#IotaDocument+metadataStateControllerAddress) ⇒ <code>string</code> \| <code>undefined</code>
        * [.metadataGovernorAddress()](#IotaDocument+metadataGovernorAddress) ⇒ <code>string</code> \| <code>undefined</code>
        * [.setMetadataPropertyUnchecked(key, value)](#IotaDocument+setMetadataPropertyUnchecked)
        * [.revokeCredentials(serviceQuery, indices)](#IotaDocument+revokeCredentials)
        * [.unrevokeCredentials(serviceQuery, indices)](#IotaDocument+unrevokeCredentials)
        * [.clone()](#IotaDocument+clone) ⇒ [<code>IotaDocument</code>](#IotaDocument)
        * [._shallowCloneInternal()](#IotaDocument+_shallowCloneInternal) ⇒ [<code>IotaDocument</code>](#IotaDocument)
        * [._strongCountInternal()](#IotaDocument+_strongCountInternal) ⇒ <code>number</code>
        * [.toJSON()](#IotaDocument+toJSON) ⇒ <code>any</code>
        * [.toCoreDocument()](#IotaDocument+toCoreDocument) ⇒ [<code>CoreDocument</code>](#CoreDocument)
        * [.generateMethod(storage, keyType, alg, fragment, scope)](#IotaDocument+generateMethod) ⇒ <code>Promise.&lt;string&gt;</code>
        * [.purgeMethod(storage, id)](#IotaDocument+purgeMethod) ⇒ <code>Promise.&lt;void&gt;</code>
        * ~~[.createJwt(storage, fragment, payload, options)](#IotaDocument+createJwt) ⇒ [<code>Promise.&lt;Jws&gt;</code>](#Jws)~~
        * [.createJws(storage, fragment, payload, options)](#IotaDocument+createJws) ⇒ [<code>Promise.&lt;Jws&gt;</code>](#Jws)
        * [.createCredentialJwt(storage, fragment, credential, options, [custom_claims])](#IotaDocument+createCredentialJwt) ⇒ [<code>Promise.&lt;Jwt&gt;</code>](#Jwt)
        * [.createPresentationJwt(storage, fragment, presentation, signature_options, presentation_options)](#IotaDocument+createPresentationJwt) ⇒ [<code>Promise.&lt;Jwt&gt;</code>](#Jwt)
        * [.generateMethodJwp(storage, alg, fragment, scope)](#IotaDocument+generateMethodJwp) ⇒ <code>Promise.&lt;string&gt;</code>
        * [.createIssuedJwp(storage, fragment, jpt_claims, options)](#IotaDocument+createIssuedJwp) ⇒ <code>Promise.&lt;string&gt;</code>
        * [.createPresentedJwp(presentation, method_id, options)](#IotaDocument+createPresentedJwp) ⇒ <code>Promise.&lt;string&gt;</code>
        * [.createCredentialJpt(credential, storage, fragment, options, [custom_claims])](#IotaDocument+createCredentialJpt) ⇒ [<code>Promise.&lt;Jpt&gt;</code>](#Jpt)
        * [.createPresentationJpt(presentation, method_id, options)](#IotaDocument+createPresentationJpt) ⇒ [<code>Promise.&lt;Jpt&gt;</code>](#Jpt)
    * _static_
        * [.newWithId(id)](#IotaDocument.newWithId) ⇒ [<code>IotaDocument</code>](#IotaDocument)
        * [.unpackFromOutput(did, aliasOutput, allowEmpty)](#IotaDocument.unpackFromOutput) ⇒ [<code>IotaDocument</code>](#IotaDocument)
        * [.unpackFromBlock(network, block)](#IotaDocument.unpackFromBlock) ⇒ [<code>Array.&lt;IotaDocument&gt;</code>](#IotaDocument)
        * [.fromJSON(json)](#IotaDocument.fromJSON) ⇒ [<code>IotaDocument</code>](#IotaDocument)

<a name="new_IotaDocument_new"></a>

### new IotaDocument(network)
Constructs an empty IOTA DID Document with a [placeholder](#IotaDID.placeholder) identifier
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
<a name="IotaDocument+setController"></a>

### iotaDocument.setController(controller)
Sets the controllers of the document.

Note: Duplicates will be ignored.
Use `null` to remove all controllers.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| controller | [<code>Array.&lt;IotaDID&gt;</code>](#IotaDID) \| <code>null</code> | 

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

### iotaDocument.methods([scope]) ⇒ [<code>Array.&lt;VerificationMethod&gt;</code>](#VerificationMethod)
Returns a list of all [VerificationMethod](#VerificationMethod) in the DID Document,
whose verification relationship matches `scope`.

If `scope` is not set, a list over the **embedded** methods is returned.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| [scope] | [<code>MethodScope</code>](#MethodScope) \| <code>undefined</code> | 

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

### iotaDocument.resolveMethod(query, [scope]) ⇒ [<code>VerificationMethod</code>](#VerificationMethod) \| <code>undefined</code>
Returns a copy of the first verification method with an `id` property
matching the provided `query` and the verification relationship
specified by `scope`, if present.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| query | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| [scope] | [<code>MethodScope</code>](#MethodScope) \| <code>undefined</code> | 

<a name="IotaDocument+attachMethodRelationship"></a>

### iotaDocument.attachMethodRelationship(didUrl, relationship) ⇒ <code>boolean</code>
Attaches the relationship to the given method, if the method exists.

Note: The method needs to be in the set of verification methods,
so it cannot be an embedded one.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| didUrl | [<code>DIDUrl</code>](#DIDUrl) | 
| relationship | [<code>MethodRelationship</code>](#MethodRelationship) | 

<a name="IotaDocument+detachMethodRelationship"></a>

### iotaDocument.detachMethodRelationship(didUrl, relationship) ⇒ <code>boolean</code>
Detaches the given relationship from the given method, if the method exists.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| didUrl | [<code>DIDUrl</code>](#DIDUrl) | 
| relationship | [<code>MethodRelationship</code>](#MethodRelationship) | 

<a name="IotaDocument+verifyJws"></a>

### iotaDocument.verifyJws(jws, options, signatureVerifier, [detachedPayload]) ⇒ [<code>DecodedJws</code>](#DecodedJws)
Decodes and verifies the provided JWS according to the passed `options` and `signatureVerifier`.
 If no `signatureVerifier` argument is provided a default verifier will be used that is (only) capable of
verifying EdDSA signatures.

Regardless of which options are passed the following conditions must be met in order for a verification attempt to
take place.
- The JWS must be encoded according to the JWS compact serialization.
- The `kid` value in the protected header must be an identifier of a verification method in this DID document.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| jws | [<code>Jws</code>](#Jws) | 
| options | [<code>JwsVerificationOptions</code>](#JwsVerificationOptions) | 
| signatureVerifier | <code>IJwsVerifier</code> | 
| [detachedPayload] | <code>string</code> \| <code>undefined</code> | 

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
| encoding | [<code>StateMetadataEncoding</code>](#StateMetadataEncoding) | 

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

### iotaDocument.setMetadataDeactivated([deactivated])
Sets the deactivated status of the DID document.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| [deactivated] | <code>boolean</code> \| <code>undefined</code> | 

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
If the document has a [RevocationBitmap](#RevocationBitmap) service identified by `serviceQuery`,
revoke all specified `indices`.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| serviceQuery | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| indices | <code>number</code> \| <code>Array.&lt;number&gt;</code> | 

<a name="IotaDocument+unrevokeCredentials"></a>

### iotaDocument.unrevokeCredentials(serviceQuery, indices)
If the document has a [RevocationBitmap](#RevocationBitmap) service identified by `serviceQuery`,
unrevoke all specified `indices`.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| serviceQuery | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| indices | <code>number</code> \| <code>Array.&lt;number&gt;</code> | 

<a name="IotaDocument+clone"></a>

### iotaDocument.clone() ⇒ [<code>IotaDocument</code>](#IotaDocument)
Returns a deep clone of the [IotaDocument](#IotaDocument).

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  
<a name="IotaDocument+_shallowCloneInternal"></a>

### iotaDocument.\_shallowCloneInternal() ⇒ [<code>IotaDocument</code>](#IotaDocument)
### Warning
This is for internal use only. Do not rely on or call this method.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  
<a name="IotaDocument+_strongCountInternal"></a>

### iotaDocument.\_strongCountInternal() ⇒ <code>number</code>
### Warning
This is for internal use only. Do not rely on or call this method.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  
<a name="IotaDocument+toJSON"></a>

### iotaDocument.toJSON() ⇒ <code>any</code>
Serializes to a plain JS representation.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  
<a name="IotaDocument+toCoreDocument"></a>

### iotaDocument.toCoreDocument() ⇒ [<code>CoreDocument</code>](#CoreDocument)
Transforms the [IotaDocument](#IotaDocument) to its [CoreDocument](#CoreDocument) representation.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  
<a name="IotaDocument+generateMethod"></a>

### iotaDocument.generateMethod(storage, keyType, alg, fragment, scope) ⇒ <code>Promise.&lt;string&gt;</code>
Generate new key material in the given `storage` and insert a new verification method with the corresponding
public key material into the DID document.

- If no fragment is given the `kid` of the generated JWK is used, if it is set, otherwise an error is returned.
- The `keyType` must be compatible with the given `storage`. `Storage`s are expected to export key type constants
for that use case.

The fragment of the generated method is returned.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| storage | [<code>Storage</code>](#Storage) | 
| keyType | <code>string</code> | 
| alg | <code>JwsAlgorithm</code> | 
| fragment | <code>string</code> \| <code>undefined</code> | 
| scope | [<code>MethodScope</code>](#MethodScope) | 

<a name="IotaDocument+purgeMethod"></a>

### iotaDocument.purgeMethod(storage, id) ⇒ <code>Promise.&lt;void&gt;</code>
Remove the method identified by the given fragment from the document and delete the corresponding key material in
the given `storage`.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| storage | [<code>Storage</code>](#Storage) | 
| id | [<code>DIDUrl</code>](#DIDUrl) | 

<a name="IotaDocument+createJwt"></a>

### ~~iotaDocument.createJwt(storage, fragment, payload, options) ⇒ [<code>Promise.&lt;Jws&gt;</code>](#Jws)~~
***Deprecated***

Sign the `payload` according to `options` with the storage backed private key corresponding to the public key
material in the verification method identified by the given `fragment.

Upon success a string representing a JWS encoded according to the Compact JWS Serialization format is returned.
See [RFC7515 section 3.1](https://www.rfc-editor.org/rfc/rfc7515#section-3.1).

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| storage | [<code>Storage</code>](#Storage) | 
| fragment | <code>string</code> | 
| payload | <code>string</code> | 
| options | [<code>JwsSignatureOptions</code>](#JwsSignatureOptions) | 

<a name="IotaDocument+createJws"></a>

### iotaDocument.createJws(storage, fragment, payload, options) ⇒ [<code>Promise.&lt;Jws&gt;</code>](#Jws)
Sign the `payload` according to `options` with the storage backed private key corresponding to the public key
material in the verification method identified by the given `fragment.

Upon success a string representing a JWS encoded according to the Compact JWS Serialization format is returned.
See [RFC7515 section 3.1](https://www.rfc-editor.org/rfc/rfc7515#section-3.1).

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| storage | [<code>Storage</code>](#Storage) | 
| fragment | <code>string</code> | 
| payload | <code>string</code> | 
| options | [<code>JwsSignatureOptions</code>](#JwsSignatureOptions) | 

<a name="IotaDocument+createCredentialJwt"></a>

### iotaDocument.createCredentialJwt(storage, fragment, credential, options, [custom_claims]) ⇒ [<code>Promise.&lt;Jwt&gt;</code>](#Jwt)
Produces a JWS where the payload is produced from the given `credential`
in accordance with [VC Data Model v1.1](https://www.w3.org/TR/vc-data-model/#json-web-token).

Unless the `kid` is explicitly set in the options, the `kid` in the protected header is the `id`
of the method identified by `fragment` and the JWS signature will be produced by the corresponding
private key backed by the `storage` in accordance with the passed `options`.

The `custom_claims` can be used to set additional claims on the resulting JWT.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| storage | [<code>Storage</code>](#Storage) | 
| fragment | <code>string</code> | 
| credential | [<code>Credential</code>](#Credential) | 
| options | [<code>JwsSignatureOptions</code>](#JwsSignatureOptions) | 
| [custom_claims] | <code>Record.&lt;string, any&gt;</code> \| <code>undefined</code> | 

<a name="IotaDocument+createPresentationJwt"></a>

### iotaDocument.createPresentationJwt(storage, fragment, presentation, signature_options, presentation_options) ⇒ [<code>Promise.&lt;Jwt&gt;</code>](#Jwt)
Produces a JWT where the payload is produced from the given presentation.
in accordance with [VC Data Model v1.1](https://www.w3.org/TR/vc-data-model/#json-web-token).

Unless the `kid` is explicitly set in the options, the `kid` in the protected header is the `id`
of the method identified by `fragment` and the JWS signature will be produced by the corresponding
private key backed by the `storage` in accordance with the passed `options`.

**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| storage | [<code>Storage</code>](#Storage) | 
| fragment | <code>string</code> | 
| presentation | [<code>Presentation</code>](#Presentation) | 
| signature_options | [<code>JwsSignatureOptions</code>](#JwsSignatureOptions) | 
| presentation_options | [<code>JwtPresentationOptions</code>](#JwtPresentationOptions) | 

<a name="IotaDocument+generateMethodJwp"></a>

### iotaDocument.generateMethodJwp(storage, alg, fragment, scope) ⇒ <code>Promise.&lt;string&gt;</code>
**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| storage | [<code>Storage</code>](#Storage) | 
| alg | [<code>ProofAlgorithm</code>](#ProofAlgorithm) | 
| fragment | <code>string</code> \| <code>undefined</code> | 
| scope | [<code>MethodScope</code>](#MethodScope) | 

<a name="IotaDocument+createIssuedJwp"></a>

### iotaDocument.createIssuedJwp(storage, fragment, jpt_claims, options) ⇒ <code>Promise.&lt;string&gt;</code>
**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| storage | [<code>Storage</code>](#Storage) | 
| fragment | <code>string</code> | 
| jpt_claims | <code>JptClaims</code> | 
| options | [<code>JwpCredentialOptions</code>](#JwpCredentialOptions) | 

<a name="IotaDocument+createPresentedJwp"></a>

### iotaDocument.createPresentedJwp(presentation, method_id, options) ⇒ <code>Promise.&lt;string&gt;</code>
**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| presentation | [<code>SelectiveDisclosurePresentation</code>](#SelectiveDisclosurePresentation) | 
| method_id | <code>string</code> | 
| options | [<code>JwpPresentationOptions</code>](#JwpPresentationOptions) | 

<a name="IotaDocument+createCredentialJpt"></a>

### iotaDocument.createCredentialJpt(credential, storage, fragment, options, [custom_claims]) ⇒ [<code>Promise.&lt;Jpt&gt;</code>](#Jpt)
**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| storage | [<code>Storage</code>](#Storage) | 
| fragment | <code>string</code> | 
| options | [<code>JwpCredentialOptions</code>](#JwpCredentialOptions) | 
| [custom_claims] | <code>Map.&lt;string, any&gt;</code> \| <code>undefined</code> | 

<a name="IotaDocument+createPresentationJpt"></a>

### iotaDocument.createPresentationJpt(presentation, method_id, options) ⇒ [<code>Promise.&lt;Jpt&gt;</code>](#Jpt)
**Kind**: instance method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| presentation | [<code>SelectiveDisclosurePresentation</code>](#SelectiveDisclosurePresentation) | 
| method_id | <code>string</code> | 
| options | [<code>JwpPresentationOptions</code>](#JwpPresentationOptions) | 

<a name="IotaDocument.newWithId"></a>

### IotaDocument.newWithId(id) ⇒ [<code>IotaDocument</code>](#IotaDocument)
Constructs an empty DID Document with the given identifier.

**Kind**: static method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| id | [<code>IotaDID</code>](#IotaDID) | 

<a name="IotaDocument.unpackFromOutput"></a>

### IotaDocument.unpackFromOutput(did, aliasOutput, allowEmpty) ⇒ [<code>IotaDocument</code>](#IotaDocument)
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
| aliasOutput | <code>AliasOutputBuilderParams</code> | 
| allowEmpty | <code>boolean</code> | 

<a name="IotaDocument.unpackFromBlock"></a>

### IotaDocument.unpackFromBlock(network, block) ⇒ [<code>Array.&lt;IotaDocument&gt;</code>](#IotaDocument)
Returns all DID documents of the Alias Outputs contained in the block's transaction payload
outputs, if any.

Errors if any Alias Output does not contain a valid or empty DID Document.

**Kind**: static method of [<code>IotaDocument</code>](#IotaDocument)  

| Param | Type |
| --- | --- |
| network | <code>string</code> | 
| block | <code>Block</code> | 

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
    * [.newDidOutput(client, address, document, [rentStructure])](#IotaIdentityClientExt.newDidOutput) ⇒ <code>Promise.&lt;AliasOutputBuilderParams&gt;</code>
    * [.updateDidOutput(client, document)](#IotaIdentityClientExt.updateDidOutput) ⇒ <code>Promise.&lt;AliasOutputBuilderParams&gt;</code>
    * [.deactivateDidOutput(client, did)](#IotaIdentityClientExt.deactivateDidOutput) ⇒ <code>Promise.&lt;AliasOutputBuilderParams&gt;</code>
    * [.resolveDid(client, did)](#IotaIdentityClientExt.resolveDid) ⇒ [<code>Promise.&lt;IotaDocument&gt;</code>](#IotaDocument)
    * [.resolveDidOutput(client, did)](#IotaIdentityClientExt.resolveDidOutput) ⇒ <code>Promise.&lt;AliasOutputBuilderParams&gt;</code>

<a name="IotaIdentityClientExt.newDidOutput"></a>

### IotaIdentityClientExt.newDidOutput(client, address, document, [rentStructure]) ⇒ <code>Promise.&lt;AliasOutputBuilderParams&gt;</code>
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
| address | <code>Address</code> | 
| document | [<code>IotaDocument</code>](#IotaDocument) | 
| [rentStructure] | <code>IRent</code> \| <code>undefined</code> | 

<a name="IotaIdentityClientExt.updateDidOutput"></a>

### IotaIdentityClientExt.updateDidOutput(client, document) ⇒ <code>Promise.&lt;AliasOutputBuilderParams&gt;</code>
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

### IotaIdentityClientExt.deactivateDidOutput(client, did) ⇒ <code>Promise.&lt;AliasOutputBuilderParams&gt;</code>
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

### IotaIdentityClientExt.resolveDidOutput(client, did) ⇒ <code>Promise.&lt;AliasOutputBuilderParams&gt;</code>
Fetches the `IAliasOutput` associated with the given DID.

**Kind**: static method of [<code>IotaIdentityClientExt</code>](#IotaIdentityClientExt)  

| Param | Type |
| --- | --- |
| client | <code>IIotaIdentityClient</code> | 
| did | [<code>IotaDID</code>](#IotaDID) | 

<a name="IssuerProtectedHeader"></a>

## IssuerProtectedHeader
**Kind**: global class  

* [IssuerProtectedHeader](#IssuerProtectedHeader)
    * [.typ](#IssuerProtectedHeader+typ) ⇒ <code>string</code> \| <code>undefined</code>
    * [.typ](#IssuerProtectedHeader+typ)
    * [.alg](#IssuerProtectedHeader+alg) ⇒ [<code>ProofAlgorithm</code>](#ProofAlgorithm)
    * [.alg](#IssuerProtectedHeader+alg)
    * [.kid](#IssuerProtectedHeader+kid) ⇒ <code>string</code> \| <code>undefined</code>
    * [.kid](#IssuerProtectedHeader+kid)
    * [.cid](#IssuerProtectedHeader+cid) ⇒ <code>string</code> \| <code>undefined</code>
    * [.cid](#IssuerProtectedHeader+cid)
    * [.claims()](#IssuerProtectedHeader+claims) ⇒ <code>Array.&lt;string&gt;</code>

<a name="IssuerProtectedHeader+typ"></a>

### issuerProtectedHeader.typ ⇒ <code>string</code> \| <code>undefined</code>
JWP type (JPT).

**Kind**: instance property of [<code>IssuerProtectedHeader</code>](#IssuerProtectedHeader)  
<a name="IssuerProtectedHeader+typ"></a>

### issuerProtectedHeader.typ
JWP type (JPT).

**Kind**: instance property of [<code>IssuerProtectedHeader</code>](#IssuerProtectedHeader)  

| Param | Type |
| --- | --- |
| [arg0] | <code>string</code> \| <code>undefined</code> | 

<a name="IssuerProtectedHeader+alg"></a>

### issuerProtectedHeader.alg ⇒ [<code>ProofAlgorithm</code>](#ProofAlgorithm)
Algorithm used for the JWP.

**Kind**: instance property of [<code>IssuerProtectedHeader</code>](#IssuerProtectedHeader)  
<a name="IssuerProtectedHeader+alg"></a>

### issuerProtectedHeader.alg
Algorithm used for the JWP.

**Kind**: instance property of [<code>IssuerProtectedHeader</code>](#IssuerProtectedHeader)  

| Param | Type |
| --- | --- |
| arg0 | [<code>ProofAlgorithm</code>](#ProofAlgorithm) | 

<a name="IssuerProtectedHeader+kid"></a>

### issuerProtectedHeader.kid ⇒ <code>string</code> \| <code>undefined</code>
ID for the key used for the JWP.

**Kind**: instance property of [<code>IssuerProtectedHeader</code>](#IssuerProtectedHeader)  
<a name="IssuerProtectedHeader+kid"></a>

### issuerProtectedHeader.kid
ID for the key used for the JWP.

**Kind**: instance property of [<code>IssuerProtectedHeader</code>](#IssuerProtectedHeader)  

| Param | Type |
| --- | --- |
| [arg0] | <code>string</code> \| <code>undefined</code> | 

<a name="IssuerProtectedHeader+cid"></a>

### issuerProtectedHeader.cid ⇒ <code>string</code> \| <code>undefined</code>
Not handled for now. Will be used in the future to resolve external claims

**Kind**: instance property of [<code>IssuerProtectedHeader</code>](#IssuerProtectedHeader)  
<a name="IssuerProtectedHeader+cid"></a>

### issuerProtectedHeader.cid
Not handled for now. Will be used in the future to resolve external claims

**Kind**: instance property of [<code>IssuerProtectedHeader</code>](#IssuerProtectedHeader)  

| Param | Type |
| --- | --- |
| [arg0] | <code>string</code> \| <code>undefined</code> | 

<a name="IssuerProtectedHeader+claims"></a>

### issuerProtectedHeader.claims() ⇒ <code>Array.&lt;string&gt;</code>
**Kind**: instance method of [<code>IssuerProtectedHeader</code>](#IssuerProtectedHeader)  
<a name="Jpt"></a>

## Jpt
A JSON Proof Token (JPT).

**Kind**: global class  

* [Jpt](#Jpt)
    * [new Jpt(jpt_string)](#new_Jpt_new)
    * [.toString()](#Jpt+toString) ⇒ <code>string</code>
    * [.clone()](#Jpt+clone) ⇒ [<code>Jpt</code>](#Jpt)

<a name="new_Jpt_new"></a>

### new Jpt(jpt_string)
Creates a new [Jpt](#Jpt).


| Param | Type |
| --- | --- |
| jpt_string | <code>string</code> | 

<a name="Jpt+toString"></a>

### jpt.toString() ⇒ <code>string</code>
**Kind**: instance method of [<code>Jpt</code>](#Jpt)  
<a name="Jpt+clone"></a>

### jpt.clone() ⇒ [<code>Jpt</code>](#Jpt)
Deep clones the object.

**Kind**: instance method of [<code>Jpt</code>](#Jpt)  
<a name="JptCredentialValidationOptions"></a>

## JptCredentialValidationOptions
Options to declare validation criteria for [Jpt](#Jpt).

**Kind**: global class  

* [JptCredentialValidationOptions](#JptCredentialValidationOptions)
    * [new JptCredentialValidationOptions([opts])](#new_JptCredentialValidationOptions_new)
    * _instance_
        * [.clone()](#JptCredentialValidationOptions+clone) ⇒ [<code>JptCredentialValidationOptions</code>](#JptCredentialValidationOptions)
        * [.toJSON()](#JptCredentialValidationOptions+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#JptCredentialValidationOptions.fromJSON) ⇒ [<code>JptCredentialValidationOptions</code>](#JptCredentialValidationOptions)

<a name="new_JptCredentialValidationOptions_new"></a>

### new JptCredentialValidationOptions([opts])
Creates a new default istance.


| Param | Type |
| --- | --- |
| [opts] | <code>IJptCredentialValidationOptions</code> \| <code>undefined</code> | 

<a name="JptCredentialValidationOptions+clone"></a>

### jptCredentialValidationOptions.clone() ⇒ [<code>JptCredentialValidationOptions</code>](#JptCredentialValidationOptions)
Deep clones the object.

**Kind**: instance method of [<code>JptCredentialValidationOptions</code>](#JptCredentialValidationOptions)  
<a name="JptCredentialValidationOptions+toJSON"></a>

### jptCredentialValidationOptions.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>JptCredentialValidationOptions</code>](#JptCredentialValidationOptions)  
<a name="JptCredentialValidationOptions.fromJSON"></a>

### JptCredentialValidationOptions.fromJSON(json) ⇒ [<code>JptCredentialValidationOptions</code>](#JptCredentialValidationOptions)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>JptCredentialValidationOptions</code>](#JptCredentialValidationOptions)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="JptCredentialValidator"></a>

## JptCredentialValidator
**Kind**: global class  
<a name="JptCredentialValidator.validate"></a>

### JptCredentialValidator.validate(credential_jpt, issuer, options, fail_fast) ⇒ [<code>DecodedJptCredential</code>](#DecodedJptCredential)
**Kind**: static method of [<code>JptCredentialValidator</code>](#JptCredentialValidator)  

| Param | Type |
| --- | --- |
| credential_jpt | [<code>Jpt</code>](#Jpt) | 
| issuer | [<code>CoreDocument</code>](#CoreDocument) \| <code>IToCoreDocument</code> | 
| options | [<code>JptCredentialValidationOptions</code>](#JptCredentialValidationOptions) | 
| fail_fast | [<code>FailFast</code>](#FailFast) | 

<a name="JptCredentialValidatorUtils"></a>

## JptCredentialValidatorUtils
Utility functions for validating JPT credentials.

**Kind**: global class  

* [JptCredentialValidatorUtils](#JptCredentialValidatorUtils)
    * [.extractIssuer(credential)](#JptCredentialValidatorUtils.extractIssuer) ⇒ [<code>CoreDID</code>](#CoreDID)
    * [.extractIssuerFromIssuedJpt(credential)](#JptCredentialValidatorUtils.extractIssuerFromIssuedJpt) ⇒ [<code>CoreDID</code>](#CoreDID)
    * [.checkTimeframesWithValidityTimeframe2024(credential, validity_timeframe, status_check)](#JptCredentialValidatorUtils.checkTimeframesWithValidityTimeframe2024)
    * [.checkRevocationWithValidityTimeframe2024(credential, issuer, status_check)](#JptCredentialValidatorUtils.checkRevocationWithValidityTimeframe2024)
    * [.checkTimeframesAndRevocationWithValidityTimeframe2024(credential, issuer, validity_timeframe, status_check)](#JptCredentialValidatorUtils.checkTimeframesAndRevocationWithValidityTimeframe2024)

<a name="JptCredentialValidatorUtils.extractIssuer"></a>

### JptCredentialValidatorUtils.extractIssuer(credential) ⇒ [<code>CoreDID</code>](#CoreDID)
Utility for extracting the issuer field of a [`Credential`](`Credential`) as a DID.
# Errors
Fails if the issuer field is not a valid DID.

**Kind**: static method of [<code>JptCredentialValidatorUtils</code>](#JptCredentialValidatorUtils)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 

<a name="JptCredentialValidatorUtils.extractIssuerFromIssuedJpt"></a>

### JptCredentialValidatorUtils.extractIssuerFromIssuedJpt(credential) ⇒ [<code>CoreDID</code>](#CoreDID)
Utility for extracting the issuer field of a credential in JPT representation as DID.
# Errors
If the JPT decoding fails or the issuer field is not a valid DID.

**Kind**: static method of [<code>JptCredentialValidatorUtils</code>](#JptCredentialValidatorUtils)  

| Param | Type |
| --- | --- |
| credential | [<code>Jpt</code>](#Jpt) | 

<a name="JptCredentialValidatorUtils.checkTimeframesWithValidityTimeframe2024"></a>

### JptCredentialValidatorUtils.checkTimeframesWithValidityTimeframe2024(credential, validity_timeframe, status_check)
**Kind**: static method of [<code>JptCredentialValidatorUtils</code>](#JptCredentialValidatorUtils)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| validity_timeframe | [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code> | 
| status_check | [<code>StatusCheck</code>](#StatusCheck) | 

<a name="JptCredentialValidatorUtils.checkRevocationWithValidityTimeframe2024"></a>

### JptCredentialValidatorUtils.checkRevocationWithValidityTimeframe2024(credential, issuer, status_check)
Checks whether the credential status has been revoked.

Only supports `RevocationTimeframe2024`.

**Kind**: static method of [<code>JptCredentialValidatorUtils</code>](#JptCredentialValidatorUtils)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| issuer | [<code>CoreDocument</code>](#CoreDocument) \| <code>IToCoreDocument</code> | 
| status_check | [<code>StatusCheck</code>](#StatusCheck) | 

<a name="JptCredentialValidatorUtils.checkTimeframesAndRevocationWithValidityTimeframe2024"></a>

### JptCredentialValidatorUtils.checkTimeframesAndRevocationWithValidityTimeframe2024(credential, issuer, validity_timeframe, status_check)
Checks whether the credential status has been revoked or the timeframe interval is INVALID

Only supports `RevocationTimeframe2024`.

**Kind**: static method of [<code>JptCredentialValidatorUtils</code>](#JptCredentialValidatorUtils)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| issuer | [<code>CoreDocument</code>](#CoreDocument) \| <code>IToCoreDocument</code> | 
| validity_timeframe | [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code> | 
| status_check | [<code>StatusCheck</code>](#StatusCheck) | 

<a name="JptPresentationValidationOptions"></a>

## JptPresentationValidationOptions
Options to declare validation criteria for a [Jpt](#Jpt) presentation.

**Kind**: global class  

* [JptPresentationValidationOptions](#JptPresentationValidationOptions)
    * [new JptPresentationValidationOptions([opts])](#new_JptPresentationValidationOptions_new)
    * _instance_
        * [.clone()](#JptPresentationValidationOptions+clone) ⇒ [<code>JptPresentationValidationOptions</code>](#JptPresentationValidationOptions)
        * [.toJSON()](#JptPresentationValidationOptions+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#JptPresentationValidationOptions.fromJSON) ⇒ [<code>JptPresentationValidationOptions</code>](#JptPresentationValidationOptions)

<a name="new_JptPresentationValidationOptions_new"></a>

### new JptPresentationValidationOptions([opts])

| Param | Type |
| --- | --- |
| [opts] | <code>IJptPresentationValidationOptions</code> \| <code>undefined</code> | 

<a name="JptPresentationValidationOptions+clone"></a>

### jptPresentationValidationOptions.clone() ⇒ [<code>JptPresentationValidationOptions</code>](#JptPresentationValidationOptions)
Deep clones the object.

**Kind**: instance method of [<code>JptPresentationValidationOptions</code>](#JptPresentationValidationOptions)  
<a name="JptPresentationValidationOptions+toJSON"></a>

### jptPresentationValidationOptions.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>JptPresentationValidationOptions</code>](#JptPresentationValidationOptions)  
<a name="JptPresentationValidationOptions.fromJSON"></a>

### JptPresentationValidationOptions.fromJSON(json) ⇒ [<code>JptPresentationValidationOptions</code>](#JptPresentationValidationOptions)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>JptPresentationValidationOptions</code>](#JptPresentationValidationOptions)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="JptPresentationValidator"></a>

## JptPresentationValidator
**Kind**: global class  
<a name="JptPresentationValidator.validate"></a>

### JptPresentationValidator.validate(presentation_jpt, issuer, options, fail_fast) ⇒ [<code>DecodedJptPresentation</code>](#DecodedJptPresentation)
Decodes and validates a Presented [Credential](#Credential) issued as a JPT (JWP Presented Form). A
[DecodedJptPresentation](#DecodedJptPresentation) is returned upon success.

The following properties are validated according to `options`:
- the holder's proof on the JWP,
- the expiration date,
- the issuance date,
- the semantic structure.

**Kind**: static method of [<code>JptPresentationValidator</code>](#JptPresentationValidator)  

| Param | Type |
| --- | --- |
| presentation_jpt | [<code>Jpt</code>](#Jpt) | 
| issuer | [<code>CoreDocument</code>](#CoreDocument) \| <code>IToCoreDocument</code> | 
| options | [<code>JptPresentationValidationOptions</code>](#JptPresentationValidationOptions) | 
| fail_fast | [<code>FailFast</code>](#FailFast) | 

<a name="JptPresentationValidatorUtils"></a>

## JptPresentationValidatorUtils
Utility functions for verifying JPT presentations.

**Kind**: global class  

* [JptPresentationValidatorUtils](#JptPresentationValidatorUtils)
    * [.extractIssuerFromPresentedJpt(presentation)](#JptPresentationValidatorUtils.extractIssuerFromPresentedJpt) ⇒ [<code>CoreDID</code>](#CoreDID)
    * [.checkTimeframesWithValidityTimeframe2024(credential, validity_timeframe, status_check)](#JptPresentationValidatorUtils.checkTimeframesWithValidityTimeframe2024)

<a name="JptPresentationValidatorUtils.extractIssuerFromPresentedJpt"></a>

### JptPresentationValidatorUtils.extractIssuerFromPresentedJpt(presentation) ⇒ [<code>CoreDID</code>](#CoreDID)
Utility for extracting the issuer field of a credential in JPT representation as DID.
# Errors
If the JPT decoding fails or the issuer field is not a valid DID.

**Kind**: static method of [<code>JptPresentationValidatorUtils</code>](#JptPresentationValidatorUtils)  

| Param | Type |
| --- | --- |
| presentation | [<code>Jpt</code>](#Jpt) | 

<a name="JptPresentationValidatorUtils.checkTimeframesWithValidityTimeframe2024"></a>

### JptPresentationValidatorUtils.checkTimeframesWithValidityTimeframe2024(credential, validity_timeframe, status_check)
Check timeframe interval in credentialStatus with `RevocationTimeframeStatus`.

**Kind**: static method of [<code>JptPresentationValidatorUtils</code>](#JptPresentationValidatorUtils)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| validity_timeframe | [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code> | 
| status_check | [<code>StatusCheck</code>](#StatusCheck) | 

<a name="Jwk"></a>

## Jwk
**Kind**: global class  

* [Jwk](#Jwk)
    * [new Jwk(jwk)](#new_Jwk_new)
    * _instance_
        * [.kty()](#Jwk+kty) ⇒ <code>JwkType</code>
        * [.use()](#Jwk+use) ⇒ <code>JwkUse</code> \| <code>undefined</code>
        * [.keyOps()](#Jwk+keyOps) ⇒ <code>Array.&lt;JwkOperation&gt;</code>
        * [.alg()](#Jwk+alg) ⇒ <code>JwsAlgorithm</code> \| <code>undefined</code>
        * [.kid()](#Jwk+kid) ⇒ <code>string</code> \| <code>undefined</code>
        * [.x5u()](#Jwk+x5u) ⇒ <code>string</code> \| <code>undefined</code>
        * [.x5c()](#Jwk+x5c) ⇒ <code>Array.&lt;string&gt;</code>
        * [.x5t()](#Jwk+x5t) ⇒ <code>string</code> \| <code>undefined</code>
        * [.x5t256()](#Jwk+x5t256) ⇒ <code>string</code> \| <code>undefined</code>
        * [.paramsEc()](#Jwk+paramsEc) ⇒ <code>JwkParamsEc</code> \| <code>undefined</code>
        * [.paramsOkp()](#Jwk+paramsOkp) ⇒ <code>JwkParamsOkp</code> \| <code>undefined</code>
        * [.paramsOct()](#Jwk+paramsOct) ⇒ <code>JwkParamsOct</code> \| <code>undefined</code>
        * [.paramsRsa()](#Jwk+paramsRsa) ⇒ <code>JwkParamsRsa</code> \| <code>undefined</code>
        * [.toPublic()](#Jwk+toPublic) ⇒ [<code>Jwk</code>](#Jwk) \| <code>undefined</code>
        * [.isPublic()](#Jwk+isPublic) ⇒ <code>boolean</code>
        * [.isPrivate()](#Jwk+isPrivate) ⇒ <code>boolean</code>
        * [.toJSON()](#Jwk+toJSON) ⇒ <code>any</code>
        * [.clone()](#Jwk+clone) ⇒ [<code>Jwk</code>](#Jwk)
    * _static_
        * [.fromJSON(json)](#Jwk.fromJSON) ⇒ [<code>Jwk</code>](#Jwk)

<a name="new_Jwk_new"></a>

### new Jwk(jwk)

| Param | Type |
| --- | --- |
| jwk | <code>IJwkParams</code> | 

<a name="Jwk+kty"></a>

### jwk.kty() ⇒ <code>JwkType</code>
Returns the value for the key type parameter (kty).

**Kind**: instance method of [<code>Jwk</code>](#Jwk)  
<a name="Jwk+use"></a>

### jwk.use() ⇒ <code>JwkUse</code> \| <code>undefined</code>
Returns the value for the use property (use).

**Kind**: instance method of [<code>Jwk</code>](#Jwk)  
<a name="Jwk+keyOps"></a>

### jwk.keyOps() ⇒ <code>Array.&lt;JwkOperation&gt;</code>
**Kind**: instance method of [<code>Jwk</code>](#Jwk)  
<a name="Jwk+alg"></a>

### jwk.alg() ⇒ <code>JwsAlgorithm</code> \| <code>undefined</code>
Returns the value for the algorithm property (alg).

**Kind**: instance method of [<code>Jwk</code>](#Jwk)  
<a name="Jwk+kid"></a>

### jwk.kid() ⇒ <code>string</code> \| <code>undefined</code>
Returns the value of the key ID property (kid).

**Kind**: instance method of [<code>Jwk</code>](#Jwk)  
<a name="Jwk+x5u"></a>

### jwk.x5u() ⇒ <code>string</code> \| <code>undefined</code>
Returns the value of the X.509 URL property (x5u).

**Kind**: instance method of [<code>Jwk</code>](#Jwk)  
<a name="Jwk+x5c"></a>

### jwk.x5c() ⇒ <code>Array.&lt;string&gt;</code>
Returns the value of the X.509 certificate chain property (x5c).

**Kind**: instance method of [<code>Jwk</code>](#Jwk)  
<a name="Jwk+x5t"></a>

### jwk.x5t() ⇒ <code>string</code> \| <code>undefined</code>
Returns the value of the X.509 certificate SHA-1 thumbprint property (x5t).

**Kind**: instance method of [<code>Jwk</code>](#Jwk)  
<a name="Jwk+x5t256"></a>

### jwk.x5t256() ⇒ <code>string</code> \| <code>undefined</code>
Returns the value of the X.509 certificate SHA-256 thumbprint property (x5t#S256).

**Kind**: instance method of [<code>Jwk</code>](#Jwk)  
<a name="Jwk+paramsEc"></a>

### jwk.paramsEc() ⇒ <code>JwkParamsEc</code> \| <code>undefined</code>
If this JWK is of kty EC, returns those parameters.

**Kind**: instance method of [<code>Jwk</code>](#Jwk)  
<a name="Jwk+paramsOkp"></a>

### jwk.paramsOkp() ⇒ <code>JwkParamsOkp</code> \| <code>undefined</code>
If this JWK is of kty OKP, returns those parameters.

**Kind**: instance method of [<code>Jwk</code>](#Jwk)  
<a name="Jwk+paramsOct"></a>

### jwk.paramsOct() ⇒ <code>JwkParamsOct</code> \| <code>undefined</code>
If this JWK is of kty OCT, returns those parameters.

**Kind**: instance method of [<code>Jwk</code>](#Jwk)  
<a name="Jwk+paramsRsa"></a>

### jwk.paramsRsa() ⇒ <code>JwkParamsRsa</code> \| <code>undefined</code>
If this JWK is of kty RSA, returns those parameters.

**Kind**: instance method of [<code>Jwk</code>](#Jwk)  
<a name="Jwk+toPublic"></a>

### jwk.toPublic() ⇒ [<code>Jwk</code>](#Jwk) \| <code>undefined</code>
Returns a clone of the [Jwk](#Jwk) with _all_ private key components unset.
Nothing is returned when `kty = oct` as this key type is not considered public by this library.

**Kind**: instance method of [<code>Jwk</code>](#Jwk)  
<a name="Jwk+isPublic"></a>

### jwk.isPublic() ⇒ <code>boolean</code>
Returns `true` if _all_ private key components of the key are unset, `false` otherwise.

**Kind**: instance method of [<code>Jwk</code>](#Jwk)  
<a name="Jwk+isPrivate"></a>

### jwk.isPrivate() ⇒ <code>boolean</code>
Returns `true` if _all_ private key components of the key are set, `false` otherwise.

**Kind**: instance method of [<code>Jwk</code>](#Jwk)  
<a name="Jwk+toJSON"></a>

### jwk.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>Jwk</code>](#Jwk)  
<a name="Jwk+clone"></a>

### jwk.clone() ⇒ [<code>Jwk</code>](#Jwk)
Deep clones the object.

**Kind**: instance method of [<code>Jwk</code>](#Jwk)  
<a name="Jwk.fromJSON"></a>

### Jwk.fromJSON(json) ⇒ [<code>Jwk</code>](#Jwk)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>Jwk</code>](#Jwk)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="JwkGenOutput"></a>

## JwkGenOutput
The result of a key generation in `JwkStorage`.

**Kind**: global class  

* [JwkGenOutput](#JwkGenOutput)
    * [new JwkGenOutput(key_id, jwk)](#new_JwkGenOutput_new)
    * _instance_
        * [.jwk()](#JwkGenOutput+jwk) ⇒ [<code>Jwk</code>](#Jwk)
        * [.keyId()](#JwkGenOutput+keyId) ⇒ <code>string</code>
        * [.toJSON()](#JwkGenOutput+toJSON) ⇒ <code>any</code>
        * [.clone()](#JwkGenOutput+clone) ⇒ [<code>JwkGenOutput</code>](#JwkGenOutput)
    * _static_
        * [.fromJSON(json)](#JwkGenOutput.fromJSON) ⇒ [<code>JwkGenOutput</code>](#JwkGenOutput)

<a name="new_JwkGenOutput_new"></a>

### new JwkGenOutput(key_id, jwk)

| Param | Type |
| --- | --- |
| key_id | <code>string</code> | 
| jwk | [<code>Jwk</code>](#Jwk) | 

<a name="JwkGenOutput+jwk"></a>

### jwkGenOutput.jwk() ⇒ [<code>Jwk</code>](#Jwk)
Returns the generated public [Jwk](#Jwk).

**Kind**: instance method of [<code>JwkGenOutput</code>](#JwkGenOutput)  
<a name="JwkGenOutput+keyId"></a>

### jwkGenOutput.keyId() ⇒ <code>string</code>
Returns the key id of the generated [Jwk](#Jwk).

**Kind**: instance method of [<code>JwkGenOutput</code>](#JwkGenOutput)  
<a name="JwkGenOutput+toJSON"></a>

### jwkGenOutput.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>JwkGenOutput</code>](#JwkGenOutput)  
<a name="JwkGenOutput+clone"></a>

### jwkGenOutput.clone() ⇒ [<code>JwkGenOutput</code>](#JwkGenOutput)
Deep clones the object.

**Kind**: instance method of [<code>JwkGenOutput</code>](#JwkGenOutput)  
<a name="JwkGenOutput.fromJSON"></a>

### JwkGenOutput.fromJSON(json) ⇒ [<code>JwkGenOutput</code>](#JwkGenOutput)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>JwkGenOutput</code>](#JwkGenOutput)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="JwpCredentialOptions"></a>

## JwpCredentialOptions
**Kind**: global class  

* [JwpCredentialOptions](#JwpCredentialOptions)
    * _instance_
        * [.kid](#JwpCredentialOptions+kid) ⇒ <code>string</code> \| <code>undefined</code>
        * [.kid](#JwpCredentialOptions+kid)
        * [.toJSON()](#JwpCredentialOptions+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(value)](#JwpCredentialOptions.fromJSON) ⇒ [<code>JwpCredentialOptions</code>](#JwpCredentialOptions)

<a name="JwpCredentialOptions+kid"></a>

### jwpCredentialOptions.kid ⇒ <code>string</code> \| <code>undefined</code>
**Kind**: instance property of [<code>JwpCredentialOptions</code>](#JwpCredentialOptions)  
<a name="JwpCredentialOptions+kid"></a>

### jwpCredentialOptions.kid
**Kind**: instance property of [<code>JwpCredentialOptions</code>](#JwpCredentialOptions)  

| Param | Type |
| --- | --- |
| [arg0] | <code>string</code> \| <code>undefined</code> | 

<a name="JwpCredentialOptions+toJSON"></a>

### jwpCredentialOptions.toJSON() ⇒ <code>any</code>
**Kind**: instance method of [<code>JwpCredentialOptions</code>](#JwpCredentialOptions)  
<a name="JwpCredentialOptions.fromJSON"></a>

### JwpCredentialOptions.fromJSON(value) ⇒ [<code>JwpCredentialOptions</code>](#JwpCredentialOptions)
**Kind**: static method of [<code>JwpCredentialOptions</code>](#JwpCredentialOptions)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="JwpIssued"></a>

## JwpIssued
**Kind**: global class  

* [JwpIssued](#JwpIssued)
    * _instance_
        * [.toJSON()](#JwpIssued+toJSON) ⇒ <code>any</code>
        * [.clone()](#JwpIssued+clone) ⇒ [<code>JwpIssued</code>](#JwpIssued)
        * [.encode(serialization)](#JwpIssued+encode) ⇒ <code>string</code>
        * [.setProof(proof)](#JwpIssued+setProof)
        * [.getProof()](#JwpIssued+getProof) ⇒ <code>Uint8Array</code>
        * [.getPayloads()](#JwpIssued+getPayloads) ⇒ [<code>Payloads</code>](#Payloads)
        * [.setPayloads(payloads)](#JwpIssued+setPayloads)
        * [.getIssuerProtectedHeader()](#JwpIssued+getIssuerProtectedHeader) ⇒ [<code>IssuerProtectedHeader</code>](#IssuerProtectedHeader)
    * _static_
        * [.fromJSON(json)](#JwpIssued.fromJSON) ⇒ [<code>JwpIssued</code>](#JwpIssued)

<a name="JwpIssued+toJSON"></a>

### jwpIssued.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>JwpIssued</code>](#JwpIssued)  
<a name="JwpIssued+clone"></a>

### jwpIssued.clone() ⇒ [<code>JwpIssued</code>](#JwpIssued)
Deep clones the object.

**Kind**: instance method of [<code>JwpIssued</code>](#JwpIssued)  
<a name="JwpIssued+encode"></a>

### jwpIssued.encode(serialization) ⇒ <code>string</code>
**Kind**: instance method of [<code>JwpIssued</code>](#JwpIssued)  

| Param | Type |
| --- | --- |
| serialization | [<code>SerializationType</code>](#SerializationType) | 

<a name="JwpIssued+setProof"></a>

### jwpIssued.setProof(proof)
**Kind**: instance method of [<code>JwpIssued</code>](#JwpIssued)  

| Param | Type |
| --- | --- |
| proof | <code>Uint8Array</code> | 

<a name="JwpIssued+getProof"></a>

### jwpIssued.getProof() ⇒ <code>Uint8Array</code>
**Kind**: instance method of [<code>JwpIssued</code>](#JwpIssued)  
<a name="JwpIssued+getPayloads"></a>

### jwpIssued.getPayloads() ⇒ [<code>Payloads</code>](#Payloads)
**Kind**: instance method of [<code>JwpIssued</code>](#JwpIssued)  
<a name="JwpIssued+setPayloads"></a>

### jwpIssued.setPayloads(payloads)
**Kind**: instance method of [<code>JwpIssued</code>](#JwpIssued)  

| Param | Type |
| --- | --- |
| payloads | [<code>Payloads</code>](#Payloads) | 

<a name="JwpIssued+getIssuerProtectedHeader"></a>

### jwpIssued.getIssuerProtectedHeader() ⇒ [<code>IssuerProtectedHeader</code>](#IssuerProtectedHeader)
**Kind**: instance method of [<code>JwpIssued</code>](#JwpIssued)  
<a name="JwpIssued.fromJSON"></a>

### JwpIssued.fromJSON(json) ⇒ [<code>JwpIssued</code>](#JwpIssued)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>JwpIssued</code>](#JwpIssued)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="JwpPresentationOptions"></a>

## JwpPresentationOptions
Options to be set in the JWT claims of a verifiable presentation.

**Kind**: global class  

* [JwpPresentationOptions](#JwpPresentationOptions)
    * [.audience](#JwpPresentationOptions+audience) ⇒ <code>string</code> \| <code>undefined</code>
    * [.audience](#JwpPresentationOptions+audience)
    * [.nonce](#JwpPresentationOptions+nonce) ⇒ <code>string</code> \| <code>undefined</code>
    * [.nonce](#JwpPresentationOptions+nonce)

<a name="JwpPresentationOptions+audience"></a>

### jwpPresentationOptions.audience ⇒ <code>string</code> \| <code>undefined</code>
Sets the audience for presentation (`aud` property in JWP Presentation Header).

**Kind**: instance property of [<code>JwpPresentationOptions</code>](#JwpPresentationOptions)  
<a name="JwpPresentationOptions+audience"></a>

### jwpPresentationOptions.audience
Sets the audience for presentation (`aud` property in JWP Presentation Header).

**Kind**: instance property of [<code>JwpPresentationOptions</code>](#JwpPresentationOptions)  

| Param | Type |
| --- | --- |
| [arg0] | <code>string</code> \| <code>undefined</code> | 

<a name="JwpPresentationOptions+nonce"></a>

### jwpPresentationOptions.nonce ⇒ <code>string</code> \| <code>undefined</code>
The nonce to be placed in the Presentation Protected Header.

**Kind**: instance property of [<code>JwpPresentationOptions</code>](#JwpPresentationOptions)  
<a name="JwpPresentationOptions+nonce"></a>

### jwpPresentationOptions.nonce
The nonce to be placed in the Presentation Protected Header.

**Kind**: instance property of [<code>JwpPresentationOptions</code>](#JwpPresentationOptions)  

| Param | Type |
| --- | --- |
| [arg0] | <code>string</code> \| <code>undefined</code> | 

<a name="JwpVerificationOptions"></a>

## JwpVerificationOptions
**Kind**: global class  

* [JwpVerificationOptions](#JwpVerificationOptions)
    * _instance_
        * [.clone()](#JwpVerificationOptions+clone) ⇒ [<code>JwpVerificationOptions</code>](#JwpVerificationOptions)
        * [.toJSON()](#JwpVerificationOptions+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#JwpVerificationOptions.fromJSON) ⇒ [<code>JwpVerificationOptions</code>](#JwpVerificationOptions)
        * [.new([opts])](#JwpVerificationOptions.new) ⇒ [<code>JwpVerificationOptions</code>](#JwpVerificationOptions)

<a name="JwpVerificationOptions+clone"></a>

### jwpVerificationOptions.clone() ⇒ [<code>JwpVerificationOptions</code>](#JwpVerificationOptions)
Deep clones the object.

**Kind**: instance method of [<code>JwpVerificationOptions</code>](#JwpVerificationOptions)  
<a name="JwpVerificationOptions+toJSON"></a>

### jwpVerificationOptions.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>JwpVerificationOptions</code>](#JwpVerificationOptions)  
<a name="JwpVerificationOptions.fromJSON"></a>

### JwpVerificationOptions.fromJSON(json) ⇒ [<code>JwpVerificationOptions</code>](#JwpVerificationOptions)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>JwpVerificationOptions</code>](#JwpVerificationOptions)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="JwpVerificationOptions.new"></a>

### JwpVerificationOptions.new([opts]) ⇒ [<code>JwpVerificationOptions</code>](#JwpVerificationOptions)
**Kind**: static method of [<code>JwpVerificationOptions</code>](#JwpVerificationOptions)  

| Param | Type |
| --- | --- |
| [opts] | <code>IJwpVerificationOptions</code> \| <code>undefined</code> | 

<a name="Jws"></a>

## Jws
A wrapper around a JSON Web Signature (JWS).

**Kind**: global class  

* [Jws](#Jws)
    * [new Jws(jws_string)](#new_Jws_new)
    * [.toString()](#Jws+toString) ⇒ <code>string</code>

<a name="new_Jws_new"></a>

### new Jws(jws_string)
Creates a new [Jws](#Jws) from the given string.


| Param | Type |
| --- | --- |
| jws_string | <code>string</code> | 

<a name="Jws+toString"></a>

### jws.toString() ⇒ <code>string</code>
Returns a clone of the JWS string.

**Kind**: instance method of [<code>Jws</code>](#Jws)  
<a name="JwsHeader"></a>

## JwsHeader
**Kind**: global class  

* [JwsHeader](#JwsHeader)
    * [new JwsHeader()](#new_JwsHeader_new)
    * _instance_
        * [.alg()](#JwsHeader+alg) ⇒ <code>JwsAlgorithm</code> \| <code>undefined</code>
        * [.setAlg(value)](#JwsHeader+setAlg)
        * [.b64()](#JwsHeader+b64) ⇒ <code>boolean</code> \| <code>undefined</code>
        * [.setB64(value)](#JwsHeader+setB64)
        * [.custom()](#JwsHeader+custom) ⇒ <code>Record.&lt;string, any&gt;</code> \| <code>undefined</code>
        * [.has(claim)](#JwsHeader+has) ⇒ <code>boolean</code>
        * [.isDisjoint(other)](#JwsHeader+isDisjoint) ⇒ <code>boolean</code>
        * [.jku()](#JwsHeader+jku) ⇒ <code>string</code> \| <code>undefined</code>
        * [.setJku(value)](#JwsHeader+setJku)
        * [.jwk()](#JwsHeader+jwk) ⇒ [<code>Jwk</code>](#Jwk) \| <code>undefined</code>
        * [.setJwk(value)](#JwsHeader+setJwk)
        * [.kid()](#JwsHeader+kid) ⇒ <code>string</code> \| <code>undefined</code>
        * [.setKid(value)](#JwsHeader+setKid)
        * [.x5u()](#JwsHeader+x5u) ⇒ <code>string</code> \| <code>undefined</code>
        * [.setX5u(value)](#JwsHeader+setX5u)
        * [.x5c()](#JwsHeader+x5c) ⇒ <code>Array.&lt;string&gt;</code>
        * [.setX5c(value)](#JwsHeader+setX5c)
        * [.x5t()](#JwsHeader+x5t) ⇒ <code>string</code> \| <code>undefined</code>
        * [.setX5t(value)](#JwsHeader+setX5t)
        * [.x5tS256()](#JwsHeader+x5tS256) ⇒ <code>string</code> \| <code>undefined</code>
        * [.setX5tS256(value)](#JwsHeader+setX5tS256)
        * [.typ()](#JwsHeader+typ) ⇒ <code>string</code> \| <code>undefined</code>
        * [.setTyp(value)](#JwsHeader+setTyp)
        * [.cty()](#JwsHeader+cty) ⇒ <code>string</code> \| <code>undefined</code>
        * [.setCty(value)](#JwsHeader+setCty)
        * [.crit()](#JwsHeader+crit) ⇒ <code>Array.&lt;string&gt;</code>
        * [.setCrit(value)](#JwsHeader+setCrit)
        * [.url()](#JwsHeader+url) ⇒ <code>string</code> \| <code>undefined</code>
        * [.setUrl(value)](#JwsHeader+setUrl)
        * [.nonce()](#JwsHeader+nonce) ⇒ <code>string</code> \| <code>undefined</code>
        * [.setNonce(value)](#JwsHeader+setNonce)
        * [.toJSON()](#JwsHeader+toJSON) ⇒ <code>any</code>
        * [.clone()](#JwsHeader+clone) ⇒ [<code>JwsHeader</code>](#JwsHeader)
    * _static_
        * [.fromJSON(json)](#JwsHeader.fromJSON) ⇒ [<code>JwsHeader</code>](#JwsHeader)

<a name="new_JwsHeader_new"></a>

### new JwsHeader()
Create a new empty [JwsHeader](#JwsHeader).

<a name="JwsHeader+alg"></a>

### jwsHeader.alg() ⇒ <code>JwsAlgorithm</code> \| <code>undefined</code>
Returns the value for the algorithm claim (alg).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  
<a name="JwsHeader+setAlg"></a>

### jwsHeader.setAlg(value)
Sets a value for the algorithm claim (alg).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  

| Param | Type |
| --- | --- |
| value | <code>JwsAlgorithm</code> | 

<a name="JwsHeader+b64"></a>

### jwsHeader.b64() ⇒ <code>boolean</code> \| <code>undefined</code>
Returns the value of the base64url-encode payload claim (b64).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  
<a name="JwsHeader+setB64"></a>

### jwsHeader.setB64(value)
Sets a value for the base64url-encode payload claim (b64).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  

| Param | Type |
| --- | --- |
| value | <code>boolean</code> | 

<a name="JwsHeader+custom"></a>

### jwsHeader.custom() ⇒ <code>Record.&lt;string, any&gt;</code> \| <code>undefined</code>
Additional header parameters.

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  
<a name="JwsHeader+has"></a>

### jwsHeader.has(claim) ⇒ <code>boolean</code>
**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  

| Param | Type |
| --- | --- |
| claim | <code>string</code> | 

<a name="JwsHeader+isDisjoint"></a>

### jwsHeader.isDisjoint(other) ⇒ <code>boolean</code>
Returns `true` if none of the fields are set in both `self` and `other`.

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  

| Param | Type |
| --- | --- |
| other | [<code>JwsHeader</code>](#JwsHeader) | 

<a name="JwsHeader+jku"></a>

### jwsHeader.jku() ⇒ <code>string</code> \| <code>undefined</code>
Returns the value of the JWK Set URL claim (jku).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  
<a name="JwsHeader+setJku"></a>

### jwsHeader.setJku(value)
Sets a value for the JWK Set URL claim (jku).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="JwsHeader+jwk"></a>

### jwsHeader.jwk() ⇒ [<code>Jwk</code>](#Jwk) \| <code>undefined</code>
Returns the value of the JWK claim (jwk).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  
<a name="JwsHeader+setJwk"></a>

### jwsHeader.setJwk(value)
Sets a value for the JWK claim (jwk).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  

| Param | Type |
| --- | --- |
| value | [<code>Jwk</code>](#Jwk) | 

<a name="JwsHeader+kid"></a>

### jwsHeader.kid() ⇒ <code>string</code> \| <code>undefined</code>
Returns the value of the key ID claim (kid).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  
<a name="JwsHeader+setKid"></a>

### jwsHeader.setKid(value)
Sets a value for the key ID claim (kid).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="JwsHeader+x5u"></a>

### jwsHeader.x5u() ⇒ <code>string</code> \| <code>undefined</code>
Returns the value of the X.509 URL claim (x5u).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  
<a name="JwsHeader+setX5u"></a>

### jwsHeader.setX5u(value)
Sets a value for the X.509 URL claim (x5u).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="JwsHeader+x5c"></a>

### jwsHeader.x5c() ⇒ <code>Array.&lt;string&gt;</code>
Returns the value of the X.509 certificate chain claim (x5c).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  
<a name="JwsHeader+setX5c"></a>

### jwsHeader.setX5c(value)
Sets values for the X.509 certificate chain claim (x5c).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  

| Param | Type |
| --- | --- |
| value | <code>Array.&lt;string&gt;</code> | 

<a name="JwsHeader+x5t"></a>

### jwsHeader.x5t() ⇒ <code>string</code> \| <code>undefined</code>
Returns the value of the X.509 certificate SHA-1 thumbprint claim (x5t).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  
<a name="JwsHeader+setX5t"></a>

### jwsHeader.setX5t(value)
Sets a value for the X.509 certificate SHA-1 thumbprint claim (x5t).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="JwsHeader+x5tS256"></a>

### jwsHeader.x5tS256() ⇒ <code>string</code> \| <code>undefined</code>
Returns the value of the X.509 certificate SHA-256 thumbprint claim
(x5t#S256).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  
<a name="JwsHeader+setX5tS256"></a>

### jwsHeader.setX5tS256(value)
Sets a value for the X.509 certificate SHA-256 thumbprint claim
(x5t#S256).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="JwsHeader+typ"></a>

### jwsHeader.typ() ⇒ <code>string</code> \| <code>undefined</code>
Returns the value of the token type claim (typ).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  
<a name="JwsHeader+setTyp"></a>

### jwsHeader.setTyp(value)
Sets a value for the token type claim (typ).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="JwsHeader+cty"></a>

### jwsHeader.cty() ⇒ <code>string</code> \| <code>undefined</code>
Returns the value of the content type claim (cty).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  
<a name="JwsHeader+setCty"></a>

### jwsHeader.setCty(value)
Sets a value for the content type claim (cty).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="JwsHeader+crit"></a>

### jwsHeader.crit() ⇒ <code>Array.&lt;string&gt;</code>
Returns the value of the critical claim (crit).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  
<a name="JwsHeader+setCrit"></a>

### jwsHeader.setCrit(value)
Sets values for the critical claim (crit).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  

| Param | Type |
| --- | --- |
| value | <code>Array.&lt;string&gt;</code> | 

<a name="JwsHeader+url"></a>

### jwsHeader.url() ⇒ <code>string</code> \| <code>undefined</code>
Returns the value of the url claim (url).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  
<a name="JwsHeader+setUrl"></a>

### jwsHeader.setUrl(value)
Sets a value for the url claim (url).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="JwsHeader+nonce"></a>

### jwsHeader.nonce() ⇒ <code>string</code> \| <code>undefined</code>
Returns the value of the nonce claim (nonce).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  
<a name="JwsHeader+setNonce"></a>

### jwsHeader.setNonce(value)
Sets a value for the nonce claim (nonce).

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="JwsHeader+toJSON"></a>

### jwsHeader.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  
<a name="JwsHeader+clone"></a>

### jwsHeader.clone() ⇒ [<code>JwsHeader</code>](#JwsHeader)
Deep clones the object.

**Kind**: instance method of [<code>JwsHeader</code>](#JwsHeader)  
<a name="JwsHeader.fromJSON"></a>

### JwsHeader.fromJSON(json) ⇒ [<code>JwsHeader</code>](#JwsHeader)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>JwsHeader</code>](#JwsHeader)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="JwsSignatureOptions"></a>

## JwsSignatureOptions
**Kind**: global class  

* [JwsSignatureOptions](#JwsSignatureOptions)
    * [new JwsSignatureOptions([options])](#new_JwsSignatureOptions_new)
    * _instance_
        * [.setAttachJwk(value)](#JwsSignatureOptions+setAttachJwk)
        * [.setB64(value)](#JwsSignatureOptions+setB64)
        * [.setTyp(value)](#JwsSignatureOptions+setTyp)
        * [.setCty(value)](#JwsSignatureOptions+setCty)
        * [.serUrl(value)](#JwsSignatureOptions+serUrl)
        * [.setNonce(value)](#JwsSignatureOptions+setNonce)
        * [.setKid(value)](#JwsSignatureOptions+setKid)
        * [.setDetachedPayload(value)](#JwsSignatureOptions+setDetachedPayload)
        * [.setCustomHeaderParameters(value)](#JwsSignatureOptions+setCustomHeaderParameters)
        * [.toJSON()](#JwsSignatureOptions+toJSON) ⇒ <code>any</code>
        * [.clone()](#JwsSignatureOptions+clone) ⇒ [<code>JwsSignatureOptions</code>](#JwsSignatureOptions)
    * _static_
        * [.fromJSON(json)](#JwsSignatureOptions.fromJSON) ⇒ [<code>JwsSignatureOptions</code>](#JwsSignatureOptions)

<a name="new_JwsSignatureOptions_new"></a>

### new JwsSignatureOptions([options])

| Param | Type |
| --- | --- |
| [options] | <code>IJwsSignatureOptions</code> \| <code>undefined</code> | 

<a name="JwsSignatureOptions+setAttachJwk"></a>

### jwsSignatureOptions.setAttachJwk(value)
Replace the value of the `attachJwk` field.

**Kind**: instance method of [<code>JwsSignatureOptions</code>](#JwsSignatureOptions)  

| Param | Type |
| --- | --- |
| value | <code>boolean</code> | 

<a name="JwsSignatureOptions+setB64"></a>

### jwsSignatureOptions.setB64(value)
Replace the value of the `b64` field.

**Kind**: instance method of [<code>JwsSignatureOptions</code>](#JwsSignatureOptions)  

| Param | Type |
| --- | --- |
| value | <code>boolean</code> | 

<a name="JwsSignatureOptions+setTyp"></a>

### jwsSignatureOptions.setTyp(value)
Replace the value of the `typ` field.

**Kind**: instance method of [<code>JwsSignatureOptions</code>](#JwsSignatureOptions)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="JwsSignatureOptions+setCty"></a>

### jwsSignatureOptions.setCty(value)
Replace the value of the `cty` field.

**Kind**: instance method of [<code>JwsSignatureOptions</code>](#JwsSignatureOptions)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="JwsSignatureOptions+serUrl"></a>

### jwsSignatureOptions.serUrl(value)
Replace the value of the `url` field.

**Kind**: instance method of [<code>JwsSignatureOptions</code>](#JwsSignatureOptions)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="JwsSignatureOptions+setNonce"></a>

### jwsSignatureOptions.setNonce(value)
Replace the value of the `nonce` field.

**Kind**: instance method of [<code>JwsSignatureOptions</code>](#JwsSignatureOptions)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="JwsSignatureOptions+setKid"></a>

### jwsSignatureOptions.setKid(value)
Replace the value of the `kid` field.

**Kind**: instance method of [<code>JwsSignatureOptions</code>](#JwsSignatureOptions)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="JwsSignatureOptions+setDetachedPayload"></a>

### jwsSignatureOptions.setDetachedPayload(value)
Replace the value of the `detached_payload` field.

**Kind**: instance method of [<code>JwsSignatureOptions</code>](#JwsSignatureOptions)  

| Param | Type |
| --- | --- |
| value | <code>boolean</code> | 

<a name="JwsSignatureOptions+setCustomHeaderParameters"></a>

### jwsSignatureOptions.setCustomHeaderParameters(value)
Add additional header parameters.

**Kind**: instance method of [<code>JwsSignatureOptions</code>](#JwsSignatureOptions)  

| Param | Type |
| --- | --- |
| value | <code>Record.&lt;string, any&gt;</code> | 

<a name="JwsSignatureOptions+toJSON"></a>

### jwsSignatureOptions.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>JwsSignatureOptions</code>](#JwsSignatureOptions)  
<a name="JwsSignatureOptions+clone"></a>

### jwsSignatureOptions.clone() ⇒ [<code>JwsSignatureOptions</code>](#JwsSignatureOptions)
Deep clones the object.

**Kind**: instance method of [<code>JwsSignatureOptions</code>](#JwsSignatureOptions)  
<a name="JwsSignatureOptions.fromJSON"></a>

### JwsSignatureOptions.fromJSON(json) ⇒ [<code>JwsSignatureOptions</code>](#JwsSignatureOptions)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>JwsSignatureOptions</code>](#JwsSignatureOptions)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="JwsVerificationOptions"></a>

## JwsVerificationOptions
**Kind**: global class  

* [JwsVerificationOptions](#JwsVerificationOptions)
    * [new JwsVerificationOptions([options])](#new_JwsVerificationOptions_new)
    * _instance_
        * [.setNonce(value)](#JwsVerificationOptions+setNonce)
        * [.setMethodScope(value)](#JwsVerificationOptions+setMethodScope)
        * [.setMethodId(value)](#JwsVerificationOptions+setMethodId)
        * [.toJSON()](#JwsVerificationOptions+toJSON) ⇒ <code>any</code>
        * [.clone()](#JwsVerificationOptions+clone) ⇒ [<code>JwsVerificationOptions</code>](#JwsVerificationOptions)
    * _static_
        * [.fromJSON(json)](#JwsVerificationOptions.fromJSON) ⇒ [<code>JwsVerificationOptions</code>](#JwsVerificationOptions)

<a name="new_JwsVerificationOptions_new"></a>

### new JwsVerificationOptions([options])
Creates a new [JwsVerificationOptions](#JwsVerificationOptions) from the given fields.


| Param | Type |
| --- | --- |
| [options] | <code>IJwsVerificationOptions</code> \| <code>undefined</code> | 

<a name="JwsVerificationOptions+setNonce"></a>

### jwsVerificationOptions.setNonce(value)
Set the expected value for the `nonce` parameter of the protected header.

**Kind**: instance method of [<code>JwsVerificationOptions</code>](#JwsVerificationOptions)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="JwsVerificationOptions+setMethodScope"></a>

### jwsVerificationOptions.setMethodScope(value)
Set the scope of the verification methods that may be used to verify the given JWS.

**Kind**: instance method of [<code>JwsVerificationOptions</code>](#JwsVerificationOptions)  

| Param | Type |
| --- | --- |
| value | [<code>MethodScope</code>](#MethodScope) | 

<a name="JwsVerificationOptions+setMethodId"></a>

### jwsVerificationOptions.setMethodId(value)
Set the DID URl of the method, whose JWK should be used to verify the JWS.

**Kind**: instance method of [<code>JwsVerificationOptions</code>](#JwsVerificationOptions)  

| Param | Type |
| --- | --- |
| value | [<code>DIDUrl</code>](#DIDUrl) | 

<a name="JwsVerificationOptions+toJSON"></a>

### jwsVerificationOptions.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>JwsVerificationOptions</code>](#JwsVerificationOptions)  
<a name="JwsVerificationOptions+clone"></a>

### jwsVerificationOptions.clone() ⇒ [<code>JwsVerificationOptions</code>](#JwsVerificationOptions)
Deep clones the object.

**Kind**: instance method of [<code>JwsVerificationOptions</code>](#JwsVerificationOptions)  
<a name="JwsVerificationOptions.fromJSON"></a>

### JwsVerificationOptions.fromJSON(json) ⇒ [<code>JwsVerificationOptions</code>](#JwsVerificationOptions)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>JwsVerificationOptions</code>](#JwsVerificationOptions)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="Jwt"></a>

## Jwt
A wrapper around a JSON Web Token (JWK).

**Kind**: global class  

* [Jwt](#Jwt)
    * [new Jwt(jwt_string)](#new_Jwt_new)
    * _instance_
        * [.toString()](#Jwt+toString) ⇒ <code>string</code>
        * [.toJSON()](#Jwt+toJSON) ⇒ <code>any</code>
        * [.clone()](#Jwt+clone) ⇒ [<code>Jwt</code>](#Jwt)
    * _static_
        * [.fromJSON(json)](#Jwt.fromJSON) ⇒ [<code>Jwt</code>](#Jwt)

<a name="new_Jwt_new"></a>

### new Jwt(jwt_string)
Creates a new [Jwt](#Jwt) from the given string.


| Param | Type |
| --- | --- |
| jwt_string | <code>string</code> | 

<a name="Jwt+toString"></a>

### jwt.toString() ⇒ <code>string</code>
Returns a clone of the JWT string.

**Kind**: instance method of [<code>Jwt</code>](#Jwt)  
<a name="Jwt+toJSON"></a>

### jwt.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>Jwt</code>](#Jwt)  
<a name="Jwt+clone"></a>

### jwt.clone() ⇒ [<code>Jwt</code>](#Jwt)
Deep clones the object.

**Kind**: instance method of [<code>Jwt</code>](#Jwt)  
<a name="Jwt.fromJSON"></a>

### Jwt.fromJSON(json) ⇒ [<code>Jwt</code>](#Jwt)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>Jwt</code>](#Jwt)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="JwtCredentialValidationOptions"></a>

## JwtCredentialValidationOptions
Options to declare validation criteria when validating credentials.

**Kind**: global class  

* [JwtCredentialValidationOptions](#JwtCredentialValidationOptions)
    * [new JwtCredentialValidationOptions([options])](#new_JwtCredentialValidationOptions_new)
    * _instance_
        * [.toJSON()](#JwtCredentialValidationOptions+toJSON) ⇒ <code>any</code>
        * [.clone()](#JwtCredentialValidationOptions+clone) ⇒ [<code>JwtCredentialValidationOptions</code>](#JwtCredentialValidationOptions)
    * _static_
        * [.fromJSON(json)](#JwtCredentialValidationOptions.fromJSON) ⇒ [<code>JwtCredentialValidationOptions</code>](#JwtCredentialValidationOptions)

<a name="new_JwtCredentialValidationOptions_new"></a>

### new JwtCredentialValidationOptions([options])

| Param | Type |
| --- | --- |
| [options] | <code>IJwtCredentialValidationOptions</code> \| <code>undefined</code> | 

<a name="JwtCredentialValidationOptions+toJSON"></a>

### jwtCredentialValidationOptions.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>JwtCredentialValidationOptions</code>](#JwtCredentialValidationOptions)  
<a name="JwtCredentialValidationOptions+clone"></a>

### jwtCredentialValidationOptions.clone() ⇒ [<code>JwtCredentialValidationOptions</code>](#JwtCredentialValidationOptions)
Deep clones the object.

**Kind**: instance method of [<code>JwtCredentialValidationOptions</code>](#JwtCredentialValidationOptions)  
<a name="JwtCredentialValidationOptions.fromJSON"></a>

### JwtCredentialValidationOptions.fromJSON(json) ⇒ [<code>JwtCredentialValidationOptions</code>](#JwtCredentialValidationOptions)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>JwtCredentialValidationOptions</code>](#JwtCredentialValidationOptions)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="JwtCredentialValidator"></a>

## JwtCredentialValidator
A type for decoding and validating [Credential](#Credential).

**Kind**: global class  

* [JwtCredentialValidator](#JwtCredentialValidator)
    * [new JwtCredentialValidator(signatureVerifier)](#new_JwtCredentialValidator_new)
    * _instance_
        * [.validate(credential_jwt, issuer, options, fail_fast)](#JwtCredentialValidator+validate) ⇒ [<code>DecodedJwtCredential</code>](#DecodedJwtCredential)
        * [.verifySignature(credential, trustedIssuers, options)](#JwtCredentialValidator+verifySignature) ⇒ [<code>DecodedJwtCredential</code>](#DecodedJwtCredential)
    * _static_
        * [.checkExpiresOnOrAfter(credential, timestamp)](#JwtCredentialValidator.checkExpiresOnOrAfter)
        * [.checkIssuedOnOrBefore(credential, timestamp)](#JwtCredentialValidator.checkIssuedOnOrBefore)
        * [.checkSubjectHolderRelationship(credential, holder, relationship)](#JwtCredentialValidator.checkSubjectHolderRelationship)
        * [.checkStatus(credential, trustedIssuers, statusCheck)](#JwtCredentialValidator.checkStatus)
        * [.checkStatusWithStatusList2021(credential, status_list, status_check)](#JwtCredentialValidator.checkStatusWithStatusList2021)
        * [.extractIssuer(credential)](#JwtCredentialValidator.extractIssuer) ⇒ [<code>CoreDID</code>](#CoreDID)
        * [.extractIssuerFromJwt(credential)](#JwtCredentialValidator.extractIssuerFromJwt) ⇒ [<code>CoreDID</code>](#CoreDID)

<a name="new_JwtCredentialValidator_new"></a>

### new JwtCredentialValidator(signatureVerifier)
Creates a new [JwtCredentialValidator](#JwtCredentialValidator). If a `signatureVerifier` is provided it will be used when
verifying decoded JWS signatures, otherwise the default which is only capable of handling the `EdDSA`
algorithm will be used.


| Param | Type |
| --- | --- |
| signatureVerifier | <code>IJwsVerifier</code> | 

<a name="JwtCredentialValidator+validate"></a>

### jwtCredentialValidator.validate(credential_jwt, issuer, options, fail_fast) ⇒ [<code>DecodedJwtCredential</code>](#DecodedJwtCredential)
Decodes and validates a [Credential](#Credential) issued as a JWS. A [DecodedJwtCredential](#DecodedJwtCredential) is returned upon
success.

The following properties are validated according to `options`:
- the issuer's signature on the JWS,
- the expiration date,
- the issuance date,
- the semantic structure.

# Warning
The lack of an error returned from this method is in of itself not enough to conclude that the credential can be
trusted. This section contains more information on additional checks that should be carried out before and after
calling this method.

## The state of the issuer's DID Document
The caller must ensure that `issuer` represents an up-to-date DID Document.

## Properties that are not validated
 There are many properties defined in [The Verifiable Credentials Data Model](https://www.w3.org/TR/vc-data-model/) that are **not** validated, such as:
`proof`, `credentialStatus`, `type`, `credentialSchema`, `refreshService` **and more**.
These should be manually checked after validation, according to your requirements.

# Errors
An error is returned whenever a validated condition is not satisfied.

**Kind**: instance method of [<code>JwtCredentialValidator</code>](#JwtCredentialValidator)  

| Param | Type |
| --- | --- |
| credential_jwt | [<code>Jwt</code>](#Jwt) | 
| issuer | [<code>CoreDocument</code>](#CoreDocument) \| <code>IToCoreDocument</code> | 
| options | [<code>JwtCredentialValidationOptions</code>](#JwtCredentialValidationOptions) | 
| fail_fast | [<code>FailFast</code>](#FailFast) | 

<a name="JwtCredentialValidator+verifySignature"></a>

### jwtCredentialValidator.verifySignature(credential, trustedIssuers, options) ⇒ [<code>DecodedJwtCredential</code>](#DecodedJwtCredential)
Decode and verify the JWS signature of a [Credential](#Credential) issued as a JWT using the DID Document of a trusted
issuer.

A [DecodedJwtCredential](#DecodedJwtCredential) is returned upon success.

# Warning
The caller must ensure that the DID Documents of the trusted issuers are up-to-date.

## Proofs
 Only the JWS signature is verified. If the [Credential](#Credential) contains a `proof` property this will not be
verified by this method.

# Errors
This method immediately returns an error if
the credential issuer' url cannot be parsed to a DID belonging to one of the trusted issuers. Otherwise an attempt
to verify the credential's signature will be made and an error is returned upon failure.

**Kind**: instance method of [<code>JwtCredentialValidator</code>](#JwtCredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Jwt</code>](#Jwt) | 
| trustedIssuers | <code>Array.&lt;(CoreDocument\|IToCoreDocument)&gt;</code> | 
| options | [<code>JwsVerificationOptions</code>](#JwsVerificationOptions) | 

<a name="JwtCredentialValidator.checkExpiresOnOrAfter"></a>

### JwtCredentialValidator.checkExpiresOnOrAfter(credential, timestamp)
Validate that the credential expires on or after the specified timestamp.

**Kind**: static method of [<code>JwtCredentialValidator</code>](#JwtCredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| timestamp | [<code>Timestamp</code>](#Timestamp) | 

<a name="JwtCredentialValidator.checkIssuedOnOrBefore"></a>

### JwtCredentialValidator.checkIssuedOnOrBefore(credential, timestamp)
Validate that the credential is issued on or before the specified timestamp.

**Kind**: static method of [<code>JwtCredentialValidator</code>](#JwtCredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| timestamp | [<code>Timestamp</code>](#Timestamp) | 

<a name="JwtCredentialValidator.checkSubjectHolderRelationship"></a>

### JwtCredentialValidator.checkSubjectHolderRelationship(credential, holder, relationship)
Validate that the relationship between the `holder` and the credential subjects is in accordance with
`relationship`. The `holder` parameter is expected to be the URL of the holder.

**Kind**: static method of [<code>JwtCredentialValidator</code>](#JwtCredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| holder | <code>string</code> | 
| relationship | [<code>SubjectHolderRelationship</code>](#SubjectHolderRelationship) | 

<a name="JwtCredentialValidator.checkStatus"></a>

### JwtCredentialValidator.checkStatus(credential, trustedIssuers, statusCheck)
Checks whether the credential status has been revoked.

Only supports `RevocationBitmap2022`.

**Kind**: static method of [<code>JwtCredentialValidator</code>](#JwtCredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| trustedIssuers | <code>Array.&lt;(CoreDocument\|IToCoreDocument)&gt;</code> | 
| statusCheck | [<code>StatusCheck</code>](#StatusCheck) | 

<a name="JwtCredentialValidator.checkStatusWithStatusList2021"></a>

### JwtCredentialValidator.checkStatusWithStatusList2021(credential, status_list, status_check)
Checks wheter the credential status has been revoked using `StatusList2021`.

**Kind**: static method of [<code>JwtCredentialValidator</code>](#JwtCredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| status_list | [<code>StatusList2021Credential</code>](#StatusList2021Credential) | 
| status_check | [<code>StatusCheck</code>](#StatusCheck) | 

<a name="JwtCredentialValidator.extractIssuer"></a>

### JwtCredentialValidator.extractIssuer(credential) ⇒ [<code>CoreDID</code>](#CoreDID)
Utility for extracting the issuer field of a [Credential](#Credential) as a DID.

### Errors

Fails if the issuer field is not a valid DID.

**Kind**: static method of [<code>JwtCredentialValidator</code>](#JwtCredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 

<a name="JwtCredentialValidator.extractIssuerFromJwt"></a>

### JwtCredentialValidator.extractIssuerFromJwt(credential) ⇒ [<code>CoreDID</code>](#CoreDID)
Utility for extracting the issuer field of a credential in JWT representation as DID.

# Errors

If the JWT decoding fails or the issuer field is not a valid DID.

**Kind**: static method of [<code>JwtCredentialValidator</code>](#JwtCredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Jwt</code>](#Jwt) | 

<a name="JwtDomainLinkageValidator"></a>

## JwtDomainLinkageValidator
A validator for a Domain Linkage Configuration and Credentials.

**Kind**: global class  

* [JwtDomainLinkageValidator](#JwtDomainLinkageValidator)
    * [new JwtDomainLinkageValidator(signatureVerifier)](#new_JwtDomainLinkageValidator_new)
    * [.validateLinkage(issuer, configuration, domain, options)](#JwtDomainLinkageValidator+validateLinkage)
    * [.validateCredential(issuer, credentialJwt, domain, options)](#JwtDomainLinkageValidator+validateCredential)

<a name="new_JwtDomainLinkageValidator_new"></a>

### new JwtDomainLinkageValidator(signatureVerifier)
Creates a new [JwtDomainLinkageValidator](#JwtDomainLinkageValidator). If a `signatureVerifier` is provided it will be used when
verifying decoded JWS signatures, otherwise the default which is only capable of handling the `EdDSA`
algorithm will be used.


| Param | Type |
| --- | --- |
| signatureVerifier | <code>IJwsVerifier</code> | 

<a name="JwtDomainLinkageValidator+validateLinkage"></a>

### jwtDomainLinkageValidator.validateLinkage(issuer, configuration, domain, options)
Validates the linkage between a domain and a DID.
[DomainLinkageConfiguration](#DomainLinkageConfiguration) is validated according to [DID Configuration Resource Verification](https://identity.foundation/.well-known/resources/did-configuration/#did-configuration-resource-verification).

Linkage is valid if no error is thrown.

# Note:
- Only the [JSON Web Token Proof Format](https://identity.foundation/.well-known/resources/did-configuration/#json-web-token-proof-format)
  is supported.
- Only the Credential issued by `issuer` is verified.

# Errors

 - Semantic structure of `configuration` is invalid.
 - `configuration` includes multiple credentials issued by `issuer`.
 - Validation of the matched Domain Linkage Credential fails.

**Kind**: instance method of [<code>JwtDomainLinkageValidator</code>](#JwtDomainLinkageValidator)  

| Param | Type |
| --- | --- |
| issuer | [<code>CoreDocument</code>](#CoreDocument) \| <code>IToCoreDocument</code> | 
| configuration | [<code>DomainLinkageConfiguration</code>](#DomainLinkageConfiguration) | 
| domain | <code>string</code> | 
| options | [<code>JwtCredentialValidationOptions</code>](#JwtCredentialValidationOptions) | 

<a name="JwtDomainLinkageValidator+validateCredential"></a>

### jwtDomainLinkageValidator.validateCredential(issuer, credentialJwt, domain, options)
Validates a [Domain Linkage Credential](https://identity.foundation/.well-known/resources/did-configuration/#domain-linkage-credential).

Error will be thrown in case the validation fails.

**Kind**: instance method of [<code>JwtDomainLinkageValidator</code>](#JwtDomainLinkageValidator)  

| Param | Type |
| --- | --- |
| issuer | [<code>CoreDocument</code>](#CoreDocument) \| <code>IToCoreDocument</code> | 
| credentialJwt | [<code>Jwt</code>](#Jwt) | 
| domain | <code>string</code> | 
| options | [<code>JwtCredentialValidationOptions</code>](#JwtCredentialValidationOptions) | 

<a name="JwtPresentationOptions"></a>

## JwtPresentationOptions
**Kind**: global class  

* [JwtPresentationOptions](#JwtPresentationOptions)
    * [new JwtPresentationOptions([options])](#new_JwtPresentationOptions_new)
    * _instance_
        * [.toJSON()](#JwtPresentationOptions+toJSON) ⇒ <code>any</code>
        * [.clone()](#JwtPresentationOptions+clone) ⇒ [<code>JwtPresentationOptions</code>](#JwtPresentationOptions)
    * _static_
        * [.fromJSON(json)](#JwtPresentationOptions.fromJSON) ⇒ [<code>JwtPresentationOptions</code>](#JwtPresentationOptions)

<a name="new_JwtPresentationOptions_new"></a>

### new JwtPresentationOptions([options])
Creates a new [JwtPresentationOptions](#JwtPresentationOptions) from the given fields.

Throws an error if any of the options are invalid.


| Param | Type |
| --- | --- |
| [options] | <code>IJwtPresentationOptions</code> \| <code>undefined</code> | 

<a name="JwtPresentationOptions+toJSON"></a>

### jwtPresentationOptions.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>JwtPresentationOptions</code>](#JwtPresentationOptions)  
<a name="JwtPresentationOptions+clone"></a>

### jwtPresentationOptions.clone() ⇒ [<code>JwtPresentationOptions</code>](#JwtPresentationOptions)
Deep clones the object.

**Kind**: instance method of [<code>JwtPresentationOptions</code>](#JwtPresentationOptions)  
<a name="JwtPresentationOptions.fromJSON"></a>

### JwtPresentationOptions.fromJSON(json) ⇒ [<code>JwtPresentationOptions</code>](#JwtPresentationOptions)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>JwtPresentationOptions</code>](#JwtPresentationOptions)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="JwtPresentationValidationOptions"></a>

## JwtPresentationValidationOptions
Options to declare validation criteria when validating presentation.

**Kind**: global class  

* [JwtPresentationValidationOptions](#JwtPresentationValidationOptions)
    * [new JwtPresentationValidationOptions([options])](#new_JwtPresentationValidationOptions_new)
    * _instance_
        * [.toJSON()](#JwtPresentationValidationOptions+toJSON) ⇒ <code>any</code>
        * [.clone()](#JwtPresentationValidationOptions+clone) ⇒ [<code>JwtPresentationValidationOptions</code>](#JwtPresentationValidationOptions)
    * _static_
        * [.fromJSON(json)](#JwtPresentationValidationOptions.fromJSON) ⇒ [<code>JwtPresentationValidationOptions</code>](#JwtPresentationValidationOptions)

<a name="new_JwtPresentationValidationOptions_new"></a>

### new JwtPresentationValidationOptions([options])
Creates a new [JwtPresentationValidationOptions](#JwtPresentationValidationOptions) from the given fields.

Throws an error if any of the options are invalid.


| Param | Type |
| --- | --- |
| [options] | <code>IJwtPresentationValidationOptions</code> \| <code>undefined</code> | 

<a name="JwtPresentationValidationOptions+toJSON"></a>

### jwtPresentationValidationOptions.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>JwtPresentationValidationOptions</code>](#JwtPresentationValidationOptions)  
<a name="JwtPresentationValidationOptions+clone"></a>

### jwtPresentationValidationOptions.clone() ⇒ [<code>JwtPresentationValidationOptions</code>](#JwtPresentationValidationOptions)
Deep clones the object.

**Kind**: instance method of [<code>JwtPresentationValidationOptions</code>](#JwtPresentationValidationOptions)  
<a name="JwtPresentationValidationOptions.fromJSON"></a>

### JwtPresentationValidationOptions.fromJSON(json) ⇒ [<code>JwtPresentationValidationOptions</code>](#JwtPresentationValidationOptions)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>JwtPresentationValidationOptions</code>](#JwtPresentationValidationOptions)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="JwtPresentationValidator"></a>

## JwtPresentationValidator
**Kind**: global class  

* [JwtPresentationValidator](#JwtPresentationValidator)
    * [new JwtPresentationValidator(signatureVerifier)](#new_JwtPresentationValidator_new)
    * _instance_
        * [.validate(presentationJwt, holder, validation_options)](#JwtPresentationValidator+validate) ⇒ [<code>DecodedJwtPresentation</code>](#DecodedJwtPresentation)
    * _static_
        * [.checkStructure(presentation)](#JwtPresentationValidator.checkStructure)
        * [.extractHolder(presentation)](#JwtPresentationValidator.extractHolder) ⇒ [<code>CoreDID</code>](#CoreDID)

<a name="new_JwtPresentationValidator_new"></a>

### new JwtPresentationValidator(signatureVerifier)
Creates a new [JwtPresentationValidator](#JwtPresentationValidator). If a `signatureVerifier` is provided it will be used when
verifying decoded JWS signatures, otherwise the default which is only capable of handling the `EdDSA`
algorithm will be used.


| Param | Type |
| --- | --- |
| signatureVerifier | <code>IJwsVerifier</code> | 

<a name="JwtPresentationValidator+validate"></a>

### jwtPresentationValidator.validate(presentationJwt, holder, validation_options) ⇒ [<code>DecodedJwtPresentation</code>](#DecodedJwtPresentation)
Validates a [Presentation](#Presentation) encoded as a [Jwt](#Jwt).

The following properties are validated according to `options`:
- the JWT can be decoded into a semantically valid presentation.
- the expiration and issuance date contained in the JWT claims.
- the holder's signature.

Validation is done with respect to the properties set in `options`.

# Warning

* This method does NOT validate the constituent credentials and therefore also not the relationship between the
credentials' subjects and the presentation holder. This can be done with [JwtCredentialValidationOptions](#JwtCredentialValidationOptions).
* The lack of an error returned from this method is in of itself not enough to conclude that the presentation can
be trusted. This section contains more information on additional checks that should be carried out before and
after calling this method.

## The state of the supplied DID Documents.

The caller must ensure that the DID Documents in `holder` are up-to-date.

# Errors

An error is returned whenever a validated condition is not satisfied or when decoding fails.

**Kind**: instance method of [<code>JwtPresentationValidator</code>](#JwtPresentationValidator)  

| Param | Type |
| --- | --- |
| presentationJwt | [<code>Jwt</code>](#Jwt) | 
| holder | [<code>CoreDocument</code>](#CoreDocument) \| <code>IToCoreDocument</code> | 
| validation_options | [<code>JwtPresentationValidationOptions</code>](#JwtPresentationValidationOptions) | 

<a name="JwtPresentationValidator.checkStructure"></a>

### JwtPresentationValidator.checkStructure(presentation)
Validates the semantic structure of the [Presentation](#Presentation).

**Kind**: static method of [<code>JwtPresentationValidator</code>](#JwtPresentationValidator)  

| Param | Type |
| --- | --- |
| presentation | [<code>Presentation</code>](#Presentation) | 

<a name="JwtPresentationValidator.extractHolder"></a>

### JwtPresentationValidator.extractHolder(presentation) ⇒ [<code>CoreDID</code>](#CoreDID)
Attempt to extract the holder of the presentation.

# Errors:
* If deserialization/decoding of the presentation fails.
* If the holder can't be parsed as DIDs.

**Kind**: static method of [<code>JwtPresentationValidator</code>](#JwtPresentationValidator)  

| Param | Type |
| --- | --- |
| presentation | [<code>Jwt</code>](#Jwt) | 

<a name="KeyBindingJWTValidationOptions"></a>

## KeyBindingJWTValidationOptions
Options to declare validation criteria when validating credentials.

**Kind**: global class  

* [KeyBindingJWTValidationOptions](#KeyBindingJWTValidationOptions)
    * [new KeyBindingJWTValidationOptions([options])](#new_KeyBindingJWTValidationOptions_new)
    * _instance_
        * [.toJSON()](#KeyBindingJWTValidationOptions+toJSON) ⇒ <code>any</code>
        * [.clone()](#KeyBindingJWTValidationOptions+clone) ⇒ [<code>KeyBindingJWTValidationOptions</code>](#KeyBindingJWTValidationOptions)
    * _static_
        * [.fromJSON(json)](#KeyBindingJWTValidationOptions.fromJSON) ⇒ [<code>KeyBindingJWTValidationOptions</code>](#KeyBindingJWTValidationOptions)

<a name="new_KeyBindingJWTValidationOptions_new"></a>

### new KeyBindingJWTValidationOptions([options])

| Param | Type |
| --- | --- |
| [options] | <code>IKeyBindingJWTValidationOptions</code> \| <code>undefined</code> | 

<a name="KeyBindingJWTValidationOptions+toJSON"></a>

### keyBindingJWTValidationOptions.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>KeyBindingJWTValidationOptions</code>](#KeyBindingJWTValidationOptions)  
<a name="KeyBindingJWTValidationOptions+clone"></a>

### keyBindingJWTValidationOptions.clone() ⇒ [<code>KeyBindingJWTValidationOptions</code>](#KeyBindingJWTValidationOptions)
Deep clones the object.

**Kind**: instance method of [<code>KeyBindingJWTValidationOptions</code>](#KeyBindingJWTValidationOptions)  
<a name="KeyBindingJWTValidationOptions.fromJSON"></a>

### KeyBindingJWTValidationOptions.fromJSON(json) ⇒ [<code>KeyBindingJWTValidationOptions</code>](#KeyBindingJWTValidationOptions)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>KeyBindingJWTValidationOptions</code>](#KeyBindingJWTValidationOptions)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="KeyBindingJwtClaims"></a>

## KeyBindingJwtClaims
Claims set for key binding JWT.

**Kind**: global class  

* [KeyBindingJwtClaims](#KeyBindingJwtClaims)
    * [new KeyBindingJwtClaims(jwt, disclosures, nonce, aud, [issued_at], [custom_properties])](#new_KeyBindingJwtClaims_new)
    * _instance_
        * [.toString()](#KeyBindingJwtClaims+toString) ⇒ <code>string</code>
        * [.iat()](#KeyBindingJwtClaims+iat) ⇒ <code>bigint</code>
        * [.aud()](#KeyBindingJwtClaims+aud) ⇒ <code>string</code>
        * [.nonce()](#KeyBindingJwtClaims+nonce) ⇒ <code>string</code>
        * [.sdHash()](#KeyBindingJwtClaims+sdHash) ⇒ <code>string</code>
        * [.customProperties()](#KeyBindingJwtClaims+customProperties) ⇒ <code>Record.&lt;string, any&gt;</code>
        * [.toJSON()](#KeyBindingJwtClaims+toJSON) ⇒ <code>any</code>
        * [.clone()](#KeyBindingJwtClaims+clone) ⇒ [<code>KeyBindingJwtClaims</code>](#KeyBindingJwtClaims)
    * _static_
        * [.keyBindingJwtHeaderTyp()](#KeyBindingJwtClaims.keyBindingJwtHeaderTyp) ⇒ <code>string</code>
        * [.fromJSON(json)](#KeyBindingJwtClaims.fromJSON) ⇒ [<code>KeyBindingJwtClaims</code>](#KeyBindingJwtClaims)

<a name="new_KeyBindingJwtClaims_new"></a>

### new KeyBindingJwtClaims(jwt, disclosures, nonce, aud, [issued_at], [custom_properties])
Creates a new [`KeyBindingJwtClaims`].
When `issued_at` is left as None, it will automatically default to the current time.

# Error
When `issued_at` is set to `None` and the system returns time earlier than `SystemTime::UNIX_EPOCH`.


| Param | Type |
| --- | --- |
| jwt | <code>string</code> | 
| disclosures | <code>Array.&lt;string&gt;</code> | 
| nonce | <code>string</code> | 
| aud | <code>string</code> | 
| [issued_at] | [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code> | 
| [custom_properties] | <code>Record.&lt;string, any&gt;</code> \| <code>undefined</code> | 

<a name="KeyBindingJwtClaims+toString"></a>

### keyBindingJwtClaims.toString() ⇒ <code>string</code>
Returns a string representation of the claims.

**Kind**: instance method of [<code>KeyBindingJwtClaims</code>](#KeyBindingJwtClaims)  
<a name="KeyBindingJwtClaims+iat"></a>

### keyBindingJwtClaims.iat() ⇒ <code>bigint</code>
Returns a copy of the issued at `iat` property.

**Kind**: instance method of [<code>KeyBindingJwtClaims</code>](#KeyBindingJwtClaims)  
<a name="KeyBindingJwtClaims+aud"></a>

### keyBindingJwtClaims.aud() ⇒ <code>string</code>
Returns a copy of the audience `aud` property.

**Kind**: instance method of [<code>KeyBindingJwtClaims</code>](#KeyBindingJwtClaims)  
<a name="KeyBindingJwtClaims+nonce"></a>

### keyBindingJwtClaims.nonce() ⇒ <code>string</code>
Returns a copy of the `nonce` property.

**Kind**: instance method of [<code>KeyBindingJwtClaims</code>](#KeyBindingJwtClaims)  
<a name="KeyBindingJwtClaims+sdHash"></a>

### keyBindingJwtClaims.sdHash() ⇒ <code>string</code>
Returns a copy of the `sd_hash` property.

**Kind**: instance method of [<code>KeyBindingJwtClaims</code>](#KeyBindingJwtClaims)  
<a name="KeyBindingJwtClaims+customProperties"></a>

### keyBindingJwtClaims.customProperties() ⇒ <code>Record.&lt;string, any&gt;</code>
Returns a copy of the custom properties.

**Kind**: instance method of [<code>KeyBindingJwtClaims</code>](#KeyBindingJwtClaims)  
<a name="KeyBindingJwtClaims+toJSON"></a>

### keyBindingJwtClaims.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>KeyBindingJwtClaims</code>](#KeyBindingJwtClaims)  
<a name="KeyBindingJwtClaims+clone"></a>

### keyBindingJwtClaims.clone() ⇒ [<code>KeyBindingJwtClaims</code>](#KeyBindingJwtClaims)
Deep clones the object.

**Kind**: instance method of [<code>KeyBindingJwtClaims</code>](#KeyBindingJwtClaims)  
<a name="KeyBindingJwtClaims.keyBindingJwtHeaderTyp"></a>

### KeyBindingJwtClaims.keyBindingJwtHeaderTyp() ⇒ <code>string</code>
Returns the value of the `typ` property of the JWT header according to
https://www.ietf.org/archive/id/draft-ietf-oauth-selective-disclosure-jwt-07.html#name-key-binding-jwt

**Kind**: static method of [<code>KeyBindingJwtClaims</code>](#KeyBindingJwtClaims)  
<a name="KeyBindingJwtClaims.fromJSON"></a>

### KeyBindingJwtClaims.fromJSON(json) ⇒ [<code>KeyBindingJwtClaims</code>](#KeyBindingJwtClaims)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>KeyBindingJwtClaims</code>](#KeyBindingJwtClaims)  

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
Constructs a new [LinkedDomainService](#LinkedDomainService) that wraps a spec compliant [Linked Domain Service Endpoint](https://identity.foundation/.well-known/resources/did-configuration/#linked-domain-service-endpoint).

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
Creates a new [LinkedDomainService](#LinkedDomainService) from a [Service](#Service).

# Error

Errors if `service` is not a valid Linked Domain Service.

**Kind**: static method of [<code>LinkedDomainService</code>](#LinkedDomainService)  

| Param | Type |
| --- | --- |
| service | [<code>Service</code>](#Service) | 

<a name="LinkedDomainService.isValid"></a>

### LinkedDomainService.isValid(service) ⇒ <code>boolean</code>
Returns `true` if a [Service](#Service) is a valid Linked Domain Service.

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
        * [.tryCustom()](#MethodData+tryCustom) ⇒ [<code>CustomMethodData</code>](#CustomMethodData)
        * [.tryDecode()](#MethodData+tryDecode) ⇒ <code>Uint8Array</code>
        * [.tryPublicKeyJwk()](#MethodData+tryPublicKeyJwk) ⇒ [<code>Jwk</code>](#Jwk)
        * [.toJSON()](#MethodData+toJSON) ⇒ <code>any</code>
        * [.clone()](#MethodData+clone) ⇒ [<code>MethodData</code>](#MethodData)
    * _static_
        * [.newBase58(data)](#MethodData.newBase58) ⇒ [<code>MethodData</code>](#MethodData)
        * [.newMultibase(data)](#MethodData.newMultibase) ⇒ [<code>MethodData</code>](#MethodData)
        * [.newJwk(key)](#MethodData.newJwk) ⇒ [<code>MethodData</code>](#MethodData)
        * [.newCustom(name, data)](#MethodData.newCustom) ⇒ [<code>MethodData</code>](#MethodData)
        * [.fromJSON(json)](#MethodData.fromJSON) ⇒ [<code>MethodData</code>](#MethodData)

<a name="MethodData+tryCustom"></a>

### methodData.tryCustom() ⇒ [<code>CustomMethodData</code>](#CustomMethodData)
Returns the wrapped custom method data format is `Custom`.

**Kind**: instance method of [<code>MethodData</code>](#MethodData)  
<a name="MethodData+tryDecode"></a>

### methodData.tryDecode() ⇒ <code>Uint8Array</code>
Returns a `Uint8Array` containing the decoded bytes of the [MethodData](#MethodData).

This is generally a public key identified by a [MethodData](#MethodData) value.

### Errors
Decoding can fail if [MethodData](#MethodData) has invalid content or cannot be
represented as a vector of bytes.

**Kind**: instance method of [<code>MethodData</code>](#MethodData)  
<a name="MethodData+tryPublicKeyJwk"></a>

### methodData.tryPublicKeyJwk() ⇒ [<code>Jwk</code>](#Jwk)
Returns the wrapped [Jwk](#Jwk) if the format is `PublicKeyJwk`.

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
Creates a new [MethodData](#MethodData) variant with Base58-BTC encoded content.

**Kind**: static method of [<code>MethodData</code>](#MethodData)  

| Param | Type |
| --- | --- |
| data | <code>Uint8Array</code> | 

<a name="MethodData.newMultibase"></a>

### MethodData.newMultibase(data) ⇒ [<code>MethodData</code>](#MethodData)
Creates a new [MethodData](#MethodData) variant with Multibase-encoded content.

**Kind**: static method of [<code>MethodData</code>](#MethodData)  

| Param | Type |
| --- | --- |
| data | <code>Uint8Array</code> | 

<a name="MethodData.newJwk"></a>

### MethodData.newJwk(key) ⇒ [<code>MethodData</code>](#MethodData)
Creates a new [MethodData](#MethodData) variant consisting of the given `key`.

### Errors
An error is thrown if the given `key` contains any private components.

**Kind**: static method of [<code>MethodData</code>](#MethodData)  

| Param | Type |
| --- | --- |
| key | [<code>Jwk</code>](#Jwk) | 

<a name="MethodData.newCustom"></a>

### MethodData.newCustom(name, data) ⇒ [<code>MethodData</code>](#MethodData)
Creates a new custom [MethodData](#MethodData).

**Kind**: static method of [<code>MethodData</code>](#MethodData)  

| Param | Type |
| --- | --- |
| name | <code>string</code> | 
| data | <code>any</code> | 

<a name="MethodData.fromJSON"></a>

### MethodData.fromJSON(json) ⇒ [<code>MethodData</code>](#MethodData)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>MethodData</code>](#MethodData)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="MethodDigest"></a>

## MethodDigest
Unique identifier of a [VerificationMethod](#VerificationMethod).

NOTE:
This class does not have a JSON representation,
use the methods `pack` and `unpack` instead.

**Kind**: global class  

* [MethodDigest](#MethodDigest)
    * [new MethodDigest(verification_method)](#new_MethodDigest_new)
    * _instance_
        * [.pack()](#MethodDigest+pack) ⇒ <code>Uint8Array</code>
        * [.clone()](#MethodDigest+clone) ⇒ [<code>MethodDigest</code>](#MethodDigest)
    * _static_
        * [.unpack(bytes)](#MethodDigest.unpack) ⇒ [<code>MethodDigest</code>](#MethodDigest)

<a name="new_MethodDigest_new"></a>

### new MethodDigest(verification_method)

| Param | Type |
| --- | --- |
| verification_method | [<code>VerificationMethod</code>](#VerificationMethod) | 

<a name="MethodDigest+pack"></a>

### methodDigest.pack() ⇒ <code>Uint8Array</code>
Packs [MethodDigest](#MethodDigest) into bytes.

**Kind**: instance method of [<code>MethodDigest</code>](#MethodDigest)  
<a name="MethodDigest+clone"></a>

### methodDigest.clone() ⇒ [<code>MethodDigest</code>](#MethodDigest)
Deep clones the object.

**Kind**: instance method of [<code>MethodDigest</code>](#MethodDigest)  
<a name="MethodDigest.unpack"></a>

### MethodDigest.unpack(bytes) ⇒ [<code>MethodDigest</code>](#MethodDigest)
Unpacks bytes into [MethodDigest](#MethodDigest).

**Kind**: static method of [<code>MethodDigest</code>](#MethodDigest)  

| Param | Type |
| --- | --- |
| bytes | <code>Uint8Array</code> | 

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
Returns the [MethodScope](#MethodScope) as a string.

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
        * [.JsonWebKey()](#MethodType.JsonWebKey) ⇒ [<code>MethodType</code>](#MethodType)
        * [.custom(type_)](#MethodType.custom) ⇒ [<code>MethodType</code>](#MethodType)
        * [.fromJSON(json)](#MethodType.fromJSON) ⇒ [<code>MethodType</code>](#MethodType)

<a name="MethodType+toString"></a>

### methodType.toString() ⇒ <code>string</code>
Returns the [MethodType](#MethodType) as a string.

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
<a name="MethodType.JsonWebKey"></a>

### MethodType.JsonWebKey() ⇒ [<code>MethodType</code>](#MethodType)
A verification method for use with JWT verification as prescribed by the [Jwk](#Jwk)
in the `publicKeyJwk` entry.

**Kind**: static method of [<code>MethodType</code>](#MethodType)  
<a name="MethodType.custom"></a>

### MethodType.custom(type_) ⇒ [<code>MethodType</code>](#MethodType)
A custom method.

**Kind**: static method of [<code>MethodType</code>](#MethodType)  

| Param | Type |
| --- | --- |
| type_ | <code>string</code> | 

<a name="MethodType.fromJSON"></a>

### MethodType.fromJSON(json) ⇒ [<code>MethodType</code>](#MethodType)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>MethodType</code>](#MethodType)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="PayloadEntry"></a>

## PayloadEntry
**Kind**: global class  

* [PayloadEntry](#PayloadEntry)
    * [.1](#PayloadEntry+1) ⇒ [<code>PayloadType</code>](#PayloadType)
    * [.1](#PayloadEntry+1)
    * [.value](#PayloadEntry+value)
    * [.value](#PayloadEntry+value) ⇒ <code>any</code>

<a name="PayloadEntry+1"></a>

### payloadEntry.1 ⇒ [<code>PayloadType</code>](#PayloadType)
**Kind**: instance property of [<code>PayloadEntry</code>](#PayloadEntry)  
<a name="PayloadEntry+1"></a>

### payloadEntry.1
**Kind**: instance property of [<code>PayloadEntry</code>](#PayloadEntry)  

| Param | Type |
| --- | --- |
| arg0 | [<code>PayloadType</code>](#PayloadType) | 

<a name="PayloadEntry+value"></a>

### payloadEntry.value
**Kind**: instance property of [<code>PayloadEntry</code>](#PayloadEntry)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="PayloadEntry+value"></a>

### payloadEntry.value ⇒ <code>any</code>
**Kind**: instance property of [<code>PayloadEntry</code>](#PayloadEntry)  
<a name="Payloads"></a>

## Payloads
**Kind**: global class  

* [Payloads](#Payloads)
    * [new Payloads(entries)](#new_Payloads_new)
    * _instance_
        * [.toJSON()](#Payloads+toJSON) ⇒ <code>any</code>
        * [.clone()](#Payloads+clone) ⇒ [<code>Payloads</code>](#Payloads)
        * [.getValues()](#Payloads+getValues) ⇒ <code>Array.&lt;any&gt;</code>
        * [.getUndisclosedIndexes()](#Payloads+getUndisclosedIndexes) ⇒ <code>Uint32Array</code>
        * [.getDisclosedIndexes()](#Payloads+getDisclosedIndexes) ⇒ <code>Uint32Array</code>
        * [.getUndisclosedPayloads()](#Payloads+getUndisclosedPayloads) ⇒ <code>Array.&lt;any&gt;</code>
        * [.getDisclosedPayloads()](#Payloads+getDisclosedPayloads) ⇒ [<code>Payloads</code>](#Payloads)
        * [.setUndisclosed(index)](#Payloads+setUndisclosed)
        * [.replacePayloadAtIndex(index, value)](#Payloads+replacePayloadAtIndex) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#Payloads.fromJSON) ⇒ [<code>Payloads</code>](#Payloads)
        * [.newFromValues(values)](#Payloads.newFromValues) ⇒ [<code>Payloads</code>](#Payloads)

<a name="new_Payloads_new"></a>

### new Payloads(entries)

| Param | Type |
| --- | --- |
| entries | [<code>Array.&lt;PayloadEntry&gt;</code>](#PayloadEntry) | 

<a name="Payloads+toJSON"></a>

### payloads.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>Payloads</code>](#Payloads)  
<a name="Payloads+clone"></a>

### payloads.clone() ⇒ [<code>Payloads</code>](#Payloads)
Deep clones the object.

**Kind**: instance method of [<code>Payloads</code>](#Payloads)  
<a name="Payloads+getValues"></a>

### payloads.getValues() ⇒ <code>Array.&lt;any&gt;</code>
**Kind**: instance method of [<code>Payloads</code>](#Payloads)  
<a name="Payloads+getUndisclosedIndexes"></a>

### payloads.getUndisclosedIndexes() ⇒ <code>Uint32Array</code>
**Kind**: instance method of [<code>Payloads</code>](#Payloads)  
<a name="Payloads+getDisclosedIndexes"></a>

### payloads.getDisclosedIndexes() ⇒ <code>Uint32Array</code>
**Kind**: instance method of [<code>Payloads</code>](#Payloads)  
<a name="Payloads+getUndisclosedPayloads"></a>

### payloads.getUndisclosedPayloads() ⇒ <code>Array.&lt;any&gt;</code>
**Kind**: instance method of [<code>Payloads</code>](#Payloads)  
<a name="Payloads+getDisclosedPayloads"></a>

### payloads.getDisclosedPayloads() ⇒ [<code>Payloads</code>](#Payloads)
**Kind**: instance method of [<code>Payloads</code>](#Payloads)  
<a name="Payloads+setUndisclosed"></a>

### payloads.setUndisclosed(index)
**Kind**: instance method of [<code>Payloads</code>](#Payloads)  

| Param | Type |
| --- | --- |
| index | <code>number</code> | 

<a name="Payloads+replacePayloadAtIndex"></a>

### payloads.replacePayloadAtIndex(index, value) ⇒ <code>any</code>
**Kind**: instance method of [<code>Payloads</code>](#Payloads)  

| Param | Type |
| --- | --- |
| index | <code>number</code> | 
| value | <code>any</code> | 

<a name="Payloads.fromJSON"></a>

### Payloads.fromJSON(json) ⇒ [<code>Payloads</code>](#Payloads)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>Payloads</code>](#Payloads)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="Payloads.newFromValues"></a>

### Payloads.newFromValues(values) ⇒ [<code>Payloads</code>](#Payloads)
**Kind**: static method of [<code>Payloads</code>](#Payloads)  

| Param | Type |
| --- | --- |
| values | <code>Array.&lt;any&gt;</code> | 

<a name="Presentation"></a>

## Presentation
**Kind**: global class  

* [Presentation](#Presentation)
    * [new Presentation(values)](#new_Presentation_new)
    * _instance_
        * [.context()](#Presentation+context) ⇒ <code>Array.&lt;(string\|Record.&lt;string, any&gt;)&gt;</code>
        * [.id()](#Presentation+id) ⇒ <code>string</code> \| <code>undefined</code>
        * [.type()](#Presentation+type) ⇒ <code>Array.&lt;string&gt;</code>
        * [.verifiableCredential()](#Presentation+verifiableCredential) ⇒ [<code>Array.&lt;UnknownCredential&gt;</code>](#UnknownCredential)
        * [.holder()](#Presentation+holder) ⇒ <code>string</code>
        * [.refreshService()](#Presentation+refreshService) ⇒ <code>Array.&lt;RefreshService&gt;</code>
        * [.termsOfUse()](#Presentation+termsOfUse) ⇒ <code>Array.&lt;Policy&gt;</code>
        * [.proof()](#Presentation+proof) ⇒ [<code>Proof</code>](#Proof) \| <code>undefined</code>
        * [.setProof([proof])](#Presentation+setProof)
        * [.properties()](#Presentation+properties) ⇒ <code>Map.&lt;string, any&gt;</code>
        * [.toJSON()](#Presentation+toJSON) ⇒ <code>any</code>
        * [.clone()](#Presentation+clone) ⇒ [<code>Presentation</code>](#Presentation)
    * _static_
        * [.BaseContext()](#Presentation.BaseContext) ⇒ <code>string</code>
        * [.BaseType()](#Presentation.BaseType) ⇒ <code>string</code>
        * [.fromJSON(json)](#Presentation.fromJSON) ⇒ [<code>Presentation</code>](#Presentation)

<a name="new_Presentation_new"></a>

### new Presentation(values)
Constructs a new presentation.


| Param | Type |
| --- | --- |
| values | <code>IPresentation</code> | 

<a name="Presentation+context"></a>

### presentation.context() ⇒ <code>Array.&lt;(string\|Record.&lt;string, any&gt;)&gt;</code>
Returns a copy of the JSON-LD context(s) applicable to the presentation.

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation+id"></a>

### presentation.id() ⇒ <code>string</code> \| <code>undefined</code>
Returns a copy of the unique `URI` identifying the presentation.

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation+type"></a>

### presentation.type() ⇒ <code>Array.&lt;string&gt;</code>
Returns a copy of the URIs defining the type of the presentation.

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation+verifiableCredential"></a>

### presentation.verifiableCredential() ⇒ [<code>Array.&lt;UnknownCredential&gt;</code>](#UnknownCredential)
Returns the JWT credentials expressing the claims of the presentation.

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation+holder"></a>

### presentation.holder() ⇒ <code>string</code>
Returns a copy of the URI of the entity that generated the presentation.

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation+refreshService"></a>

### presentation.refreshService() ⇒ <code>Array.&lt;RefreshService&gt;</code>
Returns a copy of the service(s) used to refresh an expired [Credential](#Credential) in the presentation.

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation+termsOfUse"></a>

### presentation.termsOfUse() ⇒ <code>Array.&lt;Policy&gt;</code>
Returns a copy of the terms-of-use specified by the presentation holder

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation+proof"></a>

### presentation.proof() ⇒ [<code>Proof</code>](#Proof) \| <code>undefined</code>
Optional cryptographic proof, unrelated to JWT.

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation+setProof"></a>

### presentation.setProof([proof])
Sets the proof property of the [Presentation](#Presentation).

Note that this proof is not related to JWT.

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  

| Param | Type |
| --- | --- |
| [proof] | [<code>Proof</code>](#Proof) \| <code>undefined</code> | 

<a name="Presentation+properties"></a>

### presentation.properties() ⇒ <code>Map.&lt;string, any&gt;</code>
Returns a copy of the miscellaneous properties on the presentation.

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

<a name="PresentationProtectedHeader"></a>

## PresentationProtectedHeader
**Kind**: global class  

* [PresentationProtectedHeader](#PresentationProtectedHeader)
    * [.alg](#PresentationProtectedHeader+alg) ⇒ [<code>PresentationProofAlgorithm</code>](#PresentationProofAlgorithm)
    * [.alg](#PresentationProtectedHeader+alg)
    * [.kid](#PresentationProtectedHeader+kid) ⇒ <code>string</code> \| <code>undefined</code>
    * [.kid](#PresentationProtectedHeader+kid)
    * [.aud](#PresentationProtectedHeader+aud) ⇒ <code>string</code> \| <code>undefined</code>
    * [.aud](#PresentationProtectedHeader+aud)
    * [.nonce](#PresentationProtectedHeader+nonce) ⇒ <code>string</code> \| <code>undefined</code>
    * [.nonce](#PresentationProtectedHeader+nonce)

<a name="PresentationProtectedHeader+alg"></a>

### presentationProtectedHeader.alg ⇒ [<code>PresentationProofAlgorithm</code>](#PresentationProofAlgorithm)
**Kind**: instance property of [<code>PresentationProtectedHeader</code>](#PresentationProtectedHeader)  
<a name="PresentationProtectedHeader+alg"></a>

### presentationProtectedHeader.alg
**Kind**: instance property of [<code>PresentationProtectedHeader</code>](#PresentationProtectedHeader)  

| Param | Type |
| --- | --- |
| arg0 | [<code>PresentationProofAlgorithm</code>](#PresentationProofAlgorithm) | 

<a name="PresentationProtectedHeader+kid"></a>

### presentationProtectedHeader.kid ⇒ <code>string</code> \| <code>undefined</code>
ID for the key used for the JWP.

**Kind**: instance property of [<code>PresentationProtectedHeader</code>](#PresentationProtectedHeader)  
<a name="PresentationProtectedHeader+kid"></a>

### presentationProtectedHeader.kid
ID for the key used for the JWP.

**Kind**: instance property of [<code>PresentationProtectedHeader</code>](#PresentationProtectedHeader)  

| Param | Type |
| --- | --- |
| [arg0] | <code>string</code> \| <code>undefined</code> | 

<a name="PresentationProtectedHeader+aud"></a>

### presentationProtectedHeader.aud ⇒ <code>string</code> \| <code>undefined</code>
Who have received the JPT.

**Kind**: instance property of [<code>PresentationProtectedHeader</code>](#PresentationProtectedHeader)  
<a name="PresentationProtectedHeader+aud"></a>

### presentationProtectedHeader.aud
Who have received the JPT.

**Kind**: instance property of [<code>PresentationProtectedHeader</code>](#PresentationProtectedHeader)  

| Param | Type |
| --- | --- |
| [arg0] | <code>string</code> \| <code>undefined</code> | 

<a name="PresentationProtectedHeader+nonce"></a>

### presentationProtectedHeader.nonce ⇒ <code>string</code> \| <code>undefined</code>
For replay attacks.

**Kind**: instance property of [<code>PresentationProtectedHeader</code>](#PresentationProtectedHeader)  
<a name="PresentationProtectedHeader+nonce"></a>

### presentationProtectedHeader.nonce
For replay attacks.

**Kind**: instance property of [<code>PresentationProtectedHeader</code>](#PresentationProtectedHeader)  

| Param | Type |
| --- | --- |
| [arg0] | <code>string</code> \| <code>undefined</code> | 

<a name="Proof"></a>

## Proof
Represents a cryptographic proof that can be used to validate verifiable credentials and
presentations.

This representation does not inherently implement any standard; instead, it
can be utilized to implement standards or user-defined proofs. The presence of the
`type` field is necessary to accommodate different types of cryptographic proofs.

Note that this proof is not related to JWT and can be used in combination or as an alternative
to it.

**Kind**: global class  

* [Proof](#Proof)
    * [new Proof(type_, properties)](#new_Proof_new)
    * _instance_
        * [.type()](#Proof+type) ⇒ <code>string</code>
        * [.properties()](#Proof+properties) ⇒ <code>any</code>
        * [.toJSON()](#Proof+toJSON) ⇒ <code>any</code>
        * [.clone()](#Proof+clone) ⇒ [<code>Proof</code>](#Proof)
    * _static_
        * [.fromJSON(json)](#Proof.fromJSON) ⇒ [<code>Proof</code>](#Proof)

<a name="new_Proof_new"></a>

### new Proof(type_, properties)

| Param | Type |
| --- | --- |
| type_ | <code>string</code> | 
| properties | <code>any</code> | 

<a name="Proof+type"></a>

### proof.type() ⇒ <code>string</code>
Returns the type of proof.

**Kind**: instance method of [<code>Proof</code>](#Proof)  
<a name="Proof+properties"></a>

### proof.properties() ⇒ <code>any</code>
Returns the properties of the proof.

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

<a name="ProofUpdateCtx"></a>

## ProofUpdateCtx
**Kind**: global class  

* [ProofUpdateCtx](#ProofUpdateCtx)
    * [.old_start_validity_timeframe](#ProofUpdateCtx+old_start_validity_timeframe) ⇒ <code>Uint8Array</code>
    * [.old_start_validity_timeframe](#ProofUpdateCtx+old_start_validity_timeframe)
    * [.new_start_validity_timeframe](#ProofUpdateCtx+new_start_validity_timeframe) ⇒ <code>Uint8Array</code>
    * [.new_start_validity_timeframe](#ProofUpdateCtx+new_start_validity_timeframe)
    * [.old_end_validity_timeframe](#ProofUpdateCtx+old_end_validity_timeframe) ⇒ <code>Uint8Array</code>
    * [.old_end_validity_timeframe](#ProofUpdateCtx+old_end_validity_timeframe)
    * [.new_end_validity_timeframe](#ProofUpdateCtx+new_end_validity_timeframe) ⇒ <code>Uint8Array</code>
    * [.new_end_validity_timeframe](#ProofUpdateCtx+new_end_validity_timeframe)
    * [.index_start_validity_timeframe](#ProofUpdateCtx+index_start_validity_timeframe) ⇒ <code>number</code>
    * [.index_start_validity_timeframe](#ProofUpdateCtx+index_start_validity_timeframe)
    * [.index_end_validity_timeframe](#ProofUpdateCtx+index_end_validity_timeframe) ⇒ <code>number</code>
    * [.index_end_validity_timeframe](#ProofUpdateCtx+index_end_validity_timeframe)
    * [.number_of_signed_messages](#ProofUpdateCtx+number_of_signed_messages) ⇒ <code>number</code>
    * [.number_of_signed_messages](#ProofUpdateCtx+number_of_signed_messages)

<a name="ProofUpdateCtx+old_start_validity_timeframe"></a>

### proofUpdateCtx.old\_start\_validity\_timeframe ⇒ <code>Uint8Array</code>
Old `startValidityTimeframe` value

**Kind**: instance property of [<code>ProofUpdateCtx</code>](#ProofUpdateCtx)  
<a name="ProofUpdateCtx+old_start_validity_timeframe"></a>

### proofUpdateCtx.old\_start\_validity\_timeframe
Old `startValidityTimeframe` value

**Kind**: instance property of [<code>ProofUpdateCtx</code>](#ProofUpdateCtx)  

| Param | Type |
| --- | --- |
| arg0 | <code>Uint8Array</code> | 

<a name="ProofUpdateCtx+new_start_validity_timeframe"></a>

### proofUpdateCtx.new\_start\_validity\_timeframe ⇒ <code>Uint8Array</code>
New `startValidityTimeframe` value to be signed

**Kind**: instance property of [<code>ProofUpdateCtx</code>](#ProofUpdateCtx)  
<a name="ProofUpdateCtx+new_start_validity_timeframe"></a>

### proofUpdateCtx.new\_start\_validity\_timeframe
New `startValidityTimeframe` value to be signed

**Kind**: instance property of [<code>ProofUpdateCtx</code>](#ProofUpdateCtx)  

| Param | Type |
| --- | --- |
| arg0 | <code>Uint8Array</code> | 

<a name="ProofUpdateCtx+old_end_validity_timeframe"></a>

### proofUpdateCtx.old\_end\_validity\_timeframe ⇒ <code>Uint8Array</code>
Old `endValidityTimeframe` value

**Kind**: instance property of [<code>ProofUpdateCtx</code>](#ProofUpdateCtx)  
<a name="ProofUpdateCtx+old_end_validity_timeframe"></a>

### proofUpdateCtx.old\_end\_validity\_timeframe
Old `endValidityTimeframe` value

**Kind**: instance property of [<code>ProofUpdateCtx</code>](#ProofUpdateCtx)  

| Param | Type |
| --- | --- |
| arg0 | <code>Uint8Array</code> | 

<a name="ProofUpdateCtx+new_end_validity_timeframe"></a>

### proofUpdateCtx.new\_end\_validity\_timeframe ⇒ <code>Uint8Array</code>
New `endValidityTimeframe` value to be signed

**Kind**: instance property of [<code>ProofUpdateCtx</code>](#ProofUpdateCtx)  
<a name="ProofUpdateCtx+new_end_validity_timeframe"></a>

### proofUpdateCtx.new\_end\_validity\_timeframe
New `endValidityTimeframe` value to be signed

**Kind**: instance property of [<code>ProofUpdateCtx</code>](#ProofUpdateCtx)  

| Param | Type |
| --- | --- |
| arg0 | <code>Uint8Array</code> | 

<a name="ProofUpdateCtx+index_start_validity_timeframe"></a>

### proofUpdateCtx.index\_start\_validity\_timeframe ⇒ <code>number</code>
Index of `startValidityTimeframe` claim inside the array of Claims

**Kind**: instance property of [<code>ProofUpdateCtx</code>](#ProofUpdateCtx)  
<a name="ProofUpdateCtx+index_start_validity_timeframe"></a>

### proofUpdateCtx.index\_start\_validity\_timeframe
Index of `startValidityTimeframe` claim inside the array of Claims

**Kind**: instance property of [<code>ProofUpdateCtx</code>](#ProofUpdateCtx)  

| Param | Type |
| --- | --- |
| arg0 | <code>number</code> | 

<a name="ProofUpdateCtx+index_end_validity_timeframe"></a>

### proofUpdateCtx.index\_end\_validity\_timeframe ⇒ <code>number</code>
Index of `endValidityTimeframe` claim inside the array of Claims

**Kind**: instance property of [<code>ProofUpdateCtx</code>](#ProofUpdateCtx)  
<a name="ProofUpdateCtx+index_end_validity_timeframe"></a>

### proofUpdateCtx.index\_end\_validity\_timeframe
Index of `endValidityTimeframe` claim inside the array of Claims

**Kind**: instance property of [<code>ProofUpdateCtx</code>](#ProofUpdateCtx)  

| Param | Type |
| --- | --- |
| arg0 | <code>number</code> | 

<a name="ProofUpdateCtx+number_of_signed_messages"></a>

### proofUpdateCtx.number\_of\_signed\_messages ⇒ <code>number</code>
Number of signed messages, number of payloads in a JWP

**Kind**: instance property of [<code>ProofUpdateCtx</code>](#ProofUpdateCtx)  
<a name="ProofUpdateCtx+number_of_signed_messages"></a>

### proofUpdateCtx.number\_of\_signed\_messages
Number of signed messages, number of payloads in a JWP

**Kind**: instance property of [<code>ProofUpdateCtx</code>](#ProofUpdateCtx)  

| Param | Type |
| --- | --- |
| arg0 | <code>number</code> | 

<a name="Resolver"></a>

## Resolver
Convenience type for resolving DID documents from different DID methods.

Also provides methods for resolving DID Documents associated with
verifiable [Credential](#Credential)s and [Presentation](#Presentation)s.

# Configuration

The resolver will only be able to resolve DID documents for methods it has been configured for in the constructor.

**Kind**: global class  

* [Resolver](#Resolver)
    * [new Resolver(config)](#new_Resolver_new)
    * [.resolve(did)](#Resolver+resolve) ⇒ <code>Promise.&lt;(CoreDocument\|IToCoreDocument)&gt;</code>
    * [.resolveMultiple(dids)](#Resolver+resolveMultiple) ⇒ <code>Promise.&lt;Array.&lt;(CoreDocument\|IToCoreDocument)&gt;&gt;</code>

<a name="new_Resolver_new"></a>

### new Resolver(config)
Constructs a new [Resolver](#Resolver).

# Errors
If both a `client` is given and the `handlers` map contains the "iota" key the construction process
will throw an error because the handler for the "iota" method then becomes ambiguous.


| Param | Type |
| --- | --- |
| config | <code>ResolverConfig</code> | 

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

<a name="Resolver+resolveMultiple"></a>

### resolver.resolveMultiple(dids) ⇒ <code>Promise.&lt;Array.&lt;(CoreDocument\|IToCoreDocument)&gt;&gt;</code>
Concurrently fetches the DID Documents of the multiple given DIDs.

# Errors
* If the resolver has not been configured to handle the method of any of the given DIDs.
* If the resolution process of any DID fails.

## Note
* The order of the documents in the returned array matches that in `dids`.
* If `dids` contains duplicates, these will be resolved only once and the resolved document
is copied into the returned array to match the order of `dids`.

**Kind**: instance method of [<code>Resolver</code>](#Resolver)  

| Param | Type |
| --- | --- |
| dids | <code>Array.&lt;string&gt;</code> | 

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
        * [.toService(serviceId)](#RevocationBitmap+toService) ⇒ [<code>Service</code>](#Service)
    * _static_
        * [.type()](#RevocationBitmap.type) ⇒ <code>string</code>
        * [.fromEndpoint(service)](#RevocationBitmap.fromEndpoint) ⇒ [<code>RevocationBitmap</code>](#RevocationBitmap)

<a name="new_RevocationBitmap_new"></a>

### new RevocationBitmap()
Creates a new [RevocationBitmap](#RevocationBitmap) instance.

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
<a name="RevocationBitmap+toService"></a>

### revocationBitmap.toService(serviceId) ⇒ [<code>Service</code>](#Service)
Return a `Service` with:
- the service's id set to `serviceId`,
- of type `RevocationBitmap2022`,
- and with the bitmap embedded in a data url in the service's endpoint.

**Kind**: instance method of [<code>RevocationBitmap</code>](#RevocationBitmap)  

| Param | Type |
| --- | --- |
| serviceId | [<code>DIDUrl</code>](#DIDUrl) | 

<a name="RevocationBitmap.type"></a>

### RevocationBitmap.type() ⇒ <code>string</code>
The name of the service type.

**Kind**: static method of [<code>RevocationBitmap</code>](#RevocationBitmap)  
<a name="RevocationBitmap.fromEndpoint"></a>

### RevocationBitmap.fromEndpoint(service) ⇒ [<code>RevocationBitmap</code>](#RevocationBitmap)
Try to construct a [RevocationBitmap](#RevocationBitmap) from a service
if it is a valid Revocation Bitmap Service.

**Kind**: static method of [<code>RevocationBitmap</code>](#RevocationBitmap)  

| Param | Type |
| --- | --- |
| service | [<code>Service</code>](#Service) | 

<a name="RevocationTimeframeStatus"></a>

## RevocationTimeframeStatus
Information used to determine the current status of a [Credential](#Credential).

**Kind**: global class  

* [RevocationTimeframeStatus](#RevocationTimeframeStatus)
    * [new RevocationTimeframeStatus(id, index, duration, [start_validity])](#new_RevocationTimeframeStatus_new)
    * _instance_
        * [.clone()](#RevocationTimeframeStatus+clone) ⇒ [<code>RevocationTimeframeStatus</code>](#RevocationTimeframeStatus)
        * [.toJSON()](#RevocationTimeframeStatus+toJSON) ⇒ <code>any</code>
        * [.startValidityTimeframe()](#RevocationTimeframeStatus+startValidityTimeframe) ⇒ [<code>Timestamp</code>](#Timestamp)
        * [.endValidityTimeframe()](#RevocationTimeframeStatus+endValidityTimeframe) ⇒ [<code>Timestamp</code>](#Timestamp)
        * [.id()](#RevocationTimeframeStatus+id) ⇒ <code>string</code>
        * [.index()](#RevocationTimeframeStatus+index) ⇒ <code>number</code> \| <code>undefined</code>
    * _static_
        * [.fromJSON(json)](#RevocationTimeframeStatus.fromJSON) ⇒ [<code>RevocationTimeframeStatus</code>](#RevocationTimeframeStatus)

<a name="new_RevocationTimeframeStatus_new"></a>

### new RevocationTimeframeStatus(id, index, duration, [start_validity])
Creates a new `RevocationTimeframeStatus`.


| Param | Type |
| --- | --- |
| id | <code>string</code> | 
| index | <code>number</code> | 
| duration | [<code>Duration</code>](#Duration) | 
| [start_validity] | [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code> | 

<a name="RevocationTimeframeStatus+clone"></a>

### revocationTimeframeStatus.clone() ⇒ [<code>RevocationTimeframeStatus</code>](#RevocationTimeframeStatus)
Deep clones the object.

**Kind**: instance method of [<code>RevocationTimeframeStatus</code>](#RevocationTimeframeStatus)  
<a name="RevocationTimeframeStatus+toJSON"></a>

### revocationTimeframeStatus.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>RevocationTimeframeStatus</code>](#RevocationTimeframeStatus)  
<a name="RevocationTimeframeStatus+startValidityTimeframe"></a>

### revocationTimeframeStatus.startValidityTimeframe() ⇒ [<code>Timestamp</code>](#Timestamp)
Get startValidityTimeframe value.

**Kind**: instance method of [<code>RevocationTimeframeStatus</code>](#RevocationTimeframeStatus)  
<a name="RevocationTimeframeStatus+endValidityTimeframe"></a>

### revocationTimeframeStatus.endValidityTimeframe() ⇒ [<code>Timestamp</code>](#Timestamp)
Get endValidityTimeframe value.

**Kind**: instance method of [<code>RevocationTimeframeStatus</code>](#RevocationTimeframeStatus)  
<a name="RevocationTimeframeStatus+id"></a>

### revocationTimeframeStatus.id() ⇒ <code>string</code>
Return the URL fo the `RevocationBitmapStatus`.

**Kind**: instance method of [<code>RevocationTimeframeStatus</code>](#RevocationTimeframeStatus)  
<a name="RevocationTimeframeStatus+index"></a>

### revocationTimeframeStatus.index() ⇒ <code>number</code> \| <code>undefined</code>
Return the index of the credential in the issuer's revocation bitmap

**Kind**: instance method of [<code>RevocationTimeframeStatus</code>](#RevocationTimeframeStatus)  
<a name="RevocationTimeframeStatus.fromJSON"></a>

### RevocationTimeframeStatus.fromJSON(json) ⇒ [<code>RevocationTimeframeStatus</code>](#RevocationTimeframeStatus)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>RevocationTimeframeStatus</code>](#RevocationTimeframeStatus)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="SdJwt"></a>

## SdJwt
Representation of an SD-JWT of the format
`<Issuer-signed JWT>~<Disclosure 1>~<Disclosure 2>~...~<Disclosure N>~<optional KB-JWT>`.

**Kind**: global class  

* [SdJwt](#SdJwt)
    * [new SdJwt(jwt, disclosures, [key_binding_jwt])](#new_SdJwt_new)
    * _instance_
        * [.presentation()](#SdJwt+presentation) ⇒ <code>string</code>
        * [.toString()](#SdJwt+toString) ⇒ <code>string</code>
        * [.jwt()](#SdJwt+jwt) ⇒ <code>string</code>
        * [.disclosures()](#SdJwt+disclosures) ⇒ <code>Array.&lt;string&gt;</code>
        * [.keyBindingJwt()](#SdJwt+keyBindingJwt) ⇒ <code>string</code> \| <code>undefined</code>
        * [.clone()](#SdJwt+clone) ⇒ [<code>SdJwt</code>](#SdJwt)
    * _static_
        * [.parse(sd_jwt)](#SdJwt.parse) ⇒ [<code>SdJwt</code>](#SdJwt)

<a name="new_SdJwt_new"></a>

### new SdJwt(jwt, disclosures, [key_binding_jwt])
Creates a new `SdJwt` from its components.


| Param | Type |
| --- | --- |
| jwt | <code>string</code> | 
| disclosures | <code>Array.&lt;string&gt;</code> | 
| [key_binding_jwt] | <code>string</code> \| <code>undefined</code> | 

<a name="SdJwt+presentation"></a>

### sdJwt.presentation() ⇒ <code>string</code>
Serializes the components into the final SD-JWT.

**Kind**: instance method of [<code>SdJwt</code>](#SdJwt)  
<a name="SdJwt+toString"></a>

### sdJwt.toString() ⇒ <code>string</code>
Serializes the components into the final SD-JWT.

**Kind**: instance method of [<code>SdJwt</code>](#SdJwt)  
<a name="SdJwt+jwt"></a>

### sdJwt.jwt() ⇒ <code>string</code>
The JWT part.

**Kind**: instance method of [<code>SdJwt</code>](#SdJwt)  
<a name="SdJwt+disclosures"></a>

### sdJwt.disclosures() ⇒ <code>Array.&lt;string&gt;</code>
The disclosures part.

**Kind**: instance method of [<code>SdJwt</code>](#SdJwt)  
<a name="SdJwt+keyBindingJwt"></a>

### sdJwt.keyBindingJwt() ⇒ <code>string</code> \| <code>undefined</code>
The optional key binding JWT.

**Kind**: instance method of [<code>SdJwt</code>](#SdJwt)  
<a name="SdJwt+clone"></a>

### sdJwt.clone() ⇒ [<code>SdJwt</code>](#SdJwt)
Deep clones the object.

**Kind**: instance method of [<code>SdJwt</code>](#SdJwt)  
<a name="SdJwt.parse"></a>

### SdJwt.parse(sd_jwt) ⇒ [<code>SdJwt</code>](#SdJwt)
Parses an SD-JWT into its components as [`SdJwt`].

## Error
Returns `DeserializationError` if parsing fails.

**Kind**: static method of [<code>SdJwt</code>](#SdJwt)  

| Param | Type |
| --- | --- |
| sd_jwt | <code>string</code> | 

<a name="SdJwtCredentialValidator"></a>

## SdJwtCredentialValidator
A type for decoding and validating [Credential](#Credential).

**Kind**: global class  

* [SdJwtCredentialValidator](#SdJwtCredentialValidator)
    * [new SdJwtCredentialValidator(signatureVerifier)](#new_SdJwtCredentialValidator_new)
    * [.validateCredential(sd_jwt, issuer, options, fail_fast)](#SdJwtCredentialValidator+validateCredential) ⇒ [<code>DecodedJwtCredential</code>](#DecodedJwtCredential)
    * [.verifySignature(credential, trustedIssuers, options)](#SdJwtCredentialValidator+verifySignature) ⇒ [<code>DecodedJwtCredential</code>](#DecodedJwtCredential)
    * [.validateKeyBindingJwt(sdJwt, holder, options)](#SdJwtCredentialValidator+validateKeyBindingJwt) ⇒ [<code>KeyBindingJwtClaims</code>](#KeyBindingJwtClaims)

<a name="new_SdJwtCredentialValidator_new"></a>

### new SdJwtCredentialValidator(signatureVerifier)
Creates a new `SdJwtCredentialValidator`. If a `signatureVerifier` is provided it will be used when
verifying decoded JWS signatures, otherwise the default which is only capable of handling the `EdDSA`
algorithm will be used.


| Param | Type |
| --- | --- |
| signatureVerifier | <code>IJwsVerifier</code> | 

<a name="SdJwtCredentialValidator+validateCredential"></a>

### sdJwtCredentialValidator.validateCredential(sd_jwt, issuer, options, fail_fast) ⇒ [<code>DecodedJwtCredential</code>](#DecodedJwtCredential)
Decodes and validates a `Credential` issued as an SD-JWT. A `DecodedJwtCredential` is returned upon success.
The credential is constructed by replacing disclosures following the
[`Selective Disclosure for JWTs (SD-JWT)`](https://www.ietf.org/archive/id/draft-ietf-oauth-selective-disclosure-jwt-07.html) standard.

The following properties are validated according to `options`:
- the issuer's signature on the JWS,
- the expiration date,
- the issuance date,
- the semantic structure.

# Warning
* The key binding JWT is not validated. If needed, it must be validated separately using
`SdJwtValidator::validate_key_binding_jwt`.
* The lack of an error returned from this method is in of itself not enough to conclude that the credential can be
trusted. This section contains more information on additional checks that should be carried out before and after
calling this method.

## The state of the issuer's DID Document
The caller must ensure that `issuer` represents an up-to-date DID Document.

## Properties that are not validated
 There are many properties defined in [The Verifiable Credentials Data Model](https://www.w3.org/TR/vc-data-model/) that are **not** validated, such as:
`proof`, `credentialStatus`, `type`, `credentialSchema`, `refreshService` **and more**.
These should be manually checked after validation, according to your requirements.

# Errors
An error is returned whenever a validated condition is not satisfied.

**Kind**: instance method of [<code>SdJwtCredentialValidator</code>](#SdJwtCredentialValidator)  

| Param | Type |
| --- | --- |
| sd_jwt | [<code>SdJwt</code>](#SdJwt) | 
| issuer | [<code>CoreDocument</code>](#CoreDocument) \| <code>IToCoreDocument</code> | 
| options | [<code>JwtCredentialValidationOptions</code>](#JwtCredentialValidationOptions) | 
| fail_fast | [<code>FailFast</code>](#FailFast) | 

<a name="SdJwtCredentialValidator+verifySignature"></a>

### sdJwtCredentialValidator.verifySignature(credential, trustedIssuers, options) ⇒ [<code>DecodedJwtCredential</code>](#DecodedJwtCredential)
Decode and verify the JWS signature of a `Credential` issued as an SD-JWT using the DID Document of a trusted
issuer and replaces the disclosures.

A `DecodedJwtCredential` is returned upon success.

# Warning
The caller must ensure that the DID Documents of the trusted issuers are up-to-date.

## Proofs
 Only the JWS signature is verified. If the `Credential` contains a `proof` property this will not be verified
by this method.

# Errors
* If the issuer' URL cannot be parsed.
* If Signature verification fails.
* If SD decoding fails.

**Kind**: instance method of [<code>SdJwtCredentialValidator</code>](#SdJwtCredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>SdJwt</code>](#SdJwt) | 
| trustedIssuers | <code>Array.&lt;(CoreDocument\|IToCoreDocument)&gt;</code> | 
| options | [<code>JwsVerificationOptions</code>](#JwsVerificationOptions) | 

<a name="SdJwtCredentialValidator+validateKeyBindingJwt"></a>

### sdJwtCredentialValidator.validateKeyBindingJwt(sdJwt, holder, options) ⇒ [<code>KeyBindingJwtClaims</code>](#KeyBindingJwtClaims)
Validates a Key Binding JWT (KB-JWT) according to `https://www.ietf.org/archive/id/draft-ietf-oauth-selective-disclosure-jwt-07.html#name-key-binding-jwt`.
The Validation process includes:
  * Signature validation using public key materials defined in the `holder` document.
  * `typ` value in KB-JWT header.
  * `sd_hash` claim value in the KB-JWT claim.
  * Optional `nonce`, `aud` and issuance date validation.

**Kind**: instance method of [<code>SdJwtCredentialValidator</code>](#SdJwtCredentialValidator)  

| Param | Type |
| --- | --- |
| sdJwt | [<code>SdJwt</code>](#SdJwt) | 
| holder | [<code>CoreDocument</code>](#CoreDocument) \| <code>IToCoreDocument</code> | 
| options | [<code>KeyBindingJWTValidationOptions</code>](#KeyBindingJWTValidationOptions) | 

<a name="SdObjectDecoder"></a>

## SdObjectDecoder
Substitutes digests in an SD-JWT object by their corresponding plaintext values provided by disclosures.

**Kind**: global class  

* [SdObjectDecoder](#SdObjectDecoder)
    * [new SdObjectDecoder()](#new_SdObjectDecoder_new)
    * [.decode(object, disclosures)](#SdObjectDecoder+decode) ⇒ <code>Record.&lt;string, any&gt;</code>

<a name="new_SdObjectDecoder_new"></a>

### new SdObjectDecoder()
Creates a new `SdObjectDecoder` with `sha-256` hasher.

<a name="SdObjectDecoder+decode"></a>

### sdObjectDecoder.decode(object, disclosures) ⇒ <code>Record.&lt;string, any&gt;</code>
Decodes an SD-JWT `object` containing by Substituting the digests with their corresponding
plaintext values provided by `disclosures`.

## Notes
* Claims like `exp` or `iat` are not validated in the process of decoding.
* `_sd_alg` property will be removed if present.

**Kind**: instance method of [<code>SdObjectDecoder</code>](#SdObjectDecoder)  

| Param | Type |
| --- | --- |
| object | <code>Record.&lt;string, any&gt;</code> | 
| disclosures | <code>Array.&lt;string&gt;</code> | 

<a name="SdObjectEncoder"></a>

## SdObjectEncoder
Transforms a JSON object into an SD-JWT object by substituting selected values
with their corresponding disclosure digests.

Note: digests are created using the sha-256 algorithm.

**Kind**: global class  

* [SdObjectEncoder](#SdObjectEncoder)
    * [new SdObjectEncoder(object)](#new_SdObjectEncoder_new)
    * [.conceal(path, [salt])](#SdObjectEncoder+conceal) ⇒ [<code>Disclosure</code>](#Disclosure)
    * [.addSdAlgProperty()](#SdObjectEncoder+addSdAlgProperty)
    * [.encodeToString()](#SdObjectEncoder+encodeToString) ⇒ <code>string</code>
    * [.toString()](#SdObjectEncoder+toString) ⇒ <code>string</code>
    * [.encodeToObject()](#SdObjectEncoder+encodeToObject) ⇒ <code>Record.&lt;string, any&gt;</code>
    * [.toJSON()](#SdObjectEncoder+toJSON) ⇒ <code>any</code>
    * [.addDecoys(path, number_of_decoys)](#SdObjectEncoder+addDecoys)

<a name="new_SdObjectEncoder_new"></a>

### new SdObjectEncoder(object)
Creates a new `SdObjectEncoder` with `sha-256` hash function.


| Param | Type |
| --- | --- |
| object | <code>any</code> | 

<a name="SdObjectEncoder+conceal"></a>

### sdObjectEncoder.conceal(path, [salt]) ⇒ [<code>Disclosure</code>](#Disclosure)
Substitutes a value with the digest of its disclosure.
If no salt is provided, the disclosure will be created with a random salt value.

`path` indicates the pointer to the value that will be concealed using the syntax of
[JSON pointer](https://datatracker.ietf.org/doc/html/rfc6901).

For the following object:

 ```
{
  "id": "did:value",
  "claim1": {
     "abc": true
  },
  "claim2": ["val_1", "val_2"]
}
```

Path "/id" conceals `"id": "did:value"`
Path "/claim1/abc" conceals `"abc": true`
Path "/claim2/0" conceals `val_1`
```

## Errors
* `InvalidPath` if pointer is invalid.
* `DataTypeMismatch` if existing SD format is invalid.

**Kind**: instance method of [<code>SdObjectEncoder</code>](#SdObjectEncoder)  

| Param | Type |
| --- | --- |
| path | <code>string</code> | 
| [salt] | <code>string</code> \| <code>undefined</code> | 

<a name="SdObjectEncoder+addSdAlgProperty"></a>

### sdObjectEncoder.addSdAlgProperty()
Adds the `_sd_alg` property to the top level of the object, with
its value set to "sha-256".

**Kind**: instance method of [<code>SdObjectEncoder</code>](#SdObjectEncoder)  
<a name="SdObjectEncoder+encodeToString"></a>

### sdObjectEncoder.encodeToString() ⇒ <code>string</code>
Returns the modified object as a string.

**Kind**: instance method of [<code>SdObjectEncoder</code>](#SdObjectEncoder)  
<a name="SdObjectEncoder+toString"></a>

### sdObjectEncoder.toString() ⇒ <code>string</code>
Returns the modified object as a string.

**Kind**: instance method of [<code>SdObjectEncoder</code>](#SdObjectEncoder)  
<a name="SdObjectEncoder+encodeToObject"></a>

### sdObjectEncoder.encodeToObject() ⇒ <code>Record.&lt;string, any&gt;</code>
Returns the modified object.

**Kind**: instance method of [<code>SdObjectEncoder</code>](#SdObjectEncoder)  
<a name="SdObjectEncoder+toJSON"></a>

### sdObjectEncoder.toJSON() ⇒ <code>any</code>
Returns the modified object.

**Kind**: instance method of [<code>SdObjectEncoder</code>](#SdObjectEncoder)  
<a name="SdObjectEncoder+addDecoys"></a>

### sdObjectEncoder.addDecoys(path, number_of_decoys)
Adds a decoy digest to the specified path.
If path is an empty slice, decoys will be added to the top level.

**Kind**: instance method of [<code>SdObjectEncoder</code>](#SdObjectEncoder)  

| Param | Type |
| --- | --- |
| path | <code>string</code> | 
| number_of_decoys | <code>number</code> | 

<a name="SelectiveDisclosurePresentation"></a>

## SelectiveDisclosurePresentation
Used to construct a JwpPresentedBuilder and handle the selective disclosure of attributes
- @context MUST NOT be blinded
- id MUST be blinded
- type MUST NOT be blinded
- issuer MUST NOT be blinded
- issuanceDate MUST be blinded (if Timeframe Revocation mechanism is used)
- expirationDate MUST be blinded (if Timeframe Revocation mechanism is used)
- credentialSubject (User have to choose which attribute must be blinded)
- credentialSchema MUST NOT be blinded
- credentialStatus MUST NOT be blinded
- refreshService MUST NOT be blinded (probably will be used for Timeslot Revocation mechanism)
- termsOfUse NO reason to use it in ZK VC (will be in any case blinded)
- evidence (User have to choose which attribute must be blinded)

**Kind**: global class  

* [SelectiveDisclosurePresentation](#SelectiveDisclosurePresentation)
    * [new SelectiveDisclosurePresentation(issued_jwp)](#new_SelectiveDisclosurePresentation_new)
    * [.concealInSubject(path)](#SelectiveDisclosurePresentation+concealInSubject)
    * [.concealInEvidence(path)](#SelectiveDisclosurePresentation+concealInEvidence)
    * [.setPresentationHeader(header)](#SelectiveDisclosurePresentation+setPresentationHeader)

<a name="new_SelectiveDisclosurePresentation_new"></a>

### new SelectiveDisclosurePresentation(issued_jwp)
Initialize a presentation starting from an Issued JWP.
The properties `jti`, `nbf`, `issuanceDate`, `expirationDate` and `termsOfUse` are concealed by default.


| Param | Type |
| --- | --- |
| issued_jwp | [<code>JwpIssued</code>](#JwpIssued) | 

<a name="SelectiveDisclosurePresentation+concealInSubject"></a>

### selectiveDisclosurePresentation.concealInSubject(path)
Selectively disclose "credentialSubject" attributes.
# Example
```
{
    "id": 1234,
    "name": "Alice",
    "mainCourses": ["Object-oriented Programming", "Mathematics"],
    "degree": {
        "type": "BachelorDegree",
        "name": "Bachelor of Science and Arts",
    },
    "GPA": "4.0",
}
```
If you want to undisclose for example the Mathematics course and the name of the degree:
```
undisclose_subject("mainCourses[1]");
undisclose_subject("degree.name");
```

**Kind**: instance method of [<code>SelectiveDisclosurePresentation</code>](#SelectiveDisclosurePresentation)  

| Param | Type |
| --- | --- |
| path | <code>string</code> | 

<a name="SelectiveDisclosurePresentation+concealInEvidence"></a>

### selectiveDisclosurePresentation.concealInEvidence(path)
Undiscloses "evidence" attributes.

**Kind**: instance method of [<code>SelectiveDisclosurePresentation</code>](#SelectiveDisclosurePresentation)  

| Param | Type |
| --- | --- |
| path | <code>string</code> | 

<a name="SelectiveDisclosurePresentation+setPresentationHeader"></a>

### selectiveDisclosurePresentation.setPresentationHeader(header)
Sets presentation protected header.

**Kind**: instance method of [<code>SelectiveDisclosurePresentation</code>](#SelectiveDisclosurePresentation)  

| Param | Type |
| --- | --- |
| header | [<code>PresentationProtectedHeader</code>](#PresentationProtectedHeader) | 

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
Returns a copy of the [Service](#Service) id.

**Kind**: instance method of [<code>Service</code>](#Service)  
<a name="Service+type"></a>

### service.type() ⇒ <code>Array.&lt;string&gt;</code>
Returns a copy of the [Service](#Service) type.

**Kind**: instance method of [<code>Service</code>](#Service)  
<a name="Service+serviceEndpoint"></a>

### service.serviceEndpoint() ⇒ <code>string</code> \| <code>Array.&lt;string&gt;</code> \| <code>Map.&lt;string, Array.&lt;string&gt;&gt;</code>
Returns a copy of the [Service](#Service) endpoint.

**Kind**: instance method of [<code>Service</code>](#Service)  
<a name="Service+properties"></a>

### service.properties() ⇒ <code>Map.&lt;string, any&gt;</code>
Returns a copy of the custom properties on the [Service](#Service).

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

<a name="StatusList2021"></a>

## StatusList2021
StatusList2021 data structure as described in [W3C's VC status list 2021](https://www.w3.org/TR/2023/WD-vc-status-list-20230427/).

**Kind**: global class  

* [StatusList2021](#StatusList2021)
    * [new StatusList2021([size])](#new_StatusList2021_new)
    * _instance_
        * [.clone()](#StatusList2021+clone) ⇒ [<code>StatusList2021</code>](#StatusList2021)
        * [.len()](#StatusList2021+len) ⇒ <code>number</code>
        * [.get(index)](#StatusList2021+get) ⇒ <code>boolean</code>
        * [.set(index, value)](#StatusList2021+set)
        * [.intoEncodedStr()](#StatusList2021+intoEncodedStr) ⇒ <code>string</code>
    * _static_
        * [.fromEncodedStr(s)](#StatusList2021.fromEncodedStr) ⇒ [<code>StatusList2021</code>](#StatusList2021)

<a name="new_StatusList2021_new"></a>

### new StatusList2021([size])
Creates a new [StatusList2021](#StatusList2021) of `size` entries.


| Param | Type |
| --- | --- |
| [size] | <code>number</code> \| <code>undefined</code> | 

<a name="StatusList2021+clone"></a>

### statusList2021.clone() ⇒ [<code>StatusList2021</code>](#StatusList2021)
Deep clones the object.

**Kind**: instance method of [<code>StatusList2021</code>](#StatusList2021)  
<a name="StatusList2021+len"></a>

### statusList2021.len() ⇒ <code>number</code>
Returns the number of entries in this [StatusList2021](#StatusList2021).

**Kind**: instance method of [<code>StatusList2021</code>](#StatusList2021)  
<a name="StatusList2021+get"></a>

### statusList2021.get(index) ⇒ <code>boolean</code>
Returns whether the entry at `index` is set.

**Kind**: instance method of [<code>StatusList2021</code>](#StatusList2021)  

| Param | Type |
| --- | --- |
| index | <code>number</code> | 

<a name="StatusList2021+set"></a>

### statusList2021.set(index, value)
Sets the value of the `index`-th entry.

**Kind**: instance method of [<code>StatusList2021</code>](#StatusList2021)  

| Param | Type |
| --- | --- |
| index | <code>number</code> | 
| value | <code>boolean</code> | 

<a name="StatusList2021+intoEncodedStr"></a>

### statusList2021.intoEncodedStr() ⇒ <code>string</code>
Encodes this [StatusList2021](#StatusList2021) into its compressed
base64 string representation.

**Kind**: instance method of [<code>StatusList2021</code>](#StatusList2021)  
<a name="StatusList2021.fromEncodedStr"></a>

### StatusList2021.fromEncodedStr(s) ⇒ [<code>StatusList2021</code>](#StatusList2021)
Attempts to decode a [StatusList2021](#StatusList2021) from a string.

**Kind**: static method of [<code>StatusList2021</code>](#StatusList2021)  

| Param | Type |
| --- | --- |
| s | <code>string</code> | 

<a name="StatusList2021Credential"></a>

## StatusList2021Credential
A parsed [StatusList2021Credential](https://www.w3.org/TR/2023/WD-vc-status-list-20230427/#statuslist2021credential).

**Kind**: global class  

* [StatusList2021Credential](#StatusList2021Credential)
    * [new StatusList2021Credential(credential)](#new_StatusList2021Credential_new)
    * _instance_
        * [.id()](#StatusList2021Credential+id) ⇒ <code>string</code>
        * [.setCredentialStatus(credential, index, revoked_or_suspended)](#StatusList2021Credential+setCredentialStatus) ⇒ [<code>StatusList2021Entry</code>](#StatusList2021Entry)
        * [.purpose()](#StatusList2021Credential+purpose) ⇒ [<code>StatusPurpose</code>](#StatusPurpose)
        * [.entry(index)](#StatusList2021Credential+entry) ⇒ [<code>CredentialStatus</code>](#CredentialStatus)
        * [.clone()](#StatusList2021Credential+clone) ⇒ [<code>StatusList2021Credential</code>](#StatusList2021Credential)
        * [.toJSON()](#StatusList2021Credential+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#StatusList2021Credential.fromJSON) ⇒ [<code>StatusList2021Credential</code>](#StatusList2021Credential)

<a name="new_StatusList2021Credential_new"></a>

### new StatusList2021Credential(credential)
Creates a new [StatusList2021Credential](#StatusList2021Credential).


| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 

<a name="StatusList2021Credential+id"></a>

### statusList2021Credential.id() ⇒ <code>string</code>
**Kind**: instance method of [<code>StatusList2021Credential</code>](#StatusList2021Credential)  
<a name="StatusList2021Credential+setCredentialStatus"></a>

### statusList2021Credential.setCredentialStatus(credential, index, revoked_or_suspended) ⇒ [<code>StatusList2021Entry</code>](#StatusList2021Entry)
Sets the given credential's status using the `index`-th entry of this status list.
Returns the created `credentialStatus`.

**Kind**: instance method of [<code>StatusList2021Credential</code>](#StatusList2021Credential)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| index | <code>number</code> | 
| revoked_or_suspended | <code>boolean</code> | 

<a name="StatusList2021Credential+purpose"></a>

### statusList2021Credential.purpose() ⇒ [<code>StatusPurpose</code>](#StatusPurpose)
Returns the [StatusPurpose](#StatusPurpose) of this [StatusList2021Credential](#StatusList2021Credential).

**Kind**: instance method of [<code>StatusList2021Credential</code>](#StatusList2021Credential)  
<a name="StatusList2021Credential+entry"></a>

### statusList2021Credential.entry(index) ⇒ [<code>CredentialStatus</code>](#CredentialStatus)
Returns the state of the `index`-th entry, if any.

**Kind**: instance method of [<code>StatusList2021Credential</code>](#StatusList2021Credential)  

| Param | Type |
| --- | --- |
| index | <code>number</code> | 

<a name="StatusList2021Credential+clone"></a>

### statusList2021Credential.clone() ⇒ [<code>StatusList2021Credential</code>](#StatusList2021Credential)
**Kind**: instance method of [<code>StatusList2021Credential</code>](#StatusList2021Credential)  
<a name="StatusList2021Credential+toJSON"></a>

### statusList2021Credential.toJSON() ⇒ <code>any</code>
**Kind**: instance method of [<code>StatusList2021Credential</code>](#StatusList2021Credential)  
<a name="StatusList2021Credential.fromJSON"></a>

### StatusList2021Credential.fromJSON(json) ⇒ [<code>StatusList2021Credential</code>](#StatusList2021Credential)
**Kind**: static method of [<code>StatusList2021Credential</code>](#StatusList2021Credential)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="StatusList2021CredentialBuilder"></a>

## StatusList2021CredentialBuilder
Builder type to construct valid [StatusList2021Credential](#StatusList2021Credential) istances.

**Kind**: global class  

* [StatusList2021CredentialBuilder](#StatusList2021CredentialBuilder)
    * [new StatusList2021CredentialBuilder([status_list])](#new_StatusList2021CredentialBuilder_new)
    * [.purpose(purpose)](#StatusList2021CredentialBuilder+purpose) ⇒ [<code>StatusList2021CredentialBuilder</code>](#StatusList2021CredentialBuilder)
    * [.subjectId(id)](#StatusList2021CredentialBuilder+subjectId) ⇒ [<code>StatusList2021CredentialBuilder</code>](#StatusList2021CredentialBuilder)
    * [.expirationDate(time)](#StatusList2021CredentialBuilder+expirationDate) ⇒ [<code>StatusList2021CredentialBuilder</code>](#StatusList2021CredentialBuilder)
    * [.issuer(issuer)](#StatusList2021CredentialBuilder+issuer) ⇒ [<code>StatusList2021CredentialBuilder</code>](#StatusList2021CredentialBuilder)
    * [.context(context)](#StatusList2021CredentialBuilder+context) ⇒ [<code>StatusList2021CredentialBuilder</code>](#StatusList2021CredentialBuilder)
    * [.type(t)](#StatusList2021CredentialBuilder+type) ⇒ [<code>StatusList2021CredentialBuilder</code>](#StatusList2021CredentialBuilder)
    * [.proof(proof)](#StatusList2021CredentialBuilder+proof) ⇒ [<code>StatusList2021CredentialBuilder</code>](#StatusList2021CredentialBuilder)
    * [.build()](#StatusList2021CredentialBuilder+build) ⇒ [<code>StatusList2021Credential</code>](#StatusList2021Credential)

<a name="new_StatusList2021CredentialBuilder_new"></a>

### new StatusList2021CredentialBuilder([status_list])
Creates a new [StatusList2021CredentialBuilder](#StatusList2021CredentialBuilder).


| Param | Type |
| --- | --- |
| [status_list] | [<code>StatusList2021</code>](#StatusList2021) \| <code>undefined</code> | 

<a name="StatusList2021CredentialBuilder+purpose"></a>

### statusList2021CredentialBuilder.purpose(purpose) ⇒ [<code>StatusList2021CredentialBuilder</code>](#StatusList2021CredentialBuilder)
Sets the purpose of the [StatusList2021Credential](#StatusList2021Credential) that is being created.

**Kind**: instance method of [<code>StatusList2021CredentialBuilder</code>](#StatusList2021CredentialBuilder)  

| Param | Type |
| --- | --- |
| purpose | [<code>StatusPurpose</code>](#StatusPurpose) | 

<a name="StatusList2021CredentialBuilder+subjectId"></a>

### statusList2021CredentialBuilder.subjectId(id) ⇒ [<code>StatusList2021CredentialBuilder</code>](#StatusList2021CredentialBuilder)
Sets `credentialSubject.id`.

**Kind**: instance method of [<code>StatusList2021CredentialBuilder</code>](#StatusList2021CredentialBuilder)  

| Param | Type |
| --- | --- |
| id | <code>string</code> | 

<a name="StatusList2021CredentialBuilder+expirationDate"></a>

### statusList2021CredentialBuilder.expirationDate(time) ⇒ [<code>StatusList2021CredentialBuilder</code>](#StatusList2021CredentialBuilder)
Sets the expiration date of the credential.

**Kind**: instance method of [<code>StatusList2021CredentialBuilder</code>](#StatusList2021CredentialBuilder)  

| Param | Type |
| --- | --- |
| time | [<code>Timestamp</code>](#Timestamp) | 

<a name="StatusList2021CredentialBuilder+issuer"></a>

### statusList2021CredentialBuilder.issuer(issuer) ⇒ [<code>StatusList2021CredentialBuilder</code>](#StatusList2021CredentialBuilder)
Sets the issuer of the credential.

**Kind**: instance method of [<code>StatusList2021CredentialBuilder</code>](#StatusList2021CredentialBuilder)  

| Param | Type |
| --- | --- |
| issuer | <code>string</code> | 

<a name="StatusList2021CredentialBuilder+context"></a>

### statusList2021CredentialBuilder.context(context) ⇒ [<code>StatusList2021CredentialBuilder</code>](#StatusList2021CredentialBuilder)
Sets the context of the credential.

**Kind**: instance method of [<code>StatusList2021CredentialBuilder</code>](#StatusList2021CredentialBuilder)  

| Param | Type |
| --- | --- |
| context | <code>string</code> | 

<a name="StatusList2021CredentialBuilder+type"></a>

### statusList2021CredentialBuilder.type(t) ⇒ [<code>StatusList2021CredentialBuilder</code>](#StatusList2021CredentialBuilder)
Adds a credential type.

**Kind**: instance method of [<code>StatusList2021CredentialBuilder</code>](#StatusList2021CredentialBuilder)  

| Param | Type |
| --- | --- |
| t | <code>string</code> | 

<a name="StatusList2021CredentialBuilder+proof"></a>

### statusList2021CredentialBuilder.proof(proof) ⇒ [<code>StatusList2021CredentialBuilder</code>](#StatusList2021CredentialBuilder)
Adds a credential's proof.

**Kind**: instance method of [<code>StatusList2021CredentialBuilder</code>](#StatusList2021CredentialBuilder)  

| Param | Type |
| --- | --- |
| proof | [<code>Proof</code>](#Proof) | 

<a name="StatusList2021CredentialBuilder+build"></a>

### statusList2021CredentialBuilder.build() ⇒ [<code>StatusList2021Credential</code>](#StatusList2021Credential)
Attempts to build a valid [StatusList2021Credential](#StatusList2021Credential) with the previously provided data.

**Kind**: instance method of [<code>StatusList2021CredentialBuilder</code>](#StatusList2021CredentialBuilder)  
<a name="StatusList2021Entry"></a>

## StatusList2021Entry
[StatusList2021Entry](https://www.w3.org/TR/2023/WD-vc-status-list-20230427/#statuslist2021entry) implementation.

**Kind**: global class  

* [StatusList2021Entry](#StatusList2021Entry)
    * [new StatusList2021Entry(status_list, purpose, index, [id])](#new_StatusList2021Entry_new)
    * _instance_
        * [.id()](#StatusList2021Entry+id) ⇒ <code>string</code>
        * [.purpose()](#StatusList2021Entry+purpose) ⇒ [<code>StatusPurpose</code>](#StatusPurpose)
        * [.index()](#StatusList2021Entry+index) ⇒ <code>number</code>
        * [.statusListCredential()](#StatusList2021Entry+statusListCredential) ⇒ <code>string</code>
        * [.toStatus()](#StatusList2021Entry+toStatus) ⇒ <code>Status</code>
        * [.clone()](#StatusList2021Entry+clone) ⇒ [<code>StatusList2021Entry</code>](#StatusList2021Entry)
        * [.toJSON()](#StatusList2021Entry+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#StatusList2021Entry.fromJSON) ⇒ [<code>StatusList2021Entry</code>](#StatusList2021Entry)

<a name="new_StatusList2021Entry_new"></a>

### new StatusList2021Entry(status_list, purpose, index, [id])
Creates a new [StatusList2021Entry](#StatusList2021Entry).


| Param | Type |
| --- | --- |
| status_list | <code>string</code> | 
| purpose | [<code>StatusPurpose</code>](#StatusPurpose) | 
| index | <code>number</code> | 
| [id] | <code>string</code> \| <code>undefined</code> | 

<a name="StatusList2021Entry+id"></a>

### statusList2021Entry.id() ⇒ <code>string</code>
Returns this `credentialStatus`'s `id`.

**Kind**: instance method of [<code>StatusList2021Entry</code>](#StatusList2021Entry)  
<a name="StatusList2021Entry+purpose"></a>

### statusList2021Entry.purpose() ⇒ [<code>StatusPurpose</code>](#StatusPurpose)
Returns the purpose of this entry.

**Kind**: instance method of [<code>StatusList2021Entry</code>](#StatusList2021Entry)  
<a name="StatusList2021Entry+index"></a>

### statusList2021Entry.index() ⇒ <code>number</code>
Returns the index of this entry.

**Kind**: instance method of [<code>StatusList2021Entry</code>](#StatusList2021Entry)  
<a name="StatusList2021Entry+statusListCredential"></a>

### statusList2021Entry.statusListCredential() ⇒ <code>string</code>
Returns the referenced [StatusList2021Credential](#StatusList2021Credential)'s url.

**Kind**: instance method of [<code>StatusList2021Entry</code>](#StatusList2021Entry)  
<a name="StatusList2021Entry+toStatus"></a>

### statusList2021Entry.toStatus() ⇒ <code>Status</code>
Downcasts [this](this) to [Status](Status)

**Kind**: instance method of [<code>StatusList2021Entry</code>](#StatusList2021Entry)  
<a name="StatusList2021Entry+clone"></a>

### statusList2021Entry.clone() ⇒ [<code>StatusList2021Entry</code>](#StatusList2021Entry)
Deep clones the object.

**Kind**: instance method of [<code>StatusList2021Entry</code>](#StatusList2021Entry)  
<a name="StatusList2021Entry+toJSON"></a>

### statusList2021Entry.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>StatusList2021Entry</code>](#StatusList2021Entry)  
<a name="StatusList2021Entry.fromJSON"></a>

### StatusList2021Entry.fromJSON(json) ⇒ [<code>StatusList2021Entry</code>](#StatusList2021Entry)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>StatusList2021Entry</code>](#StatusList2021Entry)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="Storage"></a>

## Storage
A type wrapping a `JwkStorage` and `KeyIdStorage` that should always be used together when
working with storage backed DID documents.

**Kind**: global class  

* [Storage](#Storage)
    * [new Storage(jwkStorage, keyIdStorage)](#new_Storage_new)
    * [.keyIdStorage()](#Storage+keyIdStorage) ⇒ <code>KeyIdStorage</code>
    * [.keyStorage()](#Storage+keyStorage) ⇒ <code>JwkStorage</code>

<a name="new_Storage_new"></a>

### new Storage(jwkStorage, keyIdStorage)
Constructs a new `Storage`.


| Param | Type |
| --- | --- |
| jwkStorage | <code>JwkStorage</code> | 
| keyIdStorage | <code>KeyIdStorage</code> | 

<a name="Storage+keyIdStorage"></a>

### storage.keyIdStorage() ⇒ <code>KeyIdStorage</code>
Obtain the wrapped `KeyIdStorage`.

**Kind**: instance method of [<code>Storage</code>](#Storage)  
<a name="Storage+keyStorage"></a>

### storage.keyStorage() ⇒ <code>JwkStorage</code>
Obtain the wrapped `JwkStorage`.

**Kind**: instance method of [<code>Storage</code>](#Storage)  
<a name="Timestamp"></a>

## Timestamp
**Kind**: global class  

* [Timestamp](#Timestamp)
    * [new Timestamp()](#new_Timestamp_new)
    * _instance_
        * [.toRFC3339()](#Timestamp+toRFC3339) ⇒ <code>string</code>
        * [.checkedAdd(duration)](#Timestamp+checkedAdd) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
        * [.checkedSub(duration)](#Timestamp+checkedSub) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
        * [.toJSON()](#Timestamp+toJSON) ⇒ <code>any</code>
    * _static_
        * [.parse(input)](#Timestamp.parse) ⇒ [<code>Timestamp</code>](#Timestamp)
        * [.nowUTC()](#Timestamp.nowUTC) ⇒ [<code>Timestamp</code>](#Timestamp)
        * [.fromJSON(json)](#Timestamp.fromJSON) ⇒ [<code>Timestamp</code>](#Timestamp)

<a name="new_Timestamp_new"></a>

### new Timestamp()
Creates a new [Timestamp](#Timestamp) with the current date and time.

<a name="Timestamp+toRFC3339"></a>

### timestamp.toRFC3339() ⇒ <code>string</code>
Returns the [Timestamp](#Timestamp) as an RFC 3339 `String`.

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
Parses a [Timestamp](#Timestamp) from the provided input string.

**Kind**: static method of [<code>Timestamp</code>](#Timestamp)  

| Param | Type |
| --- | --- |
| input | <code>string</code> | 

<a name="Timestamp.nowUTC"></a>

### Timestamp.nowUTC() ⇒ [<code>Timestamp</code>](#Timestamp)
Creates a new [Timestamp](#Timestamp) with the current date and time.

**Kind**: static method of [<code>Timestamp</code>](#Timestamp)  
<a name="Timestamp.fromJSON"></a>

### Timestamp.fromJSON(json) ⇒ [<code>Timestamp</code>](#Timestamp)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>Timestamp</code>](#Timestamp)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="UnknownCredential"></a>

## UnknownCredential
**Kind**: global class  

* [UnknownCredential](#UnknownCredential)
    * _instance_
        * [.tryIntoJwt()](#UnknownCredential+tryIntoJwt) ⇒ [<code>Jwt</code>](#Jwt) \| <code>undefined</code>
        * [.tryIntoCredential()](#UnknownCredential+tryIntoCredential) ⇒ [<code>Credential</code>](#Credential) \| <code>undefined</code>
        * [.tryIntoRaw()](#UnknownCredential+tryIntoRaw) ⇒ <code>Record.&lt;string, any&gt;</code> \| <code>undefined</code>
        * [.toJSON()](#UnknownCredential+toJSON) ⇒ <code>any</code>
        * [.clone()](#UnknownCredential+clone) ⇒ [<code>UnknownCredential</code>](#UnknownCredential)
    * _static_
        * [.fromJSON(json)](#UnknownCredential.fromJSON) ⇒ [<code>UnknownCredential</code>](#UnknownCredential)

<a name="UnknownCredential+tryIntoJwt"></a>

### unknownCredential.tryIntoJwt() ⇒ [<code>Jwt</code>](#Jwt) \| <code>undefined</code>
Returns a [Jwt](#Jwt) if the credential is of type string, `undefined` otherwise.

**Kind**: instance method of [<code>UnknownCredential</code>](#UnknownCredential)  
<a name="UnknownCredential+tryIntoCredential"></a>

### unknownCredential.tryIntoCredential() ⇒ [<code>Credential</code>](#Credential) \| <code>undefined</code>
Returns a [Credential](#Credential) if the credential is of said type, `undefined` otherwise.

**Kind**: instance method of [<code>UnknownCredential</code>](#UnknownCredential)  
<a name="UnknownCredential+tryIntoRaw"></a>

### unknownCredential.tryIntoRaw() ⇒ <code>Record.&lt;string, any&gt;</code> \| <code>undefined</code>
Returns the contained value as an Object, if it can be converted, `undefined` otherwise.

**Kind**: instance method of [<code>UnknownCredential</code>](#UnknownCredential)  
<a name="UnknownCredential+toJSON"></a>

### unknownCredential.toJSON() ⇒ <code>any</code>
Serializes this to a JSON object.

**Kind**: instance method of [<code>UnknownCredential</code>](#UnknownCredential)  
<a name="UnknownCredential+clone"></a>

### unknownCredential.clone() ⇒ [<code>UnknownCredential</code>](#UnknownCredential)
Deep clones the object.

**Kind**: instance method of [<code>UnknownCredential</code>](#UnknownCredential)  
<a name="UnknownCredential.fromJSON"></a>

### UnknownCredential.fromJSON(json) ⇒ [<code>UnknownCredential</code>](#UnknownCredential)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>UnknownCredential</code>](#UnknownCredential)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="VerificationMethod"></a>

## VerificationMethod
A DID Document Verification Method.

**Kind**: global class  

* [VerificationMethod](#VerificationMethod)
    * [new VerificationMethod(id, controller, type_, data)](#new_VerificationMethod_new)
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
        * [.newFromJwk(did, key, [fragment])](#VerificationMethod.newFromJwk) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
        * [.fromJSON(json)](#VerificationMethod.fromJSON) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)

<a name="new_VerificationMethod_new"></a>

### new VerificationMethod(id, controller, type_, data)
Create a custom [VerificationMethod](#VerificationMethod).


| Param | Type |
| --- | --- |
| id | [<code>DIDUrl</code>](#DIDUrl) | 
| controller | [<code>CoreDID</code>](#CoreDID) | 
| type_ | [<code>MethodType</code>](#MethodType) | 
| data | [<code>MethodData</code>](#MethodData) | 

<a name="VerificationMethod+id"></a>

### verificationMethod.id() ⇒ [<code>DIDUrl</code>](#DIDUrl)
Returns a copy of the [DIDUrl](#DIDUrl) of the [VerificationMethod](#VerificationMethod)'s `id`.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod+setId"></a>

### verificationMethod.setId(id)
Sets the id of the [VerificationMethod](#VerificationMethod).

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  

| Param | Type |
| --- | --- |
| id | [<code>DIDUrl</code>](#DIDUrl) | 

<a name="VerificationMethod+controller"></a>

### verificationMethod.controller() ⇒ [<code>CoreDID</code>](#CoreDID)
Returns a copy of the `controller` `DID` of the [VerificationMethod](#VerificationMethod).

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod+setController"></a>

### verificationMethod.setController(did)
Sets the `controller` `DID` of the [VerificationMethod](#VerificationMethod) object.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  

| Param | Type |
| --- | --- |
| did | [<code>CoreDID</code>](#CoreDID) | 

<a name="VerificationMethod+type"></a>

### verificationMethod.type() ⇒ [<code>MethodType</code>](#MethodType)
Returns a copy of the [VerificationMethod](#VerificationMethod) type.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod+setType"></a>

### verificationMethod.setType(type_)
Sets the [VerificationMethod](#VerificationMethod) type.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  

| Param | Type |
| --- | --- |
| type_ | [<code>MethodType</code>](#MethodType) | 

<a name="VerificationMethod+data"></a>

### verificationMethod.data() ⇒ [<code>MethodData</code>](#MethodData)
Returns a copy of the [VerificationMethod](#VerificationMethod) public key data.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod+setData"></a>

### verificationMethod.setData(data)
Sets [VerificationMethod](#VerificationMethod) public key data.

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
<a name="VerificationMethod.newFromJwk"></a>

### VerificationMethod.newFromJwk(did, key, [fragment]) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
Creates a new [VerificationMethod](#VerificationMethod) from the given `did` and [Jwk](#Jwk). If `fragment` is not given
the `kid` value of the given `key` will be used, if present, otherwise an error is returned.

### Recommendations
The following recommendations are essentially taken from the `publicKeyJwk` description from the [DID specification](https://www.w3.org/TR/did-core/#dfn-publickeyjwk):
- It is recommended that verification methods that use `Jwks` to represent their public keys use the value of
  `kid` as their fragment identifier. This is
done automatically if `None` is passed in as the fragment.
- It is recommended that [Jwk](#Jwk) kid values are set to the public key fingerprint.

**Kind**: static method of [<code>VerificationMethod</code>](#VerificationMethod)  

| Param | Type |
| --- | --- |
| did | [<code>CoreDID</code>](#CoreDID) \| <code>IToCoreDID</code> | 
| key | [<code>Jwk</code>](#Jwk) | 
| [fragment] | <code>string</code> \| <code>undefined</code> | 

<a name="VerificationMethod.fromJSON"></a>

### VerificationMethod.fromJSON(json) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
Deserializes an instance from a JSON object.

**Kind**: static method of [<code>VerificationMethod</code>](#VerificationMethod)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 


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
<a name="SerializationType"></a>

## SerializationType
**Kind**: global variable  
<a name="MethodRelationship"></a>

## MethodRelationship
**Kind**: global variable  
<a name="SubjectHolderRelationship"></a>

## SubjectHolderRelationship
Declares how credential subjects must relate to the presentation holder.

See also the [Subject-Holder Relationship](https://www.w3.org/TR/vc-data-model/#subject-holder-relationships) section of the specification.

**Kind**: global variable  
<a name="AlwaysSubject"></a>

## AlwaysSubject
The holder must always match the subject on all credentials, regardless of their [`nonTransferable`](https://www.w3.org/TR/vc-data-model/#nontransferable-property) property.
This variant is the default.

**Kind**: global variable  
<a name="SubjectOnNonTransferable"></a>

## SubjectOnNonTransferable
The holder must match the subject only for credentials where the [`nonTransferable`](https://www.w3.org/TR/vc-data-model/#nontransferable-property) property is `true`.

**Kind**: global variable  
<a name="Any"></a>

## Any
The holder is not required to have any kind of relationship to any credential subject.

## StateMetadataEncoding
**Kind**: global variable  
<a name="StateMetadataEncoding"></a>

## StateMetadataEncoding
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

**Kind**: global variable  
<a name="verifyEd25519"></a>

## verifyEd25519(alg, signingInput, decodedSignature, publicKey)
Verify a JWS signature secured with the `EdDSA` algorithm and curve `Ed25519`.

This function is useful when one is composing a `IJwsVerifier` that delegates
`EdDSA` verification with curve `Ed25519` to this function.

# Warning

This function does not check whether `alg = EdDSA` in the protected header. Callers are expected to assert this
prior to calling the function.

**Kind**: global function  

| Param | Type |
| --- | --- |
| alg | <code>JwsAlgorithm</code> | 
| signingInput | <code>Uint8Array</code> | 
| decodedSignature | <code>Uint8Array</code> | 
| publicKey | [<code>Jwk</code>](#Jwk) | 

<a name="start"></a>

## start()
Initializes the console error panic hook for better error messages

**Kind**: global function  
<a name="encodeB64"></a>

## encodeB64(data) ⇒ <code>string</code>
Encode the given bytes in url-safe base64.

**Kind**: global function  

| Param | Type |
| --- | --- |
| data | <code>Uint8Array</code> | 

<a name="decodeB64"></a>

## decodeB64(data) ⇒ <code>Uint8Array</code>
Decode the given url-safe base64-encoded slice into its raw bytes.

**Kind**: global function  

| Param | Type |
| --- | --- |
| data | <code>Uint8Array</code> | 

<a name="start"></a>

## start()
Initializes the console error panic hook for better error messages

**Kind**: global function  

# DID Communications Message Specification

> #### Contents
>
> ◈ <a href="#did-discovery">**DID Discovery**</a> - Requesting a DID from an agent.
>
> ◈ <a href="#authentication">**Authentication**</a> - Proving control over an identifier.
>
> ◈ <a href="#authorization">**Authorization**</a> - Giving consent or permission.
>
> ◈ <a href="#credential-issuance">**Credential Issuance**</a> - Creating an authenticated statement about an identifier.
>
> ◈ <a href="#credential-verification">**Credential Verification**</a> - Proving a set of statements about an identifier.
>
> ◈ <a href="#did-resolution">**DID Resolution**</a> - Using another Agent as a Resolver.

---
# DID Discovery

Requesting a DID from an agent.

### Roles
- <u>**Requester**</u>: Agent who requests a DID from the <u>endpoint</u>
- <u>**Endpoint**</u>: Agent who provides the requested DID to the <u>requester</u>

### Messages

#### DID Request
The <u>requester</u> sends the `didRequest` to the <u>endpoint</u>, specifying a `callbackURL` for the `didResponse` to be posted to. 

###### Layout

```JSON
didRequest: {
    callbackURL: "<URL as String>"
}
```

#### DID Response
The <u>endpoint</u> answers with a `didResponse`, containing its DID.

###### Layout

```JSON
didResponse: {
    did: "<DID as String>"
}
```

### Examples

The <u>requester</u> sends the `didRequest` to the <u>endpoint</u>'s API:

```JSON
{
    callbackURL: "https://www.aliceswonderland.com/didreq"
}
```

The <u>endpoint</u> answers with a `didResponse` to the `callbackURL`:

```JSON
{
    did: "did:iota:zsdbfg897s34bgez"
}
```

---
# Authentication

Proving control over an identifier.

The authentication flow consists of a simple request-response message exchange, where the contents of the response must match those of the request. Because all messages are signed and authenticated, the response functions as proof of control by nature of being correctly signed by the keys listed in the DID Document of the issuer. Because of this, in scenarios where a more complex functionality (e.g. Credential Verification) is needed, an additional authentication flow is not necessary.

### Roles
- <u>**Verifier**</u>: Agent who requests and verifies the authenticity of the <u>authenticator</u>
- <u>**Authenticator**</u>: Agent who proves control over their identifier

### Messages

#### Authentication Request
The <u>verifier</u> sends the `authenticationRequest` to the authentication service endpoint of the <u>authenticator</u>, specifying a `callbackURL` for the `authenticationResponse` to be posted to, as well as an arbitrary `description` which is to be signed by the <u>authenticator</u>. 

###### Layout

```JSON
authenticationRequest: {
    callbackURL: "<URL as String>",
    description: "<Text as String>",
}
```

#### Authentication Response
The <u>authenticator</u> answers with an `authenticationResponse`, quoting the `authenticationRequest` it answers to and providing a `signature` of the `authenticationRequest` field, which is the complete original `authenticationRequest`.

###### Layout

```JSON
authenticationResponse: {
    authenticationRequest: {
        callbackURL: "<URL as String>",
        description: "<Text as String>",
    },
    signature: {
      type: "<Signature Type as String>",
      verificationMethod: "<Verification Method as String>",
      signatureValue: "<Signature as String>"
   }
}
```

### Examples

The <u>verifier</u> wants to know whether an identity he received earlier corresponds to the domain https://www.bob.com. He sends an `authenticationRequest` to the domain specified in the identity's service endpoint:

```JSON
{
    callbackURL: "https://example.com/auth",
    description: "Are you Bob?",
}
```

The service endpoint of the <u>authenticator</u> receives the `authenticationRequest` and answers with e.g. the following `authenticationResponse`:

```JSON
{
    authenticationRequest: {
        callbackURL: "https://www.bob.com/auth",
        description: "Are you Bob?",
    },
    signature: {
      type: "JcsEd25519Signature2020",
      verificationMethod: "#authentication",
      signatureValue: "5Hw1JWv4a6hZH5obtAshbbKZQAJK6h8YbEwZvdxgWCXSL81fvRYoMCjt22vaBtZewgGq641dqR31C27YhDusoo4N"
   }
}
```

The `signature` provided here must correspond with the `#authentication` public key provided in the DID Document of the identity that the <u>verifier</u> has received earlier. If that is the case, the domain is authenticated successfully.

---
# Authorization

Giving consent or permission.

The Authorization flow consists of a simple request-response message exchange, where the Initiator requests authorization from the <u>authorizer</u> to carry out some action. It is similar to the authentication flow in structure, however the intent of the interaction is different. Authentication is about proving the identity of an agent (e.g. SSO), while authorization is about giving permission or privilege for a service to act on an agents behalf.

### Roles
- **Authorized**: Agent requesting authorization to perform some action
- **Authorizer**: Agent granting authorization to the <u>authorized</u>

### Messages

#### Authorization Request
The <u>authorized</u> broadcasts a message representing the intent of the action which permission is required for.

###### Layout

```JSON
authorizationRequest: {
    callbackURL: "<URL as String>",
    description: "<Text as String>",
    imageURL: "<Image URL as String>",
    action: "<Text as String>",
}
```

#### Authorization Response
The <u>authorizer</u> responds with a message containing the same contents as the `authorizationRequest` as consent.

###### Layout

```JSON
authorizationResponse: {
    callbackURL: "<URL as String>",
    description: "<Text as String>",
    imageURL: "<Image URL as String>",
    action: "<Text as String>",
}
```

### Examples

The <u>authorized</u> would like to open the <u>authorizers</u> door and sends an `authorizationRequest` for said action to the <u>authorizer</u>:

```JSON
{
    callbackURL: "https://example.com/authz",
    description: "Front Door",
    imageURL: "https://example.com/lockImage.png",
    action: "Open the door",
}
```

The <u>authorizer</u> reponds with the same content, consenting to the action:

```JSON
{
    callbackURL: "https://example.com/authz",
    description: "Front Door",
    imageURL: "https://example.com/lockImage.png",
    action: "Open the door",
}
```

---
# Credential Issuance

Creating an authenticated statement about an identifier.

The issuance flow consists of a three step message exchange between two parties, the <u>issuer</u> and the <u>holder</u>.

### Roles
- **Issuer**: Agent who offers and issues one or more Verifiable Credentials
- **Holder**: Agent who selects and receives one or more Verifiable Credentials

### Messages

#### Credential Offer
The <u>issuer</u> broadcasts a message containing a list of credential types offered for issuance in this interaction, each with it's own list of requirements which must be satisfied by the <u>holder</u> in order to qualify for the credential.

###### Layout

```JSON
{
    callbackURL: "<URL as String>",
    offeredCredentials: [
        {
            type: "<Type as String>",
        },
    ],
}
```

#### Credential Selection
The <u>holder</u> responds with a message containing a list of selected credentials with associated data for satisfying requirements.

###### Layout

```JSON
{
    callbackURL: "<URL as String>",
    selectedCredentials: [{type: "<Type as String>"}]
}
```

#### Credential Issuance
The <u>issuer</u> responds with a message containing a list of newly issued credentials corrosponding to the selected set.

###### Layout

```JSON
{
    issued: [
        [{type: "<Type as String>"}]
    ]
}
```

### Examples

TBD after above flow is cleared up

---
# Credential Verification

Proving a set of statements about an identifier.

The credential verification flow is a simple request-response message exchange between the <u>verifier</u> and the <u>prover</u>.

### Roles
- **Verifier**: Agent who requests a set of Verifiable Credentials with associated requirements
- **Prover**: Agent who provides a set of Verifiable Credentials attempting to satisfy the request

### Messages

#### Credential Request
The <u>verifier</u> broadcasts a `credentialRequest` containing a list of credential types, each with it's own list of requirements to be satisfied by the <u>prover</u>.

###### Layout

```JSON
credentialRequest: {
    callbackURL: "<URL as String>",
    credentialRequirements: [
        {
            type: "<Type as String>",
            constraints: [
                <Constraint 1>,
                <Constraint 2>,
            ],
        },
    ],
}
```

#### Credential Response
The <u>prover</u> responds with a list of credentials which should satisfy the corrosponding requirements in the `credentialRequest`.

###### Layout

```JSON
credentialResponse: {
    credentials: [{type: "<Type as String>"}]
}
```

### Examples

TBD after above flow is cleared up

---
# DID Resolution

Using another Agent as a Resolver.

Peer resolution consists of a simple request-response message exchange, where the Requester asks the Resolver to perform DID resolution and return the result.

### Roles
- **Requester**: Agent who requests the resolution of a DID
- **Resolver**: Agent who resolves the given DID (or their own) and returns the result

### Messages

#### Resolution Request
The Requester broadcasts a message which may or may not contain a DID.

###### Layout

```JSON
resolutionRequest: {
    callbackURL: "<URL as String>",
    did: "<DID as String>",
}
```

#### Resolution Result
If the message contains a DID, the Resolver resolves the DID and returns the DID Resolution Result. Otherwise, the Resolver returns the result of resolving it's own DID. This is intended for the special case of "local" DID methods, which do not have a globally resolvable state.

###### Layout

```JSON
resolutionResult: {
    didDocument: "<DID Document as JSON>",
}
```

### Examples

The <u>requester</u> sends a `resolutionRequest` to the <u>resolver</u>:

```JSON
{
    callbackURL: "https://alice.com/res",
    did: "did:iota:sdbgik8s34htosebgo9se34hg9so3ehg",
}
```

The <u>resolver</u> answers with a `resolutionResult` to the <u>requester</u>:

```JSON
{
    didDocument: {
        @context: "https://www.w3.org/ns/did/v1",
        id: "did:example:123456789abcdefghi",
        authentication: [{
            id: "did:example:123456789abcdefghi#keys-1",
            type: "Ed25519VerificationKey2020",
            controller: "did:example:123456789abcdefghi",
            publicKeyMultibase: "zH3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
        }]
    }
}
```
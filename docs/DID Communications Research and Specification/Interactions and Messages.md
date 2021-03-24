# DID Communications Message Specification
TODO https://identity.foundation/didcomm-messaging/spec/#discover-features-protocol-10
TODO dec https://github.com/hyperledger/aries-rfcs/blob/master/features/0032-message-timing/README.md
TODO thread id

> ### Contents
>
> ◈ <a href="#trust-ping">**Trust Ping**</a> - Testing a pairwise channel.
>
> ◈ <a href="#did-discovery">**DID Discovery**</a> - Requesting a DID from an agent.
>
> ◈ <a href="#did-introduction">**DID Introduction**</a> - Introducing two parties through an intermediary.
>
> ◈ <a href="#did-resolution">**DID Resolution**</a> - Using another agent as a Resolver.
>
> ◈ <a href="#authentication">**Authentication**</a> - Proving control over a DID.
>
> ◈ <a href="#authorization">**Authorization**</a> - Giving consent or permission.
>
> ◈ <a href="#credential-issuance">**Credential Issuance**</a> - Creating an authenticated statement about a DID.
>
> ◈ <a href="#credential-revocation">**Credential Revocation**</a> - Notifying a holder that a previously issued credential has been revoked.
>
> ◈ <a href="#presentation-verification">**Presentation Verification**</a> - Proving a set of statements about a DID.

---
## Trust Ping

Testing a pairwise channel.

### Roles
- <u>**Sender**</u>: Agent who initiates the trust ping
- <u>**Receiver**</u>: Agent who responds to the <u>senders</u> trust ping

### Messages

#### Ping
The <u>senders</u> sends the `trustPing` to the <u>receiver</u>, specifying a `callbackURL` for the `didResponse` to be posted to. TODO say what is OPTIONAL

###### Layout

```JSON
trustPing: {
    "callbackURL": "<URL as String>",
    "responseRequested": <Boolean>, TODO: make obvious if MUST or optional
    "type": "<URL as String>", TODO REM
    "id": "<DID as String>", TODO make optional / REM
    "timing": {
TODO
 "out_time": "2018-12-15 04:29:23Z",
    "expires_time": "2018-12-15 05:29:23Z",


    }
}
```

#### Response
The <u>receiver</u> answers with a `trustPingResponse`:

###### Layout

```JSON
trustPingResponse: {
    "type": "<URL as String>",
    "id": "<DID as String>",
    "timing": {...}
}
```

### Examples

The <u>senders</u> sends the `trustPing` to the <u>receiver</u>'s API:

```JSON
{
    "callbackURL": "https://www.bobsworld.com/ping",
    "responseRequested": true,
    "type": "https://didcomm.org/trust_ping/1.0/ping",
    "id": "did:iota:sdgf786sdgfi87sedzgf",
    "timing": {...}
}
```

Only if `responseRequested` is `true` may the <u>receiver</u> answer the ping with a `trustPingResponse`:

```JSON
{
    "type": "https://didcomm.org/trust_ping/1.0/ping",
    "id": "did:iota:hd8f7hg84e5hbtg8drg",
    "timing": {...}
}
```

---
## DID Discovery

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
    "callbackURL": "<URL as String>"
    TODO OPTIONAL DID FIELD
}
```

#### DID Response
The <u>endpoint</u> answers with a `didResponse`, containing its DID.

###### Layout

```JSON
didResponse: {
    "did": "<DID as String>"
}
```

### Examples

The <u>requester</u> sends the `didRequest` to the <u>endpoint</u>'s API:

```JSON
{
    "callbackURL": "https://www.aliceswonderland.com/didreq"
}
```

The <u>endpoint</u> answers with a `didResponse` to the `callbackURL`:

```JSON
{
    "did": "did:iota:zsdbfg897s34bgez"
}
```

---
## DID Introduction
TODO open an issue for this and remove it here
TODO call it "now kiss" protocol
Introducing two parties through an intermediary.

### Roles
- <u>**Introducer**</u>: Agent who introduces two <u>introducees</u> to each other
- <u>**Introducee**</u>: Agents who get introduced to each other by the <u>introducer</u>

### Messages

TBD

### Examples

TBD

---
## DID Resolution
TODO put down sources for all interactions into the document
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
    "callbackURL": "<URL as String>",
    "did": "<DID as String>",
}
```

#### Resolution Result
If the message contains a DID, the Resolver resolves the DID and returns the DID Resolution Result. Otherwise, the Resolver returns the result of resolving it's own DID. This is intended for the special case of "local" DID methods, which do not have a globally resolvable state.

###### Layout

```JSON
resolutionResult: {
    "didDocument": "<DID Document as JSON>",
}
```

### Examples

The <u>requester</u> sends a `resolutionRequest` to the <u>resolver</u>:

```JSON
{
    "callbackURL": "https://alice.com/res",
    "did": "did:iota:sdbgik8s34htosebgo9se34hg9so3ehg",
}
```

The <u>resolver</u> answers with a `resolutionResult` to the <u>requester</u>:

```JSON
{
    "didDocument": {
        "@context": "https://www.w3.org/ns/did/v1",
        "id": "did:example:123456789abcdefghi",
        "authentication": [{
            "id": "did:example:123456789abcdefghi#keys-1",
            "type": "Ed25519VerificationKey2020",
            "controller": "did:example:123456789abcdefghi",
            "publicKeyMultibase": "zH3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
        }]
    }
}
```

---
## Authentication

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
    "callbackURL": "<URL as String>",
    "description": "<Text as String>",
    TODO timestamp or random value or challenge/nonce, sign everything
    TODO check WHAT EXACTLY others are actually signing
    TODO thread if instead of sending challenge back
}
```

#### Authentication Response
The <u>authenticator</u> answers with an `authenticationResponse`, quoting the `authenticationRequest` it answers to and providing a `signature` of the `authenticationRequest` field, which is the complete original `authenticationRequest`.

###### Layout

```JSON
authenticationResponse: {
    "authenticationRequest": {
        "callbackURL": "<URL as String>",
        "description": "<Text as String>",
    },
    "signature": {
      "type": "<Signature Type as String>",
      "verificationMethod": "<Verification Method as String>",
      "signatureValue": "<Signature as String>"
   }
}
```

### Examples

The <u>verifier</u> wants to know whether an identity he received earlier corresponds to the domain https://www.bob.com. He sends an `authenticationRequest` to the domain specified in the identity's service endpoint:

```JSON
{
    "callbackURL": "https://example.com/auth",
    "description": "Are you Bob?",
}
```

The service endpoint of the <u>authenticator</u> receives the `authenticationRequest` and answers with e.g. the following `authenticationResponse`:

```JSON
{
    "authenticationRequest": {
        "callbackURL": "https://www.bob.com/auth",
        "description": "Are you Bob?",
    },
    "signature": {
        "type": "JcsEd25519Signature2020",
        "verificationMethod": "#authentication",
        "signatureValue": "5Hw1JWv4a6hZH5obtAshbbKZQAJK6h8YbEwZvdxgWCXSL81fvRYoMCjt22vaBtZewgGq641dqR31C27YhDusoo4N"
   }
}
```

The `signature` provided here must correspond with the `#authentication` public key provided in the DID Document of the identity that the <u>verifier</u> has received earlier. If that is the case, the domain is authenticated successfully.

---
## Authorization

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
    "callbackURL": "<URL as String>",
    "description": "<Text as String>",
    "imageURL": "<Image URL as String>",
    "action": "<Text as String>",
}
```

#### Authorization Response
The <u>authorizer</u> responds with a message containing the same contents as the `authorizationRequest` as consent.
TODO: respond with a VC, think about including frost into the vc for the action field, remove for now and submit an issue
###### Layout

```JSON
authorizationResponse: {
    "callbackURL": "<URL as String>",
    "description": "<Text as String>",
    "imageURL": "<Image URL as String>",
    "action": "<Text as String>",
}
```

### Examples

The <u>authorized</u> would like to open the <u>authorizers</u> door and sends an `authorizationRequest` for said action to the <u>authorizer</u>:

```JSON
{
    "callbackURL": "https://example.com/authz",
    "description": "Front Door",
    "imageURL": "https://example.com/lockImage.png",
    "action": "Open the door",
}
```

The <u>authorizer</u> reponds with the same content, consenting to the action:

```JSON
{
    "callbackURL": "https://example.com/authz",
    "description": "Front Door",
    "imageURL": "https://example.com/lockImage.png",
    "action": "Open the door",
}
```

---
## Credential Issuance
TODO split into 3 interactions: requesting possibler VCs, requesting the schema, issuing the VC
Creating an authenticated statement about an identifier.
TODO pqwjefiosdgfuikzsdfg

The issuance flow consists of a three step message exchange between two parties, the <u>issuer</u> and the <u>holder</u>.

### Roles
- **Issuer**: Agent who offers and issues one or more Verifiable Credentials
- **Holder**: Agent who selects and receives one or more Verifiable Credentials

### Messages

#### Credential Offer
The <u>issuer</u> broadcasts a message containing a list of credential types offered for issuance in this interaction, each with it's own list of requirements which must be satisfied by the <u>holder</u> in order to qualify for the credential.

###### Layout
TODO return the whole schema of VC offered
```JSON
{
    "callbackURL": "<URL as String>",
    "offeredCredentials": [
        {
            "type": "<Type as String>",
        },
    ],
}
```

#### Credential Selection
The <u>holder</u> responds with a message containing a list of selected credentials with associated data for satisfying requirements.

###### Layout

```JSON
{
    "callbackURL": "<URL as String>",
    "selectedCredentials": [
        {
            "type": "<Type as String>"
        },
    ],
}
```

#### Credential Issuance
The <u>issuer</u> responds with a message containing a list of newly issued credentials corrosponding to the selected set.

###### Layout

```JSON
{
    "issued": [
        {
            "type": "<Type as String>"
        },
    ],
}
```

### Examples

TBD after above flow is cleared up

---
## Credential Revocation

Notifying a holder that a previously issued credential has been revoked.

### Roles
- <u>**Issuer**</u>: Agent who revokes the credential
- <u>**Holder**</u>: Agent who holds the credential to be revoked

### Messages

#### Ping
The <u>issuer</u> sends the `credentialRevocation` to the <u>holder</u>, notifying him of the revocation. 

###### Layout

```JSON
credentialRevocation: {
    "@type": "<Type as String>",
    "@id": "<uuid-revocation-notification>",
    "credential_id": "<uuid-credential>",
    "comment": "Some comment"
}
```

### Examples

The <u>issuer</u> sends the `credentialRevocation` to the <u>holder</u>:

```JSON
{
  "@type": "https://didcomm.org/revocation_notification/1.0/revoke",
  "@id": "<uuid-revocation-notification>",
  "credential_id": "<uuid-credential>",
  "comment": "Some comment"
}
```

---
## Presentation Verification

Proving a set of statements about an identifier.

The credential verification flow is a simple request-response message exchange between the <u>verifier</u> and the <u>prover</u>.

### Roles
- **Verifier**: Agent who requests a set of Verifiable Credentials with associated requirements
- **Prover**: Agent who provides a set of Verifiable Credentials in the form of a Verifiable Presentation attempting to satisfy the request

### Messages

#### Credential Request
The <u>verifier</u> broadcasts a `credentialRequest` containing a list of credential types, each with it's own list of requirements to be satisfied by the <u>prover</u>.

###### Layout

```JSON
credentialRequest: {
    "callbackURL": "<URL as String>",
    "credentialRequirements": [
        {
            "type": "<Type as String>",
            "constraints": [
                <Constraint 1>,
                <Constraint 2>,
            ],
        },
    ],
}
```

#### Credential Response
The <u>prover</u> responds with a Verifiable Presentation which should satisfy the corrosponding requirements in the `credentialRequest`.

###### Layout

```JSON
credentialResponse: {
    "verifiablePresentation": {...}
}
```

### Examples

TBD after above flow is cleared up

TODO make it one interaction with 3 messages but the first one is optional

message 1: (request a presentation)
message 2: send a presentation
message 3: (ack)
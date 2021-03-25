# DID Communications Message Specification

> ### Field Definitions

`callbackURL` as URL/String, e.g. `https://www.bobsworld.com/ping` or `https://www.aliceswonderland/authz`: Defines the URL or API call where a request or response is to be delivered to.

`responseRequested` as Boolean, e.g. `true` or `false`: In Messages where it is defined a reponse is to be sent to a request if and only if this is `true`. Undefined counts as `false`.

`context` as URL/String, e.g. `https://didcomm.org/trust_ping/1.0/ping`: Defines the context that a specific message adheres to.

`id` as String, e.g. `did:iota:3b8mZHjb6r6inMcDVZU4DZxNdLnxUigurg42tJPiFV9v`: An IOTA decentralized identifier.

`didDocument` as JSON: An IOTA DID Document (see e.g. in <a href="#did-resolution">DID Resolution</a>).

`thread` as String, e.g. `jdhgbksdbgjksdbgkjdkg` or `thread-132-a`: A String, defined by the agent, to be used to identify this specific interaction to track it agent-locally.

`challenge` as String, e.g. `please-sign-this`: A String acting as a signing challenge.

`signature` as JSON, e.g. `{...}`: Includes a signature. Fields defined below.

`signature[type]` as String, e.g. `JcsEd25519Signature2020`: Signature type.

`signature[verificationMethod]` as String, e.g. `#authentication`: Reference to verification method in signer's DID Document used to produce the signature.

`signature[signatureValue]` as String, e.g. `5Hw1JWv4a6hZH5obtAshbbKZQAJK6h8YbEwZvdxgWCXSL81fvRYoMCjt22vaBtZewgGq641dqR31C27YhDusoo4N`: Actual signature.

`timing` as JSON, e.g. `{...}`: A decorator to include timing information into a message. Fields defined below.

`timing[out_time]` as ISO 8601 timestamp, e.g. `2069-04-20T13:37:00Z`: The timestamp when the message was emitted.

`timing[in_time]` as ISO 8601 timestamp, e.g. `2069-04-20T13:37:00Z`: The timestamp when the preceding message in this thread (the one that elicited this message as a response) was received.

`timing[stale_time]` as ISO 8601 timestamp, e.g. `2069-04-20T13:37:00Z`: Ideally, the decorated message should be processed by the the specified timestamp. After that, the message may become irrelevant or less meaningful than intended. This is a hint only.

`timing[expires_time]` as ISO 8601 timestamp, e.g. `2069-04-20T13:37:00Z`: The decorated message should be considered invalid or expired if encountered after the specified timestamp. This is a much stronger claim than the one for stale_time; it says that the receiver should cancel attempts to process it once the deadline is past, because the sender won't stand behind it any longer. While processing of the received message should stop, the thread of the message should be retained as the sender may send an updated/replacement message. In the case that the sender does not follow up, the policy of the receiver agent related to abandoned threads would presumably be used to eventually delete the thread.

`timing[delay_milli]` as Integer, e.g. `1337`: Wait at least this many milliseconds before processing the message. This may be useful to defeat temporal correlation. It is recommended that agents supporting this field should not honor requests for delays longer than 10 minutes (600,000 milliseconds).

`timing[wait_until_time]` as ISO 8601 timestamp, e.g. `2069-04-20T13:37:00Z`: Wait until this time before processing the message.

[(Source 1: Aries Message Timing)](https://github.com/hyperledger/aries-rfcs/blob/master/features/0032-message-timing/README.md)
  
> ### Interactions

◈ <a href="#trust-ping">**Trust Ping**</a> - Testing a pairwise channel.

◈ <a href="#did-discovery">**DID Discovery**</a> - Requesting a DID from an agent.

◈ <a href="#did-resolution">**DID Resolution**</a> - Using another agent as a Resolver.

◈ <a href="#authentication">**Authentication**</a> - Proving control over a DID.

◈ (WIP) <a href="#credential-issuance">**Credential Issuance**</a> - Creating an authenticated statement about a DID.

◈ (WIP) <a href="#credential-revocation">**Credential Revocation**</a> - Notifying a holder that a previously issued credential has been revoked.

◈ (WIP) <a href="#presentation-verification">**Presentation Verification**</a> - Proving a set of statements about a DID.

---
## Trust Ping

Testing a pairwise channel.

### Roles
- <u>**Sender**</u>: Agent who initiates the trust ping
- <u>**Receiver**</u>: Agent who responds to the <u>senders</u> trust ping

### Messages

#### trustPing
The <u>senders</u> sends the `trustPing` to the <u>receiver</u>, specifying a `callbackURL` for the `trustPingResponse` to be posted to.

###### Layout

```JSON
trustPing: {
    "callbackURL",
    "responseRequested", //OPTIONAL! Counts as false if omitted!
    "context", // OPTIONAL!
    "id", // OPTIONAL!
    "thread", // OPTIONAL!
    "timing": {...} // OPTIONAL! All subfields OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "callbackURL": "https://www.bobsworld.com/ping",
    "responseRequested": true,
    "context": "https://didcomm.org/trust_ping/1.0/ping",
    "id": "did:iota:3b8mZHjb6r6inMcDVZU4DZxNdLnxUigurg42tJPiFV9v",
    "timing": {
        "delay_milli": 1337
    }
}
```

#### trustPingResponse
The <u>receiver</u> answers with a `trustPingResponse` if and only if `responseRequested` was `true` in the `trustPing` message:

###### Layout

```JSON
trustPingResponse: {
    "id", // OPTIONAL!
    "thread", // OPTIONAL!
    "timing": {...} // OPTIONAL! All subfields OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "id": "did:iota:86b7t9786tb9JHFGJKHG8796UIZGUk87guzgUZIuggez",
}
```

[Source 1: DIF Trust Ping](https://identity.foundation/didcomm-messaging/spec/#trust-ping-protocol-10); [Source 2: Aries Trust Ping](https://github.com/hyperledger/aries-rfcs/tree/master/features/0048-trust-ping)

---
## DID Discovery

Requesting a DID from an agent.

### Roles
- <u>**Requester**</u>: Agent who requests a DID from the <u>endpoint</u>
- <u>**Endpoint**</u>: Agent who provides the requested DID to the <u>requester</u>

### Messages

#### didRequest
The <u>requester</u> sends the `didRequest` to the <u>endpoint</u>, specifying a `callbackURL` for the `didResponse` to be posted to. 

###### Layout

```JSON
didRequest: {
    "callbackURL",
    "context", // OPTIONAL!
    "id", // OPTIONAL!
    "thread", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "callbackURL": "https://www.aliceswonderland.com/didreq",
    "id": "did:iota:3b8mZHjb6r6inMcDVZU4DZxNdLnxUigurg42tJPiFV9v",
}
```

#### didResponse
The <u>endpoint</u> answers with a `didResponse`, containing its DID.

###### Layout

```JSON
didResponse: {
    "id"
}
```

###### Example(s)

```JSON
{
    "id": "did:iota:86b7t9786tb9JHFGJKHG8796UIZGUk87guzgUZIuggez"
}
```

---
## DID Resolution

Using another Agent as a Resolver.

DID resolution consists of a simple request-response message exchange, where the Requester asks the Resolver to perform DID resolution and return the result.

### Roles
- **Requester**: Agent who requests the resolution of a DID
- **Resolver**: Agent who resolves the given DID (or their own) and returns the result

### Messages

#### resolutionRequest
The Requester broadcasts a message which may or may not contain a DID.

###### Layout

```JSON
resolutionRequest: {
    "callbackURL",
    "id", // OPTIONAL!
    "thread", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "callbackURL": "https://www.aliceswonderland.com/res",
    "id": "did:iota:86b7t9786tb9JHFGJKHG8796UIZGUk87guzgUZIuggez",
    "thread": "req-1-1337b"
}
```

#### resolutionResult
If the message contains a DID (in the `id` field), the Resolver resolves the DID and returns the DID Resolution Result. Otherwise, the Resolver returns the result of resolving it's own DID. This is intended for the special case of "local" DID methods, which do not have a globally resolvable state.

###### Layout

```JSON
resolutionResult: {
    "didDocument"
    "id", // OPTIONAL!
    "thread", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "thread": "req-1-1337b",
    "didDocument": {
        "@context": "https://www.w3.org/ns/did/v1",
        "id": "did:iota:86b7t9786tb9JHFGJKHG8796UIZGUk87guzgUZIuggez",
        "authentication": [{
            "id": "did:iota:86b7t9786tb9JHFGJKHG8796UIZGUk87guzgUZIuggez#keys-1",
            "type": "Ed25519VerificationKey2020",
            "controller": "did:iota:86b7t9786tb9JHFGJKHG8796UIZGUk87guzgUZIuggez",
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

#### authenticationRequest
The <u>verifier</u> sends the `authenticationRequest` to the authentication service endpoint of the <u>authenticator</u>, specifying a `callbackURL` for the `authenticationResponse` to be posted to. The `thread` is used as a challenge and also serves to identity the `authenticationRequest`. The whole request is to be signed by the <u>authenticator</u>. 

###### Layout

```JSON
authenticationRequest: {
    "callbackURL",
    "thread",
    "challenge",
    "id", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "callbackURL": "https://www.aliceswonderland.com/auth",
    "thread": "69-420-1337",
    "challenge": "please sign this",
    "id": "did:iota:86b7t9786tb9JHFGJKHG8796UIZGUk87guzgUZIuggez",
    "timing": {
        "out_time": "1991-04-20T13:37:11Z",
        "expires_time": "2069-04-20T13:37:02Z",
        "wait_until_time": "2069-04-20T13:37:00Z"
    }
}
```

#### authenticationResponse
The <u>authenticator</u> answers with an `authenticationResponse`, providing a `signature` of the whole `authenticationRequest` JSON - the complete original `authenticationRequest`.

###### Layout

```JSON
authenticationResponse: {
    "thread",
    "signature"
}
```

###### Example(s)

```JSON
{
    "thread": "69-420-1337",
    "signature": {
        "type": "JcsEd25519Signature2020",
        "verificationMethod": "#authentication",
        "signatureValue": "5Hw1JWv4a6hZH5obtAshbbKZQAJK6h8YbEwZvdxgWCXSL81fvRYoMCjt22vaBtZewgGq641dqR31C27YhDusoo4N"
   }
}
```

The `signature` provided here must correspond to the `#authentication` public key provided in the DID Document of the identity that the <u>verifier</u> has received earlier. If that is the case, the identifier is authenticated successfully.















VVVVV HERE BE DRAGONS VVVVV













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



















{
    "callbackURL",
    "context", // OPTIONAL!
    "id", // OPTIONAL!
    "thread", // OPTIONAL!
    "timing" // OPTIONAL!
}

`callbackURL`
`responseRequested`
`context`
`id`
`thread`
`timing`

    TODO timestamp or random value or challenge/nonce, sign everything
    TODO check WHAT EXACTLY others are actually signing
    TODO thread if instead of sending challenge back
TODO put down sources for all interactions into the document
 TODO say what is OPTIONAL
TODO https://identity.foundation/didcomm-messaging/spec/#discover-features-protocol-10
TODO thread id



























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
# DID Communications Message Specification

## Field Definitions

`context` as URL/String, e.g. `did-resolution/1.0/resolutionResponse`: Defines the context that a specific message adheres to.

`ackContext` & `errContext` as URL/String, e.g. `did-resolution/1.0/resolutionResponse`: Variants of `context` that describe the context of a message that is being acknowledged or the message that errored.

`thread` as String, e.g. `jdhgbksdbgjksdbgkjdkg` or `thread-132-a`: A String, defined by the agent, to be used to identify this specific interaction to track it agent-locally.

`callbackURL` as URL/String, e.g. `https://www.bobsworld.com/ping` or `https://www.aliceswonderland/authz`: Defines the URL or API call where a request or response is to be delivered to.

`id` as String, e.g. `did:iota:3b8mZHjb6r6inMcDVZU4DZxNdLnxUigurg42tJPiFV9v`: An IOTA decentralized identifier.

`didDocument` as JSON: An IOTA DID Document (see e.g. in <a href="#did-resolution">DID Resolution</a>).

`responseRequested` as Boolean, e.g. `true` or `false`: In messages where it is defined a reponse is to be sent to a request if and only if this is `true`. Undefined counts as `false`.

`comment` as String: A comment.

`challenge` as JSON, e.g. `{"foo": "sign this"}`: A JSON acting as a signing challenge.

`offeredCredentialTypes` as JSON: A field specific to VC issuance, contains a list of possible credential types, see <a href="#credential-options">Credential Options</a>.

`credentialType` as String, e.g. `SimpleDiplomaCredential`: A VC type.

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

## Standalone Messages

Messages that are shared across interactions.

#### Roles
- <u>**Sender**</u>: Agent who sends the message
- <u>**Receiver**</u>: Agent who receives the message

##### report
The <u>sender</u> sends a `report` message to the <u>receiver</u> to provide him with details about a previously received message. This can be a simple acknowledgement or e.g. an error report.
TODO only do acks on request
TODO merge ack and error into report
TODO make thread not optional
TODO make thread a UUID
TODO ackkontext errkontext
TODO make callbackURL optional except in the first of each interaction messages

###### Layout

```JSON
report: {
    "context",
    "thread",
    "ackContext",
    "comment" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "report/1.0/report",
    "thread": "sdfgfjghsdfg-12345-sdf-b",
    "errContext": "did-resolution/1.0/resolutionResponse",
    "comment": "Can't resolve: Signature invalid!"
}
```

##### acknowledgement
The <u>sender</u> sends an `acknowledgement` message to the <u>receiver</u> to let him know that a previous message has been received.

###### Layout

```JSON
acknowledgement: {
    "context",
    "thread",
    "ackContext"
}
```

###### Example(s)

```JSON
{
    "context": "acknowledgement/1.0/acknowledgement",
    "thread": "sdfgfjghsdfg-12345-sdf-b",
    "ackContext": "did-resolution/1.0/resolutionResponse"
}
```

##### error
The <u>sender</u> sends an `error` message to the <u>receiver</u> to let him know that a previous message has resulted in an error.

###### Layout

```JSON
error: {
    "context",
    "thread",
    "ackContext",
    "comment" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "error/1.0/error",
    "thread": "sdfgfjghsdfg-12345-sdf-b",
    "errContext": "did-resolution/1.0/resolutionResponse",
    "comment": "Can't resolve: Signature invalid!"
}
```

## Interactions

◈ <a href="#trust-ping">**trust-ping**</a> (*ping*, *pingResponse*): Testing a pairwise channel.

◈ <a href="#did-discovery">**did-discovery**</a> (*didRequest*, *didResponse*): Requesting a DID from an agent.

◈ <a href="#did-resolution">**did-resolution**</a> (*resolutionRequest*, *resolutionResponse*): Using another agent as a Resolver.

◈ <a href="#authentication">**authentication**</a> (*authenticationRequest*, *authenticationResponse*): Proving control over a DID.

◈ <a href="#credential-options">**credential-options**</a> (*credentialOptionsRequest*, *credentialOptionsResponse*): Querying an agent for the VCs that the agent can issue.

◈ <a href="#credential-schema">**credential-schema**</a> (*credentialSchemaRequest*, *credentialSchemaResponse*): Querying an agent for the schema of a specific VC that the agent can issue.

◈ <a href="#credential-issuance">**credential-issuance**</a> (*credentialSelection*, *credentialIssuance*): Creating an authenticated statement about a DID.

◈ <a href="#credential-revocation">**credential-revocation**</a> (*revocation*): Notifying a holder that a previously issued credential has been revoked.

◈ <a href="#presentation-verification">**presentation-verification**</a> (*presentationRequest*, *presentationResponse*): Proving a set of statements about an identifier.

---
### trust-ping

Testing a pairwise channel.

#### Roles
- <u>**Sender**</u>: Agent who initiates the trust ping
- <u>**Receiver**</u>: Agent who responds to the <u>senders</u> trust ping

#### Messages

##### ping
The <u>sender</u> sends the `ping` to the <u>receiver</u>, specifying a `callbackURL` for the `pingResponse` to be posted to.

###### Layout

```JSON
ping: {
    "context",
    "callbackURL",
    "responseRequested", //OPTIONAL! Counts as false if omitted!
    "id", // OPTIONAL!
    "thread", // OPTIONAL!
    "timing": {...} // OPTIONAL! All subfields OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "trust-ping/1.0/ping",
    "callbackURL": "https://www.bobsworld.com/",
    "responseRequested": true,
    "id": "did:iota:3b8mZHjb6r6inMcDVZU4DZxNdLnxUigurg42tJPiFV9v",
    "timing": {
        "delay_milli": 1337
    }
}
```

##### pingResponse
The <u>receiver</u> answers with a `pingResponse` if and only if `responseRequested` was `true` in the `ping` message:

###### Layout

```JSON
pingResponse: {
    "context",
    "id", // OPTIONAL!
    "thread", // OPTIONAL!
    "timing": {...} // OPTIONAL! All subfields OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "trust-ping/1.0/pingResponse",
    "id": "did:iota:86b7t9786tb9JHFGJKHG8796UIZGUk87guzgUZIuggez",
}
```

[Source 1: DIF Trust Ping](https://identity.foundation/didcomm-messaging/spec/#trust-ping-protocol-10); [Source 2: Aries Trust Ping](https://github.com/hyperledger/aries-rfcs/tree/master/features/0048-trust-ping)

---
### did-discovery

Requesting a DID from an agent.

#### Roles
- <u>**Requester**</u>: Agent who requests a DID from the <u>endpoint</u>
- <u>**Endpoint**</u>: Agent who provides the requested DID to the <u>requester</u>

#### Messages

##### didRequest
The <u>requester</u> sends the `didRequest` to the <u>endpoint</u>, specifying a `callbackURL` for the `didResponse` to be posted to. 

###### Layout

```JSON
didRequest: {
    "context",
    "callbackURL",
    "id", // OPTIONAL!
    "thread", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "did-discovery/1.0/didRequest",
    "callbackURL": "https://www.aliceswonderland.com/didreq",
    "id": "did:iota:3b8mZHjb6r6inMcDVZU4DZxNdLnxUigurg42tJPiFV9v",
}
```

##### didResponse
The <u>endpoint</u> answers with a `didResponse`, containing its DID.

###### Layout

```JSON
didResponse: {
    "context",
    "id"
}
```

###### Example(s)

```JSON
{
    "context": "did-discovery/1.0/didResponse",
    "id": "did:iota:86b7t9786tb9JHFGJKHG8796UIZGUk87guzgUZIuggez"
}
```

---
### did-resolution

Using another Agent as a Resolver.

DID resolution consists of a simple request-response message exchange, where the Requester asks the Resolver to perform DID resolution and return the result.

#### Roles
- **Requester**: Agent who requests the resolution of a DID
- **Resolver**: Agent who resolves the given DID (or their own) and returns the result

#### Messages

##### resolutionRequest
The Requester broadcasts a message which may or may not contain a DID.

###### Layout

```JSON
resolutionRequest: {
    "context",
    "callbackURL",
    "id", // OPTIONAL!
    "thread", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "did-resolution/1.0/resolutionRequest",
    "callbackURL": "https://www.aliceswonderland.com/res",
    "id": "did:iota:86b7t9786tb9JHFGJKHG8796UIZGUk87guzgUZIuggez",
    "thread": "req-1-1337b"
}
```

##### resolutionResponse
If the message contains a DID (in the `id` field), the Resolver resolves the DID and returns the DID Resolution Result. Otherwise, the Resolver returns the result of resolving it's own DID. This is intended for the special case of "local" DID methods, which do not have a globally resolvable state.

###### Layout

```JSON
resolutionResponse: {
    "context",
    "didDocument"
    "id", // OPTIONAL!
    "thread", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)
TODO why is ID optional
```JSON
{
    "context": "did-resolution/1.0/resolutionResponse",
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
### authentication

Proving control over an identifier.

The authentication flow consists of a simple request-response message exchange, where the contents of the response must match those of the request. Because all messages are signed and authenticated, the response functions as proof of control by nature of being correctly signed by the keys listed in the DID Document of the issuer. Because of this, in scenarios where a more complex functionality (e.g. Credential Verification) is needed, an additional authentication flow is not necessary.

#### Roles
- <u>**Verifier**</u>: Agent who requests and verifies the authenticity of the <u>authenticator</u>
- <u>**Authenticator**</u>: Agent who proves control over their identifier

#### Messages

##### authenticationRequest
The <u>verifier</u> sends the `authenticationRequest` to the authentication service endpoint of the <u>authenticator</u>, specifying a `callbackURL` for the `authenticationResponse` to be posted to. The `thread` is used as a challenge and also serves to identity the `authenticationRequest`. The whole request is to be signed by the <u>authenticator</u>. 

###### Layout

```JSON
authenticationRequest: {
    "context",
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
    "context": "authentication/1.0/authenticationRequest",
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

##### authenticationResponse
The <u>authenticator</u> answers with an `authenticationResponse`, providing a `signature` of the whole `authenticationRequest` JSON - the complete original `authenticationRequest`.

###### Layout

```JSON
authenticationResponse: {
    "context",
    "thread",
    "signature"
}
```

###### Example(s)

```JSON
{
    "context": "authentication/1.0/authenticationResponse",
    "thread": "69-420-1337",
    "signature": {
        "type": "JcsEd25519Signature2020",
        "verificationMethod": "#authentication",
        "signatureValue": "5Hw1JWv4a6hZH5obtAshbbKZQAJK6h8YbEwZvdxgWCXSL81fvRYoMCjt22vaBtZewgGq641dqR31C27YhDusoo4N"
   }
}
```

The `signature` provided here must correspond to the `#authentication` public key provided in the DID Document of the identity that the <u>verifier</u> has received earlier. If that is the case, the identifier is authenticated successfully.

---
### credential-options

Querying an agent for the VCs that the agent can issue.

The Verifiable Credential (VC) issuance flow consists of a three step interaction process between two parties, the <u>issuer</u> and the <u>holder</u>. This is the first interaction in this process. In this interaction, the <u>holder</u> queries the <u>issuer</u> for a list of VC types that the <u>issuer</u> offers to issue.

#### Roles
- **Issuer**: Agent who offers and issues one or more Verifiable Credentials
- **Holder**: Agent who selects and receives one or more Verifiable Credentials

#### Messages

##### credentialOptionsRequest
The <u>holder</u> queries the <u>issuer</u> for a list of VC types that the <u>issuer</u> offers.

###### Layout

```JSON
credentialOptionsRequest: {
    "context",
    "callbackURL",
    "thread", // OPTIONAL!
    "id", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "credential-options/1.0/credentialOptionsRequest",
    "callbackURL": "https://www.alicesworld.com/credsList"
}
```

##### credentialOptionsResponse
The <u>issuer</u> responds with a list of offered VC types.

###### Layout

```JSON
credentialOptionsResponse: {
    "context",
    "offeredCredentialTypes": [
        "credentialType 1",
        "credentialType 2",
        "credentialType n"
    ],
    "supportedIssuers": [
        "issuer id 1",
        "issuer id 2",
        "issuer id n"
    ],
    "thread", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "credential-options/1.0/credentialOptionsResponse",
    "offeredCredentialTypes": [
        "DiplomaCredential",
        "YouHaveNiceHairCredential",
        "DriversLicenseCredential"
    ],
    "supportedIssuers": [
        "did:iota:afbsdjhfbasuidfb8asifb4bfkawuiefjhdfgsukdfb",
        "did:iota:jahsdbfukgsiudfgisdufgi8sdfgzsbegbesudgbudf"
    ]
}
```

---
### credential-schema
Querying an agent for the schema of a specific VC that the agent can issue.

The Verifiable Credential (VC) issuance flow consists of a three step interaction process between two parties, the <u>issuer</u> and the <u>holder</u>. This is the second interaction in this process. In this interaction, the <u>holder</u> queries the <u>issuer</u> for the precise schema of one of the VCs that the <u>issuer</u> offers to issue, with it's own list of requirements which must be satisfied by the <u>holder</u> in order to qualify for the credential.

#### Roles
- **Issuer**: Agent who offers and issues one or more Verifiable Credentials
- **Holder**: Agent who selects and receives one or more Verifiable Credentials

#### Messages

##### credentialSchemaRequest
The <u>holder</u> queries the <u>issuer</u> for the schema of a specific VC that the <u>issuer</u> offers.

###### Layout

```JSON
credentialSchemaRequest: {
    "context",
    "callbackURL",
    "credentialTypes",
    "thread", // OPTIONAL!
    "id", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "credential-options/1.0/credentialSchemaRequest",
    "callbackURL": "https://www.alicesworld.com/credsList",
    "credentialTypes": [
        "YouHaveNiceHairCredential",
        "DriversLicenseCredential"
    ]
}
```

##### credentialSchemaResponse
The <u>issuer</u> responds with the schema of the requested `credentialTypes`.

###### Layout

```JSON
credentialSchemaResponse: {
    "context",
    "schemas",
    "thread", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "credential-options/1.0/credentialSchemaResponse",
    "schemas": [
        "YouHaveNiceHairCredential": {
            "type": "YouHaveNiceHairCredential",
            ...
        },
        "DriversLicenseCredential": {
            "type": "DriversLicenseCredential",
            ...
        },
    ]
}
```

---
### credential-issuance
Creating an authenticated statement about a DID.

The Verifiable Credential (VC) issuance flow consists of a three step interaction process between two parties, the <u>issuer</u> and the <u>holder</u>. This is the third interaction in this process. In this interaction, the <u>holder</u> asks the <u>issuer</u> for issuance of a specific VC.

#### Roles
- **Issuer**: Agent who offers and issues one or more Verifiable Credentials
- **Holder**: Agent who selects and receives one or more Verifiable Credentials

#### Messages

##### credentialSelection
The <u>holder</u> sends a message containing a list of selected credentials with associated data for satisfying requirements.

###### Layout

```JSON
credentialSelection: {
    "context",
    "callbackURL",
    "selectedCredentials": [
            "type 1",
            "type 2",
            "type n"
    ],
}
```

###### Example(s)

```JSON
{
    "context": "credential-issuance/1.0/credentialSelection",
    "callbackURL": "https://www.bobsworld.com/serviceEndpoint",
    "selectedCredentials": [
            "YouHaveNiceHairCredential"
    ],
}
```

##### credentialIssuance
The <u>issuer</u> responds with a message containing a list of newly issued credentials corrosponding to the selected set.

###### Layout

```JSON
credentialIssuance: {
    "context",
    "issued": [
        ...
    ],
}
```

###### Example(s)

```JSON
{
    "context": "credential-issuance/1.0/credentialIssuance",
    "issued": [
            {...},
            {...}
    ],
}
```

---
### credential-revocation

Notifying a holder that a previously issued credential has been revoked. Note that this revocation is declaratory, not constitutive, so the actual revocation has to be done elsewhere (e.g. in the backend of the issuer).

#### Roles
- <u>**Issuer**</u>: Agent who revokes the credential
- <u>**Holder**</u>: Agent who holds the credential to be revoked

#### Messages

##### revocation
The <u>issuer</u> sends the `revocation` to the <u>holder</u>, notifying him of the revocation. The most important field here is `credentialId`, which specifies the credential that has been revoked.

###### Layout

```JSON
revocation: {
    "context",
    "credentialId",
    "comment", // OPTIONAL!
    "id" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "credential-revocation/1.0/revocation",
    "credentialId": "gfiweuzg89w3bgi8wbgi8wi8t",
    "comment": "Revoked because reasons.",
    "id": "did:iota:3b8mZHjb6r6inMcDVZU4DZxNdLnxUigurg42tJPiFV9v",
}
```

---
### presentation-verification

Proving a set of statements about an identifier.

The credential verification flow is a simple request-response message exchange between the <u>verifier</u> and the <u>prover</u>. The interaction can consist of up to two messages, the first one is OPTIONAL.

#### Roles
- **Verifier**: Agent who requests a set of Verifiable Credentials with associated requirements
- **Prover**: Agent who provides a set of Verifiable Credentials in the form of a Verifiable Presentation attempting to satisfy the request

#### Messages

##### presentationRequest
The <u>verifier</u> requests a set of Verifiable Credentials from the <u>prover</u>. This message is OPTIONAL within this interaction.

###### Layout

```JSON
presentationRequest: {
    "context",
    "callbackURL",
    "credentialRequirements": [
        {
            "type"
        },
    ],
}
```

###### Example(s)

```JSON
{
    "context": "presentation-verification/1.0/presentationRequest",
    "callbackURL": "https://www.bobsworld.com/pres",
    "credentialRequirements": [
        {
            "type": "YouHaveNiceHairCredential"
        },
        {
            "type": "DriversLicenseCredential"
        }
    ]
}
```

##### presentationResponse
The <u>holder</u> sends a Verifiable Presentation to the <u>verifier</u> using a `presentationResponse` message.

###### Layout

```JSON
presentationResponse: {
    "context",
    "verifiablePresentation"
}
```

###### Example(s)

```JSON
{
    "context": "presentation-verification/1.0/presentationResponse",
    "verifiablePresentation": {...}
}
```





vvvvv here be dragons vvvvv





TODO add more sources to everything
TODO presentationRequest:
    TODO add field challenge (nonce (or sign with timestamp)), maybe optional
    TODO signature issues
    field "credentialRequirements"
TODO credentialOptionsReponse:
    TODO and i offer therse types of sig suites that this supports

    TODO also add the actual signature types / methods like ed25519-merkle (verification method types)
    "error": {
        "errorCode": 200
        "comment": Shit's on fire, yo
        TODO
        https://github.com/hyperledger/aries-rfcs/blob/master/features/0035-report-problem/README.md
    }

TODO credentialSchemaResponse:
    TODO SRC https://w3c-ccg.github.io/vc-json-schemas/


https://w3c-ccg.github.io/vc-json-schemas/
https://github.com/hyperledger/aries-rfcs/blob/master/features/0035-report-problem/README.md

    TODO make nice interaction pictures / state machines





TODO more sources https://identity.foundation/#wgs
TODO VERSION, date, last changed, etc
TODO all PRS
TODO all issues:

Open discussion points, among others:

How exactly do we define the credentialSchemas?
How exactly will these schemas be structured and communicated?
Do we offer single issuance or do we use lists everywhere?

    Probably some more stuff we need to talk about

Other ToDos:

timestamp or random value or challenge/nonce, sign everything
check WHAT EXACTLY others are actually signing
thread if instead of sending challenge back
put down sources for all interactions into the document
say what is OPTIONAL
https://identity.foundation/didcomm-messaging/spec/#discover-features-protocol-10
thread id
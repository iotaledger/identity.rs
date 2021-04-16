# DID Communications Message Specification

*version 0.2, last changed April 2021*

## Field Definitions

`context` as URL/String, e.g. `did-resolution/1.0/resolutionResponse`: Defines the context that a specific message adheres to.

`reference` as URL/String, e.g. `did-resolution/1.0/resolutionResponse`: Defines the context that a report message refers to.

`thread` as UUID, e.g. `936DA01F9ABD4d9d80C702AF85C822A8`: A [UUID](https://docs.rs/uuid/0.8.2/uuid/) as String, defined by the agent that initiated an interaction, to be used to identify this specific interaction to track it agent-locally. Together with the context, these two fields can be used to identity a message / state within a specific interaction.

`callbackURL` as URL/String, e.g. `https://www.bobsworld.com/` or `https://www.aliceswonderland/serviceEndpoint`: Defines the URL (or API call) where a message is to be delivered to.

`responseRequested` as Boolean, e.g. `true` or `false`: Undefined counts as `false`. In messages where it is defined it asks the recipient of the message to repond in the form of an acknowledging report. This request SHOULD be honored. The only exception to this behaviour is in `trust-ping`, where the acknowledging report MUST be sent if and only if this field is `true` - if it is `false`, the report MUST NOT be send.

`features` as an array of Strings, e.g. `["trust-ping/1.0", "did-discovery/1.0"]`: An array used for <a href="#features-discovery">Features Discovery</a> that lists all available interactions that an agent supports, and their respective versions.

`id` as String, e.g. `did:iota:57edacef81828010b314b96c0915780f206341e0ce8892a1b56678c174eef2e8`: A decentralized identifier.

`didDocument` as JSON: A DID Document (see e.g. in <a href="#did-resolution">DID Resolution</a>).

`comment` as String: A comment, mostly used to provide more information about something. Can be literally any String.

`challenge` as String, e.g. `Sign this`: A String acting as a signing challenge. Can contain basically anything.

`credential` as [VC JSON](https://w3c-ccg.github.io/vc-json-schemas/): A syntactically valid credential.

`credentials`: An array of valid `credential`s as defined above.

`credentialId` as String, e.g.`credential-69420-delicious-lasagna`: The id of a credential.

`credentialType` as String, e.g. `YouHaveNiceHairCredential`: A VC type.

`credentialTypes` as JSON, e.g. `["YouHaveNiceHairCredential", "YourLasagnaIsDeliciousCredential"]`: Contains an array of possible credential types, see e.g. <a href="#credential-options">Credential Options</a>.

`supportedIssuers` as JSON: Contains an array of supported issuer `id`, see <a href="#credential-options">Credential Options</a>.

`trustedIssuers` as JSON: An array of `{credentialTypes, supportedIssuers}` pairs, see e.g. <a href="#presentation-verification">Presentation Verification</a>.

`schemata` as JSON: A named list of credential schemata, see <a href="#credential-schema">Credential Schema</a>.

`verifiablePresentation` as JSON: A Verifiable Presentation.

`signature` as JSON: Defines a signature. Fields defined below.

`signature[type]` as String, e.g. `JcsEd25519Signature2020`: Signature type.

`signature[verificationMethod]` as String, e.g. `#authentication`: Reference to verification method in signer's DID Document used to produce the signature.

`signature[signatureValue]` as String, e.g. `5Hw1JWv4a6hZH5obtAshbbKZQAJK6h8YbEwZvdxgWCXSL81fvRYoMCjt22vaBtZewgGq641dqR31C27YhDusoo4N`: Actual signature.

`timing` as JSON: A decorator to include timing information into a message. Fields defined below.

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

#### report
The <u>sender</u> sends a `report` message to the <u>receiver</u> to provide him with details about a previously received message. This can be a simple acknowledgement or e.g. an error report. The `reference` field refers to the message that is either acknowledged or has resulted in an error. Further information can be passed through the `comment` field.

###### Layout

```JSON
report: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "reference", // REQUIRED!
    "comment", // OPTIONAL!
    "timing" // OPTIONAL! All subfields OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "report/1.0/report",
    "thread": "f7771b285a971ba25d66dbe2d82f0bf5f956f4fe548bdf8617c3f24ebc10ed8c",
    "reference": "did-resolution/1.0/resolutionResponse",
    "comment": "Can't resolve DID: Signature invalid!",
    "timing": {
        "out_time": "2069-04-20T13:37:42Z",
        "in_time": "2069-04-20T13:37:00Z"
    }
}
```
[Source 1: Aries Report Problem Protocol](https://github.com/hyperledger/aries-rfcs/blob/master/features/0035-report-problem/README.md);

## Interactions

◈ <a href="#trust-ping">**trust-ping**</a> (*ping*, *pingResponse*): Testing a pairwise channel.

◈ <a href="#did-discovery">**did-discovery**</a> (*didRequest*, *didResponse*): Requesting a DID from an agent.

◈ <a href="#features-discovery">**features-discovery**</a> (*featuresRequest*, *featuresResponse*): Enabling agents to discover which interactions other agents support.

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

#### ping
The <u>sender</u> sends the `ping` to the <u>receiver</u>. The `responseRequested` field counts as `false` if omitted. If and only if the `responseRequested` field is true should the <u>receiver</u> respond to the ping with a `report` message, posted to the `callbackURL`. If it is `true`, a `thread` should be passed as well to reference the `ping`. The `callbackURL` is OPTIONAL here because the <u>sender</u> could, for example, just include the `id` field and timing information to let the <u>receiver</u> know of transport delays.

###### Layout

```JSON
ping: {
    "context", // REQUIRED!
    "callbackURL", // OPTIONAL!
    "thread", // OPTIONAL!
    "responseRequested", //OPTIONAL!
    "id", // OPTIONAL!
    "timing" // OPTIONAL! All subfields OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "trust-ping/1.0/ping",
    "callbackURL": "https://www.bobsworld.com/",
    "thread": "f7771b285a971ba25d66dbe2d82f0bf5f956f4fe548bdf8617c3f24ebc10ed8c",
    "responseRequested": true,
    "id": "did:iota:57edacef81828010b314b96c0915780f206341e0ce8892a1b56678c174eef2e8",
    "timing": {
        "delay_milli": 1337
    }
}
```

[Source 1: DIF Trust Ping](https://identity.foundation/didcomm-messaging/spec/#trust-ping-protocol-10); [Source 2: Aries Trust Ping](https://github.com/hyperledger/aries-rfcs/tree/master/features/0048-trust-ping);

---
### did-discovery

Requesting a DID from an agent.

#### Roles
- <u>**Requester**</u>: Agent who requests a DID from the <u>endpoint</u>
- <u>**Endpoint**</u>: Agent who provides the requested DID to the <u>requester</u>

#### Messages

#### didRequest
The <u>requester</u> sends the `didRequest` to the <u>endpoint</u>, specifying a `callbackURL` for the `didResponse` to be posted to. 

###### Layout

```JSON
didRequest: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "callbackURL", // REQUIRED!
    "responseRequested", //OPTIONAL!
    "id", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "did-discovery/1.0/didRequest",
    "thread": "f7771b285a971ba25d66dbe2d82f0bf5f956f4fe548bdf8617c3f24ebc10ed8c",
    "callbackURL": "https://www.bobsworld.com/",
    "id": "did:iota:57edacef81828010b314b96c0915780f206341e0ce8892a1b56678c174eef2e8",
}
```

#### didResponse
The <u>endpoint</u> answers with a `didResponse`, containing its DID.

###### Layout

```JSON
didResponse: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "id", // REQUIRED!
    "callbackURL", // OPTIONAL!
    "responseRequested", //OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "did-discovery/1.0/didResponse",
    "thread": "f7771b285a971ba25d66dbe2d82f0bf5f956f4fe548bdf8617c3f24ebc10ed8c",
    "id": "did:iota:42edacef81828010b314b96c0915780f206341e0ce8892a1b56678c174eef242"
}
```

---
### features-discovery

Enabling agents to discover which interactions other agents support.

#### Roles
- <u>**Requester**</u>: Agent who requests an array of interactions that the <u>responder</u> supports
- <u>**Responder**</u>: Agent who provides the requested array of interactions to the <u>requester</u>

#### Messages

#### featuresRequest
The <u>requester</u> sends the `featuresRequest` to the <u>responder</u>, asking for the array of supported interactions. 

###### Layout

```JSON
featuresRequest: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "callbackURL", // REQUIRED!
    "responseRequested", //OPTIONAL!
    "id", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "features-discovery/1.0/featuresRequest",
    "thread": "f7771b285a971ba25d66dbe2d82f0bf5f956f4fe548bdf8617c3f24ebc10ed8c",
    "callbackURL": "https://www.bobsworld.com/"
}
```

#### featuresResponse
The <u>responder</u> answers with a `featuresResponse`, containing the array of supported interactions.

###### Layout

```JSON
featuresResponse: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "features", // REQUIRED!
    "callbackURL", // OPTIONAL!
    "responseRequested", //OPTIONAL!
    "id", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "features-discovery/1.0/featuresResponse",
    "thread": "f7771b285a971ba25d66dbe2d82f0bf5f956f4fe548bdf8617c3f24ebc10ed8c",
    "features": [
        "trust-ping/1.0",
        "did-discovery/1.0",
        "features-discovery/1.0",
        "authentication/1.0",
    ]
}
```

[Source 1: Aries Discover Features Protocol](https://github.com/hyperledger/aries-rfcs/blob/master/features/0031-discover-features/README.md); [Source 2: DIF Discover Features Protocol](https://identity.foundation/didcomm-messaging/spec/#discover-features-protocol-10);

---
### did-resolution

Using another Agent as a Resolver.

DID resolution consists of a simple request-response message exchange, where the Requester asks the Resolver to perform DID resolution and return the result.

#### Roles
- **Requester**: Agent who requests the resolution of a DID
- **Resolver**: Agent who resolves the given DID (or their own) and returns the result

#### Messages

#### resolutionRequest
The Requester broadcasts a message which may or may not contain a DID (see below).

###### Layout

```JSON
resolutionRequest: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "callbackURL", // REQUIRED!
    "responseRequested", //OPTIONAL!
    "id", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "did-resolution/1.0/resolutionRequest",
    "thread": "f7771b285a971ba25d66dbe2d82f0bf5f956f4fe548bdf8617c3f24ebc10ed8c",
    "callbackURL": "https://www.bobsworld.com/",
    "id": "did:iota:57edacef81828010b314b96c0915780f206341e0ce8892a1b56678c174eef2e8",
}
```

#### resolutionResponse
If the message contains a DID (in the `id` field), the Resolver resolves the DID and returns the DID Resolution Result. Otherwise, the Resolver returns the result of resolving it's own DID. This is intended for the special case of "local" DID methods, which do not have a globally resolvable state.

###### Layout

```JSON
resolutionResponse: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "didDocument", // REQUIRED!
    "callbackURL", // OPTIONAL!
    "responseRequested", //OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "did-resolution/1.0/resolutionResponse",
    "thread": "f7771b285a971ba25d66dbe2d82f0bf5f956f4fe548bdf8617c3f24ebc10ed8c",
    "didDocument": {
        "id": "did:iota:57edacef81828010b3--SNIP--a1b56678c174eef2e8",
        "authentication": [{
            "id": "did:iota:57edacef81828010b314--SNIP--a1b56678c174eef2e8#keys-1",
            "type": "Ed25519VerificationKey2020",
            "controller": "did:iota:57edacef81828010b3--SNIP--a1b56678c174eef2e8",
            "publicKeyMultibase": "zH3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
        }]
    },
    "callbackURL": "https://www.aliceswonderland.com/"
}
```

[Source 1: Jolocom Peer Resolution](https://jolocom.github.io/jolocom-sdk/1.0.0/guides/interaction_flows/#peer-resolution); [Source 2: Aries DID Resolution Protocol](https://github.com/hyperledger/aries-rfcs/tree/master/features/0124-did-resolution-protocol);

---
### authentication

Proving control over an identifier.

The authentication flow consists of a simple request-response message exchange, where the contents of the response must match those of the request. Because all messages are signed and authenticated, the response functions as proof of control by nature of being correctly signed by the keys listed in the DID Document of the issuer. Because of this, in scenarios where a more complex functionality (e.g. Credential Verification) is needed, an additional authentication flow is not necessary.

#### Roles
- <u>**Verifier**</u>: Agent who requests and verifies the authenticity of the <u>authenticator</u>
- <u>**Authenticator**</u>: Agent who proves control over their identifier

#### Messages

#### authenticationRequest
The <u>verifier</u> sends the `authenticationRequest` to the authentication service endpoint of the <u>authenticator</u>, specifying a `callbackURL` for the `authenticationResponse` to be posted to. The whole request is to be signed by the <u>authenticator</u>. 

###### Layout

```JSON
authenticationRequest: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "callbackURL", // REQUIRED!
    "challenge", // REQUIRED!
    "responseRequested", //OPTIONAL!
    "id", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "authentication/1.0/authenticationRequest",
    "thread": "f7771b285a971ba25d66dbe2d82f0bf5f956f4fe548bdf8617c3f24ebc10ed8c",
    "callbackURL": "https://www.bobsworld.com/",
    "challenge": "Sign this!",
    "id": "did:iota:57edacef81828010b314b96c0915780f206341e0ce8892a1b56678c174eef2e8",
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
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "signature", // REQUIRED!
    "callbackURL", // OPTIONAL!
    "responseRequested", //OPTIONAL!
    "id", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "authentication/1.0/authenticationResponse",
    "thread": "f7771b285a971ba25d66dbe2d82f0bf5f956f4fe548bdf8617c3f24ebc10ed8c",
    "signature": {
        "type": "JcsEd25519Signature2020",
        "verificationMethod": "did:iota:42edacef81828010b314b96c0915780f206341e0ce8892a1b56678c174eef242#authentication",
        "signatureValue": "5Hw1JWv4a6hZH5obtAshbbKZQAJK6h8YbEwZvdxgWCXSL81fvRYoMCjt22vaBtZewgGq641dqR31C27YhDusoo4N"
   },
   "callbackURL": "https://www.aliceswonderland.com/",
   "id": "did:iota:42edacef81828010b314b96c0915780f206341e0ce8892a1b56678c174eef242",
}
```

The `signature` provided here must correspond to the `#authentication` public key provided in the DID Document of the identity that the <u>verifier</u> has received earlier. If that is the case, the identifier is authenticated successfully.

[Source 1: Jolocom Authentication](https://jolocom.github.io/jolocom-sdk/1.0.0/guides/interaction_flows/#authentication);

---
### credential-options

Querying an agent for the VCs that the agent can issue.

The Verifiable Credential (VC) issuance flow consists of a three step interaction process between two parties, the <u>issuer</u> and the <u>holder</u>. This is the first interaction in this process. In this interaction, the <u>holder</u> queries the <u>issuer</u> for a list of VC types that the <u>issuer</u> offers to issue.

#### Roles
- **Issuer**: Agent who offers and issues one or more Verifiable Credentials
- **Holder**: Agent who selects and receives one or more Verifiable Credentials

#### Messages

#### credentialOptionsRequest
The <u>holder</u> queries the <u>issuer</u> for a list of VC types that the <u>issuer</u> offers.

###### Layout

```JSON
credentialOptionsRequest: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "callbackURL", // REQUIRED!
    "responseRequested", //OPTIONAL!
    "id", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "credential-options/1.0/credentialOptionsRequest",
    "thread": "f7771b285a971ba25d66dbe2d82f0bf5f956f4fe548bdf8617c3f24ebc10ed8c",
    "callbackURL": "https://www.bobsworld.com/"
}
```

#### credentialOptionsResponse
The <u>issuer</u> responds with a list of offered VC types.

###### Layout

```JSON
credentialOptionsResponse: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "credentialTypes": [
        "credentialType 1",
        "credentialType 2",
        "credentialType n"
    ], // REQUIRED!
    "callbackURL", // OPTIONAL!
    "responseRequested", //OPTIONAL!
    "id", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "credential-options/1.0/credentialOptionsResponse",
    "thread": "f7771b285a971ba25d66dbe2d82f0bf5f956f4fe548bdf8617c3f24ebc10ed8c",
    "credentialTypes": [
        "YourCatHasAnAttitudeCredential",
        "YouHaveNiceHairCredential",
        "YourLasagnaIsDeliciousCredential"
    ],
    "callbackURL": "https://www.aliceswonderland.com/",
}
```

[Source 1: Jolocom VC Issuance](https://jolocom.github.io/jolocom-sdk/1.0.0/guides/interaction_flows/#verifiable-credential-issuance); [Source 2: Aries Issue Credential Protocol](https://github.com/hyperledger/aries-rfcs/tree/master/features/0453-issue-credential-v2);

---
### credential-schema
Querying an agent for the schema of a specific VC that the agent can issue.

The Verifiable Credential (VC) issuance flow consists of a three step interaction process between two parties, the <u>issuer</u> and the <u>holder</u>. This is the second interaction in this process. In this interaction, the <u>holder</u> queries the <u>issuer</u> for the precise schema of one of the VCs that the <u>issuer</u> offers to issue, with it's own list of requirements which must be satisfied by the <u>holder</u> in order to qualify for the credential.

#### Roles
- **Issuer**: Agent who offers and issues one or more Verifiable Credentials
- **Holder**: Agent who selects and receives one or more Verifiable Credentials

#### Messages

#### credentialSchemaRequest
The <u>holder</u> queries the <u>issuer</u> for the schema of a specific VC that the <u>issuer</u> offers.

###### Layout

```JSON
credentialSchemaRequest: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "callbackURL", // REQUIRED!
    "credentialTypes", // REQUIRED!
    "responseRequested", //OPTIONAL!
    "id", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "credential-options/1.0/credentialSchemaRequest",
    "thread": "f7771b285a971ba25d66dbe2d82f0bf5f956f4fe548bdf8617c3f24ebc10ed8c",
    "callbackURL": "https://www.bobsworld.com/",
    "credentialTypes": [
        "YouHaveNiceHairCredential",
        "YourLasagnaIsDeliciousCredential"
    ]
}
```

#### credentialSchemaResponse
The <u>issuer</u> responds with the schemata of the requested `credentialTypes`.

###### Layout

```JSON
credentialSchemaResponse: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "schemata", // REQUIRED!
    "callbackURL", // OPTIONAL!
    "responseRequested", //OPTIONAL!
    "id", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "credential-options/1.0/credentialSchemaResponse",
    "thread": "f7771b285a971ba25d66dbe2d82f0bf5f956f4fe548bdf8617c3f24ebc10ed8c",
    "schemata": [
        "YouHaveNiceHairCredential": {
            "type": "YouHaveNiceHairCredential",
            ...
        },
        "YourLasagnaIsDeliciousCredential": {
            "type": "YourLasagnaIsDeliciousCredential",
            ...
        },
    ]
}
```

[Source 1: Jolocom VC Issuance](https://jolocom.github.io/jolocom-sdk/1.0.0/guides/interaction_flows/#verifiable-credential-issuance); [Source 2: Aries Issue Credential Protocol](https://github.com/hyperledger/aries-rfcs/tree/master/features/0453-issue-credential-v2);

---
### credential-issuance
Creating an authenticated statement about a DID.

The Verifiable Credential (VC) issuance flow consists of a three step interaction process between two parties, the <u>issuer</u> and the <u>holder</u>. This is the third interaction in this process. In this interaction, the <u>holder</u> asks the <u>issuer</u> for issuance of a specific VC.

#### Roles
- **Issuer**: Agent who offers and issues one or more Verifiable Credentials
- **Holder**: Agent who selects and receives one or more Verifiable Credentials

#### Messages

#### credentialSelection
The <u>holder</u> sends a message containing a list of selected credentials.

###### Layout

```JSON
credentialSelection: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "callbackURL", // REQUIRED!
    "credentialTypes", // REQUIRED!
    "responseRequested", //OPTIONAL!
    "id", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "credential-issuance/1.0/credentialSelection",
    "thread": "f7771b285a971ba25d66dbe2d82f0bf5f956f4fe548bdf8617c3f24ebc10ed8c",
    "callbackURL": "https://www.bobsworld.com/",
    "credentialTypes": [
        "YourCatHasAnAttitudeCredential",
        "YourLasagnaIsDeliciousCredential"
    ],
}
```

#### credentialIssuance
The <u>issuer</u> responds with a message containing a list of newly issued credentials corrosponding to the selected set.

###### Layout

```JSON
credentialIssuance: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "credentials", // REQUIRED!
    "callbackURL", // OPTIONAL!
    "responseRequested", //OPTIONAL!
    "id", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "credential-issuance/1.0/credentialIssuance",
    "thread": "f7771b285a971ba25d66dbe2d82f0bf5f956f4fe548bdf8617c3f24ebc10ed8c",
    "credentials": [
            "credential 1",
            "credential 2",
            "credential n"
    ],
    "callbackURL": "https://www.aliceswonderland.com/"
}
```

[Source 1: Jolocom VC Issuance](https://jolocom.github.io/jolocom-sdk/1.0.0/guides/interaction_flows/#verifiable-credential-issuance); [Source 2: Aries Issue Credential Protocol](https://github.com/hyperledger/aries-rfcs/tree/master/features/0453-issue-credential-v2);

---
### credential-revocation

Notifying a holder that a previously issued credential has been revoked. Note that this revocation is declaratory, not constitutive, so the actual revocation has to be done elsewhere (e.g. in the backend of the issuer).

#### Roles
- <u>**Issuer**</u>: Agent who revokes the credential
- <u>**Holder**</u>: Agent who holds the credential to be revoked

#### Messages

#### revocation
The <u>issuer</u> sends the `revocation` to the <u>holder</u>, notifying him of the revocation. The most important field here is `credentialId`, which specifies the credential that has been revoked.

###### Layout

```JSON
revocation: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "credentialId", // REQUIRED!
    "callbackURL", // OPTIONAL!
    "responseRequested", //OPTIONAL!
    "id", // OPTIONAL!
    "comment", // OPTIONAL!
    "timing" // OPTIONAL!
}
```


###### Example(s)

```JSON
{
    "context": "credential-revocation/1.0/revocation",
    "thread": "f7771b285a971ba25d66dbe2d82f0bf5f956f4fe548bdf8617c3f24ebc10ed8c",
    "credentialId": "credential-69420-delicious-lasagna",
    "callbackURL": "https://www.aliceswonderland.com/",
    "id": "did:iota:42edacef81828010b314b96c0915780f206341e0ce8892a1b56678c174eef242",
    "comment": "Revoked because your Lasagna isn't actually that good."
}
```

[Source 1: Aries Revocation Notification](https://github.com/hyperledger/aries-rfcs/tree/master/features/0183-revocation-notification);

---
### presentation-verification

Proving a set of statements about an identifier.

The credential verification flow is a simple request-response message exchange between the <u>verifier</u> and the <u>prover</u>. The interaction can consist of up to two messages, the first one is OPTIONAL.

#### Roles
- **Verifier**: Agent who requests a set of Verifiable Credentials with associated requirements
- **Prover**: Agent who provides a set of Verifiable Credentials in the form of a Verifiable Presentation attempting to satisfy the request

#### Messages

#### presentationRequest
The <u>verifier</u> requests a set of Verifiable Credentials from the <u>prover</u>. This message is OPTIONAL within this interaction.

###### Layout

```JSON
presentationRequest: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "callbackURL", // REQUIRED!
    "trustedIssuers", // OPTIONAL!
    "responseRequested", //OPTIONAL!
    "id", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "presentation-verification/1.0/presentationRequest",
    "thread": "f7771b285a971ba25d66dbe2d82f0bf5f956f4fe548bdf8617c3f24ebc10ed8c",
    "callbackURL": "https://www.bobsworld.com/",
    "trustedIssuers" : [
        {
            "credentialTypes": [
                "YourCatHasAnAttitudeCredential",
                "YouHaveNiceHairCredential"
            ],
            "supportedIssuers": [
                "did:iota:58c35471071b3dbb97585ee06bb1dd0239ca338629534296cfbb2db6bc857e21"
            ]
        },
        {
            "credentialTypes": [
                "YourLasagnaIsDeliciousCredential"
            ],
            "supportedIssuers": [
                "did:iota:58c35471071b3dbb97585ee06bb1dd0239ca338629534296cfbb2db6bc857e21",
                "did:iota:23f0b94812c402a1dea1c424303b178d01485a5dcf26cf977333f3b629bd90ec"
            ]
        }
    ]
}
```

#### presentationResponse
The <u>holder</u> sends a Verifiable Presentation to the <u>verifier</u> using a `presentationResponse` message.

###### Layout

```JSON
presentationResponse: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "verifiablePresentation", // REQUIRED!
    "callbackURL", // OPTIONAL!
    "responseRequested", //OPTIONAL!
    "id", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "presentation-verification/1.0/presentationResponse",
    "thread": "f7771b285a971ba25d66dbe2d82f0bf5f956f4fe548bdf8617c3f24ebc10ed8c",
    "verifiablePresentation": {...}, // Omitted for brevity
    "callbackURL": "https://www.aliceswonderland.com/"
}
```

[Source 1: Jolocom Credential Verification](https://jolocom.github.io/jolocom-sdk/1.0.0/guides/interaction_flows/#credential-verification);

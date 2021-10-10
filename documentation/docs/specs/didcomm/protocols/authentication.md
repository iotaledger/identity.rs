---
title: Authentication
sidebar_label: Authentication
---

# Authentication

- Version: 0.1
- Status: `IN-PROGRESS`
- Last Updated: 2021-10-08

## Overview
This protocol allows two parties to mutually authenticate by disclosing and verifying the [DID](https://www.w3.org/TR/did-core/#dfn-decentralized-identifiers) of each other. On successful completion of this protocol, it is expected that [sender authenticated encryption](https://identity.foundation/didcomm-messaging/spec/#sender-authenticated-encryption) may be used between the parties for continuous authentication.

### Relationships

- [Connection](./connection): it is RECOMMENDED to establish [anonymous encryption](https://identity.foundation/didcomm-messaging/spec/#anonymous-encryption) on [connection](./connection) to prevent revealing the DID of either party to eavesdroppers.

### Example Use-Cases
- A connected sensor wants to make sure only valid well known parties connect to it, before allowing access.
- A customer wants to make sure they are actually connecting to the bank, before presenting information.
- An organisation wants to verify the DID of the employer before issuing access credentials. 

### Roles
- Requester: initiates the authentication.
- Responder: responds to the authentication request.

### Interaction

<div style={{textAlign: 'center'}}>

![AuthenticationDiagram](/img/didcomm/authentication.drawio.svg)

</div>


## Messages

### 1. authentication-request {#authentication-request}

- Type: `didcomm:iota/authentication/0.1/authentication-request`
- Role: [requester](#roles)

Sent to initiate the authentication process. This MUST be a [signed DIDComm message](https://identity.foundation/didcomm-messaging/spec/#didcomm-signed-message) to provide some level of trust to the [responder](#roles). However, even when signed it is possible to replay an [authentication-request](#authentication-request), so this message alone is insufficient to prove the DID of the [requester](#roles). In addition to a unique `requesterChallenge`, the `created_time` and `expires_time` [DIDComm message headers](https://identity.foundation/didcomm-messaging/spec/#message-headers) SHOULD be used to mitigate such replay attacks. Note that even a successful replay would only reveal the DID of the responder, authentication of a malicious requester will still fail without access to the original requester's private keys due to the use of challenges.

#### Structure
```json
{
  "did": DID,                   // REQUIRED
  "requesterChallenge": string, // REQUIRED
  "upgradeEncryption": string,  // REQUIRED
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| [`did`](https://www.w3.org/TR/did-core/#dfn-decentralized-identifiers) | [DID](https://www.w3.org/TR/did-core/#dfn-decentralized-identifiers) of the [requester](#roles).[^1]. | ✔ |
| `requesterChallenge` |  A random string unique per [authentication-request](#authentication-request) by a [requester](#roles) to help mitigate replay attacks. | ✔ |
| `upgradeEncryption` | A string indicating whether [sender authenticated encryption](https://identity.foundation/didcomm-messaging/spec/#sender-authenticated-encryption) should be used in the following messages. One of `["required", "optional", "unsupported"]`.[^2] | ✔ |

[^1] The signing key used for the [signed DIDComm envelope](https://identity.foundation/didcomm-messaging/spec/#didcomm-signed-message) wrapping this message MUST be an [authentication method](https://www.w3.org/TR/did-core/#authentication) in the DID document corresponding to `did`, as per the [DIDComm specification](https://identity.foundation/didcomm-messaging/spec/#did-document-keys).

[^2] The `upgradeEncryption` field allows negotiation of whether or not to use [sender authenticated encryption](https://identity.foundation/didcomm-messaging/spec/#sender-authenticated-encryption) for the [authentication](#authentication) protocol and for all messages that follow it. It is RECOMMENDED to specify `"required"` as it offers various guarantees of continuous authentication and payload integrity for every message. The available options are:
- `"required"`: the [responder](#roles) MUST initiate [sender authenticated encryption](https://identity.foundation/didcomm-messaging/spec/#sender-authenticated-encryption), from the following [authentication-response](#authentication-response) message onwards, or send a problem-report.
- `"optional"`: the [responder](#roles) chooses whether or not to use [sender authenticated encryption](https://identity.foundation/didcomm-messaging/spec/#sender-authenticated-encryption).
- `"unsupported"`: the [responder](#roles) MUST NOT use [sender authenticated encryption](https://identity.foundation/didcomm-messaging/spec/#sender-authenticated-encryption). A [responder] MAY reject [authentication-requests](#authentication-request) that do not support encryption.
Any other value for `upgradeEncryption` is invalid and should result in an invalid-request problem-report.

#### Examples

1. Request payload requiring encryption:

```json
{
  "did": "did:iota:9rK6DPF46MCEzgfLD8AHFsaTuMqvmRo6kbXfjqQJPJmC",                   
  "requesterChallenge": "81285532-b72a-4a99-a9bd-b470475bc24f",                     
  "upgradeEncryption": "required",
}
```

2. Request payload with optional encryption:

```json
{
  "did": "did:iota:9rK6DPF46MCEzgfLD8AHFsaTuMqvmRo6kbXfjqQJPJmC",                   
  "requesterChallenge": "fd9965cc-bc3f-42e7-8fdf-933b9a3a6138",                   
  "upgradeEncryption": "optional",
}
```

3. Full DIDComm message with header fields and signature:

```json
{
  TBD
}
```


### 2. authentication-response {#authentication-response}

- Type: `didcomm:iota/authentication/0.1/authentication-response`
- Role: [responder](#roles)

Sent in response to an [authentication-request](#authentication-request), proving the DID of the [responder](#roles). Optionally establishes [sender authenticated encryption](https://identity.foundation/didcomm-messaging/spec/#sender-authenticated-encryption) based on the value of `upgradeEncryption` in the preceding [authentication-request](#authentication-request). If `upgradeEncryption` was `"required"` and this message is not encrypted, or `"unsupported"` and this message is encrypted, the [requester](#roles) MUST issue a problem-report and abort the authentication.

This message MUST be a [signed DIDComm message](https://identity.foundation/didcomm-messaging/spec/#didcomm-signed-message), even if [sender authenticated encryption](https://identity.foundation/didcomm-messaging/spec/#sender-authenticated-encryption) is used. This is to ensure an [authentication key](https://www.w3.org/TR/did-core/#authentication) is used to sign the challenge, in accordance with the [DID specification](https://www.w3.org/TR/did-core/#authentication), and because the [keyAgreement](https://www.w3.org/TR/did-core/#key-agreement) keys used for [sender authenticated encryption](https://identity.foundation/didcomm-messaging/spec/#sender-authenticated-encryption) may be less tightly controlled.

#### Structure
```json
{
  "did": DID,                   // REQUIRED
  "requesterChallenge": string, // REQUIRED
  "responderChallenge": string, // REQUIRED
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| [`did`](https://www.w3.org/TR/did-core/#dfn-decentralized-identifiers) | [DID](https://www.w3.org/TR/did-core/#dfn-decentralized-identifiers) of the [responder](#roles).[^1]. | ✔ |
| `requesterChallenge` | Must match the `requesterChallenge` in the preceding [authentication-request](#authentication-request). | ✔ |
| `responderChallenge` | A random string unique per [authentication-response](#authentication-response) by a [responder](#roles) to help mitigate replay attacks. | ✔ |

[^1] The signing key used for the [signed DIDComm envelope](https://identity.foundation/didcomm-messaging/spec/#didcomm-signed-message) wrapping this message MUST be an [authentication method](https://www.w3.org/TR/did-core/#authentication) in the DID document corresponding to the `did`, as per the [DIDComm specification](https://identity.foundation/didcomm-messaging/spec/#did-document-keys).


#### Examples

1. Responder presenting their DID and offering a challenge to the the Requester:

```json
{
  "did": "did:iota:8cU6DPF56MDEugfLF8AHFaaTuMQvmRo6kbxfjqQJpJmC",
  "requesterChallenge": "81285532-b72a-4a99-a9bd-b470475bc24f",
  "responderChallenge": "b1f0dc02-85a3-4438-8786-b625f11f1be4",
}
```

2. Full signed DIDComm message:

```json
{
  TBD
}
```

3. Full encrypted DIDComm message:

```json
{
  TBD
}
```

### 3. authentication-result {#authentication-result}

- Type: `didcomm:iota/authentication/0.1/authentication-result`
- Role: [requester](#roles)

This message finalises the mutual authentication, proving control over the DID of the [requester](#roles) to the [responder](#roles). Similar to [authentication-response](#authentication-response), this message MUST be a [signed DIDComm message](https://identity.foundation/didcomm-messaging/spec/#didcomm-signed-message).

This MUST or MUST NOT use [sender authenticated encryption](https://identity.foundation/didcomm-messaging/spec/#sender-authenticated-encryption) depending on the outcome of the `upgradeEncryption` negotiation in the preceding [authentication-response](#authentication-response) message, otherwise resulting in a problem-report and failure of the authentication protocol. For example, if `upgradeEncryption` was `optional` and the [authentication-response](#authentication-response) used [sender authenticated encryption](https://identity.foundation/didcomm-messaging/spec/#sender-authenticated-encryption), then the [authentication-result](#authentication-result) MUST be encrypted to be valid. 

#### Structure
```json
{
  "responderChallenge": string // REQUIRED
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `responderChallenge` | Must match the `challenge` in the preceding [authentication-response](#authentication-response).[^1] | ✔ |

[^1] The signing key used for the [signed DIDComm envelope](https://identity.foundation/didcomm-messaging/spec/#didcomm-signed-message) wrapping this message MUST be an [authentication method](https://www.w3.org/TR/did-core/#authentication) in the DID document corresponding to the `did` of the [requester](#roles) in the [authentication-request](#authentication-request), as per the [DIDComm specification](https://identity.foundation/didcomm-messaging/spec/#did-document-keys).

#### Examples

1. Requester responding with the responders challenge from the previous message:

```json
{
  "responderChallenge": "0768e82d-f498-4f38-8686-918325f9560d"
}
```

2. Full signed DIDComm message:

```json
{
  TBD
}
```

3. Full encrypted DIDComm message:

```json
{
  TBD
}
```

### Problem Reports

See: https://identity.foundation/didcomm-messaging/spec/#descriptors
TODO

For general guidance see [problem reports](../resources/problem-reports).

Custom error messages for problem-reports that are expected in the course of this protocol. Non-exhaustive, just a normative list of errors that are expected to be thrown.
- e.p.prot.iota.authencation.reject-message
- e.p.prot.iota.authencation.untrusted-identity
- e.p.prot.iota.authencation.encyption-required
- e.p.prot.iota.authencation.encyption-unsupported
- e.p.prot.iota.authencation.invalid-message

Also problem reports from embedded protocols can be thrown.

## Considerations

This section is non-normative.

- **Trust**: this [authentication](#authentication) protocol only verifies that both parties have access to the necessary private keys (which could become compromised) associated with their DID documents; establishing and verifying their real-world identities may require additional interactions. For instance, requesting a verifiable presentation of credentials issued by a trusted third-party (like a government) is one possible way to verify a real-world identity.
- **Authorisation**: the permissions and capabalities of either party may still need to be established after [authentication](#authentication), either by verifiable presentation as above or other methods such as JWT tokens
- **Privacy**: the [responder](#roles) may be subject to probing whereby their DID may be revealed even with the use of [sender authenticated encryption](https://identity.foundation/didcomm-messaging/spec/#sender-authenticated-encryption), as the `skid` message header is linked to their DID. This is possible if the [responder](#roles) chooses to accept the [authentication-request](#authentication-request) of an unknown [requester](#roles), or the [requester](#roles) succesfully replays an [authentication-request](#authentication-request) from a DID the [requester](#roles) trusts.

## Unresolved Questions

- Enforce signed DIDComm messages on top of sender-authenticated encryption? Usually unnecessary and DIDComm recommends against this since it's redundent and due to non-repudiation may decrease security and privacy by allowing participants to prove to third parties that authentication occurred.
  - https://identity.foundation/didcomm-messaging/spec/#didcomm-signed-message
  - https://github.com/hyperledger/aries-rfcs/blob/master/concepts/0049-repudiation/README.md#summary

- How to protect the DID of the responder (`skid` field)?
  - https://github.com/decentralized-identity/didcomm-messaging/issues/197
  - https://github.com/decentralized-identity/didcomm-messaging/issues/219

- Man-in-the-middle attacks?
   - note possible attack vectors for the requester and responder, including intercepting or modifying the invitation in the connection protocol.
   - TODO: links to discussions

## Related Work

- Aries Hyperledger:
  - DID Exchange protocol: https://github.com/hyperledger/aries-rfcs/tree/main/features/0023-did-exchange
  - DIDAuthZ: https://github.com/hyperledger/aries-rfcs/tree/main/features/0309-didauthz
- Jolocom: https://jolocom.github.io/jolocom-sdk/1.0.0/guides/interaction_flows/#authentication

## Further Reading

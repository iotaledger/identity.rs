---
title: Authentication
sidebar_label: Authentication
---

# Authentication

- Version: 0.1
- Status: `IN-PROGRESS`
- Last Updated: 2021-10-06

## Overview
This protocol allows two parties to mutually authenticate, verifying the DID of each other. On completion of this protocol, it is expected that [sender authenticated encryption](https://identity.foundation/didcomm-messaging/spec/#sender-authenticated-encryption) will be used between the parties for continuous authentication.

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

Sent to start the authentication process. This MUST be a [signed DIDComm message](https://identity.foundation/didcomm-messaging/spec/#didcomm-signed-message) to provide some level of trust to the [responder](#roles). However, it is possible to replay [authentication-request](#authentication-request) messages so this alone is insufficient to prove the DID of the [requester](#roles).

#### Structure
```json
{
  "did": DID,           // REQUIRED
  "challenge": String,  // REQUIRED
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| [`did`](https://www.w3.org/TR/did-core/#dfn-decentralized-identifiers) | [DID](https://www.w3.org/TR/did-core/#dfn-decentralized-identifiers) of the [requester](#roles).[^1]. | ✔ |

[^1] The signing key used for the [signed DIDComm envelope](TODO) wrapping this message MUST be authentication section of the DID document corresponding to the `did` used in the [authentication-request](#authentication-request) MUST match the key used for the signature of the [signed DIDComm envelope](https://identity.foundation/didcomm-messaging/spec/#didcomm-signed-message). 

#### Examples

1. Requester presenting their DID and offering a challenge to the the Responder:

```json
{
  TBD
}
```

### 2. authentication-response {#authentication-response}

- Type: `didcomm:iota/authentication/0.1/authentication-response`
- Role: [responder](#roles)

This message is send as a response to a [authentication request](#authentication-request) after checking the validity of the request.

#### Structure  
```json
{
  "did": DID,                 // REQUIRED
  "challenge": String,        // REQUIRED
  "challengeResponse": String // REQUIRED
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| [`subject`](https://www.w3.org/TR/vc-data-model/#credential-subject-0) | [DID](https://www.w3.org/TR/did-core/#dfn-decentralized-identifiers) of the [credential subject](https://www.w3.org/TR/vc-data-model/#credential-subject-0)[^1]. | ✔ |


#### Examples

1. Responder presenting their DID and offering a challenge to the the Requester:

```json
{
  TBD
}
```

### 3. authentication-result {#authentication-result}

- Type: `didcomm:iota/authentication/0.1/authentication-result`
- Role: [requester](#roles)

This message finalises the mutual authentication. 

#### Structure
```json
{
  "did": DID,                 // REQUIRED
  "challengeResponse": String // REQUIRED
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| [`subject`](https://www.w3.org/TR/vc-data-model/#credential-subject-0) | [DID](https://www.w3.org/TR/did-core/#dfn-decentralized-identifiers) of the [credential subject](https://www.w3.org/TR/vc-data-model/#credential-subject-0)[^1]. | ✔ |


#### Examples

1. Responder presenting their DID and offering a challenge to the the Requester:

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
- TBD

Also problem reports from embedded protocols can be thrown.

## Considerations

This section is non-normative.

- **Trust**: TODO - only verifies that the other party has access to a private key corresponding to an authentication section of their DID or establishing verifying their real-world identity is still a problem - requesting a verifiable presentation (credentials) is one possible solution if you have trusted issuers.
- **Authorisation**: TODO - similar to trust, the capabilities of either party still need to be established, either by verifiable presentation as above or other methods such as JWT tokens etc.
- **Security**: TODO - subject to probing if we use sender-authentication encryption?
- **Man-in-the-Middle**: TODO - note possible attack vectors for the requester and responder, including intercepting or modifying the invitation in the connection protocol.

## Unresolved Questions

1. Make sender-authenticated encryption optional or negotiated (by requester or responder)?
2. In which messages will negotiation take place (in authentication protocol or subsequent "upgrade" flow)
3. Explicit proofs/signatures in payload vs signed messages (leaning towards signed) using authentication methods

## Related Work

- Aries Hyperledger:
  - DID Exchange protocol: https://github.com/hyperledger/aries-rfcs/tree/main/features/0023-did-exchange
  - DIDAuthZ: https://github.com/hyperledger/aries-rfcs/tree/main/features/0309-didauthz
- Jolocom: https://jolocom.github.io/jolocom-sdk/1.0.0/guides/interaction_flows/#authentication

## Further Reading

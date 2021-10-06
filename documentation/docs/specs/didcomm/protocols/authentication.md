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


### Roles
- Requester: initiates the authentication.
- Responder: responds to the authentication request.

### Interaction

<div style={{textAlign: 'center'}}>
TODO
![IssuanceDiagram](/img/didcomm/issuance.drawio.svg)

</div>


## Messages

### 1. did-challenge {#did-challenge}

- Type: `didcomm:iota/authentication/0.1/did-challenge`
- Role: [requester](#roles)

TBD

#### Structure
```json
{
  "did": DID,           // REQUIRED
  "challenge": String,  // REQUIRED
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| [`subject`](https://www.w3.org/TR/vc-data-model/#credential-subject-0) | [DID](https://www.w3.org/TR/did-core/#dfn-decentralized-identifiers) of the [credential subject](https://www.w3.org/TR/vc-data-model/#credential-subject-0)[^1]. | ✔ |


#### Examples

1. Requester presenting their DID and offering a challenge to the the Responder:

```json
{
  TBD
}
```

### 2. did-response {#did-response}

- Type: `didcomm:iota/authentication/0.1/did-response`
- Role: [responder](#roles)

TBD

#### Structure
```json
{
  "did": DID,           // REQUIRED
  "challenge": String,  // REQUIRED
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

### 3. did-connection {#did-connection}

- Type: `didcomm:iota/authentication/0.1/did-connection`
- Role: [requester](#roles)

TBD

#### Structure
```json
{
  "did": DID,  // REQUIRED
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

## Related Work


## Further Reading

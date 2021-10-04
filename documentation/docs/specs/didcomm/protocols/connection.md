---
title: Connection
sidebar_label: Connection
---

# Connection

- Version: 0.1
- Status: `IN-PROGRESS`
- Last Updated: 2021-10-01

## Overview

Allows establishment of a [DIDComm connection](https://identity.foundation/didcomm-messaging/spec/#connections) between two parties. The connection may be established by an explicit invitation delivered [out-of-band](https://github.com/decentralized-identity/didcomm-messaging/blob/49935b7b119591a009ce61d044ba9ad6fa40c7b7/docs/spec-files/out_of_band.md)&mdash;such as a QR code, URL, or email&mdash;or by following an implicit invitation in the form of a [service endpoint](https://identity.foundation/didcomm-messaging/spec/#did-document-service-endpoint) attached to a public DID Document.

### Relationships
- [Termination](./termination): the DIDComm connection may be gracefully concluded using the [termination protocol](./termination).

### Example Use-Cases

- TBD

### Roles
- Inviter: offers methods to establish connections.
- Invitee: may connect to the inviter using offered methods.

### Interaction

<div style={{textAlign: 'center'}}>

![ConnectionDiagram](/img/didcomm/connection.drawio.svg)

</div>


## Messages

### 1. invitation {#invitation}

- Type: `https://didcomm.org/out-of-band/2.0/invitation`
- Role: [inviter](#roles)

A message containing information on how to connect to the inviter. This message is delivered out-of-band, e.g. in form of a link or QR code. The message contains all information required to establish a connection. 

#### Structure

The general structure of the invitation message is described in the [Out Of Band Messages in the DIDComm specification](https://github.com/decentralized-identity/didcomm-messaging/blob/49935b7b119591a009ce61d044ba9ad6fa40c7b7/docs/spec-files/out_of_band.md).

The actual invitation is contained in the `attachments` field in the message, which is structured as following:


```json
{
  "serviceId": DIDUrl,                  // OPTIONAL
  "service": {                          
    "serviceEndpoint": string, // REQUIRED
    "accept": [string],                 // OPTIONAL
    "recipientKeys": [DIDUrl | KeyDID], // OPTIONAL
    "routingKeys": [DIDUrl | KeyDID]    // OPTIONAL
  },
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `serviceId` | A string representing a [DIDUrl](https://www.w3.org/TR/did-core/#did-url-syntax) referencing a resolvable [service](https://www.w3.org/TR/did-core/#services).[^1] [^2] | ✖ |
| [`service`](https://www.w3.org/TR/did-core/#services) | A structure analogous to the [DID service specification](https://www.w3.org/TR/did-core/#services), including all information necessary to establish a connection with the [inviter](#roles).[^1] | ✖ |
| [`serviceEndpoint`](https://www.w3.org/TR/did-core/#dfn-serviceendpoint) | A [URI](https://www.rfc-editor.org/rfc/rfc3986) including all details needed to connect to the [inviter](#roles). | ✔ |
| [`accept`](https://identity.foundation/didcomm-messaging/spec/#did-document-service-endpoint) | An optional array of [DIDComm profiles](https://identity.foundation/didcomm-messaging/spec/#defined-profiles) in the order of preference for sending a message to the endpoint. If omitted, defer to the `accept` field of the invitation body. | ✖ |
| `recipientKeys` | An ordered array of [DIDUrls](https://www.w3.org/TR/did-core/#did-url-syntax) or [`did:key`](https://w3c-ccg.github.io/did-method-key/) strings referencing keys to be used for [anonymous encryption](https://identity.foundation/didcomm-messaging/spec/#anonymous-encryption).[^3] [^4] | ✖ |
| [`routingKeys`](https://identity.foundation/didcomm-messaging/spec/#did-document-service-endpoint) | An ordered array of [DIDUrls](https://www.w3.org/TR/did-core/#did-url-syntax) or [`did:key`](https://w3c-ccg.github.io/did-method-key/) strings referencing keys to be used when preparing the message for transmission; see [DIDComm Routing](https://identity.foundation/didcomm-messaging/spec/#routing).[^4] | ✖ |

[^1] One of `serviceId` or `service` MUST be present for the [invitee](#roles) to be able to connect. If both fields are present, the [invitee](#roles) SHOULD default to the `serviceId`.

[^2] A public `serviceId` may reveal the identity of the [inviter](#roles) to anyone able to view the invitation, if privacy is a concern using an inline `service` should be preferred. For a public organisation whose DID is already public knowledge a `serviceId` has several benefits: it establishes trust that the [invitee](#roles) is connecting to the correct party since a service from their public DID document is used, and the invitation may be re-used indefinitely even if the service referenced is updated with different endpoints. It is RECOMMENDED that the service referenced by `serviceId` conforms to the ["DIDCommMessaging" service type from the DIDComm specification](https://identity.foundation/didcomm-messaging/spec/#did-document-service-endpoint) as it allows `routingKeys` to be included if necessary. The DID document referenced by `serviceId` SHOULD include one or more [`keyAgreement`](https://www.w3.org/TR/did-core/#key-agreement) sections to use for [anonymous encryption](https://identity.foundation/didcomm-messaging/spec/#anonymous-encryption); the absence of any [`keyAgreement`](https://www.w3.org/TR/did-core/#key-agreement) section implies no [anonymous encryption](https://identity.foundation/didcomm-messaging/spec/#anonymous-encryption) will be used for the connection and an [invitee](#roles) may choose to reject such an invitation.

[^3] Omitting `recipientKeys` implies that [anonymous encryption](https://identity.foundation/didcomm-messaging/spec/#anonymous-encryption) will not be used in the ensuing DIDComm connection. It is RECOMMENDED to include as [anonymous encryption](https://identity.foundation/didcomm-messaging/spec/#anonymous-encryption) ensures message integrity and protects communications from eavesdroppers over insecure channels. [Invitees](#roles) may choose to reject invitations that do not include `recipientKeys` if they want to enforce [anonymous encryption](https://identity.foundation/didcomm-messaging/spec/#anonymous-encryption).

[^4] Implementors should avoid using a `DIDUrl` for the `recipientKeys` or `routingKeys` if privacy is a concern, as may reveal the identity of the [inviter](#roles) to any party other than the [invitee](#roles) that intercepts the invitation. However, using a `DIDUrl` may be useful as it allows for key-rotation without needing to update the invitation.

#### Examples

1. Full invitation message with two attachments:
```json
{
  "typ": "application/didcomm-plain+json",
  "type": "https://didcomm.org/out-of-band/%VER/invitation",
  "id": "<id used for context as pthid>",
  "body": {
    "goal_code": "issue-vc",
    "goal": "To issue a Faber College Graduate credential",
    "accept": [
      "didcomm/v2"
    ],
  },
  "attachments": [
    {
      "@id": "request-0",
      "mime-type": "application/json",
      "data": {
          "json": {
            "service": {
              "id": "service-1",
              "type": "DIDCommMessaging",
              "serviceEndpoint": "http://example.com/path",
              "accept": [
                "didcomm/v2",
              ],
              "recipientKeys": ["9hFgmPVfmBZwRvFEyniQDBkz9LmV7gDEqytWyGZLmDXE"],
              "routingKeys": ["9hFgmPVfmBZwRvFEyniQDBkz9LmV7gDEqytWyGZLmDXE"]
            }
          }
      }
    },
    {
      "@id": "request-1",
      "mime-type": "application/json",
      "data": {
          "json": {
            "serviceId": "did:iota:123456789abcdefghi#didcomm-1",
          }
      }
    }
  ]
}
```

2. An invitation with a reference to a service in a DID document encoded in Base 64:
TODO
`eyJ0eXAiOiJhcHBsaWNhdGlvbi9kaWRjb21tLXBsYWluK2pzb24iLCJ0eXBlIjoiaHR0cHM6Ly9kaWRjb21tLm9yZy9vdXQtb2YtYmFuZC8wLjEvaW52aXRhdGlvbiIsImlkIjoiNjkyMTJhM2EtZDA2OC00ZjlkLWEyZGQtNDc0MWJjYTg5YWYzIiwiZnJvbSI6ImRpZDpleGFtcGxlOmFsaWNlIiwiYm9keSI6eyJnb2FsX2NvZGUiOiIiLCJnb2FsIjoiIn0sImF0dGFjaG1lbnRzIjpbeyJAaWQiOiJyZXF1ZXN0LTAiLCJtaW1lLXR5cGUiOiJhcHBsaWNhdGlvbi9qc29uIiwiZGF0YSI6eyJqc29uIjoiPGpzb24gb2YgcHJvdG9jb2wgbWVzc2FnZT4ifX1dfQ`

3. The invitation from Example 2. attached to a URL:
TODO
`http://example.com/path?_oob=eyJ0eXAiOiJhcHBsaWNhdGlvbi9kaWRjb21tLXBsYWluK2pzb24iLCJ0eXBlIjoiaHR0cHM6Ly9kaWRjb21tLm9yZy9vdXQtb2YtYmFuZC8wLjEvaW52aXRhdGlvbiIsImlkIjoiNjkyMTJhM2EtZDA2OC00ZjlkLWEyZGQtNDc0MWJjYTg5YWYzIiwiZnJvbSI6ImRpZDpleGFtcGxlOmFsaWNlIiwiYm9keSI6eyJnb2FsX2NvZGUiOiIiLCJnb2FsIjoiIn0sImF0dGFjaG1lbnRzIjpbeyJAaWQiOiJyZXF1ZXN0LTAiLCJtaW1lLXR5cGUiOiJhcHBsaWNhdGlvbi9qc29uIiwiZGF0YSI6eyJqc29uIjoiPGpzb24gb2YgcHJvdG9jb2wgbWVzc2FnZT4ifX1dfQ==`

### 2. connection {#connection}

- Type: `didcomm:iota/connection/0.1/connection`
- Role: [invitee](#roles)

Following a successful connection, the [invitee](#roles) sends its public keys necessary to establish anonymous encryption.

#### Structure
```json
{
  "recipientKeys": [string], // OPTIONAL
  "routingKeys": [string],   // OPTIONAL
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `recipientKeys` | TBD | ✖ |
| `routingKeys` | TBD | ✖ |

#### Examples

1. Connection with no :

```json
{}
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

TBD

- Implementors SHOULD NOT use any information transmitted in the connection protocol for authentication or proof of identity.

## Related Work

- Aries Hyperledger:
  - Connection protocol: https://github.com/hyperledger/aries-rfcs/tree/main/features/0160-connection-protocol
  - Out-of-Band protocol: https://github.com/hyperledger/aries-rfcs/tree/main/features/0434-outofband
  - DID Exchange protocol: https://github.com/hyperledger/aries-rfcs/tree/main/features/0023-did-exchange

## Further Reading

- [DIDComm Connections](https://identity.foundation/didcomm-messaging/spec/#connections)
- [DIDComm Out Of Band Messages](https://github.com/decentralized-identity/didcomm-messaging/blob/49935b7b119591a009ce61d044ba9ad6fa40c7b7/docs/spec-files/out_of_band.md)
- [DIDComm Service Endpoint](https://github.com/decentralized-identity/didcomm-messaging/blob/49935b7b119591a009ce61d044ba9ad6fa40c7b7/docs/spec-files/routing.md#did-document-service-endpoint)
- [DID Services](https://www.w3.org/TR/did-core/#services)
- [Aries Hyperledger Goal Codes](https://github.com/hyperledger/aries-rfcs/tree/main/concepts/0519-goal-codes)

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

A message containing information on how to connect to the inviter. This message is delivered out-of-band, e.g. in form of a link or a qr-code. THe message contains all information requires to estblish a connection. 

#### Structure

The general structure of the invitation message is decribed in the [Out Of Band Messages in the DIDComm specification](https://github.com/decentralized-identity/didcomm-messaging/blob/49935b7b119591a009ce61d044ba9ad6fa40c7b7/docs/spec-files/out_of_band.md).

The actual in invitation is contained in the `attachments` field in the message. The structure of the message described by [DID Document Service Endpoint in the DIDComm specification](https://identity.foundation/didcomm-messaging/spec/#did-document-service-endpoint).

The connection information, like which address or channel to connect to, is embedded in the `serviceEndpoint` field, following the [DID Specification](https://www.w3.org/TR/did-core/#service-properties).


#### Examples

1. Full Invitation from https://didcomm.org/out-of-band/%VER/invitation:
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

            "type": "DIDCommMessaging",
            "serviceEndpoint": "http://example.com/path",
            "accept": [
              "didcomm/v2",
            ],
            "routingKeys": ["did:example:somemediator#somekey"]
          }
      }
    },  {
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

2. Whitespace removed:
```json

{"typ":"application/didcomm-plain+json","type":"https://didcomm.org/out-of-band/0.1/invitation","id":"69212a3a-d068-4f9d-a2dd-4741bca89af3","from":"did:example:alice","body":{"goal_code":"","goal":""},"attachments":[{"@id":"request-0","mime-type":"application/json","data":{"json":"<json of protocol message>"}}]}
```
3. Base 64 URL Encoded:
`eyJ0eXAiOiJhcHBsaWNhdGlvbi9kaWRjb21tLXBsYWluK2pzb24iLCJ0eXBlIjoiaHR0cHM6Ly9kaWRjb21tLm9yZy9vdXQtb2YtYmFuZC8wLjEvaW52aXRhdGlvbiIsImlkIjoiNjkyMTJhM2EtZDA2OC00ZjlkLWEyZGQtNDc0MWJjYTg5YWYzIiwiZnJvbSI6ImRpZDpleGFtcGxlOmFsaWNlIiwiYm9keSI6eyJnb2FsX2NvZGUiOiIiLCJnb2FsIjoiIn0sImF0dGFjaG1lbnRzIjpbeyJAaWQiOiJyZXF1ZXN0LTAiLCJtaW1lLXR5cGUiOiJhcHBsaWNhdGlvbi9qc29uIiwiZGF0YSI6eyJqc29uIjoiPGpzb24gb2YgcHJvdG9jb2wgbWVzc2FnZT4ifX1dfQ`

4. URL: 
`http://example.com/path?_oob=eyJ0eXAiOiJhcHBsaWNhdGlvbi9kaWRjb21tLXBsYWluK2pzb24iLCJ0eXBlIjoiaHR0cHM6Ly9kaWRjb21tLm9yZy9vdXQtb2YtYmFuZC8wLjEvaW52aXRhdGlvbiIsImlkIjoiNjkyMTJhM2EtZDA2OC00ZjlkLWEyZGQtNDc0MWJjYTg5YWYzIiwiZnJvbSI6ImRpZDpleGFtcGxlOmFsaWNlIiwiYm9keSI6eyJnb2FsX2NvZGUiOiIiLCJnb2FsIjoiIn0sImF0dGFjaG1lbnRzIjpbeyJAaWQiOiJyZXF1ZXN0LTAiLCJtaW1lLXR5cGUiOiJhcHBsaWNhdGlvbi9qc29uIiwiZGF0YSI6eyJqc29uIjoiPGpzb24gb2YgcHJvdG9jb2wgbWVzc2FnZT4ifX1dfQ==`


### Problem Reports

See: https://identity.foundation/didcomm-messaging/spec/#descriptors
TODO

For gerneral guidance see [problem reports](../resources/problem-reports).

Custom error messages for problem-reports that are expected in the course of this protocol. Non-exhaustive, just a normative list of errors that are expected to be thrown.
- TBD

Also problem reports from embedded protocols can be thrown.

## Considerations

This section is non-normative.

TBD

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

---
title: Connection
sidebar_label: Connection
---

:::info

The IOTA DIDComm Specification is in the RFC phase and may undergo changes. Suggestions are welcome at [GitHub #464](https://github.com/iotaledger/identity.rs/discussions/464).

:::

- Version: 0.1
- Status: `IN-PROGRESS`
- Last Updated: 2021-10-29

## Overview

Allows establishment of a [DIDComm connection](https://identity.foundation/didcomm-messaging/spec/#connections) between two parties. The connection may be established by an explicit invitation delivered [out-of-band][OUT_OF_BAND]&mdash;such as a QR code, URL, or email&mdash;or by following an implicit invitation in the form of a [service endpoint][DIDCOMM_SERVICE_ENDPOINT] attached to a public DID document.

### Relationships
- [Termination](./termination.md): the DIDComm connection may be gracefully concluded using the [termination protocol](./termination.md).
- [Authentication](./authentication.md): the authentication protocol can be used to authenticate parties participating in the established [connection](./connection.md).
- [Feature Discovery](https://github.com/decentralized-identity/didcomm-messaging/blob/ef997c9d3cd1cd24eb182ffa2930a095d3b856a9/docs/spec-files/feature_discovery.md): feature discovery can be used to learn about the capabilities of the other party after connection.

### Example Use-Cases

- A corporation offers a QR code on their website for customers to connect to their services.
- A person sends an invitation as an email to a friend, to exchange information.
- A device has a service embedded in their DID, that allows others to connect to it, in order to read data.

### Roles
- Inviter: offers methods to establish connections.
- Invitee: may connect to the inviter using offered methods.

### Interaction

![ConnectionDiagram](/img/didcomm/connection.drawio.svg)

<div style={{textAlign: 'center'}}>

<sub>For guidance on diagrams see the <a href="../../overview#diagrams">corresponding section in the overview</a>.</sub>

</div>


## Messages

### 1. invitation {#invitation}

- Type: `https://didcomm.org/out-of-band/2.0/invitation`
- Role: [inviter](#roles)

A message containing information on how to connect to the inviter. This message is delivered out-of-band, e.g. in form of a link or QR code. The message contains all information required to establish a DIDComm connection. 

#### Structure

The general structure of the invitation message is described in the [Out Of Band Messages of the DIDComm specification][OUT_OF_BAND]. Note that the invitation message may be [signed](https://identity.foundation/didcomm-messaging/spec/#didcomm-signed-message) to provide [tamper resistance](https://identity.foundation/didcomm-messaging/spec/#tamper-resistant-oob-messages).

The actual invitation is contained in the `attachments` field in the message, which is structured as follows:

```json
{
  "serviceId": DIDUrl,                  // OPTIONAL
  "service": {                          
    "serviceEndpoint": string,          // REQUIRED
    "accept": [string],                 // OPTIONAL
    "recipientKeys": [DIDUrl | DIDKey], // OPTIONAL
    "routingKeys": [DIDUrl | DIDKey],   // OPTIONAL
  }, // OPTIONAL
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `serviceId` | A string representing a [DIDUrl][DIDURL] referencing a resolvable [service][SERVICE].[^1] [^2] | ✖ |
| [`service`][SERVICE] | A structure analogous to the [DID service specification][SERVICE], including all information necessary to establish a connection with the [inviter](#roles).[^1] | ✖ |
| [`serviceEndpoint`](https://www.w3.org/TR/did-core/#dfn-serviceendpoint) | A [URI](https://www.rfc-editor.org/rfc/rfc3986) including all details needed to connect to the [inviter](#roles). | ✔ |
| [`accept`][DIDCOMM_SERVICE_ENDPOINT] | An optional array of [DIDComm profiles](https://identity.foundation/didcomm-messaging/spec/#defined-profiles) in the order of preference for sending a message to the endpoint. If omitted, defer to the `accept` field of the invitation body. | ✖ |
| `recipientKeys` | An ordered array of [DIDUrls][DIDURL] or [`did:key`][DID_KEY] strings referencing public keys, any of which may be used for [anonymous encryption][ANONCRYPT].[^3] [^4] | ✖ |
| [`routingKeys`][DIDCOMM_SERVICE_ENDPOINT] | An ordered array of [DIDUrls][DIDURL] or [`did:key`][DID_KEY] strings referencing keys to be used when preparing the message for transmission; see [DIDComm Routing](https://identity.foundation/didcomm-messaging/spec/#routing).[^4] | ✖ |

[^1] One of `serviceId` or `service` MUST be present for the [invitee](#roles) to be able to connect. If both fields are present, the [invitee](#roles) SHOULD default to the `serviceId`.

[^2] It is RECOMMENDED that the service referenced by `serviceId` conforms to the ["DIDCommMessaging" service type from the DIDComm specification][DIDCOMM_SERVICE_ENDPOINT] as it allows `routingKeys` to be included if necessary. The DID document referenced by `serviceId` SHOULD include one or more [`keyAgreement`](https://www.w3.org/TR/did-core/#key-agreement) sections to use for [anonymous encryption][ANONCRYPT]; the absence of any [`keyAgreement`](https://www.w3.org/TR/did-core/#key-agreement) section implies no [anonymous encryption][ANONCRYPT] will be used for the connection and an [invitee](#roles) may choose to reject such an invitation. A public `serviceId` may reveal the identity of the [inviter](#roles) to anyone able to view the invitation; if privacy is a concern using an inline `service` should be preferred. For a public organisation whose DID is already public knowledge, using `serviceId` has a few benefits: it establishes some level of trust that the [invitee](#roles) may be connecting to the correct party since a service from their public DID document is used, and the invitation may be re-used indefinitely even if the service referenced is updated with different endpoints. When using `service` instead of `serviceId`, a signed invitation may provide a similar level of trust. However, neither should be used as a complete replacement for interactive authentication due to the risk of man-in-the-middle attacks.

[^3] Note that `recipientKeys` may have multiple entries in order of preference of the [inviter](#roles); this is to offer multiple key types (e.g. Ed25519, X25519) and an [invitee](#roles) may choose any key with which they are compatible. These keys may be static or generated once per invitation. Omitting `recipientKeys` implies that [anonymous encryption][ANONCRYPT] will not be used in the ensuing DIDComm connection. It is RECOMMENDED to include as [anonymous encryption][ANONCRYPT] ensures message integrity and protects communications from eavesdroppers over insecure channels. [Invitees](#roles) may choose to reject invitations that do not include `recipientKeys` if they want to enforce [anonymous encryption][ANONCRYPT].

[^4] Implementors should avoid using a `DIDUrl` for the `recipientKeys` or `routingKeys` if privacy is a concern, as may reveal the identity of the [inviter](#roles) to any party other than the [invitee](#roles) that intercepts the invitation. However, using a `DIDUrl` may be useful as it allows for key-rotation without needing to update the invitation.

#### Examples

The following examples include the entire DIDComm message structure for illustration, including [message headers](https://identity.foundation/didcomm-messaging/spec/#message-headers) with the actual [invitation payload](#invitation) defined in this specification included in the [attachments](https://identity.foundation/didcomm-messaging/spec/#attachments) section.

For further information on how to encode the invitation message for delivery refer to the [DIDComm specification](https://identity.foundation/didcomm-messaging/spec/#standard-message-encoding).

1. Invitation with a single attachment:
```json
{
  "typ": "application/didcomm-plain+json",
  "type": "https://didcomm.org/out-of-band/2.0/invitation",
  "id": "fde5eb9e-0560-48cf-b860-acd178c1e0b0",
  "body": {
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
              "serviceEndpoint": "wss://example.com/path",
              "recipientKeys": ["did:key:z6LSoMdmJz2Djah2P4L9taDmtqeJ6wwd2HhKZvNToBmvaczQ"],
              "routingKeys": []
            }
          }
      }
    }
  ]
}
```

Refer to the [DIDComm specification](https://identity.foundation/didcomm-messaging/spec/#standard-message-encoding) for further information on how to encode the invitation message for delivery.

2. Invitation with a goal indicated and two attachments in order of preference. An [invitee](#roles) should pick the first one with which they are compatible:
```json
{
  "typ": "application/didcomm-plain+json",
  "type": "https://didcomm.org/out-of-band/2.0/invitation",
  "id": "fde5eb9e-0560-48cf-b860-acd178c1e0b0",
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
              "serviceEndpoint": "wss://example.com/path",
              "accept": [
                "didcomm/v2",
              ],
              "recipientKeys": [
                "did:key:z6LSoMdmJz2Djah2P4L9taDmtqeJ6wwd2HhKZvNToBmvaczQ",
                "did:key:z82Lm1MpAkeJcix9K8TMiLd5NMAhnwkjjCBeWHXyu3U4oT2MVJJKXkcVBgjGhnLBn2Kaau9"
              ],
              "routingKeys": ["did:key:z6LStiZsmxiK4odS4Sb6JmdRFuJ6e1SYP157gtiCyJKfrYha"]
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

### 2. connection {#connection}

- Type: `iota/connection/0.1/connection`
- Role: [invitee](#roles)

Following a successful connection, the [invitee](#roles) sends its public keys necessary to establish [anonymous encryption][ANONCRYPT]. This may be preceded by an [invitation](#invitation) message, or the [invitee](#roles) may connect directly to the [inviter](#roles) in the case of an implicit invitation.

#### Structure
```json
{
  "recipientKey": DIDUrl | DIDKey,  // OPTIONAL
  "routingKeys": [DIDUrl | DIDKey], // OPTIONAL
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `recipientKey` | A [DIDUrl][DIDURL] or [`did:key`][DID_KEY] strings referencing a public key of the [invitee](#roles) to be used for [anonymous encryption][ANONCRYPT].[^1] [^2] | ✖ |
| [`routingKeys`][DIDCOMM_SERVICE_ENDPOINT] | An ordered array of [DIDUrls][DIDURL] or [`did:key`][DID_KEY] strings referencing keys to be used by the [inviter](#roles) when preparing the message for transmission; see [DIDComm Routing](https://identity.foundation/didcomm-messaging/spec/#routing).[^2] | ✖ |

The `id` of the preceding [invitation](#invitation) message MUST be used as the `pthid` header property on this message. Both the `thid` and `pthid` properties MUST be omitted in the case of an implicit invitation when connecting to a public service endpoint of an [inviter](#roles). See [DIDComm Message Headers](https://identity.foundation/didcomm-messaging/spec/#message-headers) for more information.

[^1] If present, the `recipientKey` sent by the [`invitee`](#roles) MUST match the key type (e.g. Ed25519, X25519) of one of the `recipientKeys` in the [invitation](#invitation) message, or of a `keyAgreement` public key attached to the [inviter`s](#roles) DID document in the case of an implicit invitation. The `recipientKey` should be omitted if no `recipientKeys` or `keyAgreement` sections are present, or if the [invitee](#roles) does not wish to use [anonymous encryption][ANONCRYPT] for the connection. An [inviter](#roles) may choose to reject connection messages that omit a `recipientKey`, terminating the connection.

[^2] Similar to the considerations for the [invitation](#invitation) message, implementors should avoid using a `DIDUrl` for the `recipientKey` or `routingKeys` as it may reveal the identity of the [invitee](#roles) to eavesdroppers prior to encryption being established. Using a `DIDUrl` for key rotation is less of a concern for a [connection](#connection) message as, unlike an [invitation](#invitation), the message is intended to be transient and should not persist beyond a single connection attempt.

#### Examples

1. Connection with a P-384 Key DID as the recipient key:

```json
{
  "recipientKey": "did:key:z82LkvCwHNreneWpsgPEbV3gu1C6NFJEBg4srfJ5gdxEsMGRJUz2sG9FE42shbn2xkZJh54"
}
```

### Problem Reports {#problem-reports}

The following problem-report codes may be raised in the course of this protocol and are expected to be recognised and handled in addition to any general problem-reports. Implementers may also introduce their custom application-specific problem-reports.

For guidance on problem-reports and a list of general codes see [problem reports](../resources/problem-reports.md).

| Code | Message | Description |
| :--- | :--- | :--- |
| `e.p.msg.iota.connection.reject-connection` | [connection](#connection) | [Inviter](#roles) rejects a connection request for any reason, e.g. untrusted [invitee](#roles) or lacking `recipientKey` for anonymous encryption. |

## Considerations

This section is non-normative.

- **Authentication**: implementors SHOULD NOT use any information transmitted in the connection protocol for direct authentication or proof of identity. See the [authentication](./authentication.md) protocol.

## Unresolved Questions

- List supported handshake protocols for authentication post-connection?
- How do parties know what to do post-connection, send protocol in the invitation, or does one party just try to start a protocol immediately? For custom/corporate applications likely hard-coded, for general SSI wallets, it is an open question.

## Related Work

- Aries Hyperledger:
  - Connection protocol: https://github.com/hyperledger/aries-rfcs/tree/main/features/0160-connection-protocol
  - Out-of-Band protocol: https://github.com/hyperledger/aries-rfcs/tree/main/features/0434-outofband
  - DID Exchange protocol: https://github.com/hyperledger/aries-rfcs/tree/main/features/0023-did-exchange

## Further Reading

- [DIDComm Connections](https://identity.foundation/didcomm-messaging/spec/#connections)
- [DIDComm Out Of Band Messages][OUT_OF_BAND]
- [DIDComm Service Endpoint][DIDCOMM_SERVICE_ENDPOINT]
- [DID Services][SERVICE]
- [Aries Hyperledger Goal Codes](https://github.com/hyperledger/aries-rfcs/tree/main/concepts/0519-goal-codes)

<!--- LINKS --->
[ANONCRYPT]: https://identity.foundation/didcomm-messaging/spec/#anonymous-encryption
[DIDURL]: https://www.w3.org/TR/did-core/#did-url-syntax
[DID_KEY]: https://w3c-ccg.github.io/did-method-key/
[SERVICE]: https://www.w3.org/TR/did-core/#services
[OUT_OF_BAND]: https://github.com/decentralized-identity/didcomm-messaging/blob/49935b7b119591a009ce61d044ba9ad6fa40c7b7/docs/spec-files/out_of_band.md
[DIDCOMM_SERVICE_ENDPOINT]: https://identity.foundation/didcomm-messaging/spec/#did-document-service-endpoint

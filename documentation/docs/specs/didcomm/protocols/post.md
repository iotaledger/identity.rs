---
title: Post
sidebar_label: Post
---

:::info

The IOTA DIDComm Specification is in the RFC phase and may undergo changes. Suggestions are welcome at [GitHub #464](https://github.com/iotaledger/identity.rs/discussions/464).

:::

- Version: 0.1
- Status: `IN-PROGRESS`
- Last Updated: 2021-10-29

## Overview

Allows the sending of a single message with arbitrary data. Multiple [post](#post-message) messages MAY be chained together in the same [DIDComm thread](https://identity.foundation/didcomm-messaging/spec/#threads) to achieve bi-directional communication.

### Relationships

- [Authentication](./authenticationmd): can be used to authenticate both parties and establish [sender authenticated encryption](https://identity.foundation/didcomm-messaging/spec/#sender-authenticated-encryption).

### Example Use-Cases
- Instant messaging between two parties, where the text payload is displayed in a chat.
- IoT devices transmit sensor data to be aggregated in a central hub for processing.

### Roles
- Sender: sends the message.
- Receiver: receives the message.

### Interaction

![PostDiagram](/img/didcomm/post.drawio.svg)

<div style={{textAlign: 'center'}}>

<sub>For guidance on diagrams see the <a href="../overview#diagrams">corresponding section in the overview</a>.</sub>

</div>


## Messages

### 1. post {#post-message}

- Type: `iota/post/0.1/post`
- Role: [sender](#roles)

The [sender](#roles) transmits a JSON `payload` to the [receiver](#roles). This MAY take advantage of [sender authenticated encryption](https://identity.foundation/didcomm-messaging/spec/#sender-authenticated-encryption) or be a [signed DIDComm message](https://identity.foundation/didcomm-messaging/spec/#didcomm-signed-message) or both.

#### Structure
```json
{
  "payload": JSON // REQUIRED
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `payload` | Any valid [JSON](https://datatracker.ietf.org/doc/html/rfc7159) text. | ✔ |

#### Examples

1. Send a single string:

```json
{
  "payload": "Hello, world"
}
```

2. Send a single number:

```json
{
  "payload": 42
}
```

3. Send a JSON object:

```json
{
  "payload": {
    "status_code": 100,
    "status": "Okay",
  }
}
```

### Problem Reports {#problem-reports}

The following problem-report codes may be raised in the course of this protocol and are expected to be recognised and handled in addition to any general problem-reports. Implementers may also introduce their own application-specific problem-reports.

For guidance on problem-reports and a list of general codes see [problem reports](../resources/problem-reports).

| Code | Message | Description |
| :--- | :--- | :--- |
| `e.p.msg.iota.post.reject-post` | [post](#post-message) | [Receiver](#roles) rejects a [post](#post-message) message for any reason. |

## Considerations

Since the `payload` JSON structure is unrestricted, a [sender](#roles) cannot make assumptions about [receivers](#roles) being able to understand the `payload` in any meaningful way unless both parties have a shared implementation or pre-negotiate the `payload` structure.

If complex and repeatable behaviour between parties is needed, implementors SHOULD define their own protocols with well-defined messages and interactions rather than using generic [post](#post-message) messages.

## Related Work

- Aries Hyperledger: https://github.com/hyperledger/aries-rfcs/blob/main/features/0095-basic-message/README.md

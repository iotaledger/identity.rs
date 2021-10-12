---
title: Problem Reports
sidebar_label: Problem Reports
---

TODO: explain sorter + scope

In addition to the [problem-report descriptors in the DIDComm specification](https://identity.foundation/didcomm-messaging/spec/#descriptors), we define the following non-exhaustive list of general problem-report codes that may be sent during the course of any protocol:

| Code | Description |
| :--- | :--- |
| `e.p.msg.invalid-message` | The message is malformed or fails field constraints validation. |
| `e.p.msg.invalid-state` | The recipient is unable to handle the type of message in its current state. Typically when an unexpected message is received in the middle of a protocol on the same thread. |
| `e.p.trust.crypto` | TODO |
| `e.p.req.time` | The party has timed out waiting for a response. |

These messages may be raised during or between protocols to inform the other party that something went wrong. A problem report with the error sorter `e` and protocol scope `p` terminates the protocol on the current thread and MAY be followed by a connection [termination](../protocols/termination).

## Considerations

This section is non-normative.

## Unresolved Questions
Are 
---
title: Authentication
sidebar_label: Authentication
---

:::info

The IOTA DIDComm Specification is in the RFC phase and may undergo changes. Suggestions are welcome at [GitHub #464](https://github.com/iotaledger/identity.rs/discussions/464).

:::

- Version: 0.1
- Status: `IN-PROGRESS`
- Last Updated: 2021-10-29

## Overview
This protocol allows two parties to mutually authenticate by disclosing and verifying the [DID] of each other. On successful completion of this protocol, it is expected that [sender authenticated encryption][SAE] may be used between the parties for continuous authentication.

### Relationships

- [Connection](./connection): it is RECOMMENDED to establish [anonymous encryption](https://identity.foundation/didcomm-messaging/spec/#anonymous-encryption) on [connection](./connection) to prevent revealing the DID of either party to eavesdroppers.

### Example Use-Cases
- A connected sensor wants to make sure only valid well-known parties connect to it, before allowing access.
- A customer wants to make sure they are actually connecting to their bank, before presenting information.
- An organisation wants to verify the DID of the employer before issuing access credentials. 

### Roles
- Requester: initiates the authentication.
- Responder: responds to the authentication request.

### Interaction

![AuthenticationDiagram](/img/didcomm/authentication.drawio.svg)

<div style={{textAlign: 'center'}}>

<sub>For guidance on diagrams see the <a href="../overview#diagrams">corresponding section in the overview</a>.</sub>

</div>


## Messages

### 1. authentication-request {#authentication-request}

- Type: `iota/authentication/0.1/authentication-request`
- Role: [requester](#roles)

Sent to initiate the authentication process. This MUST be a [signed DIDComm message][SDM] to provide some level of trust to the [responder](#roles). However, even when signed it is possible to replay an [authentication-request](#authentication-request), so this message alone is insufficient to prove the DID of the [requester](#roles). In addition to a unique `requesterChallenge`, the `created_time` and `expires_time` [DIDComm message headers](https://identity.foundation/didcomm-messaging/spec/#message-headers) SHOULD be used to mitigate such replay attacks. Note that even a successful replay would only reveal the DID of the responder, authentication of a malicious requester will still fail without access to the original requester's private keys due to the use of challenges.

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
| [`did`][DID] | [DID] of the [requester](#roles).[^1] | ✔ |
| `requesterChallenge` |  A random string unique per [authentication-request](#authentication-request) by a [requester](#roles) to help mitigate replay attacks. | ✔ |
| `upgradeEncryption` | A string indicating whether [sender authenticated encryption][SAE] should be used in the following messages. One of `["required", "optional", "unsupported"]`.[^2] | ✔ |

[^1] The signing key used for the [signed DIDComm envelope][SDM] wrapping this message MUST be an [authentication method][AUTH_METHOD] in the DID document corresponding to `did`, as per the [DIDComm specification][DIDCOMM_KEYS].

[^2] The `upgradeEncryption` field allows negotiation of whether or not to use [sender authenticated encryption][SAE] for the [authentication](#authentication) protocol and for all messages that follow it. It is RECOMMENDED to specify `"required"` as it offers various guarantees of continuous authentication and payload integrity for every message. The available options are:
- `"required"`: the [responder](#roles) MUST initiate [sender authenticated encryption][SAE], from the following [authentication-response](#authentication-response) message onwards, or send a problem-report.
- `"optional"`: the [responder](#roles) chooses whether or not to use [sender authenticated encryption][SAE].
- `"unsupported"`: the [responder](#roles) MUST NOT use [sender authenticated encryption][SAE]. A [responder](#roles) MAY reject [authentication-requests](#authentication-request) that do not support encryption.
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
### 2. authentication-response {#authentication-response}

- Type: `iota/authentication/0.1/authentication-response`
- Role: [responder](#roles)

Sent in response to an [authentication-request](#authentication-request), proving the DID of the [responder](#roles). Optionally establishes [sender authenticated encryption][SAE] based on the value of `upgradeEncryption` in the preceding [authentication-request](#authentication-request). If `upgradeEncryption` was `"required"` and this message is not encrypted, or `"unsupported"` and this message is encrypted, the [requester](#roles) MUST issue a problem-report and abort the authentication.

This message MUST be a [signed DIDComm message][SDM], even if [sender authenticated encryption][SAE] is used. This is to ensure an [authentication key][AUTH_METHOD] is used to sign the challenge, in accordance with the [DID specification][AUTH_METHOD], and because there may be increased security controls or guarantees compared to the [keyAgreement](https://www.w3.org/TR/did-core/#key-agreement) keys used for [sender authenticated encryption][SAE].

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
| [`did`][DID] | [DID] of the [responder](#roles).[^1] | ✔ |
| `requesterChallenge` | Must match the `requesterChallenge` in the preceding [authentication-request](#authentication-request). | ✔ |
| `responderChallenge` | A random string unique per [authentication-response](#authentication-response) by a [responder](#roles) to help mitigate replay attacks. | ✔ |

[^1] The signing key used for the [signed DIDComm envelope][SDM] wrapping this message MUST be an [authentication method][AUTH_METHOD] in the DID document corresponding to the `did`, as per the [DIDComm specification][DIDCOMM_KEYS].


#### Examples

1. Responder presenting their [DID] and offering a challenge to the requester:

```json
{
  "did": "did:iota:8cU6DPF56MDEugfLF8AHFaaTuMQvmRo6kbxfjqQJpJmC",
  "requesterChallenge": "81285532-b72a-4a99-a9bd-b470475bc24f",
  "responderChallenge": "b1f0dc02-85a3-4438-8786-b625f11f1be4",
}
```

### 3. authentication-result {#authentication-result}

- Type: `iota/authentication/0.1/authentication-result`
- Role: [requester](#roles)

This message finalises the mutual authentication, proving control over the DID of the [requester](#roles) to the [responder](#roles). Similar to [authentication-response](#authentication-response), this message MUST be a [signed DIDComm message][SDM].

This MUST or MUST NOT use [sender authenticated encryption][SAE] depending on the outcome of the `upgradeEncryption` negotiation in the preceding [authentication-response](#authentication-response) message, otherwise resulting in a problem-report and failure of the authentication protocol. For example, if `upgradeEncryption` was `optional` and the [authentication-response](#authentication-response) used [sender authenticated encryption][SAE], then the [authentication-result](#authentication-result) MUST be encrypted to be valid. 

#### Structure
```json
{
  "responderChallenge": string // REQUIRED
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `responderChallenge` | Must match the `challenge` in the preceding [authentication-response](#authentication-response).[^1] | ✔ |

[^1] The signing key used for the [signed DIDComm envelope][SDM] wrapping this message MUST be an [authentication method][AUTH_METHOD] in the DID document corresponding to the `did` of the [requester](#roles) in the [authentication-request](#authentication-request), as per the [DIDComm specification][DIDCOMM_KEYS].

#### Examples

1. Requester responding with the responder's challenge from the previous message:

```json
{
  "responderChallenge": "0768e82d-f498-4f38-8686-918325f9560d"
}
```

### Problem Reports {#problem-reports}

The following problem-report codes may be raised in the course of this protocol and are expected to be recognised and handled in addition to any general problem-reports. Implementers may also introduce their own application-specific problem-reports.

For guidance on problem-reports and a list of general codes see [problem reports](../resources/problem-reports).

| Code | Message | Description |
| :--- | :--- | :--- |
| `e.p.msg.iota.authentication.reject-authentication` | [authentication-request](#authentication-request), [authentication-response](#authentication-response), [authentication-result](#authentication-result) | The party rejects an authentication request/response/result for any reason. |
| `e.p.msg.iota.authentication.reject-authentication.missing-keys` | [authentication-request](#authentication-request), [authentication-response](#authentication-response), [authentication-result](#authentication-result) | The party rejects an authentication request/response due to the other party lacking a supported `keyAgreement` section in the DID document. |
| `e.p.msg.iota.authentication.reject-authentication.untrusted-identity` | [authentication-request](#authentication-request), [authentication-response](#authentication-response) | The party rejects an authentication request/response due to the claimed DID of the other party. |
| `e.p.msg.iota.authentication.reject-authentication.encyption-required` | [authentication-request](#authentication-request), [authentication-response](#authentication-response), [authentication-result](#authentication-result) | The party rejects an authentication request/response/result due to the lack of [sender authenticated encryption][SAE]. |
| `e.p.msg.iota.authentication.reject-authentication.encyption-unsupported` | [authentication-request](#authentication-request), [authentication-response](#authentication-response), [authentication-result](#authentication-result) | The party rejects an authentication request/response/result because it does not support [sender authenticated encryption][SAE]. |

## Considerations

This section is non-normative.

- **Trust**: this [authentication](#authentication) protocol only verifies that both parties have access to the necessary private keys (which could become compromised) associated with their DID documents. Verifying whether a DID document is [bound to a physical identity](https://www.w3.org/TR/did-core/#binding-to-physical-identity) may require additional interactions. Verifying whether a DID can be trusted can be achieved by, for instance:
  - requesting a verifiable presentation of credentials issued by a trusted third party, such as a government,
  - using the [Well Known DID Configuration](https://identity.foundation/.well-known/resources/did-configuration/) or embedding the DID in a DNS record to tie an identity to a website or domain,
  - using an allowlist of trusted DIDs,
  - exchanging DIDs out-of-band in a secure manner (note that some [connection](./connection) invitations could be altered by malicious parties depending on the medium).
- **Authorisation**: the permissions and capabilities of either party may still need to be established after [authentication](#authentication), either by [verifiable presentation](./presentation) as mentioned above or other methods such as JWT tokens
- **Privacy**: the [responder](#roles) may be subject to probing whereby their DID may be revealed even with the use of [sender authenticated encryption][SAE], as the `skid` message header is linked to their DID. This is possible if the [responder](#roles) chooses to accept the [authentication-request](#authentication-request) of an unknown [requester](#roles), or the [requester](#roles) successfully replays an [authentication-request](#authentication-request) from a DID the [requester](#roles) trusts.

## Unresolved Questions

- Enforce signed DIDComm messages on top of sender-authenticated encryption or keep them optional? Usually unnecessary and DIDComm recommends against this since it's redundant and due to non-repudiation may decrease security and privacy by allowing participants to prove to third parties that authentication occurred.
  - https://identity.foundation/didcomm-messaging/spec/#didcomm-signed-message
  - https://github.com/hyperledger/aries-rfcs/blob/master/concepts/0049-repudiation/README.md#summary

- How to protect the DID of the responder (`skid` field in sender-authenticated message) to prevent probing identities even with anonymous encryption?
  - https://github.com/decentralized-identity/didcomm-messaging/issues/197
  - https://github.com/decentralized-identity/didcomm-messaging/issues/219

- Add examples of full signed and sender-authenticated messages with headers for better illustration?

## Related Work

- Aries Hyperledger:
  - DID Exchange protocol: https://github.com/hyperledger/aries-rfcs/tree/main/features/0023-did-exchange
  - DIDAuthZ: https://github.com/hyperledger/aries-rfcs/tree/main/features/0309-didauthz
- Jolocom: https://jolocom.github.io/jolocom-sdk/1.0.0/guides/interaction_flows/#authentication


<!--- LINKS --->
[DID]: https://www.w3.org/TR/did-core/#dfn-decentralized-identifiers
[AUTH_METHOD]: https://www.w3.org/TR/did-core/#authentication
[DIDCOMM_KEYS]: https://identity.foundation/didcomm-messaging/spec/#did-document-keys
[SAE]: https://identity.foundation/didcomm-messaging/spec/#sender-authenticated-encryption
[SDM]: https://identity.foundation/didcomm-messaging/spec/#didcomm-signed-message

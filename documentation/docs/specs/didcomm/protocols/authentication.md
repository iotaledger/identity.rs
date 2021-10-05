---
title: Authentication
sidebar_label: Authentication
---

# Authentication

- Version: 0.1
- Status: `IN-PROGRESS`
- Last Updated: 2021-10-05

## Overview
This protocol allows two parties to mutually authenticate each other. Each party may abort then authentication if he does not want to communicate with the other party.

### Relationships


### Example Use-Cases


### Roles
- Requester: Initiates the authentication.
- Responder: Responds to the authentication request.

### Interaction

<div style={{textAlign: 'center'}}>
TODO
![IssuanceDiagram](/img/didcomm/issuance.drawio.svg)

</div>


## Messages

### 1. issuance-request {#issuance-request}

- Type: `didcomm:iota/issuance/0.1/issuance-request`
- Role: [holder](#roles)

TBD

#### Structure
```json
{
  "subject": DID,             // REQUIRED
  "@context": [string],       // OPTIONAL
  "type": [string],           // REQUIRED
  "trustedIssuers": [string]  // OPTIONAL
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| [`subject`](https://www.w3.org/TR/vc-data-model/#credential-subject-0) | [DID](https://www.w3.org/TR/did-core/#dfn-decentralized-identifiers) of the [credential subject](https://www.w3.org/TR/vc-data-model/#credential-subject-0)[^1]. | ✔ |


#### Examples

1. Request a drivers licence credential:

```json
{
  "subject": "did:example:c6ef1fe11eb22cb711e6e227fbc",
  "type": ["VerifiableCredential", "DriversLicence"],
}
```

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


## Further Reading

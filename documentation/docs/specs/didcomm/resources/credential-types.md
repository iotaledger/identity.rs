---
title: Credential Types
sidebar_label: Credential Types
---

## CredentialInfo

CredentialInfo types allow parties to negotiate which [`verifiable credentials`](https://www.w3.org/TR/vc-data-model) they want issued or exchanged.

We currently prescribe only `CredentialType2021` but additional types may be introduced in the future, to account for selective disclosure of particular fields for instance. If full schema negotiation of credentials is required, refer to the external [Presentation Exchange 1.0 specification](https://identity.foundation/presentation-exchange/spec/v1.0.0/).

### CredentialType2021

- Type: `CredentialType2021`

Negotiates credentials by allowing to specify the [`type`](https://www.w3.org/TR/vc-data-model/#types) and optionally [`@context`](https://www.w3.org/TR/vc-data-model/#contexts) of the credential. Also the [`issuer`](https://www.w3.org/TR/vc-data-model/#issuer) can be used for negotiation.

```json
{
  "credentialInfoType": string,   // REQUIRED
  "@context": [string],           // OPTIONAL
  "type": [string],               // REQUIRED
  "issuer": [string],             // OPTIONAL
}
```

| Field | Description | Required |
| :--- | :--- | :--- |
| `credentialInfoType` | String indicating the `CredentialInfo` type, MUST be `"CredentialType2021"`. | ✔ | 
| [`@context`](https://www.w3.org/TR/vc-data-model/#contexts) | Array of JSON-LD contexts referenced in the credential. | ✖ |
| [`type`](https://www.w3.org/TR/vc-data-model/#types) | Array of credential types specifying the kind of credential offered.[^1] | ✔ | 
| [`issuer`](https://www.w3.org/TR/vc-data-model/#issuer) | Array of credential issuer DIDs or URIs.[^2] | ✖ |

[^1] During [presentation](./presentation), the types MAY be under-specified to preserve privacy but SHOULD always include the most general types. For example, a credential with the types `["VerifiableCredential", "DriversLicence", "EUDriversLicence", "GermanDriversLicence"]` could be offered as `["VerifiableCredential", "DriversLicence"]`.

[^2] The `issuer` field MAY either be the single issuer of an existing credential, one or more issuers that a [verifier](./presentation#roles) would trust during a [presentation](./presentation), or one or more trusted issuers that a [holder](./issuance#roles) requests to sign their credential during an [issuance](./issuance). The `issuer` field is OPTIONAL as the [holder](./presentation#roles) may not want to reveal too much information up-front about the exact credentials they possess during a [presentation](./presentation); they may want a non-repudiable signed request from the verifier first. 

#### Examples

1. TBD

```json
{
  "credentialInfoType": "CredentialType2021", 
  "type": ["VerifiableCredential", "UniversityDegreeCredential"],
  "issuer": ["did:example:76e12ec712ebc6f1c221ebfeb1f"]
}
```



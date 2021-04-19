---
### credential-options

Querying an agent for the VCs that the agent can issue.

The Verifiable Credential (VC) issuance flow consists of a three step interaction process between two parties, the <u>issuer</u> and the <u>holder</u>. This is the first interaction in this process. In this interaction, the <u>holder</u> queries the <u>issuer</u> for a list of VC types that the <u>issuer</u> offers to issue.

#### Roles
- **Issuer**: Agent who offers and issues one or more Verifiable Credentials
- **Holder**: Agent who selects and receives one or more Verifiable Credentials

#### Messages

#### credentialOptionsRequest
The <u>holder</u> queries the <u>issuer</u> for a list of VC types that the <u>issuer</u> offers.

###### Layout

```JSON
credentialOptionsRequest: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "callbackURL", // REQUIRED!
    "responseRequested", //OPTIONAL!
    "id", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "credential-options/1.0/credentialOptionsRequest",
    "thread": "936DA01F9ABD4d9d80C702AF85C822A8",
    "callbackURL": "https://www.bobsworld.com/"
}
```

#### credentialOptionsResponse
The <u>issuer</u> responds with a list of offered VC types.

###### Layout

```JSON
credentialOptionsResponse: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "credentialTypes": [
        "credentialType 1",
        "credentialType 2",
        "credentialType n"
    ], // REQUIRED!
    "callbackURL", // OPTIONAL!
    "responseRequested", //OPTIONAL!
    "id", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "credential-options/1.0/credentialOptionsResponse",
    "thread": "936DA01F9ABD4d9d80C702AF85C822A8",
    "credentialTypes": [
        "YourCatHasAnAttitudeCredential",
        "YouHaveNiceHairCredential",
        "YourLasagnaIsDeliciousCredential"
    ],
    "callbackURL": "https://www.aliceswonderland.com/",
}
```

[Source 1: Jolocom VC Issuance](https://jolocom.github.io/jolocom-sdk/1.0.0/guides/interaction_flows/#verifiable-credential-issuance); [Source 2: Aries Issue Credential Protocol](https://github.com/hyperledger/aries-rfcs/tree/master/features/0453-issue-credential-v2);

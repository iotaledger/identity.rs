# presentation-verification

Proving a set of statements about an identifier.

The credential verification flow is a simple request-response message exchange between the <u>verifier</u> and the <u>prover</u>. The interaction can consist of up to two messages, the first one is OPTIONAL.

### Roles
- **Verifier**: Agent who requests a set of Verifiable Credentials with associated requirements
- **Prover**: Agent who provides a set of Verifiable Credentials in the form of a Verifiable Presentation attempting to satisfy the request

### Messages

#### presentationRequest
The <u>verifier</u> requests a set of Verifiable Credentials from the <u>prover</u>. This message is OPTIONAL within this interaction.

###### Layout

```JSON
presentationRequest: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "callbackURL", // REQUIRED!
    "trustedIssuers", // OPTIONAL!
    "responseRequested", //OPTIONAL!
    "id", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "presentation-verification/1.0/presentationRequest",
    "thread": "936DA01F9ABD4d9d80C702AF85C822A8",
    "callbackURL": "https://www.bobsworld.com/",
    "trustedIssuers" : [
        {
            "credentialTypes": [
                "YourCatHasAnAttitudeCredential",
                "YouHaveNiceHairCredential"
            ],
            "supportedIssuers": [
                "did:iota:58c35471071b3dbb97585ee06bb1dd0239ca338629534296cfbb2db6bc857e21"
            ]
        },
        {
            "credentialTypes": [
                "YourLasagnaIsDeliciousCredential"
            ],
            "supportedIssuers": [
                "did:iota:58c35471071b3dbb97585ee06bb1dd0239ca338629534296cfbb2db6bc857e21",
                "did:iota:23f0b94812c402a1dea1c424303b178d01485a5dcf26cf977333f3b629bd90ec"
            ]
        }
    ]
}
```

#### presentationResponse
The <u>holder</u> sends a Verifiable Presentation to the <u>verifier</u> using a `presentationResponse` message.

###### Layout

```JSON
presentationResponse: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "verifiablePresentation", // REQUIRED!
    "callbackURL", // OPTIONAL!
    "responseRequested", //OPTIONAL!
    "id", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "presentation-verification/1.0/presentationResponse",
    "thread": "936DA01F9ABD4d9d80C702AF85C822A8",
    "verifiablePresentation": {...}, // Omitted for brevity
    "callbackURL": "https://www.aliceswonderland.com/"
}
```

[Source 1: Jolocom Credential Verification](https://jolocom.github.io/jolocom-sdk/1.0.0/guides/interaction_flows/#credential-verification);
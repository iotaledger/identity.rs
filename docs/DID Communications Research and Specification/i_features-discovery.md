---
### features-discovery

Enabling agents to discover which interactions other agents support.

#### Roles
- <u>**Requester**</u>: Agent who requests an array of interactions that the <u>responder</u> supports
- <u>**Responder**</u>: Agent who provides the requested array of interactions to the <u>requester</u>

#### Messages

#### featuresRequest
The <u>requester</u> sends the `featuresRequest` to the <u>responder</u>, asking for the array of supported interactions. 

###### Layout

```JSON
featuresRequest: {
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
    "context": "features-discovery/1.0/featuresRequest",
    "thread": "936DA01F9ABD4d9d80C702AF85C822A8",
    "callbackURL": "https://www.bobsworld.com/"
}
```

#### featuresResponse
The <u>responder</u> answers with a `featuresResponse`, containing the array of supported interactions.

###### Layout

```JSON
featuresResponse: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "features", // REQUIRED!
    "callbackURL", // OPTIONAL!
    "responseRequested", //OPTIONAL!
    "id", // OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "features-discovery/1.0/featuresResponse",
    "thread": "936DA01F9ABD4d9d80C702AF85C822A8",
    "features": [
        "trust-ping/1.0",
        "did-discovery/1.0",
        "features-discovery/1.0",
        "authentication/1.0"
    ]
}
```

[Source 1: Aries Discover Features Protocol](https://github.com/hyperledger/aries-rfcs/blob/master/features/0031-discover-features/README.md); [Source 2: DIF Discover Features Protocol](https://identity.foundation/didcomm-messaging/spec/#discover-features-protocol-10);

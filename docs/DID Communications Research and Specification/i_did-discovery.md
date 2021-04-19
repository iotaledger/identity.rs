---
### did-discovery

Requesting a DID from an agent.

#### Roles
- <u>**Requester**</u>: Agent who requests a DID from the <u>endpoint</u>
- <u>**Endpoint**</u>: Agent who provides the requested DID to the <u>requester</u>

#### Messages

#### didRequest
The <u>requester</u> sends the `didRequest` to the <u>endpoint</u>, specifying a `callbackURL` for the `didResponse` to be posted to. 

###### Layout

```JSON
didRequest: {
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
    "context": "did-discovery/1.0/didRequest",
    "thread": "936DA01F9ABD4d9d80C702AF85C822A8",
    "callbackURL": "https://www.bobsworld.com/",
    "id": "did:iota:57edacef81828010b314b96c0915780f206341e0ce8892a1b56678c174eef2e8",
}
```

#### didResponse
The <u>endpoint</u> answers with a `didResponse`, containing its DID.

###### Layout

```JSON
didResponse: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "id", // REQUIRED!
    "callbackURL", // OPTIONAL!
    "responseRequested", //OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "did-discovery/1.0/didResponse",
    "thread": "936DA01F9ABD4d9d80C702AF85C822A8",
    "id": "did:iota:42edacef81828010b314b96c0915780f206341e0ce8892a1b56678c174eef242"
}
```

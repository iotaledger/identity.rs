# did-resolution

Using another Agent as a Resolver.

DID resolution consists of a simple request-response message exchange, where the Requester asks the Resolver to perform DID resolution and return the result.

### Roles
- **Requester**: Agent who requests the resolution of a DID
- **Resolver**: Agent who resolves the given DID (or their own) and returns the result

### Messages

#### resolutionRequest
The Requester broadcasts a message which may or may not contain a DID (see below).

###### Layout

```JSON
resolutionRequest: {
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
    "context": "did-resolution/1.0/resolutionRequest",
    "thread": "936DA01F9ABD4d9d80C702AF85C822A8",
    "callbackURL": "https://www.bobsworld.com/",
    "id": "did:iota:57edacef81828010b314b96c0915780f206341e0ce8892a1b56678c174eef2e8",
}
```

#### resolutionResponse
If the message contains a DID (in the `id` field), the Resolver resolves the DID and returns the DID Resolution Result. Otherwise, the Resolver returns the result of resolving it's own DID. This is intended for the special case of "local" DID methods, which do not have a globally resolvable state.

###### Layout

```JSON
resolutionResponse: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "didDocument", // REQUIRED!
    "callbackURL", // OPTIONAL!
    "responseRequested", //OPTIONAL!
    "timing" // OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "did-resolution/1.0/resolutionResponse",
    "thread": "936DA01F9ABD4d9d80C702AF85C822A8",
    "didDocument": {
        "id": "did:iota:57edacef81828010b3--SNIP--a1b56678c174eef2e8",
        "authentication": [{
            "id": "did:iota:57edacef81828010b314--SNIP--a1b56678c174eef2e8#keys-1",
            "type": "Ed25519VerificationKey2020",
            "controller": "did:iota:57edacef81828010b3--SNIP--a1b56678c174eef2e8",
            "publicKeyMultibase": "zH3C2AVvLMv6gmMNam3uVAjZpfkcJCwDwnZn6z3wXmqPV"
        }]
    },
    "callbackURL": "https://www.aliceswonderland.com/"
}
```

[Source 1: Jolocom Peer Resolution](https://jolocom.github.io/jolocom-sdk/1.0.0/guides/interaction_flows/#peer-resolution); [Source 2: Aries DID Resolution Protocol](https://github.com/hyperledger/aries-rfcs/tree/master/features/0124-did-resolution-protocol);

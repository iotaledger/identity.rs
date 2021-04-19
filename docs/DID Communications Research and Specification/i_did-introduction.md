WIP


---
### did-introduction

Describes how a go-between can introduce parties that it already knows, but that do not know each other.

#### Roles
- <u>**Introducer**</u>: Agent who introduces multiple <u>introducee</u>s to each other
- <u>**Introducee**</u>: Agents that are introduced to each other through the <u>introducer</u>

Note that we have two roles here, but multiple introducees.

#### Messages

#### introductionProposal
The <u>introducer</u> sends the `introductionProposal` to every <u>introducee</u>, asking for their consent to the introduction.

###### Layout
TODO comment as question
TODO introduction rationale for IoT devices / standardize comment field? -> reason? -> explanation into intro
TODO write about privacy in the intro -> cant have privacy of intro when 1 denies, other accepts
```JSON
introductionProposal: {
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

#### introductionResponse
The <u>introducee</u>s answer with a `introductionResponse`, signaling their consent.

###### Layout

```JSON
introductionResponse: {
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
        "authentication/1.0",
    ]
}
```

#### introduction
The <u>introducer</u> finishes with a series of `introduction` messages, introducing all <u>introducee</u>s that consented to each other.

###### Layout

```JSON
introduction: {
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
        "authentication/1.0",
    ]
}
```

[Source 1: Aries Introduce Protocol](https://github.com/hyperledger/aries-rfcs/blob/master/features/0028-introduce/README.md);


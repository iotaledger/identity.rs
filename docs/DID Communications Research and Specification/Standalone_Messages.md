## Standalone Messages

Messages that are shared across interactions.

#### Roles
- <u>**Sender**</u>: Agent who sends the message
- <u>**Receiver**</u>: Agent who receives the message

#### report
The <u>sender</u> sends a `report` message to the <u>receiver</u> to provide him with details about a previously received message. This can be a simple acknowledgement or e.g. an error report. The `reference` field refers to the message that is either acknowledged or has resulted in an error. Further information can be passed through the `comment` field.

###### Layout
TODO standardize error


```JSON
report: {
    "context", // REQUIRED!
    "thread", // REQUIRED!
    "reference", // REQUIRED!
    TODO
    "error" : 200
    TODO source https://en.wikipedia.org/wiki/List_of_HTTP_status_codes
    "comment", // OPTIONAL!
    "timing" // OPTIONAL! All subfields OPTIONAL!
}
```

###### Example(s)

```JSON
{
    "context": "report/1.0/report",
    "thread": "936DA01F9ABD4d9d80C702AF85C822A8",
    "reference": "did-resolution/1.0/resolutionResponse",
    "comment": "Can't resolve DID: Signature invalid!",
    "timing": {
        "out_time": "2069-04-20T13:37:42Z",
        "in_time": "2069-04-20T13:37:00Z"
    }
}
```
[Source 1: Aries Report Problem Protocol](https://github.com/hyperledger/aries-rfcs/blob/master/features/0035-report-problem/README.md);
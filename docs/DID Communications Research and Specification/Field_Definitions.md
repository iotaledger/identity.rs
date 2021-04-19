## Field Definitions

`context` as URL/String, e.g. `did-resolution/1.0/resolutionResponse`: Defines the context that a specific message adheres to.

`reference` as URL/String, e.g. `did-resolution/1.0/resolutionResponse`: Defines the context that a report message refers to.

`thread` as UUID, e.g. `936DA01F9ABD4d9d80C702AF85C822A8`: A [UUID](https://docs.rs/uuid/0.8.2/uuid/) as String, defined by the agent that initiated an interaction, to be used to identify this specific interaction to track it agent-locally. Together with the context, these two fields can be used to identity a message / state within a specific interaction.

`callbackURL` as URL/String, e.g. `https://www.bobsworld.com/` or `https://www.aliceswonderland/serviceEndpoint`: Defines the URL (or API call) where a message is to be delivered to.

`responseRequested` as Boolean, e.g. `true` or `false`: Undefined counts as `false`. In messages where it is defined it asks the recipient of the message to repond in the form of an acknowledging report. This request SHOULD be honored. The only exception to this behaviour is in `trust-ping`, where the acknowledging report MUST be sent if and only if this field is `true` - if it is `false`, the report MUST NOT be send.

`features` as an array of Strings, e.g. `["trust-ping/1.0", "did-discovery/1.0"]`: An array used for <a href="#features-discovery">features-discovery</a> that lists all available interactions that an agent supports, and their respective versions.

`id` as String, e.g. `did:iota:57edacef81828010b314b96c0915780f206341e0ce8892a1b56678c174eef2e8`: A decentralized identifier.

`ids` as JSON: An array of `id`s as defined above.

`didDocument` as JSON: A DID Document (see e.g. in <a href="#did-resolution">did-resolution</a>).

`comment` as String: A comment, mostly used to provide more information about something. Can be literally any String.

`challenge` as String, e.g. `Sign this`: A String acting as a signing challenge. Can contain basically anything.

`credential` as [VC JSON](https://w3c-ccg.github.io/vc-json-schemas/): A syntactically valid credential.

`credentials`: An array of valid `credential`s as defined above.

`credentialId` as String, e.g.`credential-69420-delicious-lasagna`: The id of a credential.

`credentialType` as String, e.g. `YouHaveNiceHairCredential`: A VC type.

`credentialTypes` as JSON, e.g. `["YouHaveNiceHairCredential", "YourLasagnaIsDeliciousCredential"]`: Contains an array of possible credential types, see e.g. <a href="#credential-options">credential-options</a>.

`supportedIssuers` as JSON: Contains an array of supported issuer `id`, see <a href="#credential-options">credential-options</a>.

`trustedIssuers` as JSON: An array of `{credentialTypes, supportedIssuers}` pairs, see e.g. <a href="#presentation-verification">presentation-verification</a>.

`schemata` as JSON: A named list of credential schemata, see <a href="#credential-schema">credential-schema</a>.

`verifiablePresentation` as JSON: A Verifiable Presentation.

`signature` as JSON: Defines a signature. Fields defined below.

`signature[type]` as String, e.g. `JcsEd25519Signature2020`: Signature type.

`signature[verificationMethod]` as String, e.g. `#authentication`: Reference to verification method in signer's DID Document used to produce the signature.

`signature[signatureValue]` as String, e.g. `5Hw1JWv4a6hZH5obtAshbbKZQAJK6h8YbEwZvdxgWCXSL81fvRYoMCjt22vaBtZewgGq641dqR31C27YhDusoo4N`: Actual signature.

`timing` as JSON: A decorator to include timing information into a message. Fields defined below.

`timing[out_time]` as ISO 8601 timestamp, e.g. `2069-04-20T13:37:00Z`: The timestamp when the message was emitted.

`timing[in_time]` as ISO 8601 timestamp, e.g. `2069-04-20T13:37:00Z`: The timestamp when the preceding message in this thread (the one that elicited this message as a response) was received.

`timing[stale_time]` as ISO 8601 timestamp, e.g. `2069-04-20T13:37:00Z`: Ideally, the decorated message should be processed by the the specified timestamp. After that, the message may become irrelevant or less meaningful than intended. This is a hint only.

`timing[expires_time]` as ISO 8601 timestamp, e.g. `2069-04-20T13:37:00Z`: The decorated message should be considered invalid or expired if encountered after the specified timestamp. This is a much stronger claim than the one for stale_time; it says that the receiver should cancel attempts to process it once the deadline is past, because the sender won't stand behind it any longer. While processing of the received message should stop, the thread of the message should be retained as the sender may send an updated/replacement message. In the case that the sender does not follow up, the policy of the receiver agent related to abandoned threads would presumably be used to eventually delete the thread.

`timing[delay_milli]` as Integer, e.g. `1337`: Wait at least this many milliseconds before processing the message. This may be useful to defeat temporal correlation. It is recommended that agents supporting this field should not honor requests for delays longer than 10 minutes (600,000 milliseconds).

`timing[wait_until_time]` as ISO 8601 timestamp, e.g. `2069-04-20T13:37:00Z`: Wait until this time before processing the message.

[(Source 1: Aries Message Timing)](https://github.com/hyperledger/aries-rfcs/blob/master/features/0032-message-timing/README.md)
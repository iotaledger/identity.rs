# IOTA DIDComm Message Specification

- [Introduction](#introduction)
- [Message Structure](#message-structure)
- [Message Encryption](#message-encryption)
- [DID Rotation](#did-rotation)
- [Routing](#routing)
- [Transports](#transports)

## Introduction
> DIDComm Messages are based on JWM ([JSON Web Messages](https://tools.ietf.org/id/draft-looker-jwm-01.html)). 

[TODO]

## Message structure
```json=
{
    "id": "1234567890",
    "type": "<message-type-uri>",
    "from": "did:example:alice",
    "to": ["did:example:bob"],
    "created_at": 1600179190,
    "expires_at": 1608768000,
    "body": {
    	"attribute_1": "value",
        "attribute_2": "value"
	}
}
```


- **id** - REQUIRED. Message ID. The id attribute value MUST be unique to the sender.
- **type** - REQUIRED. Message Type. The type attribute value MUST be a valid Message Type URI , that when resolved gives human readable information about the message. The attributes value also informs the content of the message, for example the presence of other attributes and how they should be processed.
- **to** - OPTIONAL. Recipient(s) identifier. The to attribute MUST be an array of strings where each element is a valid DID which identifies the recipients of the message.
- **from** - OPTIONAL. Sender identifier. The from attribute MUST be a string that is a valid DID which identifies the sender of the message. For DID methods that use query parameters to carry additional information, they might also be present in the from string. When a message is encrypted, the sender key MUST be authorized for encryption by this DID. Authorization of the encryption key for this DID MUST be verified by message recipient with the proper proof purposes. See the message authentication section for additional details.
- **created_at** - OPTIONAL. Message Created Time. The created_at attribute is used for the sender to express when they created the message, expressed in UTC Epoch Seconds.
- **expires_at** - OPTIONAL. Message Expired Time. The expires_at attribute is used for the sender to express when they consider the message to be expired, expressed in UTC Epoch Seconds. This attribute signals when the message is no longer valid, and is to be used by the recipient to discard expired messages on receipt.

- **body**– Everything inside the body object is different. Here can be everything JSON conformed, like nested objects and arrays. 

JSON-LD
- DIDComm is not dependent on JSON-LD, but it is compatible

## Message Encryption
- Authenticated Sender Encryption
- Anonymous Sender Encryption

[TODO]

## DID Rotation
> DID Rotation means to switch from one DID method to another.

[TODO]

## Routing

Roles:
- sender
- mediator
- recipiant

> In this protocol, the sender and the receiver never interact directly; they only interact via the mediator.

[TODO]

## Transports
> DIDComm Messaging is designed to be transport independent, including message encryption and agent message format. The encryption envelope provides both encryption and authentication, providing trust as a feature of each message. Each transport does have unique features, and we need to standardize how the transport features are (or are not) applied.

[TODO]

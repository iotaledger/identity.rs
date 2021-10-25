# Identity Communication

This crate implements DID-based messaging utilities that partially adheres to the [DIDComm Messaging Specification](https://identity.foundation/didcomm-messaging/spec/) by the [Decentralized Identity Foundation (DIF)](https://identity.foundation/).

:warning: **WARNING** :warning:
The IOTA Identity team is currently working on a new IOTA DIDComm specification. Once this new spec has been finalized this crate will most likely be heavily refactored, or even replaced in its entirety. One should expect many breaking changes! 

## Very brief introduction to DIDComm Messaging
**This section essentially consists of extracts from the [DIDComm spec](https://identity.foundation/didcomm-messaging/spec/).**

The purpose of DIDComm Messaging is to provide a secure, private communication methodology built atop the decentralized design of [DIDs](https://www.w3.org/TR/did-core/). Higher-order protocols such as issuing a verifiable credential can be built in the context of the DIDComm Messaging specification.

DIDComm messages can exist in three different formats:
1.The simplest and most fundamental of these is the [DIDComm Plaintext message](https://identity.foundation/didcomm-messaging/spec/#didcomm-plaintext-messages) which is a [JSON Web Message (JWM)](https://datatracker.ietf.org/doc/html/draft-looker-jwm-01) containing [headers](https://identity.foundation/didcomm-messaging/spec/#message-headers) and conveys application-level data inside a JSON `body`. 

2. A DIDComm plaintext message can optionally be packed in a *signed envelope* that associates a non-repudiable signature with the plaintext message inside it. A DIDComm message of this format is called a [DIDComm Signed Message](https://identity.foundation/didcomm-messaging/spec/#didcomm-signed-message). 

3. The third format for DIDComm messages is the [DIDComm encrypted message](https://identity.foundation/didcomm-messaging/spec/#didcomm-encrypted-message). In this case either a DIDComm plaintext message or a DIDComm signed message is packed in an envelope that applies encryption to the content it encloses. 
## Crate details 
- The envelope module provides algorithms for packing messages into envelopes used in the definitions of the various [DIDComm Message Types](https://identity.foundation/didcomm-messaging/spec/#message-types). 
- The message module contains a `Message` trait that can be utilized to pack messages into envelopes. 

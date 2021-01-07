# Decentralized Identifiers

Decentralized Identifiers serve as a reference to a DID Document. This document contains data such as public keys, enabling the holder to prove ownership over their personal data.


[Decentralized Identifiers (DIDs) v1.0 Specification](https://w3c.github.io/did-core/)

## DID Messages
DID Documents associated to the did:iota method consist of a chain of data messages, also known as zero-value transactions, published to a Tangle called "DID messages". The Tangle has no understanding of "DID messages" and acts purely as an immutable database. The chain of DID messages and the resulting DID Documents must therefore be validated on the client side.

A DID Message can be part of one of two different message chains, the "Authentication Chain" (Auth Chain) and the "Differentiation Chain" (Diff Chain). The Auth Chain is a chain of "DID Auth Messages" that contain full DID Documents. The Diff Chain is a chain of "DID Diff Messages" that contain JSON objects which only list the differences between one DID Document and the next state.

[link to specification](https://github.com/iotaledger/identity.rs/blob/feat/method-spec/docs/iota-did-method-spec.md#did-messages)
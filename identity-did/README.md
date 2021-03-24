Agnostic implementation of the Decentralized Identifiers (DID) standard from W3C.

Decentralized Identifiers (DID) is a proposed standard from the World Wide Web Consortium (W3C) to enable a
verifiable and decentralized identity. The standard provides a unique identifier (DID), which can be used to look up
more information about the associated identity in the form of a DID Document. The DID Document contains public keys,
to prove control over the identity, and service endpoints which are URI's that can be resolved to find more public
information about the identity. Often the DID Documents are stored on an Distributed Ledger Technology (DLT) such as
Bitcoin, Ethereum and IOTA, but this is not a requirement.

This is an agnostic implementation of the [DID specifications v1.0 Working Draft 20200731](https://www.w3.org/TR/2020/WD-did-core-20200731/).

It has been implemented in the following DID Methods:

- [IOTA Identity](https://github.com/iotaledger/identity.rs/tree/dev/identity-iota): Developed and maintained by the
  IOTA Foundation, utilizing the IOTA Tangle.

See [our documentation portal](https://identity.docs.iota.org/overview/did.html) for additional documentations, conceptual explainations and usage examples.

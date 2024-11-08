# Post-Quantum (PQ) and Post-Quantum/Traditional (PQ/T) hybrid signatures for VCs
This repository extends IOTA Identity by implementing both pure **Post-Quantum (PQ)** and **Post-Quantum/Traditional (PQ/T) hybrid** signatures and JWT encoding for VCs with a crypto-agility approach.

### Overview

1. **PQ Signatures**: IOTA Identity extends its support for selected PQ signature algorithms, such as [ML-DSA](https://csrc.nist.gov/pubs/fips/204/final), [SLH-DSA](https://csrc.nist.gov/pubs/fips/205/final) and [FALCON](https://falcon-sign.info/). The implementation of these algorithms is provided by [liboqs](https://github.com/open-quantum-safe/liboqs-rust).

2. **PQ/T hybrid Signatures**: mitigate risks associated with the relative immaturity of Post-Quantum Cryptography (PQC), IOTA Identity extends its support for PQ/T hybrid signatures. The hybrid scheme combines a PQ signature with a Traditional signature in a single composite signature. This ensures secure authentication, even if one of the two algorithms becomes compromised. The PQ/T hybrid signature requires a PQ/T hybrid key pair; the PQ/T hybrid public key is handled using the newly introduced [verification material property](https://www.w3.org/TR/did-core/#verification-material) type called `compositeJwk`, which stores both types of public keys within the DID document. This setup enforces the `Weak Non-Separability` (WSN) property of signatures, protecting against stripping attack.

```json
"compositeJwk": {
  "algId": ".. composite key OID ..",
  "pqPublicKey": {
     ".. PQ JWK encoded key .."
  },
  "traditionalPublicKey": {
    ".. Traditional JWK encoded key .."
  }
}
```

**Supported Algorithms**: Currently, the implmentation supports **id-MLDSA44-Ed25519-SHA512** and **id-MLDSA65-Ed25519-SHA512** algorithms. The first combines ML-DSA-44 with Ed25519 signatures, while the second combines ML-DSA-65 with Ed25519 signatures.

# did:compositejwk

The transition to PQC is a delicate and lengthy process. Today, the Distributed Ledger Technologies (DLT) that underpin decentralised identity are not yet quantum-secure, so this repository extends the IOTA Identity library with a new DID method called `did:compositejwk` for Holders to use PQ/T hybrid signatures. Refer to [did:compositejwk](https://github.com/Cybersecurity-LINKS/did-compositejwk/blob/main/spec.md) specification for the details.

**Note**: this repository also extends the existing `did:jwk` method to deal with pure PQ keys and signatures (ML-DSA, SLH-DSA and FALCON), and adds a simple `did:web` method for the Issuers. 

# Zero-Knowledge (ZK)

The IOTA Identity now supports Zero-Knowledge functionalities, thanks to the [integration](https://github.com/iotaledger/identity.rs/pull/1285) of two key components:

* **BBS+ Signature**: the scheme has been integrated through the [ZKryptium](https://github.com/Cybersecurity-LINKS/zkryptium) library for secure and privacy-preserving VC management with ZK selective disclosure.
* **JSON Web Proof Representation**: the [json-proof-token](https://github.com/Cybersecurity-LINKS/json-proof-token) library implements the JSON Web Proof (JWP) specification, enabling verifiable claims with selective disclosure.

For more details on the implementation and how to use these features, refer to the [full documentation](https://wiki.iota.org/identity.rs/how-tos/verifiable-credentials/zero-knowledge-selective-disclosure/).

# Examples

To test all the above functionalities, refer to practical code snippets available in the [example](https://github.com/Cybersecurity-LINKS/pq-zk-identity/tree/PQ/T-Hybrid/examples) directory.
> **Note**: The examples in the `example/demo` directory are configured to use the [DID Web Method](https://w3c-ccg.github.io/did-method-web/) for the Issuer. To run these examples, you must
> have a server instance that hosts the Issuer's DID Document. You can use the default server provided in the `example/demo/server` folder, or configure one yourself. However,
> ensure that the following variables in `utils.rs` are correctly set to point to your server instance:
> ```rust
> pub static DID_URL: &str = "https://localhost:4443/.well-known/";
> pub static PATH_DID_FILE: &str = "C:/Projects/did-web-server/.well-known/";
> ```
Make sure your server is set up before running the examples to avoid any configuration issues.


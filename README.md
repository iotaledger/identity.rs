# Zero-Knowledge (ZK)

The IOTA Identity Framework now supports Zero-Knowledge functionalities, thanks to the [integration](https://github.com/iotaledger/identity.rs/pull/1285) of two key components:

* **BBS+ Signature Scheme**: This scheme has been integrated through the [ZKryptium](https://github.com/Cybersecurity-LINKS/zkryptium) library, allowing for secure and privacy-preserving credential management.
* **JSON Web Proof Representation**: The [json-proof-token](https://github.com/Cybersecurity-LINKS/json-proof-token) library implements the JSON Web Proof specification, enabling verifiable claims with selective disclosure.

For more details on the implementation and how to use these features, you can find the full documentation [here](https://wiki.iota.org/identity.rs/how-tos/verifiable-credentials/zero-knowledge-selective-disclosure/).

# PQ/T Hybrid
This repository extends the IOTA Identity framework by implementing both **Post-Quantum (PQ)** and **Post-Quantum/Traditional (PQ/T) Hybrid** cryptographic approaches. These approaches address emerging threats to traditional cryptography posed by quantum computing.

### Overview

1. **PQ Approach**: To transition to quantum-resistant cryptography, the framework has been updated to support selected PQ signature algorithms, such as [**ML-DSA**](https://csrc.nist.gov/pubs/fips/204/final), [**SLH-DSA**](https://csrc.nist.gov/pubs/fips/205/final) and [**FALCON**](https://falcon-sign.info/). The implementation of these algorithms is provided by [**liboqs**](https://github.com/open-quantum-safe/liboqs-rust).

2. **PQ/T Hybrid Approach**: To mitigate risks associated with the relative immaturity of certain PQ algorithms, the PQ/T Hybrid combines a PQ algorithm with a traditional one in a composite signature. This ensures secure authentication, even if one of the two algorithms becomes compromised.
   - **Hybrid Signatures and Composite Key**: In the PQ/T Hybrid approach, both PQ and traditional keys are managed and verified using the newly introduced [verification material property](https://www.w3.org/TR/did-core/#verification-material) type called `compositeJwk`, which stores both types of keys within the DID document. This setup enforces the non-separability of signatures, protecting against stripping attacks.
   - **Supported Algorithms**: Currently, there are two supported algorithms: **id-MLDSA44-Ed25519-SHA512** and **id-MLDSA65-Ed25519-SHA512**. The first combines ML-DSA-44 with Ed25519, while the second combines ML-DSA-65 with Ed25519.

# Examples

To test the above functionalities, you can refer to practical code snippets available in the [example](https://github.com/Cybersecurity-LINKS/pq-zk-identity/tree/PQ/T-Hybrid/examples) directory.
> **Note**: The examples in the `example/demo` directory are configured to use the [DID Web Method](https://w3c-ccg.github.io/did-method-web/). To run these examples, you must
> have a server instance that hosts the Issuer's DID Document. You can use the default server provided in the `example/demo/server` folder, or configure one yourself. However,
> ensure that the following variables in `utils.rs` are correctly set to point to your server instance:
> ```rust
> pub static DID_URL: &str = "https://localhost:4443/.well-known/";
> pub static PATH_DID_FILE: &str = "C:/Projects/did-web-server/.well-known/";
> ```
Make sure your server is set up before running the examples to avoid any configuration issues.

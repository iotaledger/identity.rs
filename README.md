# Zero-Knowledge (ZK)

The IOTA Identity Framework now supports Zero-Knowledge functionalities, thanks to the [integration](https://github.com/iotaledger/identity.rs/pull/1285) of two key components:

* **BBS+ Signature Scheme**: This scheme has been integrated through the [ZKryptium](https://github.com/Cybersecurity-LINKS/zkryptium) library, allowing for secure and privacy-preserving credential management.
* **JSON Web Proof Representation**: The [json-proof-token](https://github.com/Cybersecurity-LINKS/json-proof-token) library implements the JSON Web Proof specification, enabling verifiable claims with selective disclosure.

For more details on the implementation and how to use these features, you can find the full documentation [here](https://wiki.iota.org/identity.rs/how-tos/verifiable-credentials/zero-knowledge-selective-disclosure/).

# PQ/T Hybrid

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

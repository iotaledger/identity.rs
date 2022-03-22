![banner](./../../../.meta/identity_banner.png)

## IOTA Identity Examples

This folder provides code examples for you to learn how the IOTA Identity WASM bindings can be used in JavaScript.

These examples are compiled with webpack for convenience but can be run independently. If you intend to use any code
examples with a published version of this package, replace `@iota/identity-wasm` imports with
`@iota/identity-wasm/node` for Node.js or `@iota/identity-wasm/web` for use in the browser.

If you are writing code against the test network then, most function calls will need to include information about the
network, since this is not automatically inferred from the arguments in all cases currently.

We recommend that you **always** use a `CLIENT_CONFIG` parameter that you define when calling any functions that take a
`ClientConfig` object. This will ensure that all the API calls use a consistent node and network throughout. If you
mismatch the network across calls you will encounter errors.

A `ClientConfig` is a record consisting of two string fields: `network` and `node`. There is an example client config
that can be found in the `config.js` file for node and in `main.js` for the browser.

### Node.js Examples

Before running the examples, make sure you have [built the bindings](../README.md#Build) for `node.js`.

- To build the examples use:
    ```bash
    npm run build:examples
    ```

- You can then run each example with:
    ```bash
    npm run example:node -- <example_name>
    ```

- For instance, to run the `create_did` example use:
    ```bash
    npm run example:node -- create_did
    ```

The following examples are currently available:

|  #  | Name                                      | Information                                                                                                                                                                                                                                |
| :-: | :---------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|  1  | [create_did](src/create_did.js)           | Generates and publishes a DID Document, the fundamental building block for decentralized identity.                                                                                                                                         |
|  2  | [manipulate_did](src/manipulate_did.js)   | Add verification methods and service endpoints to a DID Document and update an already existing DID Document.                                                                                                                              |
|  3  | [diff_chain](src/diff_chain.js)           | Creates a diff chain update for a DID Document and publishes it to the Tangle.                                                                                                                                                             |
|  4  | [resolve_history](src/resolve_history.js) | Advanced example that performs multiple diff chain and integration chain updates and demonstrates how to resolve the DID Document history to view these chains.                                                                            |
|  5  | [create_vc](src/create_vc.js)             | Generates and publishes subject and issuer DID Documents, then creates a Verifiable Credential (VC) specifying claims about the subject, and verifies it.                                                                                  |
|  6  | [create_vp](src/create_vp.js)             | Create a Verifiable Presentation, the data model for sharing VCs, out of a Verifiable Credential and verifies it.                                                                                                                          |
|  7  | [revoke_vc](src/revoke_vc.js)             | Remove a verification method from the Issuers DID Document, making the Verifiable Credential it signed unable to verify, effectively revoking the VC.                                                                                      |
|  8  | [resolution](src/resolution.js)           | Resolves an existing DID to return the latest DID Document.                                                                                                                                                                                |
|  9   | [key_exchange](src/key_exchange.js) | Demonstrates Elliptic-curve Diffie-Hellman (ECDH) cryptographic key exchange with DID Documents. |
|  10  | [merkle_key](src/merkle_key.js)           | Adds a MerkleKeyCollection verification method to an issuer's DID Document and signs a Verifiable Credential with one of its keys. Afterwards the key is deactivated, revoking the VC.                                                     |
|  11 | [private_tangle](src/private_tangle.js)   | Showcases the same procedure as `create_did`, but on a private tangle - a locally running hornet node.                                                                                                                                     |


### Browser Examples

All the Node.js examples are also available for the browser.

Before running the examples, make sure you have [built the bindings](../README.md#Build) for `web`.

- To build the examples use:
    ```bash
    npm run build:examples
    ```

- You can then run the browser examples with:
    ```bash
    npm run example:browser
    ```

Note: the webpage will be served on port 8080

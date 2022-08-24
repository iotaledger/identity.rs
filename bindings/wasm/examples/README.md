![banner](./../../../documentation/static/img/Banner/banner_identity.svg)

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
|  2  | [manipulate_did](src/manipulate_did.js)   | Add verification methods and service endpoints to a DID Document and update an already existing DID Document.                                                                                                                              | |
|  3  | [resolve_history](src/resolve_history.js) | Advanced example that performs multiple updates and demonstrates how to resolve the DID Document history to view them.                                                                            |
|  4  | [resolve_did](src/resolve_did.js)          | Resolves an existing DID to return the latest DID Document.                                                                                                                                                                                |
|  5   | [key_exchange](src/key_exchange.js) | Demonstrates Elliptic-curve Diffie-Hellman (ECDH) cryptographic key exchange with DID Documents. | |
|  6   | [private_tangle](src/private_tangle.js)   | Showcases the same procedure as `create_did`, but on a private tangle - a locally running hornet node.                                                                                                                                     |

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

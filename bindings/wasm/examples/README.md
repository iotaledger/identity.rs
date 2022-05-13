![banner](./../../../.meta/identity_banner.png)

## IOTA Identity Examples

This folder provides code examples for you to learn how the IOTA Identity WASM bindings can be used in JavaScript/Typescript.

The examples are written in Typescript and can be run independently with Node.js.

If you are writing code against the test or a private network, see the [config example](node/advanced/3_config.ts) on how to configure the account to use non default networks.

### Node.js Examples

Before running the examples, make sure you have [built the bindings](../README.md#Build) for `node.js`.

- Install dependencies in the example (`/example`) folder:
    ```bash
    npm i
    ```

- You can then run each example with from the root of the wasm bindings:
    ```bash
    npm run example:node -- <example_name>
    ```

- For instance, to run the `create_did` example use:
    ```bash
    npm run example:node -- create_did
    ```

The following examples are currently available:

| # | Name | Details |
| -------- | -------- | -------- |
|1| [create_did](node/basic/1_create_did.ts)| Generate and publish a DID Document, the fundamental building block for decentralized identity.    |
|2| [resolve_did](node/basic/2_resolve_did.js)          | Resolves an existing DID to return the latest DID Document. |  
|3| [manipulate_did](node/basic/3_manipulate_did.ts)|  Manipulate a DID Document by adding/removing Verification Methods and Services. |
|4| [create_vc](node/basic/4_create_vc.ts)             | Generate and publish subject and issuer DID Documents, then create a Verifiable Credential (VC) specifying claims about the subject, and verify it.|
|5| [create_vp](node/basic/5_create_vp.ts)             | Create a Verifiable Presentation, the data model for sharing VCs, out of a Verifiable Credential and verify it.      
|6| [revoke_vc](node/basic/6_revoke_vc.ts)             | Remove a verification method from the Issuers DID Document, making the Verifiable Credential it signed unable to verify, effectively revoking the VC.            
|7| [multiple_identities](node/basic/7_multiple_identities.ts) | Create multiple identities from a builder and load existing identities into an account. |
|8| [signing](node/basic/8_signing.ts) | Using a DID to sign arbitrary statements and validating them. |
|9| [config](node/basic/9_config.ts) | Configure the account to work with different networks and other settings. |
|10| [lazy](node/basic/10_lazy.ts)| Configure the account to allow manual batching and publishing changes for granular control | 
|11| [key_exchange](node/advanced/1_key_exchange.ts) | Use DID key-material to negotiate a shred communication key |
|12| [resolve_history](node/advanced/2_resolve_history.ts) | Resolve the history of a DID and inspect changes over historical versions |
|13| [unchecked](node/advanced/3_unchecked.ts) |  How to update the custom properties of a DID document directly by using the account's unchecked methods. |
|14| [custom_storage](node/advanced/4_custom_storage.ts) | Example implementation of a custom storage and testing it with the storage test suite. |

### Browser Examples

All examples are also available for the browser.

Before running the examples, make sure you have [built the bindings](../README.md#Build) for `web`.


- You can then run the browser examples with:
    ```bash
    npm run example:browser
    ```

Note: the webpage will be served on port 8080

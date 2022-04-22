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
|2| [manipulate_did](node/basic/2_manipulate_did.ts)|  Manipulate a DID Document by adding/removing Verification Methods and Services. |
|3| [create_vc](node/basic/3_create_vc.ts)             | Generate and publish subject and issuer DID Documents, then create a Verifiable Credential (VC) specifying claims about the subject, and verify it.|
|4| [create_vp](node/basic/4_create_vp.ts)             | Create a Verifiable Presentation, the data model for sharing VCs, out of a Verifiable Credential and verify it.      
|5| [revoke_vc](node/basic/5_revoke_vc.ts)             | Remove a verification method from the Issuers DID Document, making the Verifiable Credential it signed unable to verify, effectively revoking the VC.            
|6| [signing](node/advanced/1_signing.ts) | Using a DID to sign arbitrary statements and validating them. |
|7| [key_exchange](node/advanced/2_key_exchange.ts) | Use DID key-material to negotiate a shred communication key |
|8| [config](node/advanced/3_config.ts) | Configure the account to work with different networks and other settings. |
|9| [lazy](node/advanced/4_lazy.ts)| Configure the account to allow manual batching and publishing changes for granular control | 
|10| [multiple_identities](node/advanced/5_multiple_identities.ts) | Create multiple identities from a builder and load existing identities into an account. |
|11| [resolve_history](node/advanced/6_resolve_history.ts) | Resolve the history of a DID and inspect changes over historical versions |
|12| [unchecked](node/advanced/7_unchecked.ts) |  How to update the custom properties of a DID document directly by using the account's unchecked methods. |
|13| [custom_storage](node/advanced/8_custom_storage.ts) | Example implementation of a custom storage and testing it with the storage test suite. |

### Browser Examples

All examples are also available for the browser.

Before running the examples, make sure you have [built the bindings](../README.md#Build) for `web`.


- You can then run the browser examples with:
    ```bash
    npm run example:browser
    ```

Note: the webpage will be served on port 8080

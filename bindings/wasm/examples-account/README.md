![banner](./../../../.meta/identity_banner.png)


## IOTA Identity Account Examples

This folder provides code examples for you to learn how the IOTA Identity WASM bindings for the Account can be used in JavaScript/Typescript.

The examples are written in Typescript and can be run independently with Node.js.

### Node.js

In order to run the examples in Node.js make sure to install the dependencies:

```bash
npm install
```


And build the bindings:

```bash
npm run build
```

Then run the example using:

```bash

npm run example:account -- <example-name>
```

For instance, to run the `create_did` example use:

```bash
npm run example:account -- create_did
```

| # | Name | Details |
| -------- | -------- | -------- |
|1| [create_did](src/create_did.ts)| A basic example that generates and publishes a DID Document, the fundamental building block for decentralized identity.    |
|2| [config](src/config.ts) | How to configure the account to work with different networks and other settings. |
|3| [manipulate_did](src/manipulate_did.ts)|  How to manipulate a DID Document by adding/removing Verification Methods and Services. |
|4| [lazy](src/lazy.ts)|  How to take control over publishing DID updates manually, instead of the default automated behavior. |
|5| [signing](src/signing.ts) | Using a DID to sign arbitrary statements and validating them. |
|6| [create_vc](src/create_vc.ts)             | Generates and publishes subject and issuer DID Documents, then creates a Verifiable Credential (VC) specifying claims about the subject, and verifies it.|
|7| [create_vp](src/create_vp.ts)             | Create a Verifiable Presentation, the data model for sharing VCs, out of a Verifiable Credential and verifies it.      
|8| [revoke_vc](src/revoke_vc.ts)             | Remove a verification method from the Issuers DID Document, making the Verifiable Credential it signed unable to verify, effectively revoking the VC.            
|9| [multiple_identities](src/multiple_identities.ts) | How to create multiple identities from a builder and how to load existing identities into an account. |
|10| [unchecked](src/unchecked.ts) |  How to update the custom properties of a DID document directly by using the account's unchecked methods. |
|11| [custom_storage](src/custom_storage.ts) | Example implementation of a custom storage and testing it with the storage test suite. |
| 12 | [encryption](./account/encryption.rs) | Demonstrates Elliptic-curve Diffie-Hellman (ECDH) cryptographic key exchange by encrypting and decrypting data with a shared key. |

## Browser

Although the examples should work in browser environment, we don't provide a browser project as for now.

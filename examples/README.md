![banner](./../.meta/identity_banner.png)



## IOTA Identity Examples

This folder provides code examples for you to learn how IOTA Identity can be used.

You can run each example using

```rust
cargo run --example <example_name>
```

For Instance, to run the example `getting_started`, use

```rust
cargo run --example getting_started
```

The following examples are available for using the basic account (A high-level API):

| # | Name | Information |
| :--: | :----------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------- |
| 1 | [getting_started](./getting_started.rs) | Introductory example for you to test whether the library is set up / working properly and compiles.                        |
| 2 | [account_create](./account/create_did.rs) | A basic example that generates and publishes a DID Document, the fundamental building block for decentralized identity.    |
| 3 | [account_config](./account/config.rs) | How to configure the account to work with different networks and other settings. |
| 4 | [account_manipulate](./account/manipulate_did.rs) | How to manipulate a DID Document by adding/removing Verification Methods and Services. |
| 5 | [account_lazy](./account/lazy.rs) | How to take control over publishing DID updates manually, instead of the default automated behavior. |
| 6 | [account_signing](./account/signing.rs) | Using a DID to sign arbitrary statements and validating them. |
| 7 | [account_create_vc](account/create_vc.rs) | Generates and publishes subject and issuer DID Documents, then creates a Verifiable Credential (VC) specifying claims about the subject, and retrieves information through the CredentialValidator API. |
| 8 | [account_create_vp](account/create_vp.rs) | This example explains how to create a Verifiable Presentation from a set of credentials and sign it. |
| 9 | [account_revoke_vc](account/revoke_vc.rs) | Removes a verification method from the Issuers DID Document, making the Verifiable Credential it signed unable to verify, effectively revoking the VC. |
| 10 | [account_multiple](./account/multiple_identities.rs) | How to create multiple identities from a builder and how to load existing identities into an account. |
| 11 | [account_unchecked](./account/unchecked.rs) | How to update the custom properties of a DID document directly by using the account's unchecked methods. |
| 12 | [account_key_exchange](./account/key_exchange.rs) | Demonstrates Elliptic-curve Diffie-Hellman (ECDH) cryptographic key exchange with DID Documents. |

The following examples are available for using the low-level APIs, which provides more flexibility at the cost of complexity:

| # | Name | Information |
| :--: | :----------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------- |
| 1 | [create_did](./low-level-api/create_did.rs) | A basic example that generates and publishes a DID Document, the fundamental building block for decentralized identity. |
| 2 | [manipulate_did](low-level-api/manipulate_did.rs) | This example demonstrates how to perform a basic update to the integration chain of a DID Document. |
| 3 | [resolve_history](low-level-api/resolve_history.rs) | Advanced example that performs multiple updates and demonstrates how to resolve the DID Document history to view them. |
| 4 | [resolution](./low-level-api/resolution.rs) | A basic example that shows how to retrieve information through DID Document resolution/dereferencing. |
| 5 | [key_exchange](./low-level-api/key_exchange.rs) | Demonstrates Elliptic-curve Diffie-Hellman (ECDH) cryptographic key exchange with DID Documents. |
| 6 | [private_tangle](./low-level-api/private_tangle.rs) | Showcases the same procedure as `create_did`, but on a private tangle - a locally running hornet node.                                                                                                                                     |

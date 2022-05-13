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

The following examples are currently available:

| # | Name | Information |
| :--: | :----------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------- |
| 0 | [getting_started](./getting_started.rs) | Introductory example for you to test whether the library is set up / working properly and compiles.                        |
| 1 | [create_did](basic/1_create_did.rs) | A basic example that generates and publishes a DID Document, the fundamental building block for decentralized identity.    |
| 2 | [resolve_did](basic/2_resolve_did.rs) | A basic example that shows how to retrieve information through DID Document resolution/dereferencing. |
| 3 | [manipulate_did](basic/3_manipulate_did.rs) | How to manipulate a DID Document by adding/removing Verification Methods and Services. |
| 4 | [create_vc](basic/4_create_vc.rs) | Generates and publishes subject and issuer DID Documents, then creates a Verifiable Credential (VC) specifying claims about the subject, and retrieves information through the CredentialValidator API. |
| 5 | [create_vp](basic/5_create_vp.rs) | This example explains how to create a Verifiable Presentation from a set of credentials and sign it. |
| 6 | [revoke_vc](basic/6_revoke_vc.rs) | Removes a verification method from the Issuers DID Document, making the Verifiable Credential it signed unable to verify, effectively revoking the VC. |
| 7 | [multiple_identities](basic/7_multiple_identities.rs) | How to create multiple identities from a builder and how to load existing identities into an account. |
| 8 | [signing](basic/8_signing.rs) | Using a DID to sign arbitrary statements and validating them. |
| 9 | [config](basic/9_config.rs) | How to configure the account to work with different networks and other settings. |
| 10 | [lazy](basic/10_lazy.rs) | How to take control over publishing DID updates manually, instead of the default automated behavior. |
| 11 | [key_exchange](advanced/1_key_exchange) | Demonstrates Elliptic-curve Diffie-Hellman (ECDH) cryptographic key exchange with DID Documents. |
| 12 | [resolve_history](advanced/2_resolve_history) | Advanced example that performs multiple updates and demonstrates how to resolve the DID Document history to view them. |
| 13 | [unchecked](advanced/3_unchecked.rs) | How to update the custom properties of a DID document directly by using the account's unchecked methods. |

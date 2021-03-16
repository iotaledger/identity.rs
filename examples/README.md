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

| #    | Name                                                | Information                                                                                                                |
| :--: | :-------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------- |
| 1    | [getting_started](getting_started.rs)               | Introductory example for you to test whether the library is set up / working properly and compiles.                        |
| 2    | [create_did_document](create_did_document.rs)       | A basic example that generates and publishes a DID Document, the fundamental building block for decentralized identity.    |
| 3    | [verifiable_credential](verifiable_credential.rs)   | A basic example that generates and publishes subject and issuer DID Documents, then creates a Verifiable Credential (vc) specifying claims about the subject, and retrieves information through the CredentialValidator API. |
| 3    | [resolution](resolution.rs)                         | A basic example that generates a DID Document, publishes it to the Tangle, and retrieves information through DID Document resolution/dereferencing. |
| 4    | [diff_chain](diff_chain.rs)                         | Introductory example for you to test whether the library is set up / working properly and compiles.                        |
| 5    | [merkle_key](merkle_key.rs)                         | Introductory example for you to test whether the library is set up / working properly and compiles.                        |
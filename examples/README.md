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

| #    | Name                                                   | Information                                                                                                                |
| :--: | :----------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------- |
| 1    | [getting_started](getting_started.rs)                  | Introductory example for you to test whether the library is set up / working properly and compiles.                        |
| 2    | [create_did_document](create_did_document.rs)          | A basic example that generates and publishes a DID Document, the fundamental building block for decentralized identity.    |
| 3    | [manipulate_did_documents](manipulate_did_document.rs) | This example goes into more detail regarding the useage of DID Documents. |
| 4    | [verifiable_credential](verifiable_credential.rs)      | A basic example that generates and publishes subject and issuer DID Documents, then creates a Verifiable Credential (vc) specifying claims about the subject, and retrieves information through the CredentialValidator API. |
| 5    | [verifiable_presentation](verifiable_presentation.rs)  | This example explains how to create a Verifiable Presentation from a set of credentials and sign it. |
| 6    | [resolution](resolution.rs)                            | A basic example that generates a DID Document, publishes it to the Tangle, and retrieves information through DID Document resolution/dereferencing. |
| 7    | [diff_chain](diff_chain.rs)                            | An example that utilizes a diff and auth chain to publish updates to a DID Document. |
| 8    | [merkle_key](merkle_key.rs)                            | An example that revokes a key and shows how verification fails as a consequence. |
![banner](./../documentation/static/img/Banner/banner_identity.svg)

# IOTA Identity Examples

This folder provides code examples to learn how IOTA Identity can be used.

You can run each example using

```rust
cargo run --example <example_name>
```

For instance, to run the example `0_create_did`, use:

```rust
cargo run --example 0_create_did
```

## Basic Examples

The following basic CRUD (Create, Read, Update, Delete) examples are available:

| Name                                              | Information                                                                          |
| :------------------------------------------------ | :----------------------------------------------------------------------------------- |
| [0_create_did](./0_basic/0_create_did.rs)         | Demonstrates how to create a DID Document and publish it in a new Alias Output.      |
| [1_update_did](./0_basic/1_update_did.rs)         | Demonstrates how to update a DID document in an existing Alias Output.               |
| [2_resolve_did](./0_basic/2_resolve_did.rs)       | Demonstrates how to resolve an existing DID in an Alias Output.                      |
| [3_deactivate_did](./0_basic/3_deactivate_did.rs) | Demonstrates how to deactivate a DID in an Alias Output.                             |
| [4_delete_did](./0_basic/4_delete_did.rs)         | Demonstrates how to delete a DID in an Alias Output, reclaiming the storage deposit. |

## Advanced Examples

The following advanced examples are available:

| Name                                                       | Information                                                                                              |
| :--------------------------------------------------------- | :------------------------------------------------------------------------------------------------------- |
| [0_did_controls_did](./1_advanced/0_did_controls_did.rs)   | Demonstrates how an identity can control another identity.                                               |
| [1_did_issues_nft](./1_advanced/1_did_issues_nft.rs)       | Demonstrates how an identity can issue and own NFTs, and how observers can verify the issuer of the NFT. |
| [2_nft_owns_did](./1_advanced/2_nft_owns_did.rs)           | Demonstrates how an identity can be owned by NFTs, and how observers can verify that relationship.       |
| [3_did_issues_tokens](./1_advanced/3_did_issues_tokens.rs) | Demonstrates how an identity can issue and control a Token Foundry and its tokens.                       |
| [4_key_exchange](./1_advanced/4_key_exchange.rs)           | Demonstrates Elliptic-curve Diffie-Hellman (ECDH) cryptographic key exchange with DID Documents.         |

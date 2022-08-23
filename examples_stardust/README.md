![banner](./../.meta/identity_banner.png)

# IOTA Identity Examples

This folder provides code examples to learn how IOTA Identity can be used.

You can run each example using

```rust
cargo run --example <example_name>
```

For instance, to run the example `00_create_did`, use:

```rust
cargo run --example 00_create_did
```

## CRUD Examples

The following basic CRUD (Create, Read, Update, Delete) examples are available:

| Name                                             | Information                                                                          |
| :----------------------------------------------- | :----------------------------------------------------------------------------------- |
| [00_create_did](./crud/00_create_did.rs)         | Demonstrates how to create a DID Document and publish it in a new Alias Output.      |
| [01_update_did](./crud/01_update_did.rs)         | Demonstrates how to update a DID document in an existing Alias Output.               |
| [02_resolve_did](./crud/02_resolve_did.rs)       | Demonstrates how to resolve an existing DID in an Alias Output.                      |
| [03_deactivate_did](./crud/03_deactivate_did.rs) | Demonstrates how to deactivate a DID in an Alias Output.                             |
| [04_delete_did](./crud/04_delete_did.rs)         | Demonstrates how to delete a DID in an Alias Output, reclaiming the storage deposit. |

## Advanced Examples

The following advanced examples, showing how DID Alias Outputs can interact with other Layer 1 primitives, are available:

| Name                                                       | Information                                                                                              |
| :--------------------------------------------------------- | :------------------------------------------------------------------------------------------------------- |
| [05_did_controls_did](./advanced/05_did_controls_did.rs)   | Demonstrates how an identity can control another identity.                                               |
| [06_did_issues_nft](./advanced/06_did_issues_nft.rs)       | Demonstrates how an identity can issue and own NFTs, and how observers can verify the issuer of the NFT. |
| [07_nft_owns_did](./advanced/07_nft_owns_did.rs)           | Demonstrates how an identity can be owned by NFTs, and how observers can verify that relationship.       |
| [08_did_issues_tokens](./advanced/08_did_issues_tokens.rs) | Demonstrates how an identity can issue and control native assets such as Token Foundries and NFTs.       |

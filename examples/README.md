![banner](https://github.com/iotaledger/identity.rs/raw/HEAD/.github/banner_identity.svg)

# IOTA Identity Examples

This folder provides code examples to learn how IOTA Identity can be used.

You can run each example using

```rust
cargo run --example <example_name>
```

For instance, to run the example `0_create_did`, use:

```rust
cargo run --release --example 0_create_did
```

### Note: Running the examples with the release flag will be significantly faster due to stronghold performance issues in debug mode.

## Basic Examples

The following basic CRUD (Create, Read, Update, Delete) examples are available:

| Name                                                    | Information                                                                          |
| :------------------------------------------------------ | :----------------------------------------------------------------------------------- |
| [0_create_did](./0_basic/0_create_did.rs)               | Demonstrates how to create a DID Document and publish it in a new identity.          |
| [1_update_did](./0_basic/1_update_did.rs)               | Demonstrates how to update a DID document in an existing identity.                   |
| [2_resolve_did](./0_basic/2_resolve_did.rs)             | Demonstrates how to resolve an existing DID in an identity.                          |
| [3_deactivate_did](./0_basic/3_deactivate_did.rs)       | Demonstrates how to deactivate a DID in an identity.                                 |
| [5_create_vc](./0_basic/5_create_vc.rs)                 | Demonstrates how to create and verify verifiable credentials.                        |
| [6_create_vp](./0_basic/6_create_vp.rs)                 | Demonstrates how to create and verify verifiable presentations.                      |
| [7_revoke_vc](./0_basic/7_revoke_vc.rs)                 | Demonstrates how to revoke a verifiable credential.                                  |
| [8_legacy_stronghold](./0_basic/8_legacy_stronghold.rs) | Demonstrates how to use stronghold for secure storage..                              |

## Advanced Examples

The following advanced examples are available:

| Name                                                                                   | Information                                                                                              |
| :------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------- |
| [4_identity_history](./1_advanced/4_identity_history.rs)                               | Demonstrates fetching the history of an identity.                                                        |
| [5_custom_resolution](./1_advanced/5_custom_resolution.rs)                             | Demonstrates how to set up a resolver using custom handlers.                                             |
| [6_domain_linkage](./1_advanced/6_domain_linkage)                                      | Demonstrates how to link a domain and a DID and verify the linkage.                                      |
| [7_sd_jwt](./1_advanced/7_sd_jwt)                                                      | Demonstrates how to create and verify selective disclosure verifiable credentials.                       |
| [8_status_list_2021](./1_advanced/8_status_list_2021.rs)                               | Demonstrates how to revoke a credential using `StatusList2021`.                                          |
| [9_zkp](./1_advanced/9_zkp.rs)                                                         | Demonstrates how to create an Anonymous Credential with BBS+.                                            |
| [10_zkp_revocation](./1_advanced/10_zkp_revocation.rs)                                 | Demonstrates how to revoke a credential.                                                                 |
| [11_linked_verifiable_presentation](./1_advanced/11_linked_verifiable_presentation.rs) | Demonstrates how to link a public Verifiable Presentation to an identity and how it can be verified.     |

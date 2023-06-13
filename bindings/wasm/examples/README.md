![banner](./../../../documentation/static/img/Banner/banner_identity.svg)

## IOTA Identity Examples

The following code examples demonstrate how to use the IOTA Identity Wasm bindings in JavaScript/TypeScript.

The examples are written in TypeScript and can be run with Node.js.

### Node.js

Install the dependencies:

```bash
npm install
```

Build the bindings:

```bash
npm run build
```

Then, run an example using:

```bash
npm run example:node -- <example-name>
```

For instance, to run the `0_create_did` example execute:

```bash
npm run example:node -- 0_create_did
```

## Basic Examples

The following basic CRUD (Create, Read, Update, Delete) examples are available:

| Name                                                | Information                                                                          |
| :-------------------------------------------------- | :----------------------------------------------------------------------------------- |
| [0_create_did](src/0_basic/0_create_did.ts)         | Demonstrates how to create a DID Document and publish it in a new Alias Output.      |
| [1_update_did](src/0_basic/1_update_did.ts)         | Demonstrates how to update a DID document in an existing Alias Output.               |
| [2_resolve_did](src/0_basic/2_resolve_did.ts)       | Demonstrates how to resolve an existing DID in an Alias Output.                      |
| [3_deactivate_did](src/0_basic/3_deactivate_did.ts) | Demonstrates how to deactivate a DID in an Alias Output.                             |
| [4_delete_did](src/0_basic/4_delete_did.ts)         | Demonstrates how to delete a DID in an Alias Output, reclaiming the storage deposit. |
| [5_create_vc](src/0_basic/5_create_vc.ts)           | Demonstrates how to create and verify verifiable credentials.                        |
| [6_create_vp](src/0_basic/6_create_vp.ts)           | Demonstrates how to create and verify verifiable presentations.                      |
| [7_revoke_vc](src/0_basic/7_revoke_vc.ts)           | Demonstrates how to revoke a verifiable credential.                                  |

## Advanced Examples

The following advanced examples are available:

| Name                                                         | Information                                                                                              |
| :----------------------------------------------------------- | :------------------------------------------------------------------------------------------------------- |
| [0_did_controls_did](src/1_advanced/0_did_controls_did.ts)   | Demonstrates how an identity can control another identity.                                               |
| [1_did_issues_nft](src/1_advanced/1_did_issues_nft.ts)       | Demonstrates how an identity can issue and own NFTs, and how observers can verify the issuer of the NFT. |
| [2_nft_owns_did](src/1_advanced/2_nft_owns_did.ts)           | Demonstrates how an identity can be owned by NFTs, and how observers can verify that relationship.       |
| [3_did_issues_tokens](src/1_advanced/3_did_issues_tokens.ts) | Demonstrates how an identity can issue and control a Token Foundry and its tokens.                       |
| [4_custom_resolution](src/1_advanced/4_custom_resolution.ts) | Demonstrates how to set up a resolver using custom handlers.                                             |
| [5_domain_linkage](src/1_advanced/5_domain_linkage.ts)       | Demonstrates how to link a domain and a DID and verify the linkage.                                      |

## Browser

While the examples should work in a browser environment, we do not provide browser examples yet.

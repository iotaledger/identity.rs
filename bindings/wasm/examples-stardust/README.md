![banner](./../../../documentation/static/img/Banner/banner_identity.svg)

## IOTA Identity UTXO Examples

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
npm run example:stardust -- <example-name>
```

For instance, to run the `ex0_create_did` example execute:

```bash
npm run example:stardust -- ex0_create_did
```

| #   | Name                                            | Details                                                          |
|-----|-------------------------------------------------|------------------------------------------------------------------|
| 0   | [ex0_create_did](src/ex0_create_did.ts)         | Create a DID Document and publish it in a new Alias Output.      |
| 1   | [ex1_update_did](src/ex1_update_did.ts)         | Update a DID document in an existing Alias Output.               |
| 2   | [ex2_resolve_did](src/ex2_resolve_did.ts)       | Resolve an existing DID in an Alias Output.                      |
| 3   | [ex3_deactivate_did](src/ex3_deactivate_did.ts) | Deactivate a DID in an Alias Output.                             |
| 4   | [ex4_delete_did](src/ex4_delete_did.ts)         | Delete a DID in an Alias Output, reclaiming the storage deposit. |

## Browser

While the examples should work in a browser environment, we do not provide browser examples yet.

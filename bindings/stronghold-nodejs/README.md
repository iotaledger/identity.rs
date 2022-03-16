# IOTA Identity Stronghold Node.js

This is the beta version of the official Stronghold Account Storage plugin for Node.js with [IOTA Identity Wasm](https://github.com/iotaledger/identity.rs/tree/main/bindings/wasm).

## Install the plugin:

Latest Release: this version matches the main branch of this repository and is stable.
```bash
npm install @iota/identity-stronghold-nodejs
```

Development Release: this version matches the dev branch of this repository, may see frequent breaking changes and has the latest code changes.
```bash
npm install @iota/identity-stronghold-nodejs@dev
```

## Build

Alternatively, you can build the bindings if you have Rust installed. If not, refer to [rustup.rs](https://rustup.rs) for the installation. Then install the necessary dependencies using:
```bash
npm install
```

To use the latest local changes from the Wasm bindings follow the build instructions from the [IOTA Identity Wasm build section](https://github.com/iotaledger/identity.rs/tree/main/bindings/#build) and then link the result with:

```bash
cd ../wasm
npm link
cd ../stronghold-nodejs
npm link @iota/identity-wasm
```

and then build the bindings

```bash
npm run build
```
If you linked the Wasm bindings, don't forget to unlink before packaging the module.

```bash
npm unlink --no-save @iota/identity-wasm
npm install
```

## Minimum Requirements

The minimum supported version for node is: `v16.0.0`



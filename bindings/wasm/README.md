# IOTA Identity WASM

> This is the beta version of the official WASM bindings for [IOTA Identity](https://github.com/iotaledger/identity.rs).

## [API Reference](https://wiki.iota.org/identity.rs/libraries/wasm/api_reference)

## [Account Examples](https://github.com/iotaledger/identity.rs/blob/main/bindings/wasm/examples-account/README.md)
## [Low-Level Examples](https://github.com/iotaledger/identity.rs/blob/main/bindings/wasm/examples/README.md)

## Install the library:

Latest Release: this version matches the main branch of this repository, is stable and will have changelogs.
```bash
npm install @iota/identity-wasm
```

Development Release: this version matches the dev branch of this repository, may see frequent breaking changes and has the latest code changes.
```bash
npm install @iota/identity-wasm@dev
```

## Build

Alternatively, you can build the bindings if you have Rust installed. If not, refer to [rustup.rs](https://rustup.rs) for the installation. 

Install [`wasm-bindgen-cli`](https://github.com/rustwasm/wasm-bindgen). A manual installation is required because we use the [Weak References](https://rustwasm.github.io/wasm-bindgen/reference/weak-references.html) feature, which [`wasm-pack` does not expose](https://github.com/rustwasm/wasm-pack/issues/930).

```bash
cargo install --force wasm-bindgen-cli
```

Then, install the necessary dependencies using:
```bash
npm install
```

and build the bindings for `node.js` with

```bash
npm run build:nodejs
```

or for the `web` with

```bash
npm run build:web
```

## Minimum Requirements

The minimum supported version for node is: `v16.0.0`

## NodeJS Usage
<!-- 
Test this example using https://github.com/anko/txm: `txm README.md`

Replace imports with local paths for txm:
!test program
cat \
| sed -e "s#require('@iota/identity-wasm/node')#require('./node/identity_wasm.js')#" \
| node
-->
<!-- !test check Nodejs Example -->
```javascript
const identity = require('@iota/identity-wasm/node')

async function main() {

    // The creation step generates a keypair, builds an identity
    // and publishes it to the IOTA mainnet.
    const builder = new identity.AccountBuilder();
    const account = await builder.createIdentity();

    // Retrieve the DID of the newly created identity.
    const did = account.did();

    // Print the DID of the created Identity.
    console.log(did.toString())

    // Print the local state of the DID Document
    console.log(account.document());

    // Print the Explorer URL for the DID.
    console.log(`Explorer Url:`, identity.ExplorerUrl.mainnet().resolverUrl(did));
}

main()
```

## Web Setup

The library loads the WASM file with an HTTP GET request, so the .wasm file must be copied to the root of the dist folder.

### Rollup

- Install `rollup-plugin-copy`:

```bash
$ npm install rollup-plugin-copy --save-dev
```

- Add the copy plugin usage to the `plugins` array under `rollup.config.js`:

```js
// Include the copy plugin
import copy from 'rollup-plugin-copy'

// Add the copy plugin to the `plugins` array of your rollup config:
copy({
  targets: [{
    src: 'node_modules/@iota/identity-wasm/web/identity_wasm_bg.wasm',
    dest: 'public',
    rename: 'identity_wasm_bg.wasm'
  }]
})
```

### Webpack

- Install `copy-webpack-plugin`:

```bash
$ npm install copy-webpack-plugin --save-dev
```

```js
// Include the copy plugin
const CopyWebPlugin= require('copy-webpack-plugin');

// Add the copy plugin to the `plugins` array of your webpack config:

new CopyWebPlugin({
  patterns: [
    {
      from: 'node_modules/@iota/identity-wasm/web/identity_wasm_bg.wasm',
      to: 'identity_wasm_bg.wasm'
    }
  ]
}),
```

### Web Usage

```js
import * as identity from "@iota/identity-wasm/web";

identity.init().then(() => {

  // The creation step generates a keypair, builds an identity
  // and publishes it to the IOTA mainnet.
  let builder = new identity.AccountBuilder();
  let account = await builder.createIdentity();

  // Retrieve the DID of the newly created identity.
  const did = account.did();

  // Print the DID of the created Identity.
  console.log(did.toString())

  // Print the local state of the DID Document
  console.log(account.document());

});

// or

(async () => {
  
  await identity.init()
    
  // The creation step generates a keypair, builds an identity
  // and publishes it to the IOTA mainnet.
  let builder = new identity.AccountBuilder();
  let account = await builder.createIdentity();

  // Retrieve the DID of the newly created identity.
  const did = account.did();

  // Print the DID of the created Identity.
  console.log(did.toString())

  // Print the local state of the DID Document
  console.log(account.document());
  
})()

// Default path is "identity_wasm_bg.wasm", but you can override it like this
await identity.init("./static/identity_wasm_bg.wasm");
```

`identity.init().then(<callback>)` or `await identity.init()` is required to load the wasm file (from the server if not available, because of that it will only be slow for the first time)

## Examples in the Wild

You may find it useful to see how the WASM bindings are being used in existing applications:

- [Zebra IOTA Edge SDK](https://github.com/ZebraDevs/Zebra-Iota-Edge-SDK) (mobile apps using Capacitor.js + Svelte)

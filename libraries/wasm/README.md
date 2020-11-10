# IOTA Identity WASM

> This is the alpha version of the official WASM binding to IOTA's Identity API.

## Install the library:

$ npm install iota-identity-wasm-test
// or using yarn
$ yarn add iota-identity-wasm-test

## NodeJS Setup

```js
const identity = require('iota-identity-wasm-test/node')

// Generate Keypairs
const alice_keypair = new identity.Key()
console.log("alice_keypair: ", alice_keypair)

// Create the DIDs
let alice_did = new identity.DID(alice_keypair.public)
console.log("alice_did: ", alice_did.toString(), alice_did.address)

// Create the DID Documents
let alice_document = new identity.Doc({ did: alice_did.did, key: alice_keypair.public })
console.log("alice_document: ", alice_document.document)

```

## Web Setup

The library loads the WASM file with an HTTP GET request, so the .wasm file must be copied to the root of the dist folder.

### Rollup

- Install `rollup-plugin-copy`:

```bash
$ npm install rollup-plugin-copy
// or using yarn
$ yarn add rollup-plugin-copy
```

- Add the copy plugin usage to the `plugins` array under `rollup.config.js`:

```js
copy({
    targets: [{
        src: 'node_modules/iota-identity-wasm-test/web/iota_identity_wasm_bg.wasm',
        dest: 'public',
        rename: 'iota_identity_wasm_bg.wasm'
    }]
})
```

### Webpack

- Install `copy-webpack-plugin`:

```bash
$ npm install copy-webpack-plugin --save-dev
// or using yarn
$ yarn add copy-webpack-plugin --dev
```

```js
// Inluce the copy plugin
const CopyWebPlugin= require('copy-webpack-plugin');

// Add the copy plugin to the `plugins` array of your webpack config:

new CopyWebPlugin({
    patterns: [
        {
          from: 'node_modules/iota-identity-wasm-test/web/iota_identity_wasm_bg.wasm',
          to: 'iota_identity_wasm_bg.wasm'
        }
    ]
}),
```

### Usage

```js
import loadWasm, * as identityLib from "iota-identity-wasm-test/web/";

loadWasm().then(() => {
    let keyPair = identityLib.Key.ed25519();
    console.log("keyPair", keyPair);
    let did = new identityLib.DID(keyPair);
    console.log("did", did);
});

// or

(async () => {
    await loadWasm()
    let keyPair = identityLib.Key.ed25519();
    console.log("keyPair", keyPair);
    let did = new identityLib.DID(keyPair);
    console.log("did", did);
 })();

```

`identity().then((lib) => {` or `let lib = await identity()` is required to load the wasm file (from the server if not available, because of that it will only be slow for the first time)

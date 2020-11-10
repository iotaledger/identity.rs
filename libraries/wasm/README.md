# IOTA Identity WASM

> This is the alpha version of the official WASM binding to IOTA's Identity API.

## Install the library:

```bash
$ npm install iota-identity-wasm-test
// or using yarn
$ yarn add iota-identity-wasm-test
```

## NodeJS Setup

```js
const identity = require('iota-identity-wasm-test/node')

// Call the initialize function to get better error messages from Wasm
identity.initialize()

// Generate Keypair
const alice_keypair = identity.Key.generateEd25519()
console.log("alice_keypair: ", alice_keypair)

// Create the DIDs
let alice_did = new identity.DID(alice_keypair)
console.log("alice_did and IOTA address: ", alice_did.toString(), alice_did.address)

// Create the public key
let alice_pubkey = identity.PubKey.generateEd25519(alice_did, alice_keypair.public)
console.log("alice_pubkey: ", alice_pubkey);

// Create the DID Documents
let alice_document = new identity.Doc(alice_pubkey)
console.log("alice_document: ", alice_document)
```

## Web Setup

The library loads the WASM file with an HTTP GET request, so the .wasm file must be copied to the root of the dist folder.

### Rollup

- Install `rollup-plugin-copy`:

```bash
$ npm install rollup-plugin-copy --save-dev
// or using yarn
$ yarn add rollup-plugin-copy --dev
```

- Add the copy plugin usage to the `plugins` array under `rollup.config.js`:

```js
// Inluce the copy plugin
import copy from 'rollup-plugin-copy'

// Add the copy plugin to the `plugins` array of your rollup config:
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
import * as identity from "iota-identity-wasm-test/web/";

identity.init().then(() => {
    let keyPair = identity.Key.ed25519();
    console.log("keyPair", keyPair);
    let did = new identity.DID(keyPair);
    console.log("did", did);
});

// or

(async () => {
    await identity.init()
    let keyPair = identity.Key.ed25519();
    console.log("keyPair", keyPair);
    let did = new identity.DID(keyPair);
    console.log("did", did);
 })();

```

`identity.init().then(() => {` or `await identity.init()` is required to load the wasm file (from the server if not available, because of that it will only be slow for the first time)

# IOTA Identity WASM

> This is the beta version of the official WASM bindings for [IOTA Identity](https://github.com/iotaledger/identity.rs).

## [API Reference](docs/api-reference.md)

## Install the library:

Latest Release: This version matches the main branch of this repository, is stable and will have changelogs.
```bash
$ npm install @iota/identity-wasm
// or using yarn
$ yarn add @iota/identity-wasm
```

Development Release: This version matches the dev branch of this repository, may see frequent breaking changes and has the latest code changes.
```bash
$ npm install @iota/identity-wasm@dev
// or using yarn
$ yarn add @iota/identity-wasm@dev
```

## NodeJS Setup

```js
const identity = require('@iota/identity-wasm/node')

// Generate a new KeyPair
const key = new identity.KeyPair(identity.KeyType.Ed25519)

// Create a new DID Document with the KeyPair as the default authentication method
const doc = identity.Document.fromKeyPair(key)

// Sign the DID Document with the sceret key
doc.sign(key)

// Publish the DID Document to the IOTA Tangle
identity.publish(doc.toJSON(), { node: "https://nodes.thetangle.org:443" })
  .then((message) => {
    console.log("Tangle Message Id: ", message)
    console.log("Tangle Message Url", `https://explorer.iota.org/mainnet/transaction/${message}`)
  }).catch((error) => {
    console.error("Error: ", error)
  })
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
// or using yarn
$ yarn add copy-webpack-plugin --dev
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

### Usage

```js
import * as identity from "@iota/identity-wasm/web";

identity.init().then(() => {
  const key = new identity.KeyPair(identity.KeyType.Ed25519)
  const doc = identity.Document.fromKeyPair(key)
  // Or, if using the testnet:
  // const doc = identity.Document.fromKeyPair(key, "test")  
  console.log("Key Pair", key)
  console.log("DID Document: ", doc)
});

// or

(async () => {
  await identity.init()
  const key = new identity.KeyPair(identity.KeyType.Ed25519)
  const doc = identity.Document.fromKeyPair(key)
  // Or, if using the testnet:
  // const doc = identity.Document.fromKeyPair(key, "test")
  console.log("Key Pair", key)
  console.log("DID Document: ", doc)
})()

// Default path is "identity_wasm_bg.wasm", but you can override it like this
await identity.init("./static/identity_wasm_bg.wasm");
```

`identity.init().then(<callback>)` or `await identity.init()` is required to load the wasm file (from the server if not available, because of that it will only be slow for the first time)

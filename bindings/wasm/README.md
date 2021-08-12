# IOTA Identity WASM

> This is the beta version of the official WASM bindings for [IOTA Identity](https://github.com/iotaledger/identity.rs).

## [API Reference](https://identity.docs.iota.org/docs/libraries/wasm/api_reference)

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

## Build

Alternatively, you can build the bindings if you have Rust installed. If not, refer to [rustup.rs](https://rustup.rs) for the installation. Then install the necessary dependencies using:

```yarn``` or ```npm install```

and then build the bindings for `node.js` with

```npm run build:nodejs```

or for the `web` with

```npm run build:web```

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

// Generate a new KeyPair
const key = new identity.KeyPair(identity.KeyType.Ed25519)

// Create a new DID Document with the KeyPair as the default authentication method
const doc = identity.Document.fromKeyPair(key)
// const doc = identity.Document.fromKeyPair(key, "test") // if using the testnet

// Sign the DID Document with the private key
doc.sign(key)

// Create a default client instance for the mainnet
const config = identity.Config.fromNetwork(identity.Network.mainnet())
// const config = identity.Config.fromNetwork(identity.Network.testnet()); // if using the testnet
const client = identity.Client.fromConfig(config)

// Publish the DID Document to the IOTA Tangle
// The message can be viewed at https://explorer.iota.org/<mainnet|testnet>/transaction/<messageId>
client.publishDocument(doc.toJSON())
    .then((receipt) => {
        console.log("Tangle Message Receipt: ", receipt)
        console.log("Tangle Message Url:", doc.id.network.messageURL(receipt.messageId))
    })
    .catch((error) => {
        console.error("Error: ", error)
        throw error
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

### Web Usage

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

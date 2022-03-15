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
    // Choose the Tangle network to publish on.
    const network = identity.Network.mainnet();
    // const network = identity.Network.devnet();

    // Generate a new Ed25519 KeyPair.
    const key = new identity.KeyPair(identity.KeyType.Ed25519);

    // Create a new DID Document using the KeyPair for the default VerificationMethod.
    const doc = new identity.Document(key, network.name());

    // Sign the DID Document with the private key.
    doc.signSelf(key, doc.defaultSigningMethod().id());

    // Create a default client instance for the network.
    const client = await identity.Client.fromConfig({network: network});
    
    // Publish the DID Document to the IOTA Tangle.
    const receipt = await client.publishDocument(doc);

    // The message can be viewed at https://explorer.iota.org/<mainnet|devnet>/identity-resolver/<did>
    const explorer = identity.ExplorerUrl.mainnet();
    // const explorer = identity.ExplorerUrl.devnet(); // if using the devnet
    console.log("Tangle Message Receipt:", receipt);
    console.log("Tangle Explorer Url:", explorer.resolverUrl(doc.id));
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
  const key = new identity.KeyPair(identity.KeyType.Ed25519)
  const doc = new identity.Document(key)
  // const doc = new identity.Document(key, "dev") // if using the devnet
  console.log("Key Pair: ", key)
  console.log("DID Document: ", doc)
});

// or

(async () => {
  await identity.init()
  const key = new identity.KeyPair(identity.KeyType.Ed25519)
  const doc = new identity.Document(key)
  // const doc = new identity.Document(key, "dev") // if using the devnet
  console.log("Key Pair: ", key)
  console.log("DID Document: ", doc)
})()

// Default path is "identity_wasm_bg.wasm", but you can override it like this
await identity.init("./static/identity_wasm_bg.wasm");
```

`identity.init().then(<callback>)` or `await identity.init()` is required to load the wasm file (from the server if not available, because of that it will only be slow for the first time)

# IOTA Identity WASM

> This is the beta version of the official WASM bindings for [IOTA Identity](https://github.com/iotaledger/identity.rs).

## [API Reference](https://wiki.iota.org/identity.rs/libraries/wasm/api_reference)

## [Account Examples](https://github.com/iotaledger/identity.rs/blob/main/bindings/wasm/examples-account/README.md)

## [Low-Level Examples](https://github.com/iotaledger/identity.rs/blob/main/bindings/wasm/examples/README.md)

## Install the library:

Latest Release: this version matches the `main` branch of this repository, is stable and will have changelogs.

```bash
npm install @iota/identity-wasm
```

## Build

Alternatively, you can build the bindings yourself if you have Rust installed. If not, refer to [rustup.rs](https://rustup.rs) for the installation.

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

The minimum supported version for node is: `v16`

## NodeJS Usage

The following code creates a new IOTA DID Document suitable for publishing to a locally running private network.
See the [instructions](https://wiki.iota.org/hornet/develop/how_tos/private_tangle) on running your own private network.

<!--
Test this example using https://github.com/anko/txm: `txm README.md`

Replace imports with local paths for txm:
!test program
cat | sed -e "s#require('@iota/identity-wasm/node')#require('./node')#" | node
-->
<!-- !test check Nodejs Example -->
```typescript
const {
  KeyPair,
  KeyType,
  MethodScope,
  IotaDocument,
  IotaVerificationMethod,
  IotaService,
  MethodRelationship,
  IotaIdentityClient,
} = require('@iota/identity-wasm/node');
const { Client } = require('@iota/iota-client-wasm/node');

// The endpoint of the IOTA node to use.
const API_ENDPOINT = "http://127.0.0.1:14265";

/** Demonstrate how to create a DID Document. */
async function main() {
  // Create a new client with the given network endpoint.
  const client = new Client({
    primaryNode: API_ENDPOINT,
    localPow: true,
  });

  const didClient = new IotaIdentityClient(client);

  // Get the Bech32 human-readable part (HRP) of the network.
  const networkHrp = await didClient.getNetworkHrp();

  // Create a new DID document with a placeholder DID.
  // The DID will be derived from the Alias Id of the Alias Output after publishing.
  const document = new IotaDocument(networkHrp);

  // Insert a new Ed25519 verification method in the DID document.
  let keypair = new KeyPair(KeyType.Ed25519);
  let method = new IotaVerificationMethod(
    document.id(),
    keypair.type(),
    keypair.public(),
    "#key-1"
  );
  document.insertMethod(method, MethodScope.VerificationMethod());

  // Attach a new method relationship to the existing method.
  document.attachMethodRelationship(
    document.id().join("#key-1"),
    MethodRelationship.Authentication
  );

  // Add a new Service.
  const service = new IotaService({
    id: document.id().join("#linked-domain"),
    type: "LinkedDomains",
    serviceEndpoint: "https://iota.org/",
  });
  document.insertService(service);

  console.log(`Created document `, JSON.stringify(document.toJSON(), null, 2));
}

main();
```

which prints

```
Created document  {
  "doc": {
    "id": "did:iota:0x0000000000000000000000000000000000000000000000000000000000000000",
    "verificationMethod": [
      {
        "id": "did:iota:0x0000000000000000000000000000000000000000000000000000000000000000#key-1",
        "controller": "did:iota:0x0000000000000000000000000000000000000000000000000000000000000000",
        "type": "Ed25519VerificationKey2018",
        "publicKeyMultibase": "z4SxypezRxr1YdMAJBePfHGxZ9hNZ53WVixZq3PbUcztW"
      }
    ],
    "authentication": [
      "did:iota:0x0000000000000000000000000000000000000000000000000000000000000000#key-1"
    ],
    "service": [
      {
        "id": "did:iota:0x0000000000000000000000000000000000000000000000000000000000000000#linked-domain",
        "type": "LinkedDomains",
        "serviceEndpoint": "https://iota.org/"
      }
    ]
  },
  "meta": {
    "created": "2022-09-09T11:29:32Z",
    "updated": "2022-09-09T11:29:32Z"
  }
}
```

**NOTE: see the [examples](https://github.com/iotaledger/identity.rs/blob/main/bindings/wasm/examples/README.md) for how to publish an IOTA DID Document.**

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
import copy from "rollup-plugin-copy";

// Add the copy plugin to the `plugins` array of your rollup config:
copy({
  targets: [
    {
      src: "node_modules/@iota/iota-client-wasm/web/wasm/client_wasm_bg.wasm",
      dest: "public",
      rename: "client_wasm_bg.wasm",
    },
    {
      src: "node_modules/@iota/identity-wasm/web/identity_wasm_bg.wasm",
      dest: "public",
      rename: "identity_wasm_bg.wasm",
    },
  ],
});
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
      from: 'node_modules/@iota/iota-client-wasm/web/wasm/client_wasm_bg.wasm',
      to: 'client_wasm_bg.wasm'
    },
    {
      from: 'node_modules/@iota/identity-wasm/web/identity_wasm_bg.wasm',
      to: 'identity_wasm_bg.wasm'
    }
  ]
}),
```

### Web Usage

```typescript
import * as client from "@iota/iota-client-wasm/web";
import * as identity from "@iota/identity-wasm/web";

/** Demonstrate how to create a DID Document. */
async function createDocument() {
  // Create a new client with the given network endpoint.
  const iotaClient = new client.Client({
    primaryNode: API_ENDPOINT,
    localPow: true,
  });

  const didClient = new identity.IotaIdentityClient(iotaClient);

  // Get the Bech32 human-readable part (HRP) of the network.
  const networkHrp = await didClient.getNetworkHrp();

  // Create a new DID document with a placeholder DID.
  // The DID will be derived from the Alias Id of the Alias Output after publishing.
  const document = new identity.IotaDocument(networkHrp);

  // Insert a new Ed25519 verification method in the DID document.
  let keypair = new identity.KeyPair(identity.KeyType.Ed25519);
  let method = new identity.IotaVerificationMethod(
    document.id(),
    keypair.type(),
    keypair.public(),
    "#key-1"
  );
  document.insertMethod(method, identity.MethodScope.VerificationMethod());

  // Attach a new method relationship to the existing method.
  document.attachMethodRelationship(
    document.id().join("#key-1"),
    identity.MethodRelationship.Authentication
  );

  // Add a new Service.
  const service = new identity.IotaService({
    id: document.id().join("#linked-domain"),
    type: "LinkedDomains",
    serviceEndpoint: "https://iota.org/",
  });
  document.insertService(service);

  console.log(`Created document `, JSON.stringify(document.toJSON(), null, 2));
}

client
  .init()
  .then(() => identity.init())
  .then(() => {
    await createDocument();
  });

// or

(async () => {
  await client.init();
  await identity.init();

  await createDocument();
})();

// Default path is "identity_wasm_bg.wasm", but you can override it like this
await identity.init("./static/identity_wasm_bg.wasm");
```

Calling `identity.init().then(<callback>)` or `await identity.init()` is required to load the Wasm file from the server if not available, because of that it will only be slow for the first time.

**NOTE: see the [examples](https://github.com/iotaledger/identity.rs/blob/main/bindings/wasm/examples/README.md) for how to publish an IOTA DID Document.**

## Examples in the Wild

You may find it useful to see how the WASM bindings are being used in existing applications:

- [Zebra IOTA Edge SDK](https://github.com/ZebraDevs/Zebra-Iota-Edge-SDK) (mobile apps using Capacitor.js + Svelte)

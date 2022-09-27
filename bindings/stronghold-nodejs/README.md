# IOTA Identity Stronghold Node.js

This is the beta version of the official Stronghold Account Storage plugin for Node.js with [IOTA Identity Wasm](https://github.com/iotaledger/identity.rs/tree/main/bindings/wasm).

## Install the plugin:

Latest Release: this version matches the main branch of this repository and is stable.
```bash
npm install @iota/identity-stronghold-nodejs
```
## Usage
<!-- 
Test this example using https://github.com/anko/txm: `txm README.md`

Replace imports with local paths for txm:
!test program
cat \
| sed -e "s#require('@iota/identity-wasm/node')#require('../wasm/node/identity_wasm.js')#" \
| sed -e "s#require('@iota/identity-stronghold-nodejs')#require('./dist/index.js')#" \
| node
-->
<!-- !test check Nodejs Example -->
```javascript
const { AccountBuilder, ExplorerUrl } = require('@iota/identity-wasm/node')
const { Stronghold } = require('@iota/identity-stronghold-nodejs')


async function main() {
    // Stronghold settings for the Account storage.
    // This will load an existing Stronghold or create a new one automatically.
    const filepath = "./example-strong.hodl";
    const password = "my-password";
    const stronghold = await Stronghold.build(filepath, password);
    
    // This generates a new keypair stored securely in the above Stronghold, 
    // constructs a new DID Document, and publishes it to the IOTA Mainnet.
    let builder = new AccountBuilder({
        storage: stronghold,
    });
    let account = await builder.createIdentity();

    // Print the DID of the newly created identity.
    const did = account.did();
    console.log(did.toString());

    // Print the local state of the DID Document.
    const document = account.document();
    console.log(JSON.stringify(document, null, 2));

    // Print the Explorer URL for the DID.
    console.log(`Explorer URL:`, ExplorerUrl.mainnet().resolverUrl(did));
}

main()
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



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
sed -e "s#require('@iota/identity-stronghold-nodejs')#require('./dist/index.js')#" && node
-->
<!-- !test check Nodejs Example -->
```javascript
const { AccountBuilder, ExplorerUrl } = require('@iota/identity-wasm/node')
const { sum } = require('@iota/identity-stronghold-nodejs')


async function main() {
    console.log(sum(1,2))
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


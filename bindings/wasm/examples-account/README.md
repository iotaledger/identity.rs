![banner](./../../../.meta/identity_banner.png)


## IOTA Identity Account Examples

This folder provides code examples for you to learn how the IOTA Identity WASM bindings for the Account can be used in JavaScript/Typescript.

The examples are written in Typescript and can be run independently with Node.js. 

### Node.js
In order to run the examples in Node.js make sure to install the dependencies: 
```bash
npm install
```

Then run the example using:
```bash
npm run example:account -- <example-name>
```

For instance, to run the `create_did` example use:
```bash
npm run example:node -- create_did
```


| # | Name | Details |
| -------- | -------- | -------- |
| 1 |[create_did](src/create_did.ts)|Generates and publishes a DID Document, the fundamental building block for decentralized identity.|
|2| [manipulate_did](src/manipulate_did.ts)|  Add verification methods and service endpoints to a DID Document and update an already existing DID Document.|           
|3| [lazy](src/lazy.ts)| Manipulates a DID Document and publishes multiple changes to the tangle at once.|
## Browser
Although the examples should work in browser environment, we don't provide a browser project as for now.


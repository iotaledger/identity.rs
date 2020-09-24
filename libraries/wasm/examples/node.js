const identity = require('../wasm-node/iota_identity_wasm')
console.log(identity)

const greet = identity.Greet()

console.log("greet: ", greet)
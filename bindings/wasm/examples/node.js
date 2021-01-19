const Identity = require('../node/iota_identity_wasm')

// ======================================================
// https://github.com/seanmonstar/reqwest/issues/910
// ======================================================
const fetch = require('node-fetch')
global.Headers = fetch.Headers
global.Request = fetch.Request
global.Response = fetch.Response
global.fetch = fetch

const { Key, PubKey, Doc, DID, VerifiableCredential, VerifiablePresentation } = Identity

const CLIENT_CONFIG = {
  network: "main",
  node: "https://nodes.thetangle.org:443",
}

function generateUser(name) {
  const key = Key.generateEd25519("main")
  const did = new DID(key, "main")
  const doc = new Doc(PubKey.generateEd25519(did, key.public))

  return {
    doc,
    did,
    key,
    name,
  }
}

async function run() {
  // Generate a KeyPair, DID, and Document for Alice and Bob
  const alice = generateUser("Alice")
  const bob = generateUser("Bob")

  console.log("User (alice): ", alice)
  console.log("User (bob):   ", bob)

  // Sign all DID documents
  alice.doc.sign(alice.key)
  bob.doc.sign(bob.key)

  console.log("Signed Doc (alice): ", alice.doc.verify(), alice.doc.toJSON())
  console.log("Signed Doc (bob):   ", bob.doc.verify(), bob.doc.toJSON())

  // Publish all DID documents
  console.log("Publish Result (alice): https://explorer.iota.org/mainnet/transaction/" + await Identity.publish(alice.doc.toJSON(), CLIENT_CONFIG))
  console.log("Publish Result (bob):   https://explorer.iota.org/mainnet/transaction/" + await Identity.publish(bob.doc.toJSON(), CLIENT_CONFIG))

  // Prepare a credential subject indicating the degree earned by Alice
  let credentialSubject = {
    id: alice.doc.id,
    name: alice.name,
    degree: {
      name: "Bachelor of Science and Arts",
      type: "BachelorDegree",
    }
  }

  // Issue a signed `UniversityDegree` credential to Alice
  let vc = new VerifiableCredential(bob.doc, bob.key, credentialSubject, "UniversityDegreeCredential", "http://example.edu/credentials/3732");

  console.log("Verifiable Credential: ", vc)
  console.log("Credential Validation: ", await Identity.checkCredential(vc.toString(), CLIENT_CONFIG))

  let vp = new VerifiablePresentation(alice.doc, alice.key, vc)

  console.log("Verifiable Presentation: ", vp)
  console.log("Presentation Validation: ", await Identity.checkPresentation(vp.toString(), CLIENT_CONFIG))

  // Resolve DID documents
  console.log("Resolve Result (alice): ", await Identity.resolve(alice.doc.id, CLIENT_CONFIG))
  console.log("Resolve Result (bob):   ", await Identity.resolve(bob.doc.id, CLIENT_CONFIG))
}

run().then((output) => {
  console.log("Ok >", output)
}).catch((error) => {
  console.log("Err >", error)
})

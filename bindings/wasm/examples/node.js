const Identity = require('../node/identity_wasm')

// ======================================================
// https://github.com/seanmonstar/reqwest/issues/910
// ======================================================
const fetch = require('node-fetch')
global.Headers = fetch.Headers
global.Request = fetch.Request
global.Response = fetch.Response
global.fetch = fetch

const { KeyPair, DID, Document, Method, VerifiableCredential, VerifiablePresentation } = Identity

const CLIENT_CONFIG = {
  network: "main",
  node: "https://nodes.thetangle.org:443",
}

function generateUser(name) {
  const { doc, key } = new Document("ed25519", {
    tag: CLIENT_CONFIG.tag,
    network: CLIENT_CONFIG.network,
    shard: CLIENT_CONFIG.shard,
  })

  return {
    doc,
    key,
    name,
  }
}

async function run() {
  // Generate a KeyPair, DID, and Document for Alice and Bob
  const user1 = generateUser("Alice")
  const user2 = generateUser("Bob")

  console.log("User (user1): ", user1)
  console.log("User (user2): ", user2)

  // Sign all DID documents
  user1.doc.sign(user1.key)
  user2.doc.sign(user2.key)

  console.log("Document (user1): ", user1.doc.toJSON())
  console.log("Verified (user1): ", user1.doc.verify())

  console.log("Document (user2): ", user2.doc.toJSON())
  console.log("Verified (user2): ", user2.doc.verify())

  // Publish all DID documents
  console.log("Publish Result (user1): https://explorer.iota.org/mainnet/transaction/" + await Identity.publish(user1.doc.toJSON(), CLIENT_CONFIG))
  console.log("Publish Result (user2): https://explorer.iota.org/mainnet/transaction/" + await Identity.publish(user2.doc.toJSON(), CLIENT_CONFIG))

  // Prepare a credential subject indicating the degree earned by Alice
  let credentialSubject = {
    id: user1.doc.id,
    name: user1.name,
    degree: {
      name: "Bachelor of Science and Arts",
      type: "BachelorDegree",
    }
  }

  // Issue a signed `UniversityDegree` credential to Alice
  let vc = new VerifiableCredential(user2.doc, user2.key, credentialSubject, "UniversityDegreeCredential", "http://example.edu/credentials/3732");

  console.log("Verifiable Credential: ", vc)
  console.log("Credential Validation: ", await Identity.checkCredential(vc.toString(), CLIENT_CONFIG))

  let vp = new VerifiablePresentation(user1.doc, user1.key, vc)

  console.log("Verifiable Presentation: ", vp)
  console.log("Presentation Validation: ", await Identity.checkPresentation(vp.toString(), CLIENT_CONFIG))

  // Resolve DID documents
  console.log("Resolve Result (user1): ", await Identity.resolve(user1.doc.id, CLIENT_CONFIG))
  console.log("Resolve Result (user2): ", await Identity.resolve(user2.doc.id, CLIENT_CONFIG))
}

run().then((output) => {
  console.log("Ok >", output)
}).catch((error) => {
  console.log("Err >", error)
})

const Identity = require('../node/identity_wasm')

// ======================================================
// https://github.com/seanmonstar/reqwest/issues/910
// ======================================================
const fetch = require('node-fetch')
global.Headers = fetch.Headers
global.Request = fetch.Request
global.Response = fetch.Response
global.fetch = fetch

const {
  Digest,
  DID,
  Document,
  KeyCollection,
  KeyPair,
  KeyType,
  Method,
  VerifiableCredential,
  VerifiablePresentation,
} = Identity

const CLIENT_CONFIG = {
  network: "main",
  node: "https://nodes.thetangle.org:443",
}

function generateUser(name) {
  const { doc, key } = new Document(KeyType.Ed25519)

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

  // Add a Merkle Key Collection method for Bob, so compromised keys can be revoked.
  const keys = new KeyCollection(KeyType.Ed25519, 8)
  const method = Method.createMerkleKey(Digest.Sha256, user2.doc.id, keys, "key-collection")

  // Add to the DID Document as a general-purpose verification method
  user2.doc.insertMethod(method, "VerificationMethod")

  // Sign all DID documents
  user1.doc.sign(user1.key)
  user2.doc.sign(user2.key)

  console.log("Verified (user1): ", user1.doc.verify())
  console.log("Verified (user2): ", user2.doc.verify())

  user1.message = await Identity.publish(user1.doc.toJSON(), CLIENT_CONFIG)
  user2.message = await Identity.publish(user2.doc.toJSON(), CLIENT_CONFIG)

  // Publish all DID documents
  console.log(`Publish Result (user1): https://explorer.iota.org/mainnet/transaction/${user1.message}`)
  console.log(`Publish Result (user2): https://explorer.iota.org/mainnet/transaction/${user2.message}`)

  // Prepare a credential subject indicating the degree earned by Alice
  let credentialSubject = {
    id: user1.doc.id.toString(),
    name: user1.name,
    degree: {
      name: "Bachelor of Science and Arts",
      type: "BachelorDegree",
    }
  }

  // Issue an unsigned `UniversityDegree` credential for Alice
  const unsignedVc = VerifiableCredential.extend({
    id: "http://example.edu/credentials/3732",
    type: "UniversityDegreeCredential",
    issuer: user2.doc.id.toString(),
    credentialSubject,
  })

  // Sign the credential with Bob's Merkle Key Collection method
  const signedVc = user2.doc.signCredential(unsignedVc, {
    method: method.id.toString(),
    public: keys.public(0),
    secret: keys.secret(0),
    proof: keys.merkleProof(Digest.Sha256, 0),
  })

  // Ensure the credential signature is valid
  console.log("Verifiable Credential", signedVc)
  console.log("Verified (credential)", user2.doc.verify(signedVc))

  // Check the validation status of the Verifiable Credential
  console.log("Credential Validation", await Identity.checkCredential(signedVc.toString(), CLIENT_CONFIG))

  // Create a Verifiable Presentation from the Credential - signed by Alice's key
  const unsignedVp = new VerifiablePresentation(user1.doc, signedVc.toJSON())

  const signedVp = user1.doc.signPresentation(unsignedVp, {
    method: "#key",
    secret: user1.key.secret,
  })

  // Check the validation status of the Verifiable Presentation
  console.log("Presentation Validation", await Identity.checkPresentation(signedVp.toString(), CLIENT_CONFIG))

  // Bobs key was compromised - mark it as revoked and publish an update
  user2.doc.revokeMerkleKey(method.id.toString(), 0)

  user2.doc = Document.fromJSON({
    previous_message_id: user2.message,
    ...user2.doc.toJSON()
  })

  // The "authentication" key was not compromised so it's safe to publish an update
  user2.doc.sign(user2.key)

  user2.message = await Identity.publish(user2.doc.toJSON(), CLIENT_CONFIG)

  console.log("Publish Result (user2): https://explorer.iota.org/mainnet/transaction/" + user2.message)

  // Resolve DID documents
  console.log("Resolve Result (user1): ", await Identity.resolve(user1.doc.id.toString(), CLIENT_CONFIG))
  console.log("Resolve Result (user2): ", await Identity.resolve(user2.doc.id.toString(), CLIENT_CONFIG))

  // Check the validation status of the Verifiable Presentation
  //
  // This should return `false` since we revoked the key used to sign the credential
  console.log("Presentation Validation", await Identity.checkPresentation(signedVp.toString(), CLIENT_CONFIG))
}

run().then((output) => {
  console.log("Ok >", output)
}).catch((error) => {
  console.log("Err >", error)
})

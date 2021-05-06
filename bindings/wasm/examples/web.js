async function run(Identity) {
  console.log(Identity)

  const {
    Digest,
    DID,
    Document,
    KeyCollection,
    KeyPair,
    KeyType,
    VerificationMethod,
    VerifiableCredential,
    VerifiablePresentation,
  } = Identity

  function generateUser(name) {
    const { doc, key } = new Document(KeyType.Ed25519)

    return {
      doc,
      key,
      name,
    }
  }

  // Generate a KeyPair, DID, and Document for Alice and Bob
  const user1 = generateUser("Alice")
  const user2 = generateUser("Bob")

  console.log("User (user1): ", user1)
  console.log("User (user2): ", user2)

  // Add a Merkle Key Collection method for Bob, so compromised keys can be revoked.
  const keys = new KeyCollection(KeyType.Ed25519, 8)
  const method = VerificationMethod.createMerkleKey(Digest.Sha256, user2.doc.id, keys, "key-collection")

  // Add to the DID Document as a general-purpose verification method
  user2.doc.insertMethod(method, "VerificationMethod")

  // Sign all DID documents
  user1.doc.sign(user1.key)
  user2.doc.sign(user2.key)

  console.log("Verified (user1): ", user1.doc.verify())
  console.log("Verified (user2): ", user2.doc.verify())

  user1MessageId = await Identity.publish(user1.doc.toJSON())
  user2MessageId = await Identity.publish(user2.doc.toJSON())

  // Publish all DID documents
  console.log(`Publish Result (user1): ${user1.doc.id.tangleExplorer}/transaction/${user1MessageId}`)
  console.log(`Publish Result (user2): ${user2.doc.id.tangleExplorer}/transaction/${user2MessageId}`)

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
  console.log("Credential Validation", await Identity.checkCredential(signedVc.toString()))

  // Create a Verifiable Presentation from the Credential - signed by Alice's key
  const unsignedVp = new VerifiablePresentation(user1.doc, signedVc.toJSON())

  const signedVp = user1.doc.signPresentation(unsignedVp, {
    method: "#key",
    secret: user1.key.secret,
  })

  // Check the validation status of the Verifiable Presentation
  console.log("Presentation Validation", await Identity.checkPresentation(signedVp.toString()))

  // Bobs key was compromised - mark it as revoked and publish an update
  user2.doc.revokeMerkleKey(method.id.toString(), 0)

  // Set the Tangle message id of the previously published DID Document
  user2.doc.previousMessageId = user2MessageId

  // The "authentication" key was not compromised so it's safe to publish an update
  user2.doc.sign(user2.key)

  user2MessageId = await Identity.publish(user2.doc.toJSON())

  console.log(`Publish Result (user2): ${user2.doc.id.tangleExplorer}/transaction/${user2MessageId}`)

  // Resolve DID documents
  console.log("Resolve Result (user1): ", await Identity.resolve(user1.doc.id.toString()))
  console.log("Resolve Result (user2): ", await Identity.resolve(user2.doc.id.toString()))

  // Check the validation status of the Verifiable Presentation
  //
  // This should return `false` since we revoked the key used to sign the credential
  console.log("Presentation Validation", await Identity.checkPresentation(signedVp.toString()))
}

import("../pkg/index.js").then(async identity => {
  try {
    await run(identity)
  } catch (e) {
    console.error(e)
  }
})

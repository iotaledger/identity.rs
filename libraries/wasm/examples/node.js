const identity = require('../node/iota_identity_wasm')

console.log("identity", identity)

// ======================================================
// https://github.com/seanmonstar/reqwest/issues/910
// ======================================================
const fetch = require('node-fetch')
global.Headers = fetch.Headers
global.Request = fetch.Request
global.Response = fetch.Response
global.fetch = fetch

const { initialize, resolve, publish, checkCredential, Key, Doc, DID, VerifiableCredential } = identity

initialize()

async function playground() {
  console.log("key", new Key())

  console.log("did", new DID((new Key()).public))

  console.log("did", new DID({ key: (new Key()).public, network: "com" }))

  const { key, doc } = Doc.generateCom()

  console.log("key (generated)", key)
  console.log("doc (generated)", doc)

  console.log("doc (unsigned)", doc.document)

  doc.sign(key)

  console.log("doc (signed)", doc.document)

  console.log("doc valid?", doc.verify())

  const json = JSON.stringify(doc.document)

  console.log("From JSON >", Doc.fromJSON(json))

  console.log("published", await publish(doc.document, { node: "https://nodes.comnet.thetangle.org:443", network: "com" }))
  console.log("resolved", await resolve(doc.did, { node: "https://nodes.comnet.thetangle.org:443", network: "com" }))
}

function alice_bob() {
  // Generate Keypairs
  const alice_keypair = new Key()
  console.log("alice_keypair: ", alice_keypair)

  const bob_keypair = new Key()
  console.log("bob_keypair: ", bob_keypair)

  // Create the DIDs
  let alice_did = new DID(alice_keypair.public)
  console.log("alice_did: ", alice_did.toString(), alice_did.address)

  let bob_did = new DID(bob_keypair.public)
  console.log("bob_did: ", bob_did.toString(), bob_did.address)

  // Create the DID Documents
  let alice_document = new Doc({ did: alice_did.did, key: alice_keypair.public })
  console.log("alice_document: ", alice_document.document)

  let bob_document = new Doc({ did: bob_did.did, key: bob_keypair.public })
  console.log("bob_document: ", bob_document.document)

  let update = { ...bob_document.document }

  update["foo"] = 123
  update["bar"] = 456
  update = Doc.fromJSON(JSON.stringify(update))

  console.log("Update: ", update)

  let diff = bob_document.diff(update, bob_keypair)

  console.log("diff: ", JSON.stringify(diff, null, 2))

  let json = JSON.stringify(diff)

  console.log("Diff has valid signature: ", bob_document.verify_diff(json))
}

async function testVC() {
  var { key, doc } = Doc.generateCom()
  doc.sign(key)
  let issuerKey = key
  let issuerDoc = doc
  console.log("vc issuer doc published", await publish(doc.document, { node: "https://nodes.comnet.thetangle.org:443", network: "com" }))
  var { key, doc } = Doc.generateCom()
  doc.sign(key)
  let subjectDoc = doc
  console.log("vc subject doc published", await publish(doc.document, { node: "https://nodes.comnet.thetangle.org:443", network: "com" }))
  let vc = new VerifiableCredential(issuerDoc, issuerKey, subjectDoc, "UniversityDegreeCredential", "http://example.edu/credentials/3732", JSON.stringify({ name: "Subject", degree: { name: "Bachelor of Science and Arts", type: "BachelorDegree" } }));
  console.log("vc", vc);
  console.log("vc valid: ", await checkCredential(vc.to_string(), { node: "https://nodes.comnet.thetangle.org:443", network: "com" }))
  let vc_fromJson = VerifiableCredential.from_json(issuerDoc, issuerKey, vc.to_string())
  console.log("vc_fromJson: ", vc_fromJson.vc);
  console.log("vc_fromJson valid: ", await checkCredential(vc_fromJson.to_string(), { node: "https://nodes.comnet.thetangle.org:443", network: "com" }))
}

async function run() {
  await playground()
  await alice_bob()
  await testVC()
}

run().then((output) => {
  console.log("Ok >", output)
}).catch((error) => {
  console.log("Err >", error)
})

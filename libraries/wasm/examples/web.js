import("../pkg/index.js").then(async identity => {
    try {
        console.log(identity)
        const { initialize, resolve, publish, checkCredential, Key, Doc, DID, PubKey, VerifiableCredential } = identity

        initialize();

        await playground()
        await alice_bob()
        await testVC()
        restore_keypair()

        function restore_keypair() {
            let secret = "Bac8bArn1tX9rrqawk9cWM6aK5KHNbhrvnV1VzBXMDrF"
            let public = "7zwMYWKnoUJKArSxigqk5kHoCVw6vhsrWoKDRTcoXuhj"
            let keypair = Key.fromBase58(secret, public)
            console.log(keypair);
        }

        async function testVC() {
            var { key, doc } = Doc.generateEd25519("com")
            doc.sign(key)
            let issuerKey = key
            let issuerDoc = doc
            console.log("vc issuer doc published", await publish(doc.toJSON(), { node: "https://nodes.comnet.thetangle.org:443", network: "com" }))
            var { key, doc } = Doc.generateEd25519("com")
            doc.sign(key)
            let subjectDoc = doc
            console.log("vc subject doc published", await publish(doc.toJSON(), { node: "https://nodes.comnet.thetangle.org:443", network: "com" }))
            let vc = new VerifiableCredential(
                issuerDoc,
                issuerKey,
                "http://example.edu/credentials/3732",
                "UniversityDegreeCredential",
                {
                    id: subjectDoc.id,
                    name: "Subject",
                    degree: {
                        name: "Bachelor of Science and Arts",
                        type: "BachelorDegree"
                    }
                }
            );
            console.log("vc", vc.toJSON());
            console.log("vc valid: ", await checkCredential(vc.toString(), { node: "https://nodes.comnet.thetangle.org:443", network: "com" }))
            let vc_fromJson = VerifiableCredential.fromJSON(vc.toString())
            console.log("vc_fromJson: ", vc_fromJson);
            console.log("vc_fromJson valid: ", await checkCredential(vc_fromJson.toString(), { node: "https://nodes.comnet.thetangle.org:443", network: "com" }))
        }

        async function playground() {
            console.log("key", Key.generateEd25519())

            console.log("did", new DID(Key.generateEd25519()))

            console.log("did", new DID(Key.generateEd25519(), "com"))

            const { key, doc } = Doc.generateEd25519("com")

            console.log("key (generated)", key)
            console.log("doc (generated)", doc)

            console.log("doc (unsigned)", doc.toJSON())

            doc.sign(key)

            console.log("doc (signed)", doc.toJSON())

            console.log("doc valid?", doc.verify())

            const json = JSON.stringify(doc.toJSON())

            console.log("From JSON >", Doc.fromJSON(json))

            console.log("published", await publish(doc.toJSON(), { node: "https://nodes.comnet.thetangle.org:443", network: "com" }))
            console.log("resolved", await resolve(doc.id, { node: "https://nodes.comnet.thetangle.org:443", network: "com" }))
        }

        function alice_bob() {
            // Generate Keypairs
            const alice_keypair = Key.generateEd25519()
            console.log("alice_keypair: ", alice_keypair)

            const bob_keypair = Key.generateEd25519()
            console.log("bob_keypair: ", bob_keypair)

            // Create the DIDs
            let alice_did = new DID(alice_keypair)
            console.log("alice_did: ", alice_did.toString(), alice_did.address)

            let bob_did = new DID(bob_keypair)
            console.log("bob_did: ", bob_did.toString(), bob_did.address)

            // Create the DID Documents
            let alice_pubkey = PubKey.generateEd25519(alice_did, alice_keypair.public)
            let alice_document = new Doc(alice_pubkey)
            console.log("alice_document: ", alice_document.toJSON())

            let bob_pubkey = PubKey.generateEd25519(bob_did, bob_keypair.public)
            let bob_document = new Doc(bob_pubkey)
            console.log("bob_document: ", bob_document.toJSON())

            let update = { ...bob_document.toJSON() }

            update["foo"] = 123
            update["bar"] = 456
            update = Doc.fromJSON(JSON.stringify(update))

            console.log("Update: ", update)

            let diff = bob_document.diff(update, bob_keypair)

            console.log("diff: ", JSON.stringify(diff, null, 2))

            let json = JSON.stringify(diff)

            console.log("Diff has valid signature: ", bob_document.verifyDiff(json))

            bob_document.updateService(DID.parse(bob_document.toJSON().id + "#messages"), "https://example.com/messages/8377464", "MessagingService")
            console.log("Doc with service ", bob_document.toJSON());
            bob_document.clearServices()
            console.log("Doc with services cleared ", bob_document.toJSON());
            let keypair = Key.generateEd25519();
            let publicKey = PubKey.generateEd25519(bob_document.did(), keypair.public, "#keys-1")
            bob_document.updatePublicKey(publicKey)
            console.log("Doc with public key ", bob_document.toJSON());
            bob_document.updatePublicKey(publicKey)
            bob_document.updateAuth(publicKey)
            bob_document.updateAssert(publicKey)
            bob_document.updateVerification(publicKey)
            bob_document.updateDelegation(publicKey)
            bob_document.updateInvocation(publicKey)
            bob_document.updateAgreement(publicKey)
            bob_document.updateTime()
            console.log("Doc with A LOT", bob_document.toJSON());
            let bob_auth_key = bob_document.resolveKey(0, "Authentication")
            console.log("bob_auth_key: ", bob_auth_key);
            setTimeout(() => {
                bob_document.updateTime()
                console.log("bob_document with updated time: ", bob_document.toJSON())
            }, 1000)
        }
    } catch (e) {
        console.error(e)
    }
});



/*

identity.ts: Implementation Guide

https://github.com/iotaledger/identity.ts/blob/master/ImplementationGuide.md

## Generate seed and keypair

let seed = GenerateSeed();
let keypair = await GenerateECDSAKeypair(); //Or GenerateRSAKeypair();

## Creating the DID Document

//The full creation of the DID
let uuid = mamRoot;
let did = new DID(uuid);
let document = DIDDocument.createDIDDocument(did);

//The simplified helper function
let document = CreateRandomDID(seed);

//Attach the keypair with an identifier keyId, which must be unique in the document (Short name recommended to reflect the purpose)
document.AddKeypair(keypair, keyId);
*/



/*

 // Create keypair
let keypair = Ed25519::generate(&Ed25519, Default::default())?;
let bs58_auth_key = bs58::encode(keypair.public()).into_string();

// Create, sign and publish DID document to the Tangle
let mut did_document = create_document(bs58_auth_key)?;

*/

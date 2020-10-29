import("../pkg/index.js").then((identity) => {

    console.log(identity)

    const { initialize, resolve, publish, Key, Doc, DID } = identity

    initialize();

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
    let alice_document = new Doc({did: alice_did.did, key: alice_keypair.public})
    console.log("alice_document: ", alice_document.document)

    let bob_document = new Doc({did: bob_did.did, key: bob_keypair.public})
    console.log("bob_document: ", bob_document.document)

    let update = {...bob_document.document}

    update["foo"] = 123
    update["bar"] = 456
    update = Doc.fromJSON(JSON.stringify(update))

    console.log("Update: ", update)

    let diff = bob_document.diff(update, bob_keypair)

    console.log("diff: ", JSON.stringify(diff, null, 2))

    let json = JSON.stringify(diff)

    console.log("Diff has valid signature: ", bob_document.verify_diff(json))

    // // identity.ResolveDID("did:iota:8gPe8EwndxtvQPfYT5KsXBXtXUGZMLCPP4Z98by33TMs", "https://nodes.thetangle.org:443").then(doc => {
    // //     console.log("resolved document: ", doc);
    // // });
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

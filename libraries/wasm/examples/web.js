import("../pkg/index.js").then((identity) => {

    console.log(identity)

    // identity_core

    // Generate Keypairs
    const alice_keypair = new identity.KeyPair();
    console.log("alice_keypair: privateKey:", alice_keypair.privateKey)
    console.log("alice_keypair: GetPublicKey:", alice_keypair.publicKey)
    const bob_keypair = new identity.KeyPair();
    console.log("bob_keypair: GetPrivateKey: ", bob_keypair.privateKey)
    console.log("bob_keypair: GetPublicKey: ", bob_keypair.publicKey)


    // Creating the DID
    let alice_did = new identity.DID("iota", alice_keypair.publicKey)
    console.log("alice_did: ", alice_did.did);
    //uuid should be replaced by the public key?
    let bob_did = new identity.DID("iota", bob_keypair.publicKey)
    console.log("bob_did: ", bob_did.did);

    // Creating the DID Document
    let alice_document = new identity.DIDDocument("github", alice_keypair.publicKey)
    console.log("alice_document: ", alice_document.document);


    let bob_document = new identity.DIDDocument("iota", bob_keypair.publicKey)
    console.log("bob_document: ", bob_document.document);
    // let what = bob_document.set_sign_unchecked(bob_keypair.privateKey);
    // console.log("bob_document: ", what);

    // identity_iota 

    let iota_did = new identity.IotaDID(alice_keypair.publicKey);
    console.log("iota_did: ", iota_did.did);
    console.log("iota_did address: ", iota_did.create_address);

    let network_iota_did = identity.IotaDID.CreateIotaDIDWithNetwork(alice_keypair.publicKey, "com");
    console.log("network_iota_did: ", network_iota_did.did);

    let iota_document = new identity.IotaDocument(network_iota_did.did, alice_keypair.publicKey);
    console.log("iota document: ", iota_document.document);
    console.log("iota document did: ", iota_document.did);
    console.log("iota document authentication_key: ", iota_document.authentication_key);

    let iota_document2 = identity.IotaDocument.TryFromDocument(iota_document.document);
    console.log(iota_document2.document);
    console.log(iota_document2.create_diff_address);
    // identity.Iota.ResolveDID("did:iota:com:HbuRS48djS5PbLQciy6iE9BTdaDTBM3GxcbGdyuv3TWo").then(doc => {
    //     console.log("resolved document: ", doc);
    // });
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
import("../pkg/index.js").then((identity) => {


    console.log(identity)
    //const did = identity.Core.createGenerateKeypair();


    const greet = identity.Greet()
    const id = "alice"
    const did = identity.Core.create_did(id);
    
    console.log("greet: ", greet)
    console.log("did: ", did)
    const document = identity.Core.createDIDDocument(did);
    console.log("document: ", document)
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
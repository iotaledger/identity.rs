import("../pkg/index.js").then((identtiy) => {

    console.log(identtiy)

    const greet = identtiy.Greet()
    const id = "alice"
    const did = identtiy.Core.create_did(id);
    
    console.log("greet: ", greet)
    console.log("did: ", did)
    const document = identtiy.Core.createDIDDocument(did);
    console.log("document: ", document)
});

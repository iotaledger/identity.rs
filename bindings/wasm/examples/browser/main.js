import * as id from "../../web/identity_wasm.js";

id.init("../../web/identity_wasm_bg.wasm")
  .then(() => {
    const { doc, key } = new id.Document(id.KeyType.Ed25519);

    function generateUser(name) {
      const { doc, key } = new id.Document(id.KeyType.Ed25519);

      return {
        doc,
        key,
        name,
      };
    }

    // Generate a KeyPair, DID, and Document for Alice and Bob
    const user1 = generateUser("Alice");
    const user2 = generateUser("Bob");

    console.log("User (user1): ", user1);
    console.log("User (user2): ", user2);

    const config = id.Config.fromNetwork(user1.doc.id.network);
    const client = id.Client.fromConfig(config);

    // Sign all DID documents
    user1.doc.sign(user1.key);
    user2.doc.sign(user2.key);

    console.log(user1.doc.toJSON());

    client.publishDocument(user1.doc.toJSON()).then((user1MessageId) => {
      console.log(
        `Publish Result (user1): ${user1.doc.id.tangleExplorer}/transaction/${user1MessageId}`
      );
    });
  })
  .catch((err) => {
    console.log(err);
  });

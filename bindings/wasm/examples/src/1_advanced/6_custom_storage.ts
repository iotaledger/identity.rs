import {
    CoreDocument,
    CoreDocumentRc,
    CoreVerificationMethod,
    IdentitySuite,
    KeyAlias,
    KeyStorage,
    KeyType,
    MethodContent,
    MethodScope,
    MethodType1,
} from "../../../node";

// Use this external package to avoid implementing the entire did:key method in this example.
import { MemStore } from "./memStore";

/** Demonstrates how to ...
 */
export async function customStorage() {
    const memStore = new MemStore();

    const fragment = "#key-2";
    var document = new CoreDocument({ id: "did:iota:0x0000" });

    const documentRc = new CoreDocumentRc(document);
    await documentRc.createMethod({
        key_storage: memStore,
        fragment,
        content: MethodContent.Generate(MethodType1.ed25519VerificationKey2018()),
    });
    document = documentRc.intoDocument();

    // Hardcoded keyAlias while we don't have a mechanism to store the mappings from method fragments to key aliases.
    const keyAlias = new KeyAlias("very_random_key");

    const signatureHandler = async function(data: Uint8Array, keyStorage: KeyStorage): Promise<Uint8Array> {
        return await keyStorage.sign(keyAlias, "Ed25519", data);
    };

    let handlerMap: Map<string, (data: Uint8Array, keyStorage: KeyStorage) => Promise<Uint8Array>> = new Map();
    handlerMap.set("Ed25519VerificationKey2018", signatureHandler);

    const suite = new IdentitySuite(memStore, handlerMap);
    const signature = await document.sign(fragment, Uint8Array.from([0, 1, 2, 3, 4, 5]), suite);

    if (signature.length === 64) {
        console.log("successfully created a signature");
    } else {
        console.error("failed to create a signature");
    }
}

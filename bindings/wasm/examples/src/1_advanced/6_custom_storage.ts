import { CoreDocument, CoreVerificationMethod, IdentitySuite, KeyStorage, KeyType, MethodScope } from "../../../node";

// Use this external package to avoid implementing the entire did:key method in this example.
import { MemStore } from "./custom_storage";

/** Demonstrates how to set up a resolver using custom handlers.
 */
export async function customResolution() {
    const memStore = new MemStore();

    const fragment = "#key-2";
    const document = new CoreDocument({ id: "did:iota:0x0000" });
    // Insert a new Ed25519 verification method in the DID document.
    let keyAlias = await memStore.generate("Ed25519");
    let keyPublic = await memStore.public(keyAlias);
    let method = new CoreVerificationMethod(document.id(), KeyType.Ed25519, keyPublic, fragment);
    document.insertMethod(method, MethodScope.VerificationMethod());

    // Set up a handler for resolving Ed25519 did:key
    const signatureHandler = async function(data: Uint8Array, keyStorage: KeyStorage): Promise<Uint8Array> {
        return await keyStorage.sign(keyAlias, "Ed25519", data);
    };

    // Construct a Resolver capable of resolving the did:key and iota methods.
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

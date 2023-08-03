import {
    CoreDocument,
    IotaDocument,
    IotaIdentityClient,
    JwkMemStore,
    KeyIdMemStore,
    Resolver,
    Storage,
} from "@iota/identity-wasm/node";
import { Client, MnemonicSecretManager, Utils } from "@iota/sdk-wasm/node";
import { API_ENDPOINT, createDid } from "../util";

// Use this external package to avoid implementing the entire did:key method in this example.
import * as ed25519 from "@transmute/did-key-ed25519";

/** Demonstrates how to set up a resolver using custom handlers.
 */
export async function customResolution() {
    // Set up a handler for resolving Ed25519 did:key
    const keyHandler = async function(didKey: string): Promise<CoreDocument> {
        let document = await ed25519.resolve(
            didKey,
            { accept: "application/did+ld+json" },
        );
        return CoreDocument.fromJSON(document.didDocument);
    };

    // Create a new Client to interact with the IOTA ledger.
    const client = new Client({
        primaryNode: API_ENDPOINT,
        localPow: true,
    });
    const didClient = new IotaIdentityClient(client);

    // Construct a Resolver capable of resolving the did:key and iota methods.
    let handlerMap: Map<string, (did: string) => Promise<IotaDocument | CoreDocument>> = new Map();
    handlerMap.set("key", keyHandler);

    const resolver = new Resolver(
        {
            client: didClient,
            handlers: handlerMap,
        },
    );

    // A valid Ed25519 did:key value taken from https://w3c-ccg.github.io/did-method-key/#example-1-a-simple-ed25519-did-key-value.
    const didKey = "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK";

    // Generate a random mnemonic for our wallet.
    const secretManager: MnemonicSecretManager = {
        mnemonic: Utils.generateMnemonic(),
    };

    // Creates a new wallet and identity for us to resolve (see "0_create_did" example).
    const storage: Storage = new Storage(new JwkMemStore(), new KeyIdMemStore());
    let { document } = await createDid(
        client,
        secretManager,
        storage,
    );
    const did = document.id();

    // Resolve didKey into a DID document.
    const didKeyDoc = await resolver.resolve(didKey);

    // Resolve the DID we created on the IOTA ledger.
    const didIotaDoc = await resolver.resolve(did.toString());

    // Check that the types of the resolved documents match our expectations:

    if (didKeyDoc instanceof CoreDocument) {
        console.log("Resolved DID Key document:", JSON.stringify(didKeyDoc, null, 2));
    } else {
        throw new Error(
            "the resolved document type should match the output type of keyHandler",
        );
    }

    if (didIotaDoc instanceof IotaDocument) {
        console.log("Resolved IOTA DID document:", JSON.stringify(didIotaDoc, null, 2));
    } else {
        throw new Error(
            "the resolved document type should match IotaDocument",
        );
    }
}

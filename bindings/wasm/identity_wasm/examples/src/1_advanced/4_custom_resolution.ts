import { CoreDocument, IotaDID, IotaDocument, Resolver } from "@iota/identity-wasm/node";
import { IotaClient } from "@iota/iota-sdk/client";
import { createDocumentForNetwork, getFundedClient, getMemstorage, NETWORK_URL } from "../util";

// Use this external package to avoid implementing the entire did:key method in this example.
import * as ed25519 from "@transmute/did-key-ed25519";

type KeyDocument = { customProperty: String } & CoreDocument;

function isKeyDocument(doc: object): doc is KeyDocument {
    return "customProperty" in doc;
}

/** Demonstrates how to set up a resolver using custom handlers.
 */
export async function customResolution() {
    // Set up a handler for resolving Ed25519 did:key
    const keyHandler = async function(didKey: string): Promise<KeyDocument> {
        let document = await ed25519.resolve(
            didKey,
            { accept: "application/did+ld+json" },
        );

        // for demo purposes we'll just inject the custom property into a core document
        // to create a new KeyDocument instance
        let coreDocument = CoreDocument.fromJSON(document.didDocument);
        (coreDocument as unknown as KeyDocument).customProperty = "foobar";
        return coreDocument as unknown as KeyDocument;
    };

    // create new clients and create new account
    const iotaClient = new IotaClient({ url: NETWORK_URL });
    const network = await iotaClient.getChainIdentifier();
    const storage = getMemstorage();
    const identityClient = await getFundedClient(storage);
    const [unpublished] = await createDocumentForNetwork(storage, network);

    // create new identity for this account and publish document for it, DID of it will be resolved later on
    const { output: identity } = await identityClient
        .createIdentity(unpublished)
        .finish()
        .execute(identityClient);
    const did = IotaDID.fromAliasId(identity.id(), identityClient.network());

    // Construct a Resolver capable of resolving the did:key and iota methods.
    let handlerMap: Map<string, (did: string) => Promise<IotaDocument | KeyDocument>> = new Map();
    handlerMap.set("key", keyHandler);

    const resolver = new Resolver<IotaDocument | KeyDocument>(
        {
            client: identityClient,
            handlers: handlerMap,
        },
    );

    // A valid Ed25519 did:key value taken from https://w3c-ccg.github.io/did-method-key/#example-1-a-simple-ed25519-did-key-value.
    const didKey = "did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK";

    // Resolve didKey into a DID document.
    const didKeyDoc = await resolver.resolve(didKey);

    // Resolve the DID we created on the IOTA network.
    const didIotaDoc = await resolver.resolve(did.toString());

    // Check that the types of the resolved documents match our expectations:

    if (isKeyDocument(didKeyDoc)) {
        console.log("Resolved DID Key document:", JSON.stringify(didKeyDoc, null, 2));
        console.log(`Resolved DID Key document has a custom property with the value '${didKeyDoc.customProperty}'`);
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

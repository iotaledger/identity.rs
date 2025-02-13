import {
    CoreDocument,
    IotaDID,
    IotaDocument,
    Resolver,
} from "@iota/identity-wasm/node";
import { IotaClient } from "@iota/iota-sdk/client";
import {
    createDocumentForNetwork,
    getClientAndCreateAccount,
    getMemstorage,
    NETWORK_URL,
} from '../utils_alpha';

// Use this external package to avoid implementing the entire did:key method in this example.
import * as ed25519 from "@transmute/did-key-ed25519";

/** Demonstrates how to set up a resolver using custom handlers.
 */
export async function customResolution() {
    // Set up a handler for resolving Ed25519 did:key
    const keyHandler = async function (didKey: string): Promise<CoreDocument> {
        let document = await ed25519.resolve(
            didKey,
            { accept: "application/did+ld+json" },
        );
        return CoreDocument.fromJSON(document.didDocument);
    };

    // create new clients and create new account
    const iotaClient = new IotaClient({ url: NETWORK_URL });
    const network = await iotaClient.getChainIdentifier();
    const storage = getMemstorage();
    const identityClient = await getClientAndCreateAccount(storage);
    const [unpublished] = await createDocumentForNetwork(storage, network);

    // create new identity for this account and publish document for it, DID of it will be resolved later on
    const { output: identity } = await identityClient
        .createIdentity(unpublished)
        .finish()
        .execute(identityClient);
    const did = IotaDID.fromAliasId(identity.id(), identityClient.network());

    // Construct a Resolver capable of resolving the did:key and iota methods.
    let handlerMap: Map<string, (did: string) => Promise<IotaDocument | CoreDocument>> = new Map();
    handlerMap.set("key", keyHandler);

    const resolver = new Resolver(
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

import * as id from "../../web/identity_wasm.js";

/*
    A short example to show how to resolve a DID. This returns the latest DID Document.

    @param {string} did
*/
export async function resolveIdentity(did) {
    const mainNet = id.Network.mainnet();

    // Create a default client configuration from mainNet.
    const config = id.Config.fromNetwork(mainNet);

    // Create a client instance to publish messages to the Tangle.
    const client = id.Client.fromConfig(config);

    return await client.resolve(did);
}

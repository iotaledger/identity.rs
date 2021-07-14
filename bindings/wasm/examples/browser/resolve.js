import * as identity from "../../web/identity_wasm.js";
import { logObjectToScreen } from "./utils.js";

/*
    A short example to show how to resolve a DID. This returns the latest DID Document.

    @param {{network: string, node: string}} clientConfig
    @param {boolean} log log the events to the output window
*/
export async function resolveIdentity(clientConfig, log = true) {
    //get the DID string from the input field
    const inputDid = document.querySelector("#resolve-did-input").value;

    // Create a default client configuration from network.
    const config = identity.Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = identity.Client.fromConfig(config);

    const res = await client.resolve(inputDid);
    if (log) logObjectToScreen(res);
    return res;
}

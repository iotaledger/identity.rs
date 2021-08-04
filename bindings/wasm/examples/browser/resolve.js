import * as identity from "../../web/identity_wasm.js";
import {logObjectToScreen} from "./utils.js";
import {createIdentity} from "./create_did.js";

/**
 A short example to show how to resolve a DID. This returns the latest DID Document.

 @param {{defaultNodeURL: string, explorerURL: string, network: Network}} clientConfig
 @param {boolean} inBrowser whether or not the function is running in the example browser
 @param {boolean} log log the events to the output window
 **/
export async function resolveIdentity(clientConfig, inBrowser = true, log = true) {
    let inputDid;
    if (inBrowser) {
        // Get the DID string from the input field
        inputDid = document.querySelector("#resolve-did-input").value;
    } else {
        // Generate a new DID to resolve
        const alice = await createIdentity(clientConfig, false);
        inputDid = alice.doc.id.toString();
    }

    // Create a default client configuration from network.
    const config = identity.Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = identity.Client.fromConfig(config);

    const res = await client.resolve(inputDid);
    if (log) logObjectToScreen(res);
    return res;
}

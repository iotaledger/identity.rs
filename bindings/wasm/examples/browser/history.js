import * as identity from "../../web/identity_wasm.js";
import { logObjectToScreen, logToScreen } from "./utils.js";
import { manipulateIdentity } from "./mainpulate_did.js";

/**

    An example for resolving the integration-message-history of a DID.
    The history is usually only useful for debugging puropses.

    @param {{network: string, node: string}} clientConfig
    @param {boolean} log log the to the output window
*/
export async function resolveHistory(clientConfig, log = true) {

    if (log) logToScreen("resolve history...");

    // Create a default client configuration from network.
    const config = identity.Config.fromNetwork(clientConfig.network);

    // Create a client instance to publish messages to the Tangle.
    const client = identity.Client.fromConfig(config);

    // Creates a new identity, that also is updated (See "manipulate_did" example).
    const did = await manipulateIdentity(clientConfig, false);

    const res = await client.resolveHistory(did.doc.id.toString());

    if (log) logObjectToScreen(res);
    return res;
}

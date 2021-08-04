import * as identity from "../../web/identity_wasm.js";

/**
 * Loads the identity wasm library.
 *
 * @returns {Promise<void>}
 */
export async function initIdentity(path = "../../web/identity_wasm_bg.wasm", log = true) {
    if(log) logToScreen("Initialization started...");
    try {
        await identity.init(path);
        if(log) logToScreen("Initialization success!");
    } catch (err) {
        if(log) logToScreen(err);
    }
}

/**
 * Returns the default client configuration to connect to the IOTA mainnet.
 *
 * N.B. initIdentity() must be called prior to this function.
 *
 * @returns {{defaultNodeURL: string, explorerURL: string, network: Network}}
 */
export function defaultClientConfig() {
    const mainNet = identity.Network.mainnet();
    return {
        network: mainNet,
        defaultNodeURL: mainNet.defaultNodeURL,
        explorerURL: mainNet.explorerURL,
    }
}

/**
 * Returns a URL to view a message published to the Tangle, depending on the network:
 * https://explorer.iota.org/<mainnet|testnet>/transaction/<messageId>
 *
 * @param doc
 * @param messageId
 * @returns {string}
 */
export function getExplorerUrl(doc, messageId) {
    return doc.id.network.messageURL(messageId);
}

/**
 * logs a string to the output window
 *
 * @param {string} message
 */
export function logToScreen(message) {
    document.querySelector("#content").innerHTML =
        document.querySelector("#content").innerHTML +
        message +
        "<br/>-------------------------------------<br/>";
}

/**
 *
 * @param {string} url
 */
export function logExplorerUrlToScreen(url) {
    logToScreen(`Explorer URL: <a target="_blank" href="${url}"> ${url} </a>`);
}

/**
 *
 * @param {object} obj
 */
export function logObjectToScreen(obj) {
    logToScreen("<pre>" + JSON.stringify(obj, null, 4) + "</pre>");
}

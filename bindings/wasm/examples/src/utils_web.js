import * as identity from "@iota/identity-wasm";

export const LINK_REGEX = /(\b(https?|ftp):\/\/[-A-Z0-9+&@#\/%?=~_|!:,.;]*[-A-Z0-9+&@#\/%=~_|])/gim;

/**
 * Loads the identity wasm library.
 *
 * @returns {Promise<void>}
 */
export async function initIdentity(path = "../../web/identity_wasm_bg.wasm") {
    console.log("Initialization started...");
    try {
        await identity.init(path);
        console.log("Initialization success!");
    } catch (err) {
        console.error(err);
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

export function linkify(inputText) {

    return inputText.replace(LINK_REGEX, '<a href="$1" target="_blank">$1</a>');

}

/**
 *
 * @param {object} obj
 */
export function logObjectToScreen(obj) {
    logToScreen("<pre>" + JSON.stringify(obj, null, 4) + "</pre>");
}

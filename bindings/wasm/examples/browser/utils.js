import * as identity from "../../web/identity_wasm.js";

/**
 * Loads the identity wasm library.
 *
 * @returns {Promise<void>}
 */
export async function init_identity() {
    logToScreen("Initialization started...");
    try {
        await identity.init("../../web/identity_wasm_bg.wasm");
        logToScreen("Initialization success!");
    } catch (err) {
        logToScreen(err);
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

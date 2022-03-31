import * as identity from "@iota/identity-wasm";

export const LINK_REGEX = /(\b(https?|ftp):\/\/[-A-Z0-9+&@#\/%?=~_|!:,.;]*[-A-Z0-9+&@#\/%=~_|])/gim;

/**
 * Loads the identity wasm library.
 *
 * @returns {Promise<void>}
 */
export async function initIdentity(path = "./identity_wasm_bg.wasm") {
    console.log("Initialization started...");
    await identity.init(path);
    console.log("Initialization success!");
}

/**
 * Returns the default client configuration to connect to the IOTA mainnet.
 *
 * N.B. initIdentity() must be called prior to this function.
 *
 * @returns {{network: Network, explorer: ExplorerUrl}}
 */
export function defaultClientConfig() {
    const mainnet = identity.Network.mainnet();
    const explorer = identity.ExplorerUrl.mainnet();
    return {
        network: mainnet,
        explorer: explorer,
    }
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
 * parses a string for urls and wraps them with link tags
 *
 * @param {string} inputText
 * @returns {string}
 */
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

export function setupDOMLog() {
    var orig = console.log;

    console.log = function () {
        Array.from(arguments).forEach(argument => {
            if (typeof argument === 'object') {
                return logObjectToScreen(argument);
            } else if (typeof argument === 'string' && argument.match(LINK_REGEX)) {
                return logToScreen(linkify(argument));
            }
            logToScreen(argument);
        });

        orig.apply(console, arguments);
    };

}

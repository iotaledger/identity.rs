export function getExplorerUrl(doc, messageId) {
    return `${doc.id.tangleExplorer}/transaction/${messageId}`;
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

/*
  Write out the Targle Explorer URL given the network and message ID, with the given preamble.
*/
export function getExplorerUrl(doc, messageId) {
    return `${doc.id.tangleExplorer}/transaction/${messageId}`;
}

export function logToScreen(message) {
    document.querySelector("#content").innerHTML =
        document.querySelector("#content").innerHTML +
        message +
        "<br/>-------------------------------------<br/>";
}

export function logExplorerUrlToScreen(url) {
    logToScreen(`Explorer URL: <a target="_blank" href="${url}"> ${url} </a>`);
}

export function logObjectToScreen(obj) {
    logToScreen("<pre>" + JSON.stringify(obj, null, 4) + "</pre>");
}

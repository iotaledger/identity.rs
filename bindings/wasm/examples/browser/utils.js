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
    "<br/> ===================== <br/>";
}



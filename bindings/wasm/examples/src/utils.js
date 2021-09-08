// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 Write out the Targle Explorer URL given the network and message ID, with the given preamble.

 @param {!string} preamble
 @param {!string} network
 @param {!string} messageId
 **/
function logExplorerUrl(preamble, network, messageId) {
    console.log(`${preamble} https://explorer.iota.org/${network}net/transaction/${messageId}`);
}

/**
 Pretty-prints data to the console as a JSON string. This avoids nested fields being printed
 as [Object], [Array] by converting it to a full JSON string first.

 @param {!Object} data
 @param {!string | null} title
 **/
function prettyPrintJSON(data, title=null) {
    if (title != null) {
        console.log(title);
    }
    console.log(JSON.stringify(JSON.parse(data.toString()), null, 2));
}

export {logExplorerUrl, prettyPrintJSON}
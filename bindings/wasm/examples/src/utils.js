// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 Write out the Tangle Explorer URL given the network and message ID, with the given preamble.

 @param {!string} preamble
 @param {ExplorerUrl} explorer
 @param {!string} messageId
 **/
function logExplorerUrl(preamble, explorer, messageId) {
    console.log(`${preamble} ${explorer.messageUrl(messageId)}`);
}

/**
 Write out the Tangle Identity Resolver URL given the network and DID, with the given preamble.

 @param {!string} preamble
 @param {ExplorerUrl} explorer
 @param {!string} did
 **/
function logResolverUrl(preamble, explorer, did) {
    console.log(`${preamble} ${explorer.resolverUrl(did)}`);
}

/**
 Pretty-prints data to the console as a JSON string. This avoids nested fields being printed
 as [Object], [Array] by converting it to a full JSON string first.

 @param {!Object} data
 @param {!string | null} title
 **/
function prettyPrintJSON(data, title = null) {
    if (title != null) {
        console.log(title);
    }
    console.log(JSON.stringify(JSON.parse(data.toString()), null, 2));
}

/**
 * If a function throws an exception, run it again to make the tests more consistent (less prone to network issues).
 *
 * @param fn asynchronous function to be tested
 * @param args parameters for fn
 * @returns {Promise<void>}
 */
async function repeatAsyncTest(fn, ...args) {
    try {
        await fn(...args);
    } catch (e) {
        console.warn("Repeating async test due to error:", e);
        await fn(...args);
    }
}

export {logExplorerUrl, logResolverUrl, prettyPrintJSON, repeatAsyncTest}
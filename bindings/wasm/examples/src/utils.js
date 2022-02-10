// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

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

export {prettyPrintJSON}

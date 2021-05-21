// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const EXPLORER_URL_BASE = "https://explorer.iota.org/mainnet/transaction";

/*
  Write out the Targle Explorer URL given the network and message ID, with the given preamble.

  @param {!string} preamble
  @param {!string} network
  @param {!string} messageId
*/
function logExplorerUrl(preamble, network, messageId) {
  console.log(`${preamble} ${EXPLORER_URL_BASE}/${network}net/transaction/${messageId}`);
}

exports.logExplorerUrl = logExplorerUrl;
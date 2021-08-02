// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { Network } = require('../../node/identity_wasm')

const MAINNET = Network.mainnet();

/* @type {{network: Network, defaultNodeURL: string, explorerURL: string}} */
const CLIENT_CONFIG = {
    network: MAINNET,
    defaultNodeURL: MAINNET.defaultNodeURL,
    explorerURL: MAINNET.explorerURL,
}

exports.CLIENT_CONFIG = CLIENT_CONFIG;

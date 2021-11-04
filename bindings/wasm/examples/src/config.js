// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Network } from '@iota/identity-wasm';

const MAINNET = Network.mainnet();

/* @type {{network: Network, defaultNodeURL: string, explorerURL: string}} */
const CLIENT_CONFIG = {
    network: MAINNET,
    defaultNodeURL: MAINNET.defaultNodeURL,
    explorerURL: MAINNET.explorerURL,
}

export {CLIENT_CONFIG};

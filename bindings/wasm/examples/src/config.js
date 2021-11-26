// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Network, ExplorerUrl } from '@iota/identity-wasm';

const MAINNET = Network.mainnet();
const EXPLORER_MAINNET = ExplorerUrl.mainnet();

/* @type {{network: Network, explorer: ExplorerUrl}} */
const CLIENT_CONFIG = {
    network: MAINNET,
    explorer: EXPLORER_MAINNET,
}

export {CLIENT_CONFIG};

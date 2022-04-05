// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { AccountBuilder, Network, ExplorerUrl, AutoSave, Storage } from '../../node';

/**
 * This example demonstrates some of the configuration options for the account.
 */
async function config(storage?: Storage) {

    // Set-up for a private Tangle
    // You can use https://github.com/iotaledger/one-click-tangle for a local setup.
    // The `network_name` needs to match the id of the network or a part of it.
    // As an example we are treating the devnet as a private tangle, so we use `dev`.
    // When running the local setup, we can use `tangle` since the id of the one-click
    // private tangle is `private-tangle`, but we can only use 6 characters.
    // Keep in mind, there are easier ways to change to devnet via `Network::Devnet`
    const network_name = "dev";
    let network = Network.tryFromName(network_name)

    // If you deployed an explorer locally this would usually be `http://127.0.0.1:8082`
    const explorer = ExplorerUrl.parse("https://explorer.iota.org/devnet");

    // In a locally running one-click tangle, this would usually be `http://127.0.0.1:14265`
    let private_node_url = "https://api.lb-0.h.chrysalis-devnet.iota.cafe";


    // The creation step generates a keypair, builds an identity
    // and publishes it to the IOTA mainnet.
    const builder = new AccountBuilder({
        // `AutoSave.never()` never auto-saves, relies on the storage drop save.
        // `AutoSave.every()` saves immediately after every action,
        autosave: AutoSave.batch(10), // saves after every 10 actions.
        autopublish: true, // publish to the tangle automatically on every update
        clientConfig: {
            network: network,
            primaryNode: { url: private_node_url }
        },
        storage,
    });

    // Create an identity and publish it.
    // The created DID will use the network name configured for the client.
    try {
        const account = await builder.createIdentity();
        const did = account.did();

        // Prints the Identity Resolver Explorer URL.
        // The entire history can be observed on this page by clicking "Loading History".
        console.log(`[Example] Explore the DID Document = ${explorer.resolverUrl(did)}`);

    } catch (e) {
        if (e instanceof Error) {
            console.log(`[Example] Error: ${e.message}`);
        }
        console.log(`[Example] Is your Tangle node listening on ${private_node_url}?`);
    }
}

export { config };

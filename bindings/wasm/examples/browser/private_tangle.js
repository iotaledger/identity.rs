// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, Config, Document, KeyType, Network } from "../../web/identity_wasm.js";
import {
    logObjectToScreen,
    logToScreen,
} from "./utils.js";

/**
    This example shows how a DID document can be created on a private tangle.
    It can be run together with a local hornet node.
    Refer to https://github.com/iotaledger/one-click-tangle/tree/chrysalis/hornet-private-net
    for setup instructions.
**/
export async function createIdentityPrivateTangle(inBrowser = true, log = true) {
    if (log) logToScreen("Identity creation started...");
    if (log) logToScreen("This might take a few seconds to complete proof of work!");

    let restURL
    let networkName

    if (inBrowser) {
        // Get the required parameters from the input fields
        restURL = document.querySelector("#create-private-rest-url").value;
        networkName = document.querySelector("#create-private-network-name").value;
    } else {
        restURL = "http://127.0.0.1:14265/";
        networkName = "custom";
    }

    // This is an arbitrarily defined network name
    const network = Network.from_name(networkName);

    // Create a DID Document (an identity).
    const { doc, key } = new Document(KeyType.Ed25519, network.toString());

    // Sign the DID Document with the generated key.
    doc.sign(key);

    // Create a client configuration and set the custom network.
    const config = new Config();
    config.setNetwork(network);

    // This URL should point to the REST API of a node.
    config.setNode(restURL);

    // Create a client instance from the configuration to publish messages to the Tangle.
    const client = Client.fromConfig(config);

    // Publish the Identity to the IOTA Network, this may take a few seconds to complete Proof-of-Work.
    const receipt = await client.publishDocument(doc.toJSON());

    if (log) logToScreen("Identity creation done!");

    // Make sure the DID can be resolved on the private tangle
    const resolved = await client.resolve(doc.id.toString());

    if (log) logToScreen("Resolved DID document:");
    if (log) logObjectToScreen(resolved);

    // Return the results.
    return { key, doc, receipt };
}

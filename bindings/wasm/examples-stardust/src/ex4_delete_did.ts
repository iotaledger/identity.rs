// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {StardustDocument} from '../../node';

import {ED25519_ADDRESS_TYPE, IAliasOutput, IEd25519Address, IRent, TransactionHelper,} from '@iota/iota.js';
import {createIdentity} from "./ex0_create_did";
import {Converter} from "@iota/util.js";

/** Demonstrates how to delete a DID in an Alias Output, reclaiming the storage deposit. */
export async function deleteIdentity() {
    // Creates a new wallet and identity (see "ex0_create_did" example).
    const {didClient, walletKeyPair, walletAddress, did} = await createIdentity();

    // Deletes the Alias Output and its contained DID Document, rendering the DID permanently destroyed.
    // This operation is *not* reversible.
    // Deletion can only be done by the governor of the Alias Output.
    const destinationAddress: IEd25519Address = {
        type: ED25519_ADDRESS_TYPE,
        pubKeyHash: Converter.bytesToHex(walletAddress.toAddress(), true)
    };
    await didClient.deleteDidOutput(destinationAddress, walletKeyPair, did);

    // Wait for the node to index the new state.
    await new Promise(f => setTimeout(f, 5000));

    // Attempting to resolve a deleted DID results in a `NotFound` error.
    let deleted = false;
    try {
        await didClient.resolveDid(did);
    } catch(err) {
        deleted = true;
    }
    if (!deleted) {
        throw new Error("failed to delete DID");
    }
}

// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import type {StardustDocument} from '../../node';
import type {IAliasOutput} from '@iota/iota.js';

import {createIdentity} from "./ex0_create_did";

/** Demonstrates how to resolve an existing DID in an Alias Output. */
export async function resolveIdentity() {
    // Creates a new wallet and identity (see "ex0_create_did" example).
    const {didClient, did} = await createIdentity();

    // Resolve the associated Alias Output and extract the DID document from it.
    const resolved: StardustDocument = await didClient.resolveDid(did);
    console.log("Resolved DID document:", JSON.stringify(resolved, null, 2));

    // We can also resolve the Alias Output directly.
    const aliasOutput: IAliasOutput = await didClient.resolveDidOutput(did);
    console.log("The Alias Output holds " + aliasOutput.amount + " tokens");
}

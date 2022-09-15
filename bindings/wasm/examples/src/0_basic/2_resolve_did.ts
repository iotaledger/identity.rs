// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IotaDocument, IotaIdentityClient } from '../../../node';
import type { IAliasOutput } from '@iota/iota.js';
import { API_ENDPOINT, createDid } from '../util';
import { Bip39 } from '@iota/crypto.js';
import { Client, MnemonicSecretManager } from '@iota/iota-client-wasm/node';

/** Demonstrates how to resolve an existing DID in an Alias Output. */
export async function resolveIdentity() {
    const client = new Client({
        primaryNode: API_ENDPOINT,
        localPow: true,
    });
    const didClient = new IotaIdentityClient(client);

    // Generate a random mnemonic for our wallet.
    const secretManager: MnemonicSecretManager = {
        Mnemonic: Bip39.randomMnemonic()
    };

    // Creates a new wallet and identity (see "0_create_did" example).
    const { did } = await createDid(client, secretManager);

    // Resolve the associated Alias Output and extract the DID document from it.
    const resolved: IotaDocument = await didClient.resolveDid(did);
    console.log("Resolved DID document:", JSON.stringify(resolved, null, 2));

    // We can also resolve the Alias Output directly.
    const aliasOutput: IAliasOutput = await didClient.resolveDidOutput(did);
    console.log("The Alias Output holds " + aliasOutput.amount + " tokens");
}

// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IotaDID } from '@iota/identity-wasm/node';
import { IotaClient } from "@iota/iota-sdk/client";
import {
    createDocumentForNetwork,
    getClientAndCreateAccount,
    getMemstorage,
    NETWORK_URL,
} from '../util';

/** Demonstrate how to create a DID Document and publish it. */
export async function createIdentity(): Promise<void> {
    // create new client to connect to IOTA network
    const iotaClient = new IotaClient({ url: NETWORK_URL });
    const network = await iotaClient.getChainIdentifier();

    // create new client that offers identity related functions
    const storage = getMemstorage();
    const identityClient = await getClientAndCreateAccount(storage);

    // create new unpublished document 
    const [unpublished] = await createDocumentForNetwork(storage, network);
    console.log(`Unpublished DID document: ${JSON.stringify(unpublished, null, 2)}`);
    let did: IotaDID;

    // TODO: decide upon wich style to use here
    // so let's go with both for now, to show that both work
    if (Math.random() > .5) {
        console.log('Creating new identity fully via client flow');
        const { output: identity } = await identityClient
            .createIdentity(unpublished)
            .finish()
            .execute(identityClient);
        did = IotaDID.fromAliasId(identity.id(), identityClient.network());
    } else {
        console.log('Publishing document to identity');
        const { output: published } = await identityClient
            .publishDidDocument(unpublished)
            .execute(identityClient);
        did = published.id();
    }

    // check if we can resolve it via client
    const resolved = await identityClient.resolveDid(did);
    console.log(`Resolved DID document: ${JSON.stringify(resolved, null, 2)}`);
}

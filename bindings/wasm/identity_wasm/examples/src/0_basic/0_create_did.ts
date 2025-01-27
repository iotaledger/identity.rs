// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    createDidDocument,
    getClientAndCreateAccount,
    getMemstorage,
} from '../utils_alpha';

// old API:
// export async function createIdentityOld(): Promise<{
//     didClient: IotaIdentityClient;
//     secretManager: SecretManager;
//     walletAddressBech32: string;
//     did: IotaDID;
// }> { /* ... */ }

/** Demonstrate how to create a DID Document and publish it. */
export async function createIdentity(): Promise<void>  {
    // create new client to interact with chain and get funded account with keys
    const storage = getMemstorage();
    const identityClient = await getClientAndCreateAccount(storage);
  
    // create new DID document and publish it
    const [document] = await createDidDocument(identityClient, storage);
    console.log(`Published DID document: ${JSON.stringify(document, null, 2)}`);
  
    // check if we can resolve it via client
    const resolved = await identityClient.resolveDid(document.id());
    console.log(`Resolved DID document: ${JSON.stringify(resolved, null, 2)}`);
  }
  
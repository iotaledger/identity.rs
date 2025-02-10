// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    IotaDID,
    JwkMemStore,
    JwsAlgorithm,
    MethodRelationship,
    MethodScope,
    Service,
    Timestamp,
    VerificationMethod,
} from "@iota/identity-wasm/node";
import { IotaClient as IotaClient } from "@iota/iota-sdk/client";
import {
    createDocumentForNetwork,
    getClientAndCreateAccount,
    getMemstorage,
    NETWORK_URL,
    TEST_GAS_BUDGET,
} from '../utils_alpha';

/** Demonstrates how to update a DID document in an existing Alias Output. */
export async function updateIdentity() {
    // create new clients and create new account
    const iotaClient = new IotaClient({ url: NETWORK_URL });
    const network = await iotaClient.getChainIdentifier();
    const storage = getMemstorage();
    const [unpublished, vmFragment1] = await createDocumentForNetwork(storage, network);
    const identityClient = await getClientAndCreateAccount(storage);

    // create new identity for this account and publish document for it
    const { output: identity } = await identityClient
        .createIdentity(unpublished)
        .finish()
        .execute(identityClient);
    const did = IotaDID.fromAliasId(identity.id(), identityClient.network());

    // Resolve the latest state of the document.
    // Technically this is equivalent to the document above.
    const resolved = await identityClient.resolveDid(did);

    if ((storage as any).__wbg_ptr === 0) {
        console.log('cannot re-use storage, skipping generating new method');
    } else {
        console.log('can re-use storage, generating new method');
        // Insert a new Ed25519 verification method in the DID document.
        await resolved.generateMethod(
            storage,
            JwkMemStore.ed25519KeyType(),
            JwsAlgorithm.EdDSA,
            "#key-2",
            MethodScope.VerificationMethod(),
        );

        // Attach a new method relationship to the inserted method.
        resolved.attachMethodRelationship(did.join("#key-2"), MethodRelationship.Authentication);

        // Remove a verification method.
        let originalMethod = resolved.resolveMethod(vmFragment1) as VerificationMethod;
        await resolved.purgeMethod(storage, originalMethod?.id());
    }

    // Add a new Service.
    const service: Service = new Service({
        id: did.join("#linked-domain"),
        type: "LinkedDomains",
        serviceEndpoint: "https://iota.org/",
    });
    resolved.insertService(service);
    resolved.setMetadataUpdated(Timestamp.nowUTC());

    let updated = await identityClient
        .publishDidDocumentUpdate(resolved.clone(), TEST_GAS_BUDGET);
    // and resolve again to make sure we're looking at the onchain information
    const resolvedAgain = await identityClient.resolveDid(did);
    console.log(`Updated DID document result: ${JSON.stringify(resolvedAgain, null, 2)}`);
}

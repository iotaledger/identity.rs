// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    CoreDocument,
    DIDJwk,
    IotaDID,
    IToCoreDocument,
    Resolver,
} from "@iota/identity-wasm/node";
import { IotaClient } from "@iota/iota-sdk/client";
import {
    createDocumentForNetwork,
    getClientAndCreateAccount,
    getMemstorage,
    NETWORK_URL,
} from '../utils_alpha';

const DID_JWK: string =
    "did:jwk:eyJjcnYiOiJQLTI1NiIsImt0eSI6IkVDIiwieCI6ImFjYklRaXVNczNpOF91c3pFakoydHBUdFJNNEVVM3l6OTFQSDZDZEgyVjAiLCJ5IjoiX0tjeUxqOXZXTXB0bm1LdG00NkdxRHo4d2Y3NEk1TEtncmwyR3pIM25TRSJ9";

/** Demonstrates how to resolve an existing DID in an Alias Output. */
export async function resolveIdentity() {
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

    // Resolve the associated Alias Output and extract the DID document from it.
    const resolved = await identityClient.resolveDid(did);
    console.log("Resolved DID document:", JSON.stringify(resolved, null, 2));

    // We can resolve the Object ID directly
    const resolvedIdentity = await identityClient.getIdentity(identity.id());
    console.dir(resolvedIdentity);
    console.log(`Resolved identity has object ID ${resolvedIdentity.toFullFledged()?.id()}`);

    // Or we can resolve it via the `Resolver` api:

    // While at it, define a custom resolver for jwk DIDs as well.
    const handlers = new Map<string, (did: string) => Promise<CoreDocument | IToCoreDocument>>();
    handlers.set("jwk", didJwkHandler);

    // Create new `Resolver` instance
    const resolver = new Resolver({ client: identityClient, handlers });

    // and resolve identity DID with it.
    const resolverResolved = await resolver.resolve(did.toString());
    console.log(`resolverResolved ${DID_JWK} resolves to:\n ${JSON.stringify(resolverResolved, null, 2)}`);

    // We can also resolve via the custom resolver defined before:
    const did_jwk_resolved_doc = await resolver.resolve(DID_JWK);
    console.log(`DID ${DID_JWK} resolves to:\n ${JSON.stringify(did_jwk_resolved_doc, null, 2)}`);
}

const didJwkHandler = async (did: string) => {
    let did_jwk = DIDJwk.parse(did);
    return CoreDocument.expandDIDJwk(did_jwk);
};

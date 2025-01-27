// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// TODO:
// - [ ] clarify if we need/want a resolver example
// - [ ] clarify if we need/want the AliasOutput -> ObjectID example


import {
    CoreDocument,
    DIDJwk,
    IotaDocument,
    IotaIdentityClient,
    IToCoreDocument,
    JwkMemStore,
    KeyIdMemStore,
    Resolver,
    Storage,
} from "@iota/identity-wasm/node";
import { AliasOutput,  } from "@iota/sdk-wasm/node";
import { API_ENDPOINT, createDid } from "../util";
import { createDidDocument, getClientAndCreateAccount, getMemstorage } from "../utils_alpha";

const DID_JWK: string =
    "did:jwk:eyJjcnYiOiJQLTI1NiIsImt0eSI6IkVDIiwieCI6ImFjYklRaXVNczNpOF91c3pFakoydHBUdFJNNEVVM3l6OTFQSDZDZEgyVjAiLCJ5IjoiX0tjeUxqOXZXTXB0bm1LdG00NkdxRHo4d2Y3NEk1TEtncmwyR3pIM25TRSJ9";

/** Demonstrates how to resolve an existing DID in an Alias Output. */
export async function resolveIdentity() {
    // create new client to interact with chain and get funded account with keys
    const storage = getMemstorage();
    const identityClient = await getClientAndCreateAccount(storage);
  
    // create new DID document and publish it
    let [document] = await createDidDocument(identityClient, storage);
    let did = document.id();

    // Resolve the associated Alias Output and extract the DID document from it.
    const resolved: IotaDocument = await identityClient.resolveDid(did);
    console.log("Resolved DID document:", JSON.stringify(resolved, null, 2));

    // We can also resolve the Object ID reictly
    const aliasOutput: AliasOutput = await identityClient.resolveDidOutput(did);
    console.log("The Alias Output holds " + aliasOutput.getAmount() + " tokens");

    // did:jwk can be resolved as well.
    const handlers = new Map<string, (did: string) => Promise<CoreDocument | IToCoreDocument>>();
    handlers.set("jwk", didJwkHandler);
    const resolver = new Resolver({ handlers });
    const did_jwk_resolved_doc = await resolver.resolve(DID_JWK);
    console.log(`DID ${DID_JWK} resolves to:\n ${JSON.stringify(did_jwk_resolved_doc, null, 2)}`);
}

const didJwkHandler = async (did: string) => {
    let did_jwk = DIDJwk.parse(did);
    return CoreDocument.expandDIDJwk(did_jwk);
};

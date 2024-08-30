// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { CoreDocument, DIDJwk, IotaDocument, IotaIdentityClient, IToCoreDocument, JwkMemStore, KeyIdMemStore, Resolver, Storage } from "@iota/identity-wasm/node";
import { AliasOutput, Client, MnemonicSecretManager, Utils } from "@iota/sdk-wasm/node";
import { API_ENDPOINT, createDid } from "../util";

const DID_JWK: string = "did:jwk:eyJjcnYiOiJQLTI1NiIsImt0eSI6IkVDIiwieCI6ImFjYklRaXVNczNpOF91c3pFakoydHBUdFJNNEVVM3l6OTFQSDZDZEgyVjAiLCJ5IjoiX0tjeUxqOXZXTXB0bm1LdG00NkdxRHo4d2Y3NEk1TEtncmwyR3pIM25TRSJ9";

/** Demonstrates how to resolve an existing DID in an Alias Output. */
export async function resolveIdentity() {
    const client = new Client({
        primaryNode: API_ENDPOINT,
        localPow: true,
    });
    const didClient = new IotaIdentityClient(client);

    // Generate a random mnemonic for our wallet.
    const secretManager: MnemonicSecretManager = {
        mnemonic: Utils.generateMnemonic(),
    };

    // Creates a new wallet and identity (see "0_create_did" example).
    const storage: Storage = new Storage(new JwkMemStore(), new KeyIdMemStore());
    let { document } = await createDid(
        client,
        secretManager,
        storage,
    );
    const did = document.id();

    // Resolve the associated Alias Output and extract the DID document from it.
    const resolved: IotaDocument = await didClient.resolveDid(did);
    console.log("Resolved DID document:", JSON.stringify(resolved, null, 2));

    // We can also resolve the Alias Output directly.
    const aliasOutput: AliasOutput = await didClient.resolveDidOutput(did);
    console.log("The Alias Output holds " + aliasOutput.getAmount() + " tokens");

    // did:jwk can be resolved as well.
    const handlers = new Map<string, (did: string) => Promise<CoreDocument | IToCoreDocument>>();
    handlers.set("jwk", didJwkHandler);
    const resolver = new Resolver({ handlers });
    const did_jwk_resolved_doc = await resolver.resolve(DID_JWK);
    console.log(`DID ${DID_JWK} resolves to:\n ${JSON.stringify(did_jwk_resolved_doc, null, 2)}`)
}

const didJwkHandler = async (did: string) => {
    let did_jwk = DIDJwk.parse(did);
    return CoreDocument.expandDIDJwk(did_jwk);
}

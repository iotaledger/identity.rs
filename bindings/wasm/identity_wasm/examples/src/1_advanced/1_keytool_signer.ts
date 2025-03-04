// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IdentityClient, IdentityClientReadOnly, IotaDocument } from "@iota/identity-wasm/node";
import { KeytoolSigner } from "@iota/iota-interaction-ts/node/iota_interaction_ts";
import { IotaClient } from "@iota/iota-sdk/client";
import { IOTA_IDENTITY_PKG_ID, NETWORK_URL } from "../util";

export async function useIotaKeytoolSigner() {
    const iotaClient = new IotaClient({ url: NETWORK_URL });
    const identityClientReadOnly = await IdentityClientReadOnly.createWithPkgId(iotaClient, IOTA_IDENTITY_PKG_ID);

    // Use IOTA Keytool's active adderess to sign transactions.
    // Without arguments, `KeytoolSigner.create` will try to use the
    // `iota` binary in PATH.
    const keytoolSigner = await KeytoolSigner.create();
    const identityClient = await IdentityClient.create(identityClientReadOnly, keytoolSigner);
    const networkId = identityClient.network();

    // Create a new identity with an empty DID Document.
    const identity = await identityClient
        .createIdentity(new IotaDocument(networkId))
        .finish()
        .execute(identityClient)
        .then(({ output }) => output);

    console.log(
        `Created new Identity ${identity.didDocument().id()} with ${identityClient.senderAddress()} as its controller.`,
    );
}

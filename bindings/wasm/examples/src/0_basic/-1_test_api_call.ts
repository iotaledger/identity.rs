// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IotaDID, IotaDocument, KinesisIdentityClient } from "@iota/identity-wasm/node";

import { IotaClient as KinesisClient } from "@iota/iota.js/client";


/** Demonstrate how to create a DID Document and publish it in a new Alias Output. */
export async function testApiCall(): Promise<void> {
    const kinesis_client = new KinesisClient({ url: 'http://127.0.0.1:9000' });
    const balanceFromKinesisClient = await kinesis_client.getChainIdentifier();
    console.dir(balanceFromKinesisClient);

    // test builder
    let clientFromBuilder = KinesisIdentityClient
      .builder()
      .identity_iota_package_id('foo')
      .network_name('bar')
      .sender_public_key(new Uint8Array([1, 2, 3, 4]))
      .iota_client(kinesis_client)
      .build()
      ;

    console.dir(await clientFromBuilder.getBalance());

    console.dir(clientFromBuilder.senderPublicKey());

    console.dir(clientFromBuilder.senderAddress());

    console.dir(clientFromBuilder.networkName());

    try {
        await clientFromBuilder.getIdentity("foobar");
    } catch(ex) {
        console.log((ex as Error).message);
    }

    const did4resolveDid = IotaDID.parse("did:iota:0x0101010101010101010101010101010101010101010101010101010101010101");
    try {
        await clientFromBuilder.resolveDid(did4resolveDid);
    } catch(ex) {
        console.log((ex as Error).message);
    }

    const document1 = new IotaDocument("foobar");
    try {
        await clientFromBuilder.publishDidDocument(document1, BigInt(12345), "dummy signer");
    } catch(ex) {
        console.log((ex as Error).message);
    }

    const document2 = new IotaDocument("foobar");
    try {
        await clientFromBuilder.publishDidDocumentUpdate(document2, BigInt(12345), "dummy signer");
    } catch(ex) {
        console.log((ex as Error).message);
    }

    const did4deactivateDidOutput = IotaDID.parse("did:iota:0x0101010101010101010101010101010101010101010101010101010101010101");
    try {
        await clientFromBuilder.deactivateDidOutput(did4deactivateDidOutput, BigInt(12345), "dummy signer");
    } catch(ex) {
        console.log((ex as Error).message);
    }

    console.log("done");
}

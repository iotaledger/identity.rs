// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { KinesisIdentityClient } from "@iota/identity-wasm/node";

import { IotaClient as KinesisClient } from "@iota/iota.js/client";


/** Demonstrate how to create a DID Document and publish it in a new Alias Output. */
export async function testApiCall(): Promise<void> {
    const kinesis_client = new KinesisClient({ url: 'http://127.0.0.1:9000' });

    const testResponse = await kinesis_client.getChainIdentifier();

    // should also fail
    const identity_client = new KinesisIdentityClient(kinesis_client);

    // should also fail
    const balance = await identity_client.getBalance();
    console.dir(balance);

    // test builder
    let clientFromBuilder = KinesisIdentityClient
      .builder()
      .identity_iota_package_id('foo')
      .network_name('bar')
      .sender_public_key(new Uint8Array())
      .iota_client(kinesis_client)
      .build()
      ;

    const balance2 = await clientFromBuilder.getBalance();
    console.dir(balance2);
}

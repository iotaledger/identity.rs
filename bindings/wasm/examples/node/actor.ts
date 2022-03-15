// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { Actor, WasmTestRequest } = require('../../node/identity_wasm')
const { Multiaddr } = require("multiaddr")
const PeerId = require('peer-id')

async function actor() {

    const addr = "/ip4/127.0.0.1/tcp/12345/ws"
    const peer_id = "12D3KooWB5i4yZNqd9YjrUvY7SQfYjYJDuGHSVeptg819MP4TNd6"

    const address = new Multiaddr(addr)

    const peerId = PeerId.createFromB58String(peer_id)

    const actor = new Actor();

    const request = new WasmTestRequest("test-request").toJSON();

    console.log(`Adding peer ${peerId} on address ${address}`);
    await actor.addAddress(peerId, address);

    console.log(`Sending request `, request);
    let result = await actor.sendRequest(peerId, request);

    console.log(`Received: `, result);

    await actor.shutdown()
}

actor()
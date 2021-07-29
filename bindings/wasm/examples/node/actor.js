// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { IdentityActor } = require('../../node/identity_wasm')
const { Multiaddr } = require("multiaddr")
const PeerId = require('peer-id')

async function actor() {

    const addr = "/ip4/0.0.0.0/tcp/12345/ws"
    const peer_id = "..."

    const address = new Multiaddr(addr)

    const peerId = PeerId.createFromB58String(peer_id)

    const actor = IdentityActor.new();

    console.log(`Adding peer ${peerId} on address ${address}`);
    await actor.addPeer(peerId, address);

    console.log(`Sending "IdentityList" request`);
    let result = await actor.sendRequest(peerId);

    console.log(`Available identities on the remote actor...`);
    console.log(result);
}

exports.actor = actor;

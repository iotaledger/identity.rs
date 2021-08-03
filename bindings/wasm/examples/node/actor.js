// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { IdentityActor, IdentityResolve, IdentityList } = require('../../node/identity_wasm')
const { Multiaddr } = require("multiaddr")
const PeerId = require('peer-id')

async function actor() {

    const addr = "/ip4/0.0.0.0/tcp/12346/ws"
    const peer_id = "12D3KooWMP7sWFDzuzxeQZojBFMazwKXcXMyZ8KGX14JGgrzUkAm"

    const address = new Multiaddr(addr)

    const peerId = PeerId.createFromB58String(peer_id)

    const actor = IdentityActor.new();

    const resolve = new IdentityResolve("did:iota:Gq9tCpiNTYGewnQbTUBxB5K1GCYsAscXdrvYEFkjJ1JJ")

    // The same, but written in JavaScript.
    // Should only be used for custom defined types or if the bindings
    // do not export the type natively.
    // const resolve = {
    //     did: "did:iota:Gq9tCpiNTYGewnQbTUBxB5K1GCYsAscXdrvYEFkjJ1JJ",
    //     requestName: () => "storage/resolve"
    // };

    console.log(`Adding peer ${peerId} on address ${address}`);
    await actor.addPeer(peerId, address);

    console.log(`Sending request`);
    let result = await actor.sendRequest(peerId, resolve);

    console.log(result);
}

exports.actor = actor;

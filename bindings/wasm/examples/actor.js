// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { Multiaddr } = require("multiaddr");
const PeerId = require('peer-id')

const { IdentityActor } = require('../node/identity_wasm')


async function actor() {

    const address = new Multiaddr("/ip4/127.0.0.1/tcp/1337/ws")
    console.log(JSON.stringify(address));

    const peerId = PeerId.createFromB58String("12D3KooWBCyKkPEpKXiJiFxLJzmv2PyEgNFpmRkoa7JAi9GVgzy8")
    console.log(JSON.stringify(peerId));

    const actor = IdentityActor.new();
    console.log(JSON.stringify(actor));

    actor.addPeer(peerId, address);

    let result = actor.sendRequest(peerId);

    console.log(JSON.stringify(result));
}

exports.actor = actor;

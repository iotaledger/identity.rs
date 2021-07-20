// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import * as identity from "../../web/identity_wasm.js";
import { logObjectToScreen, logToScreen } from "./utils.js";

export async function actor() {

    const addr = document.querySelector("#actor-addr").value;
    const peer_id = document.querySelector("#actor-peer-id").value;

    const address = new Multiaddr.Multiaddr(addr)

    const peerId = PeerId.createFromB58String(peer_id)

    const actor = identity.IdentityActor.new();

    logToScreen(`Adding peer ${peerId} on address ${address}`);
    await actor.addPeer(peerId, address);

    logToScreen(`Sending "IdentityList" request`);
    let result = await actor.sendRequest(peerId);

    logToScreen(`Available identities on the remote actor...`);
    logObjectToScreen(result);
}

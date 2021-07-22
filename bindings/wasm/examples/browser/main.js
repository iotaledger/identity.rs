import { createIdentity } from "./create_did.js";
import * as identity from "../../web/identity_wasm.js";

import { logToScreen } from "./utils.js";
import { createVC } from "./create_vc.js";
import { manipulateIdentity } from "./mainpulate_did.js";
import { resolveIdentity } from "./resolve.js";
import { createVP } from "./create_vp.js";
import { revoke } from "./revocation.js";
import { merkleKey } from "./merkle_key.js";

logToScreen("Initialization started...");

try {
    await identity.init("../../web/identity_wasm_bg.wasm");
    logToScreen("Initialization success!");
} catch (err) {
    logToScreen(err);
}

const mainNet = identity.Network.mainnet();

const CLIENT_CONFIG = {
    network: mainNet,
    defaultNodeURL: mainNet.defaultNodeURL,
    explorerURL: mainNet.explorerURL,
};

//handle create identity on click event
document
    .querySelector("#create-identity-btn")
    .addEventListener("click", () => createIdentity(CLIENT_CONFIG));

//handle resolve DID on click event
document
    .querySelector("#resolve-did-btn")
    .addEventListener("click", () => resolveIdentity(CLIENT_CONFIG));

//handle manipulate DID on click event
document
    .querySelector("#manipulate_did_btn")
    .addEventListener("click", () => manipulateIdentity(CLIENT_CONFIG));

//handle create VC on click event
document
    .querySelector("#create_vc_btn")
    .addEventListener("click", () => createVC(CLIENT_CONFIG));

//handle create VP on click event
document
    .querySelector("#create_vp_btn")
    .addEventListener("click", () => createVP(CLIENT_CONFIG));

//handle revoke VC on click event
document
    .querySelector("#revoke_vc_btn")
    .addEventListener("click", () => revoke(CLIENT_CONFIG));

//handle merkle key on click event
document
    .querySelector("#merkle_key_btn")
    .addEventListener("click", () => merkleKey(CLIENT_CONFIG));

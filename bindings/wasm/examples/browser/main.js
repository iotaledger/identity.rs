import { initIdentity, defaultClientConfig } from "./utils.js";
import { createIdentity } from "./create_did.js";
import { createVC } from "./create_vc.js";
import { manipulateIdentity } from "./mainpulate_did.js";
import { resolveIdentity } from "./resolve.js";
import { createVP } from "./create_vp.js";
import { revoke } from "./revocation.js";
import { merkleKey } from "./merkle_key.js";

await initIdentity();
const clientConfig = defaultClientConfig();

//handle create identity on click event
document
    .querySelector("#create-identity-btn")
    .addEventListener("click", () => createIdentity(clientConfig));

//handle resolve DID on click event
document
    .querySelector("#resolve-did-btn")
    .addEventListener("click", () => resolveIdentity(clientConfig));

//handle manipulate DID on click event
document
    .querySelector("#manipulate_did_btn")
    .addEventListener("click", () => manipulateIdentity(clientConfig));

//handle create VC on click event
document
    .querySelector("#create_vc_btn")
    .addEventListener("click", () => createVC(clientConfig));

//handle create VP on click event
document
    .querySelector("#create_vp_btn")
    .addEventListener("click", () => createVP(clientConfig));

//handle revoke VC on click event
document
    .querySelector("#revoke_vc_btn")
    .addEventListener("click", () => revoke(clientConfig));

//handle merkle key on click event
document
    .querySelector("#merkle_key_btn")
    .addEventListener("click", () => merkleKey(clientConfig));

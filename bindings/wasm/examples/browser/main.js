import { initIdentity, defaultClientConfig } from "./utils.js";
import { createIdentity } from "./create_did.js";
import { createVC } from "./create_vc.js";
import { manipulateIdentity } from "./mainpulate_did.js";
import { resolveIdentity } from "./resolve.js";
import { createVP } from "./create_vp.js";
import { revoke } from "./revoke_vc.js";
import { merkleKey } from "./merkle_key.js";
import { createIdentityPrivateTangle } from "./private_tangle.js";
import { createDiff } from "./diff_chain.js";
import { resolveHistory } from "./resolve_history.js";

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

//handle private tangle DID creation on click event
document
    .querySelector("#private_tangle_btn")
    .addEventListener("click", () => createIdentityPrivateTangle());

//handle diff chain on click event
document
    .querySelector("#diff_chain_btn")
    .addEventListener("click", () => createDiff(clientConfig));

//handle resolve history on click event
document
.querySelector("#did_history_btn")
.addEventListener("click", () => resolveHistory(clientConfig));

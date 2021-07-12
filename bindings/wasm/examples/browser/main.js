import * as id from "../../web/identity_wasm.js";
import { handleCreateIdentity } from "./button_handler/create_did.js";
import { handleCreateVC } from "./button_handler/create_vc.js";
import { handleManipulateDid } from "./button_handler/manipulate_did.js";
import { handleResolveDid } from "./button_handler/resolve_did.js";

import { logToScreen } from "./utils.js";

logToScreen("Initialization started...");

try {
    await id.init("../../web/identity_wasm_bg.wasm");
    logToScreen("Initialization success!");
} catch (err) {
    logToScreen(err);
}

//handle create identity on click event
document
    .querySelector("#create-identity-btn")
    .addEventListener("click", handleCreateIdentity);

//handle resolve DID on click event
document
    .querySelector("#resolve-did-btn")
    .addEventListener("click", handleResolveDid);

//handle resolve DID on click event
document
    .querySelector("#manipulate_did_btn")
    .addEventListener("click", handleManipulateDid);

//handle resolve DID on click event
document
    .querySelector("#create_vc_btn")
    .addEventListener("click", handleCreateVC);

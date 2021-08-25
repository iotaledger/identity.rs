import { initIdentity, defaultClientConfig, logToScreen, logObjectToScreen, linkify, LINK_REGEX } from "./utils_web.js";
import { createIdentity } from "./create_did.js";
import { createVC } from "./create_vc.js";
// import { manipulateIdentity } from "./manipulate_did.js";
// // import { resolveIdentity } from "./resolve.js";
// import { createVP } from "./create_vp.js";
// import { revokeVC } from "./revoke_vc.js";
// import { merkleKey } from "./merkle_key.js";
// import { createIdentityPrivateTangle } from "./private_tangle.js";
// import { createDiff } from "./diff_chain.js";
// import { resolveHistory } from "./resolve_history.js";

await initIdentity();
const CLIENT_CONFIG = defaultClientConfig();

var orig = console.log;

console.log = function() {

    
    Array.from(arguments).forEach(argument => {
        if (typeof argument === 'object') {
            return logObjectToScreen(argument);
        } else if (typeof argument === 'string' && argument.match(LINK_REGEX)) {
            return logToScreen(linkify(argument));
        }
        logToScreen(argument);
    });

    orig.apply(console, arguments);
};

//handle create identity on click event
document
    .querySelector("#create-identity-btn")
    .addEventListener("click", () => createIdentity(CLIENT_CONFIG));

// // //handle resolve DID on click event
// // document
// //     .querySelector("#resolve-did-btn")
// //     .addEventListener("click", () => resolveIdentity(CLIENT_CONFIG));

// //handle manipulate DID on click event
// document
//     .querySelector("#manipulate_did_btn")
//     .addEventListener("click", () => manipulateIdentity(CLIENT_CONFIG));

//handle create VC on click event
document
    .querySelector("#create_vc_btn")
    .addEventListener("click", () => createVC(CLIENT_CONFIG));

// //handle create VP on click event
// document
//     .querySelector("#create_vp_btn")
//     .addEventListener("click", () => createVP(CLIENT_CONFIG));

// //handle revoke VC on click event
// document
//     .querySelector("#revoke_vc_btn")
//     .addEventListener("click", () => revoke(CLIENT_CONFIG));

// //handle merkle key on click event
// document
//     .querySelector("#merkle_key_btn")
//     .addEventListener("click", () => merkleKey(CLIENT_CONFIG));

// //handle private tangle DID creation on click event
// document
//     .querySelector("#private_tangle_btn")
//     .addEventListener("click", () => createIdentityPrivateTangle());

// //handle diff chain on click event
// document
//     .querySelector("#diff_chain_btn")
//     .addEventListener("click", () => createDiff(CLIENT_CONFIG));

// //handle resolve history on click event
// document
// .querySelector("#did_history_btn")
// .addEventListener("click", () => resolveHistory(CLIENT_CONFIG));

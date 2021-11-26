// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {createIdentity} from "./create_did";
import {manipulateIdentity} from "./manipulate_did";
import {privateTangle} from "./private_tangle";
import {resolution} from "./resolution";
import {createVC} from "./create_vc";
import {createVP} from "./create_vp";
import {revokeVC} from "./revoke_vc";
import {merkleKey} from "./merkle_key";
import {CLIENT_CONFIG} from "./config";
import {resolveHistory} from "./resolve_history";
import {createDiff} from "./diff_chain";

async function main() {
    //Check if an example is mentioned
    if (process.argv.length != 3) {
        throw "Please provide one command line argument with the example name.";
    }

    //Take out the argument
    let argument = process.argv[2];
    switch (argument) {
        case "create_did":
            return await createIdentity(CLIENT_CONFIG);
        case "manipulate_did":
            return await manipulateIdentity(CLIENT_CONFIG);
        case "resolution":
            return await resolution(CLIENT_CONFIG);
        case "create_vc":
            return await createVC(CLIENT_CONFIG);
        case "revoke_vc":
            return await revokeVC(CLIENT_CONFIG);
        case "create_vp":
            return await createVP(CLIENT_CONFIG);
        case "merkle_key":
            return await merkleKey(CLIENT_CONFIG);
        case "private_tangle":
            return await privateTangle();
        case "resolve_history":
            return await resolveHistory(CLIENT_CONFIG);
        case "diff_chain":
            return await createDiff(CLIENT_CONFIG);
        case "all":
            console.log(">>> Run All Examples");

            await createIdentity(CLIENT_CONFIG);
            await manipulateIdentity(CLIENT_CONFIG);
            await resolution(CLIENT_CONFIG);
            await createVC(CLIENT_CONFIG);
            await revokeVC(CLIENT_CONFIG);
            await createVP(CLIENT_CONFIG);
            await merkleKey(CLIENT_CONFIG);
            await resolveHistory(CLIENT_CONFIG);
            await createDiff(CLIENT_CONFIG);

            console.log(">>> End All Examples");
            return "all";
        default:
            throw "Unknown example name";
    }
}

main()
    .then((output) => {
        console.log("Ok >", output);
    })
    .catch((error) => {
        console.log("Err >", error);
    });

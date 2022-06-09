// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { createIdentity } from "./create_did";
import { manipulateIdentity } from "./manipulate_did";
import { privateTangle } from "./private_tangle";
import { resolveIdentity } from "./resolve_did";
import { CLIENT_CONFIG } from "./config";
import { resolveHistory } from "./resolve_history";
import { keyExchange } from "./key_exchange";

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
        case "resolve_did":
            return await resolveIdentity(CLIENT_CONFIG);
        case "key_exchange":
            return await keyExchange(CLIENT_CONFIG);
        case "private_tangle":
            return await privateTangle();
        case "resolve_history":
            return await resolveHistory(CLIENT_CONFIG);
        case "all":
            console.log(">>> Run All Examples");

            await createIdentity(CLIENT_CONFIG);
            await manipulateIdentity(CLIENT_CONFIG);
            await resolution(CLIENT_CONFIG);
            await keyExchange(CLIENT_CONFIG);
            await resolveHistory(CLIENT_CONFIG);

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

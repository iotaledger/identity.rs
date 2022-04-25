// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { createIdentity } from "./basic/1_create_did";
import { manipulateIdentity } from "./basic/2_manipulate_did";
import { createVC } from "./basic/3_create_vc";
import { createVP } from "./basic/4_create_vp";
import { revokeVC } from "./basic/5_revoke_vc";
import { signing } from "./basic/7_signing";
import { keyExchange } from "./advanced/1_key_exchange";
import { config } from "./basic/8_config";
import { lazy } from "./basic/9_lazy";
import { multipleIdentities } from "./basic/6_multiple_identities";
import { resolveHistory } from "./advanced/2_resolve_history";
import { unchecked } from "./advanced/3_unchecked";
import { storageTestSuite } from "./advanced/4_custom_storage";

async function main() {
    //Check if an example is mentioned
    if (process.argv.length != 3) {
        throw "Please provide one command line argument with the example name.";
    }

    //Take out the argument
    let argument = process.argv[2];
    switch (argument) {
        case "create_did":
            return await createIdentity();
        case "manipulate_did":
            return await manipulateIdentity();
        case "lazy":
            return await lazy();
        case "signing":
            return await signing();
        case "config":
            return await config();
        case "unchecked":
            return await unchecked();
        case "multiple_identities":
            return await multipleIdentities();
        case "create_vc":
            return await createVC();
        case "create_vp":
            return await createVP();
        case "revoke_vc":
            return await revokeVC();
        case "custom_storage":
            return await storageTestSuite()
        case "resolve_history":
            return await resolveHistory()
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

// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { config } from "./config";
import { createIdentity } from "./create_did";
import { createVC } from "./create_vc";
import { createVP } from "./create_vp";
import { lazy } from "./lazy";
import { manipulateIdentity } from "./manipulate_did";
import { multipleIdentities } from "./multiple_identities";
import { revokeVC } from "./revoke_vc";
import { signing } from "./signing";
import { storageTestSuite } from "./memory_storage";
import { unchecked } from "./unchecked";

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

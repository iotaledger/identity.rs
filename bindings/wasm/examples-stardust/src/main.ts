// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {createIdentity} from "./ex0_create_did";
import {updateIdentity} from "./ex1_update_did";
import {resolveIdentity} from "./ex2_resolve_did";
import {deactivateIdentity} from "./ex3_deactivate_did";
import {deleteIdentity} from "./ex4_delete_did";
import {customResolution} from "./ex5_custom_resolution";

async function main() {
    // Extract example name.
    if (process.argv.length != 3) {
        throw "Please specify an example name, e.g. 'ex0_create_did'";
    }
    const argument = process.argv[2].toLowerCase();

    switch (argument) {
        case "ex0_create_did":
            return await createIdentity();
        case "ex1_update_did":
            return await updateIdentity();
        case "ex2_resolve_did":
            return await resolveIdentity();
        case "ex3_deactivate_did":
            return await deactivateIdentity();
        case "ex4_delete_did":
            return await deleteIdentity();
        case "ex5_custom_resolution":
            return await customResolution();
        default:
            throw "Unknown example name: '" + argument + "'";
    }
}

main()
    .catch((error) => {
        console.log("Example error:", error);
    });

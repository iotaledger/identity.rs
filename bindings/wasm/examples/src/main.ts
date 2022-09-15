// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { didControlsDid } from "./0_did_controls_did";
import { didIssuesNft } from "./1_did_issues_nft";
import { nftOwnsDid } from "./2_nft_owns_did";
import { didIssuesTokens } from "./3_did_issues_tokens";
import {createIdentity} from "./ex0_create_did";
import {updateIdentity} from "./ex1_update_did";
import {resolveIdentity} from "./ex2_resolve_did";
import {deactivateIdentity} from "./ex3_deactivate_did";
import {deleteIdentity} from "./ex4_delete_did";

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
        case "0_did_controls_did":
            return await didControlsDid();
        case "1_did_issues_nft":
            return await didIssuesNft();
        case "2_nft_owns_did":
            return await nftOwnsDid();
        case "3_did_issues_tokens":
            return await didIssuesTokens();
        default:
            throw "Unknown example name: '" + argument + "'";
    }
}

main()
    .catch((error) => {
        console.log("Example error:", error);
    });

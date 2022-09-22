// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { createIdentity } from "./0_basic/0_create_did";
import { updateIdentity } from "./0_basic/1_update_did";
import { resolveIdentity } from "./0_basic/2_resolve_did";
import { deactivateIdentity } from "./0_basic/3_deactivate_did";
import { deleteIdentity } from "./0_basic/4_delete_did";
import { didControlsDid } from "./1_advanced/0_did_controls_did";
import { didIssuesNft } from "./1_advanced/1_did_issues_nft";
import { nftOwnsDid } from "./1_advanced/2_nft_owns_did";
import { didIssuesTokens } from "./1_advanced/3_did_issues_tokens";
import { keyExchange } from "./1_advanced/4_key_exchange";
import { customResolution } from "./1_advanced/5_custom_resolution";

async function main() {
    // Extract example name.
    if (process.argv.length != 3) {
        throw "Please specify an example name, e.g. '0_create_did'";
    }
    const argument = process.argv[2].toLowerCase();

    switch (argument) {
        case "0_create_did":
            return await createIdentity();
        case "1_update_did":
            return await updateIdentity();
        case "2_resolve_did":
            return await resolveIdentity();
        case "3_deactivate_did":
            return await deactivateIdentity();
        case "4_delete_did":
            return await deleteIdentity();
        case "0_did_controls_did":
            return await didControlsDid();
        case "1_did_issues_nft":
            return await didIssuesNft();
        case "2_nft_owns_did":
            return await nftOwnsDid();
        case "3_did_issues_tokens":
            return await didIssuesTokens();
        case "4_key_exchange":
            return await keyExchange();
        case "5_custom_resolution":
            return await customResolution();
        default:
            throw "Unknown example name: '" + argument + "'";
    }
}

main()
    .catch((error) => {
        console.log("Example error:", error);
    });

// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    StatusList2021CredentialBuilder,
    Credential,
    JwtCredentialValidator,
    StatusCheck,
    Timestamp,
} from "@iota/identity-wasm/node";

export function statusList2021() {
    // Create a new status list to be stored off-chain, for the sake of this example
    // its going to stay in memory.
    const statusListCredential = new StatusList2021CredentialBuilder()
        .subjectId("https://example.com/credentials/status")
        .issuer("did:example:1234")
        .build();
    
    // Let's revoke a credential using this status list.
    // First we create a credential.
    const credential = new Credential({
        id: "https://example.com/credentials/12345678",
        issuer: "did:example:1234",
        issuanceDate: new Timestamp(),
        credentialSubject: {
            id: "did:example:4321",
            type: "UniversityDegree",
            gpa: "4.0",
        }
    });

    // We add to this credential a status which references the 420th entry
    // in the status list we previously created. Its JSON representation would look like this:
    // {
    //   "id": "https://example.com/credentials/status#420",
    //   "type": "StatusList2021Entry",
    //   "statusPurpose": "revocation",
    //   "statusListIndex": "420",
    //   "statusListCredential": "https://example.com/credentials/status"
    // }
    const _entry = statusListCredential.setCredentialStatus(credential, 420, true);

    // The credential is now revoked and won't be successfully validated
    try {
        JwtCredentialValidator.checkStatusWithStatusList2021(credential, statusListCredential, StatusCheck.Strict)
    } catch (e) {
        console.log((e as Error).message);
    }
}
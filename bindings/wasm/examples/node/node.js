// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { createIdentity } = require('./create_did');
const { manipulateIdentity } = require('./manipulate_did');
const { resolution } = require('./resolution');
const { createVC } = require('./create_vc');
const { createVP } = require('./create_vp');
const { revokeVC } = require('./revocation');
const { merkleKey } = require('./merkle_key');
const { CLIENT_CONFIG } = require('./config')

async function main() {
    //Check if an example is mentioned
    if (process.argv.length != 3) {
        throw 'Please provide one command line argument with the example name.';
    }

    //Take out the argument
    let argument = process.argv[2];
    switch (argument) {
        case 'create_did':
            return await createIdentity(CLIENT_CONFIG);
        case 'manipulate_did':
            return await manipulateIdentity(CLIENT_CONFIG);
        case 'resolution':
            return await resolution(CLIENT_CONFIG);
        case 'create_vc':
            return await createVC(CLIENT_CONFIG);
        case 'revocation':
            return await revokeVC(CLIENT_CONFIG);
        case 'create_vp':
            return await createVP(CLIENT_CONFIG);
        case 'merkle_key':
            return await merkleKey(CLIENT_CONFIG);
        case 'all':
            console.log(">>> Run All Examples");

            await createIdentity(CLIENT_CONFIG);
            await manipulateIdentity(CLIENT_CONFIG);
            await resolution(CLIENT_CONFIG);
            await createVC(CLIENT_CONFIG);
            await revokeVC(CLIENT_CONFIG);
            await createVP(CLIENT_CONFIG);
            await merkleKey(CLIENT_CONFIG);

            console.log(">>> End All Examples");
            return "all"
        default:
            throw 'Unknown example name';
    }
}

main().then((output) => {
    console.log("Ok >", output)
}).catch((error) => {
    console.log("Err >", error)
})

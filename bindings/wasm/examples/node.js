// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const { createIdentity } = require('./create_did');
const { manipulateIdentity } = require('./manipulate_did');
const { resolution } = require('./resolution');
const { createVC } = require('./create_vc');
const { createVP } = require('./create_vp');
const { revokeVC } = require('./revocation');
const { merkleKey } = require('./merkle_key');

async function main() {
    //Check if an example is mentioned
    if(process.argv.length != 3) {
        throw 'Please provide one command line argument with the example name.';
    }

    //Take out the argument
    let argument = process.argv[2];
    switch(argument) {
        case 'create_did':
            return await createIdentity();
        case 'manipulate_did':
            return await manipulateIdentity();
        case 'resolution':
            return await resolution();
        case 'create_vc':
            return await createVC();
        case 'revocation':
            return await revokeVC();
        case 'create_vp':
            return await createVP();
        case 'merkle_key':
            return await merkleKey();
        default:
            throw 'Unknown example name';
    }
}

main().then((output) => {
    console.log("Ok >", output)
}).catch((error) => {
    console.log("Err >", error)
})

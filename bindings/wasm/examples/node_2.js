// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

const {createIdentity} = require('./create_did');
const {manipulateIdentity} = require('./manipulate_did');
const {resolution} = require('./resolution');

async function main() {
    //Check if an example is mentioned
    if(process.argv.length != 3) {
        throw 'Please provide one command line argument with the example name.'; 
    }

    //Take out the argument
    let argument = process.argv[2];
    argument = argument.substr(1);
    switch(argument) {
        case 'create_did':
            return await createIdentity();
        case 'manipulate_did':
            return await manipulateIdentity();
        case 'resolution':
            return await resolution();
        default:
            throw 'Unknown example name';
            break;
    }
}

main().then((output) => {
    console.log("Ok >", output)
}).catch((error) => {
    console.log("Err >", error)
})
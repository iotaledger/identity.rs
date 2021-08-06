const { createIdentity } = require("./create_did");
const { manipulateIdentity } = require("./manipulate_did");
const { resolution } = require("./resolution");
const { createVC } = require("./create_vc");
const { createVP } = require("./create_vp");
const { revokeVC } = require("./revocation");
const { merkleKey } = require("./merkle_key");
const { CLIENT_CONFIG } = require("./config");

jest.setTimeout(120000); // 2 minutes to account for spurious network delays, most tests pass in a few seconds

// Run all Node.js examples as jest tests in parallel.
// If a function throws an exception, it will run again to make the tests more consistent (less prone to network issues). 
// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
test.concurrent("Create Identity", async () => {
    try {
        await createIdentity(CLIENT_CONFIG);
    } catch (e) {
        await createIdentity(CLIENT_CONFIG);
    }
});
test.concurrent("Manipulate Identity", async () => {
    try {
        await manipulateIdentity(CLIENT_CONFIG);
    } catch (e) {
        await manipulateIdentity(CLIENT_CONFIG);
    }
});
test.concurrent("Resolution", async () => {
    try {
        await resolution(CLIENT_CONFIG);
    } catch (e) {
        await resolution(CLIENT_CONFIG);
    }
});
test.concurrent("Create Verifiable Credential", async () => {
    try {
        await createVC(CLIENT_CONFIG);
    } catch (e) {
        await createVC(CLIENT_CONFIG);
    }
});
test.concurrent("Create Verifiable Presentation", async () => {
    try {
        await createVP(CLIENT_CONFIG);
    } catch (e) {
        await createVP(CLIENT_CONFIG);
    }
});
test.concurrent("Revoke Verifiable Credential", async () => {
    try {
        await revokeVC(CLIENT_CONFIG);
    } catch (e) {
        await revokeVC(CLIENT_CONFIG);
    }
});
test.concurrent("Merkle Key", async () => {
    try {
        await merkleKey(CLIENT_CONFIG);
    } catch (e) {
        await merkleKey(CLIENT_CONFIG);
    }
});

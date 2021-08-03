const { createIdentity } = require('./create_did');
const { manipulateIdentity } = require('./manipulate_did');
const { resolution } = require('./resolution');
const { createVC } = require('./create_vc');
const { createVP } = require('./create_vp');
const { revokeVC } = require('./revocation');
const { merkleKey } = require('./merkle_key');
const { CLIENT_CONFIG } = require('./config')

jest.setTimeout(120000) // 2 minutes to account for spurious network delays, most tests pass in a few seconds

// Run all Node.js examples as jest tests in parallel.
// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
test.concurrent("Create Identity", async () => {
    await createIdentity(CLIENT_CONFIG);
});
test.concurrent("Manipulate Identity", async () => {
    await manipulateIdentity(CLIENT_CONFIG);
});
test.concurrent("Resolution", async () => {
    await resolution(CLIENT_CONFIG);
});
test.concurrent("Create Verifiable Credential", async () => {
    await createVC(CLIENT_CONFIG);
});
test.concurrent("Create Verifiable Presentation", async () => {
    await createVP(CLIENT_CONFIG);
});
test.concurrent("Revoke Verifiable Credential", async () => {
    await revokeVC(CLIENT_CONFIG);
});
test.concurrent("Merkle Key", async () => {
    await merkleKey(CLIENT_CONFIG);
});

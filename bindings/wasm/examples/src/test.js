import { createIdentity } from "./create_did";
import { manipulateIdentity } from "./manipulate_did";
import { resolution } from "./resolution";
import { createVC } from "./create_vc";
import { createVP } from "./create_vp";
import { createDiff } from "./diff_chain";
import { revokeVC } from "./revoke_vc";
import { merkleKey } from "./merkle_key";
import { resolveHistory } from "./resolve_history";
import { CLIENT_CONFIG } from "./config";
import { createIdentityPrivateTangle } from "./private_tangle";

jest.setTimeout(180000); // 3 minutes to account for spurious network delays, most tests pass in a few seconds

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
test.concurrent("Private Tangle", async () => {
    try {
        await createIdentityPrivateTangle()
        throw new Error("Did not throw.")
    } catch (err) {
        // Example is expected to throw an error because no private Tangle is running
        expect(err.name).toEqual("ClientError")
        expect(err.message).toContain("error sending request")
    }
});
test.concurrent("Diff Chain", async () => {
    try {
        await createDiff(CLIENT_CONFIG);
    } catch (e) {
        await createDiff(CLIENT_CONFIG);
    }
});
test.concurrent("Resolve History", async () => {
    try {
        await resolveHistory(CLIENT_CONFIG);
    } catch (e) {
        await resolveHistory(CLIENT_CONFIG);
    }
});

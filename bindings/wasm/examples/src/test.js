import * as assert from "assert";
import * as parallel from "mocha.parallel"

import {createIdentity} from "./create_did";
import {manipulateIdentity} from "./manipulate_did";
import {resolution} from "./resolution";
import {createVC} from "./create_vc";
import {createVP} from "./create_vp";
import {createDiff} from "./diff_chain";
import {revokeVC} from "./revoke_vc";
import {merkleKey} from "./merkle_key";
import {resolveHistory} from "./resolve_history";
import {CLIENT_CONFIG} from "./config";
import {privateTangle} from "./private_tangle";

const TIMEOUT = 300000; // 5 minutes to account for spurious network delays, most tests pass in a few seconds

// Run all Node.js examples as mocha tests in parallel.
// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
parallel("Test node examples", function () {
    this.timeout(TIMEOUT);
    it("Create Identity", async () => {
        await createIdentity(CLIENT_CONFIG);
    });
    it("Manipulate Identity", async () => {
        await manipulateIdentity(CLIENT_CONFIG);
    });
    it("Resolution", async () => {
        await resolution(CLIENT_CONFIG);
    });
    it("Create Verifiable Credential", async () => {
        await createVC(CLIENT_CONFIG);
    });
    it("Create Verifiable Presentation", async () => {
        await createVP(CLIENT_CONFIG);
    });
    it("Revoke Verifiable Credential", async () => {
        await revokeVC(CLIENT_CONFIG);
    });
    it("Merkle Key", async () => {
        await merkleKey(CLIENT_CONFIG);
    });
    it("Private Tangle", async () => {
        try {
            await privateTangle()
            throw new Error("Did not throw.")
        } catch (err) {
            // Example is expected to throw an error because no private Tangle is running
            assert.strictEqual(err.name, "ClientError")
            assert.strictEqual(err.message.includes("error sending request"), true)
        }
    });
    it("Diff Chain", async () => {
        await createDiff(CLIENT_CONFIG);
    });
    it("Resolve History", async () => {
        await resolveHistory(CLIENT_CONFIG);
    });
})

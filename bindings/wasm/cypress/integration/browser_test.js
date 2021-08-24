import { defaultClientConfig, initIdentity } from "../../examples/browser/utils";
import { createIdentity } from "../../examples/browser/create_did.js";
import { createVC } from "../../examples/browser/create_vc.js";
import { manipulateIdentity } from "../../examples/browser/mainpulate_did.js";
import { resolveIdentity } from "../../examples/browser/resolve.js";
import { createVP } from "../../examples/browser/create_vp.js";
import { createDiff } from "../../examples/browser/diff_chain.js";
import { revoke } from "../../examples/browser/revoke_vc.js";
import { merkleKey } from "../../examples/browser/merkle_key.js";
import { createIdentityPrivateTangle } from "../../examples/browser/private_tangle";
import { resolveHistory } from "../../examples/browser/resolve_history";

// Test that the browser examples do not throw uncaught exceptions twice, including syntax errors etc.
describe(
    "Test browser examples",
    {
        defaultCommandTimeout: 180000, // 3 minutes to account for spurious network delays
    },
    () => {
        beforeEach(async () => {
            // The working directory is under __cypress at test runtime, so we need to go up one more level than usual
            await initIdentity("../../../web/identity_wasm_bg.wasm", false);

            // NOTE: `cy.wrap(defaultClientConfig()).as('config')` does not always work to make the config available
            // from the shared context as `this.config` because it has a race condition with initializing the wasm.
            // So call `defaultClientConfig()` manually for now.
        });

        it("create identity", async function () {
            let identityResult;
            try {
                identityResult = await createIdentity(defaultClientConfig(), false);
            } catch (e) {
                identityResult = await createIdentity(defaultClientConfig(), false);
            }
            // example of testing the output, can remove if needed
            expect(identityResult).to.have.all.keys("key", "doc", "receipt", "explorerUrl");
        });

        it("manipulate identity", async function () {
            try {
                await manipulateIdentity(defaultClientConfig(), false);
            } catch (e) {
                await manipulateIdentity(defaultClientConfig(), false);
            }
        });

        it("resolve identity", async function () {
            try {
                await resolveIdentity(defaultClientConfig(), false, false);
            } catch (e) {
                await resolveIdentity(defaultClientConfig(), false, false);
            }
        });

        it("create verifiable credential", async function () {
            try {
                await createVC(defaultClientConfig(), false);
            } catch (e) {
                await createVC(defaultClientConfig(), false);
            }
        });

        it("revoke verifiable credential", async function () {
            try {
                await revoke(defaultClientConfig(), false);
            } catch (e) {
                await revoke(defaultClientConfig(), false);
            }
        });

        it("create verifiable presentation", async function () {
            try {
                await createVP(defaultClientConfig(), false);
            } catch (e) {
                await createVP(defaultClientConfig(), false);
            }
        });
        
        it("merkle key", async function () {
            try {
                await merkleKey(defaultClientConfig(), false);
            } catch (e) {
                await merkleKey(defaultClientConfig(), false);
            }
        });

        it("private tangle", async function () {
            try {
                await createIdentityPrivateTangle(false, false)
                throw new Error("Did not throw.")
            } catch (err) {
                // Example is expected to throw an error because no private Tangle is running
                expect(err.name).to.eq("ClientError")
                expect(err.message).to.contain("error sending request")
            }
        });

        it("diff chain", async function () {
            try {
                await createDiff(defaultClientConfig(), false);
            } catch (e) {
                await createDiff(defaultClientConfig(), false);
            }
        });

        it("resolve history", async function () {
            try {
                await resolveHistory(defaultClientConfig(), false);
            } catch (e) {
                await resolveHistory(defaultClientConfig(), false);
            }
        });
    }
);

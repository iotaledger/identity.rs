import { defaultClientConfig, initIdentity } from "../../examples/browser/utils";
import { createIdentity } from "../../examples/browser/create_did.js";
import { createVC } from "../../examples/browser/create_vc.js";
import { manipulateIdentity } from "../../examples/browser/mainpulate_did.js";
import { resolveIdentity } from "../../examples/browser/resolve.js";
import { createVP } from "../../examples/browser/create_vp.js";
import { revoke } from "../../examples/browser/revocation.js";
import { merkleKey } from "../../examples/browser/merkle_key.js";

// Test that the browser examples do not throw uncaught exceptions twice, including syntax errors etc.
describe(
    "Test browser examples",
    {
        defaultCommandTimeout: 120000, // 2 minutes to account for spurious network delays
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
    }
);

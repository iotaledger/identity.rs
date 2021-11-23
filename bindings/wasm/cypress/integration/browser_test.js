import {
    defaultClientConfig, initIdentity, createIdentity, createVC, manipulateIdentity, resolution, createVP, createDiff, revokeVC, merkleKey, createIdentityPrivateTangle, resolveHistory
} from '../../examples/dist/web'

// Test that the browser examples do not throw uncaught exceptions twice, including syntax errors etc.
describe(
    "Test browser examples",
    {
        defaultCommandTimeout: 300000, // 5 minutes to account for spurious network delays
    },
    () => {
        beforeEach(async () => {
            // The working directory is under __cypress at test runtime, so we need to go up one more level than usual
            await initIdentity('../../../examples/dist/identity_wasm_bg.wasm');

            // NOTE: `cy.wrap(defaultClientConfig()).as('config')` does not always work to make the config available
            // from the shared context as `this.config` because it has a race condition with initializing the wasm.
            // So call `defaultClientConfig()` manually for now.
        });

        it("create identity", async function () {
            let identityResult;
            try {
                identityResult = await createIdentity(defaultClientConfig());
            } catch (e) {
                identityResult = await createIdentity(defaultClientConfig());
            }
            // example of testing the output, can remove if needed
            expect(identityResult).to.have.all.keys("key", "doc", "receipt");
        });

        it("manipulate identity", async function () {
            try {
                await manipulateIdentity(defaultClientConfig());
            } catch (e) {
                await manipulateIdentity(defaultClientConfig());
            }
        });

        it("resolve identity", async function () {
            try {
                await resolution(defaultClientConfig());
            } catch (e) {
                await resolution(defaultClientConfig());
            }
        });

        it("create verifiable credential", async function () {
            try {
                await createVC(defaultClientConfig());
            } catch (e) {
                await createVC(defaultClientConfig());
            }
        });

        it("revoke verifiable credential", async function () {
            try {
                await revokeVC(defaultClientConfig());
            } catch (e) {
                await revokeVC(defaultClientConfig());
            }
        });

        it("create verifiable presentation", async function () {
            try {
                await createVP(defaultClientConfig());
            } catch (e) {
                await createVP(defaultClientConfig());
            }
        });

        it("merkle key", async function () {
            try {
                await merkleKey(defaultClientConfig());
            } catch (e) {
                await merkleKey(defaultClientConfig());
            }
        });

        it("private tangle", async function () {
            try {
                await createIdentityPrivateTangle()
                throw new Error("Did not throw.")
            } catch (err) {
                // Example is expected to throw an error because no private Tangle is running
                expect(err.name).to.eq("ClientError")
                expect(err.message).to.contain("error sending request")
            }
        });

        it("diff chain", async function () {
            try {
                await createDiff(defaultClientConfig());
            } catch (e) {
                await createDiff(defaultClientConfig());
            }
        });

        it("resolve history", async function () {
            try {
                await resolveHistory(defaultClientConfig());
            } catch (e) {
                await resolveHistory(defaultClientConfig());
            }
        });
    }
);

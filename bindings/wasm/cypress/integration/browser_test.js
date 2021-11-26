import {
    createDiff,
    createIdentity,
    createVC,
    createVP,
    defaultClientConfig,
    initIdentity,
    manipulateIdentity,
    merkleKey,
    privateTangle,
    repeatAsyncTest,
    resolution,
    resolveHistory,
    revokeVC
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
        it("Create Identity", async () => {
            await repeatAsyncTest(createIdentity, defaultClientConfig());
        });
        it("Manipulate Identity", async () => {
            await repeatAsyncTest(manipulateIdentity, defaultClientConfig());
        });
        it("Resolution", async () => {
            await repeatAsyncTest(resolution, defaultClientConfig());
        });
        it("Create Verifiable Credential", async () => {
            await repeatAsyncTest(createVC, defaultClientConfig());
        });
        it("Create Verifiable Presentation", async () => {
            await repeatAsyncTest(createVP, defaultClientConfig());
        });
        it("Revoke Verifiable Credential", async () => {
            await repeatAsyncTest(revokeVC, defaultClientConfig());
        });
        it("Merkle Key", async () => {
            await repeatAsyncTest(merkleKey, defaultClientConfig());
        });
        it("private tangle", async function () {
            try {
                await privateTangle()
                throw new Error("Did not throw.")
            } catch (err) {
                // Example is expected to throw an error because no private Tangle is running
                expect(err.name).to.eq("ClientError")
                expect(err.message).to.contain("error sending request")
            }
        });
        it("Diff Chain", async () => {
            await repeatAsyncTest(createDiff, defaultClientConfig());
        });
        it("Resolve History", async () => {
            await repeatAsyncTest(resolveHistory, defaultClientConfig());
        });
    }
);

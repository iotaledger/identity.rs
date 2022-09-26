import {
    initIdentity,
    privateTangle,
} from '../../examples/dist/web'

describe(
    "privateTangle",
    {
        defaultCommandTimeout: 1000*60*3, // 3 minutes to account for spurious network delays
    },
    () => {
        before(async () => {
            // The working directory is under __cypress at test runtime, so we need to go up one more level than usual
            await initIdentity('../../../examples/dist/identity_wasm_bg.wasm');

            // NOTE: `cy.wrap(defaultClientConfig()).as('config')` does not always work to make the config available
            // from the shared context as `this.config` because it has a race condition with initializing the wasm.
            // So call `defaultClientConfig()` manually for now.
        });
        it("Private Tangle", async function () {
            try {
                await privateTangle("http://127.0.0.1:1111/")
                throw new Error("Did not throw.")
            } catch (err) {
                // Example is expected to throw an error because no private Tangle is running
                expect(err.name).to.eq("ClientError")
                expect(err.message).to.contain("error sending request")
            }
        });
    }
);

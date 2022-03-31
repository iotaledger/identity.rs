import {
    defaultClientConfig,
    initIdentity,
    resolution,
} from '../../examples/dist/web'

describe(
    "resolution",
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
        it("Resolution", async () => {
            await resolution(defaultClientConfig());
        });
    }
);

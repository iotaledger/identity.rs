import { init, AccountBuilder, ExplorerUrl, Storage } from '../../../web';

describe(
    "multipleIdentities",
    {
        defaultCommandTimeout: 1000 * 60 * 3, // 3 minutes to account for spurious network delays
    },
    () => {
        before(async () => {
            // The working directory is under __cypress at test runtime, so we need to go up one more level than usual
            await init('../../../web/identity_wasm_bg.wasm');

            // NOTE: `cy.wrap(defaultClientConfig()).as('config')` does not always work to make the config available
            // from the shared context as `this.config` because it has a race condition with initializing the wasm.
            // So call `defaultClientConfig()` manually for now.
        });
        it("Multiple Identities", async () => {

            // Create an AccountBuilder to make it easier to create multiple identities.
            // Every account created from the builder will use the same storage.
            const builder = new AccountBuilder({});

            // The creation step generates a keypair, builds an identity
            // and publishes it to the IOTA mainnet.
            const account1 = await builder.createIdentity();

            // Create a second identity.
            const account2 = await builder.createIdentity();

            // Retrieve the did of the identity that account1 manages.
            const did1 = account1.did();

            // Suppose we're done with account1 and free it.
            account1.free();

            // Now we want to modify the first identity - how do we do that?
            // We can load the identity from storage into an account using the builder.
            const account1Reconstructed = await builder.loadIdentity(did1);

            // Now we can make modifications to the identity.
            // We can even do so concurrently.
            const account1Promise = account1Reconstructed.createMethod({
                fragment: "my_key"
            })
            const account2Promise = account2.createMethod({
                fragment: "my_other_key"
            })

            await Promise.all([account1Promise, account2Promise]);

            // Print the Explorer URL for the DID.
            const did = account1Reconstructed.did().toString();
            console.log(`Explorer Url:`, ExplorerUrl.mainnet().resolverUrl(did));



        });
    }
);

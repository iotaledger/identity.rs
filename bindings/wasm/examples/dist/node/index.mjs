import { AccountBuilder, ExplorerUrl } from '@iota/identity-wasm/node/identity_wasm.js';
import { Stronghold } from '@iota/identity-stronghold-nodejs'

const strongholdPath = "./example-strong.hodl";
const password = "my-password";
const stronghold = await Stronghold.build(strongholdPath, password, true);

// The creation step generates a keypair, builds an identity
// and publishes it to the IOTA mainnet.
const builder = new AccountBuilder({
    storage: stronghold,
});
const account = await builder.createIdentity();

// Retrieve the DID of the newly created identity.
const did = account.did();

// Print the DID of the created Identity.
console.log(did.toString())

// Print the local state of the DID Document
console.log(account.document());

// Print the Explorer URL for the DID.
console.log(`Explorer Url:`, ExplorerUrl.mainnet().resolverUrl(did));
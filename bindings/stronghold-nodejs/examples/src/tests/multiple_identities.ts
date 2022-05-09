import {multipleIdentities} from "../../../../wasm/examples-account/src/multiple_identities";
import { stronghold } from '../stronghold';

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test Stronghold Node.js examples", function () {
    it("Multiple Identities", async () => {
        // TODO: Temporarily disabled until iotaledger/stronghold.rs#353 is fixed.
        // await multipleIdentities(await stronghold());
    });
})

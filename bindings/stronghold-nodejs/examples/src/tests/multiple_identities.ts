import {multipleIdentities} from "../../../../wasm/examples-account/src/multiple_identities";
import { stronghold } from '../stronghold';

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Create Verifiable Credential", async () => {
        await multipleIdentities(stronghold);
    });
})

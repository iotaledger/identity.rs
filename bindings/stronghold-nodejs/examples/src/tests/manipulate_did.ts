import {manipulateIdentity} from "../../../../wasm/examples-account/src/manipulate_did";
import { stronghold } from '../stronghold';

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test Stronghold Node.js examples", function () {
    it("Manipulate DID", async () => {
        await manipulateIdentity(await stronghold());
    });
})

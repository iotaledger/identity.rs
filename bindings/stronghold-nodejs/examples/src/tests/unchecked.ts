import {unchecked} from "../../../../wasm/examples-account/src/advanced/unchecked";
import { stronghold } from '../stronghold';

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test Stronghold Node.js examples", function () {
    it("Unchecked", async () => {
        await unchecked(await stronghold());
    });
})

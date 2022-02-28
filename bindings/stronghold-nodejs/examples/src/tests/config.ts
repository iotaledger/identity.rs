import {config} from "../../../../wasm/examples-account/src/config";
import { stronghold } from '../stronghold';

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Config", async () => {
        await config(stronghold);
    });
})

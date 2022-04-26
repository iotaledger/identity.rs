import {config} from "../../../../wasm/examples/node/basic/8_config";
import { stronghold } from '../stronghold';

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test Stronghold Node.js examples", function () {
    it("Config", async () => {
        await config(await stronghold());
    });
})

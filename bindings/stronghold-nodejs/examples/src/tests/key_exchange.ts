import { keyExchange } from "../../../../wasm/examples-account/src/key_exchange";
import { stronghold } from '../stronghold';

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test Stronghold Node.js examples", function () {
    it("Key Exchange", async () => {
        await keyExchange(await stronghold());
    });
})

import { encryption } from "../../../../wasm/examples-account/src/encryption";
import { stronghold } from "../stronghold";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test Stronghold Node.js examples", function() {
    it("encryption", async () => {
        await encryption(await stronghold());
    });
});

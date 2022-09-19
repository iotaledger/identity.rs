import { signing } from "../../../../wasm/examples-account/src/signing";
import { stronghold } from "../stronghold";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test Stronghold Node.js examples", function() {
    it("Signing", async () => {
        await signing(await stronghold());
    });
});

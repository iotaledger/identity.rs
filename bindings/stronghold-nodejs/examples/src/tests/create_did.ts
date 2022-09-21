import { createIdentity } from "../../../../wasm/examples-account/src/create_did";
import { stronghold } from "../stronghold";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test Stronghold Node.js examples", function() {
    it("Create Identity", async () => {
        await createIdentity(await stronghold());
    });
});

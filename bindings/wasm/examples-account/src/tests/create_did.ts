import { createIdentity } from "../create_did";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function() {
    it("Create Identity", async () => {
        await createIdentity();
    });
});

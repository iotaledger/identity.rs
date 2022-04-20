import { revokeVC } from "../revoke_vc";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Revoke verifiable credential", async () => {
        await revokeVC();
    });
})


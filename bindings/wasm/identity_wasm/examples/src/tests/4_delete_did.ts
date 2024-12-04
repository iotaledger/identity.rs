import { deleteIdentity } from "../0_basic/4_delete_did";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function() {
    it("Delete Identity", async () => {
        await deleteIdentity();
    });
});

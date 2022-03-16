import { multipleIdentities } from "../multiple_identities";
import { MemStore } from "../advanced/memory_storage";


// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("MemStore works with Multiple Identities", async () => {
        await multipleIdentities(new MemStore());
    });
})

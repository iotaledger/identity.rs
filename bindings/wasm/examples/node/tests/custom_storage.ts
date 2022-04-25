import {MemStore} from "../advanced/4_custom_storage";
import {multipleIdentities} from "../basic/6_multiple_identities";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Custom Storage", async () => {
        await multipleIdentities(new MemStore());
    });
})

import {MemStore} from "../advanced/8_custom_storage";
import {multipleIdentities} from "../advanced/5_multiple_identities";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Custom Storage", async () => {
        await multipleIdentities(new MemStore());
    });
})

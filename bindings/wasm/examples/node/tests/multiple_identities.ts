import {multipleIdentities} from "../advanced/5_multiple_identities";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Multiple Identities", async () => {
        await multipleIdentities();
    });
})

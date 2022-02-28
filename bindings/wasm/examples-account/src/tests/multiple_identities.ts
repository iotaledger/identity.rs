import {multipleIdentities} from "../multiple_identities";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Create Verifiable Credential", async () => {
        await multipleIdentities();
    });
})

import {manipulateIdentity} from "../basic/2_manipulate_did";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Manipulate DID", async () => {
        await manipulateIdentity();
    });
})

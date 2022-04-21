import {createVC} from "../basic/3_create_vc";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Create verifiable credential", async () => {
        await createVC();
    });
})


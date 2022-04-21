import { createVP } from "../basic/4_create_vp";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Create verifiable presentation", async () => {
        await createVP();
    });
})


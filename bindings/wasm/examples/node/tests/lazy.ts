import {lazy} from "../basic/9_lazy";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Lazy", async () => {
        await lazy();
    });
})

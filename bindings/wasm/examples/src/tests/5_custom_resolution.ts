import { customResolution } from "../1_advanced/5_custom_resolution";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function() {
    it("Key exchange", async () => {
        await customResolution();
    });
});

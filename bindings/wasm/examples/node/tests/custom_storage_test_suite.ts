import {storageTestSuite} from "../advanced/8_custom_storage";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Custom Storage Test Suite", async () => {
        await storageTestSuite();
    });
})

import {config} from "../config";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Config", async () => {
        await config();
    });
})

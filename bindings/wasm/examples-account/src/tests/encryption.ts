import {encryption} from "../encryption";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("encryption", async () => {
        await encryption();
    });
})

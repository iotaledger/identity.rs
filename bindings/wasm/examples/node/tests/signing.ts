import {signing} from "../basic/7_signing";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Signing", async () => {
        await signing();
    });
})

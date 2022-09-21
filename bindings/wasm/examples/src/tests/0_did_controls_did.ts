import { didControlsDid } from "../1_advanced/0_did_controls_did";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function() {
    it("Did controls Did", async () => {
        await didControlsDid();
    });
});

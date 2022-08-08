import {run} from "../stardust_ex0_create_did";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Stardust Create DID", async () => {
        await run();
    });
})

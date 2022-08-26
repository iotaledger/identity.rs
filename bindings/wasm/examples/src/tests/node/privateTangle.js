import * as assert from "assert";
import {privateTangle} from "../../private_tangle";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Private Tangle", async () => {
        try {
            await privateTangle("http://127.0.0.1:1111/")
            throw new Error("Did not throw.")
        } catch (err) {
            // Example is expected to throw an error because no private Tangle is running
            assert.strictEqual(err.name, "ClientError")
            assert.strictEqual(err.message.includes("error sending request"), true)
        }
    });
})

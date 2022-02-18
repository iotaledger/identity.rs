import * as assert from "assert";

import {privateTangle} from "../../private_tangle";

const TIMEOUT = 1000*60*3; // 3 minutes to account for spurious network delays

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    this.timeout(TIMEOUT);
    it("Private Tangle", async () => {
        try {
            await privateTangle()
            throw new Error("Did not throw.")
        } catch (err) {
            // Example is expected to throw an error because no private Tangle is running
            assert.strictEqual(err.name, "ClientError")
            assert.strictEqual(err.message.includes("error sending request"), true)
        }
    });
})

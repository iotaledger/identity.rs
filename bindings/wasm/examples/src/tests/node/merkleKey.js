import {merkleKey} from "../../merkle_key";
import {CLIENT_CONFIG} from "../../config";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Merkle Key", async () => {
        await merkleKey(CLIENT_CONFIG);
    });
})

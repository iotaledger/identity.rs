import {createDiff} from "../../diff_chain";
import {CLIENT_CONFIG} from "../../config";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Diff Chain", async () => {
        await createDiff(CLIENT_CONFIG);
    });
})

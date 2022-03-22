import {resolveHistory} from "../../resolve_history";
import {CLIENT_CONFIG} from "../../config";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Resolve History", async () => {
        await resolveHistory(CLIENT_CONFIG);
    });
})

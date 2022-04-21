import {resolveHistory} from "../advanced/6_resolve_history";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Resolve History", async () => {
        await resolveHistory();
    });
})

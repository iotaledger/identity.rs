import {resolveIdentity} from "../../resolve_did";
import {CLIENT_CONFIG} from "../../config";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Resolve Identity", async () => {
        await resolveIdentity(CLIENT_CONFIG);
    });
})

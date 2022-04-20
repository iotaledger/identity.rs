import {createIdentity} from "../../create_did";
import {CLIENT_CONFIG} from "../../config";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Create Identity", async () => {
        await createIdentity(CLIENT_CONFIG);
    });
})

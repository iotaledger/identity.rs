import {revokeVC} from "../../revoke_vc";
import {CLIENT_CONFIG} from "../../config";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Revoke Verifiable Credential", async () => {
        await revokeVC(CLIENT_CONFIG);
    });
})

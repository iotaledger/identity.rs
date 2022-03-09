import {createVC} from "../../create_vc";
import {CLIENT_CONFIG} from "../../config";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Create Verifiable Credential", async () => {
        await createVC(CLIENT_CONFIG);
    });
})

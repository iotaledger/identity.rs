import {createVP} from "../../create_vp";
import {CLIENT_CONFIG} from "../../config";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Create Verifiable Presentation", async () => {
        await createVP(CLIENT_CONFIG);
    });
})

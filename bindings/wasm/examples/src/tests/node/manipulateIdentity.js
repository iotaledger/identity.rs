import {manipulateIdentity} from "../../manipulate_did";
import {CLIENT_CONFIG} from "../../config";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Manipulate Identity", async () => {
        await manipulateIdentity(CLIENT_CONFIG);
    });
})

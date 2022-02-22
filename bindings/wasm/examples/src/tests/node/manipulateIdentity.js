import {manipulateIdentity} from "../../manipulate_did";
import {CLIENT_CONFIG} from "../../config";

const TIMEOUT = 1000*60*3; // 3 minutes to account for spurious network delays

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    this.timeout(TIMEOUT);
    it("Manipulate Identity", async () => {
        await manipulateIdentity(CLIENT_CONFIG);
    });
})

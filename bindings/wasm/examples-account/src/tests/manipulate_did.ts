import {manipulateIdentity} from "../manipulate_did";

const TIMEOUT = 1000*60*3; // 3 minutes to account for spurious network delays

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    this.timeout(TIMEOUT);
    it("Create Verifiable Credential", async () => {
        await manipulateIdentity();
    });
})

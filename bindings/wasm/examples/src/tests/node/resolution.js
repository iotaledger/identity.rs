import {resolution} from "../../resolution";
import {CLIENT_CONFIG} from "../../config";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Resolution", async () => {
        await resolution(CLIENT_CONFIG);
    });
})

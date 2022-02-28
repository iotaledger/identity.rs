import {lazy} from "../../../../wasm/examples-account/src/lazy";
import { stronghold } from '../stronghold';

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Lazy", async () => {
        await lazy(stronghold);
    });
})

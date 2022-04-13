import {keyExchange} from "../key_exchange";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function () {
    it("Key Exchange", async () => {
        await keyExchange();
    });
})

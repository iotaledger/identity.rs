import { keyExchange } from "../1_advanced/4_key_exchange";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function() {
    it("Key exchange", async () => {
        await keyExchange();
    });
});

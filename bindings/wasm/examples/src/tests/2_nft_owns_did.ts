import { nftOwnsDid } from "../1_advanced/2_nft_owns_did";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function() {
    it("Nft owns Did", async () => {
        await nftOwnsDid();
    });
});

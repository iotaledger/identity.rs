import { didIssuesNft } from "../1_advanced/1_did_issues_nft";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function() {
    it("Did issues Nft", async () => {
        await didIssuesNft();
    });
});

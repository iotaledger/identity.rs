import { didIssuesTokens } from "../1_advanced/3_did_issues_tokens";

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test node examples", function() {
    it("Did issues tokens", async () => {
        await didIssuesTokens();
    });
});

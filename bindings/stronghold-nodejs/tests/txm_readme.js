const { execSync } = require("child_process");

describe("Test TXM", function() {
    it("README examples pass", async () => {
        execSync("txm README.md");
    });
});

import { setup } from '../../support/setup';
import { didIssuesTokens } from "../../../examples/dist/web/1_advanced/3_did_issues_tokens";

describe(
  "didIssuesTokens",
  () => {
    it("DID Issues Tokens", async () => {
      await setup(didIssuesTokens)
    });
  }
);

import { didIssuesTokens } from "../../../examples/dist/web/1_advanced/3_did_issues_tokens";
import { setup } from "../../support/setup";

describe(
    "didIssuesTokens",
    () => {
        it("DID Issues Tokens", async () => {
            await setup(didIssuesTokens);
        });
    },
);

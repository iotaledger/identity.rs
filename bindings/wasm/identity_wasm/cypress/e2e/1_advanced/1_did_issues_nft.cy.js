import { didIssuesNft } from "../../../examples/dist/web/1_advanced/1_did_issues_nft";
import { setup } from "../../support/setup";

describe(
    "didIssuesNft",
    () => {
        it("DID Issues NFT", async () => {
            await setup(didIssuesNft);
        });
    },
);

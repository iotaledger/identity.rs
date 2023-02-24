import { setup } from '../../support/setup';
import { didIssuesNft } from "../../../examples/dist/web/1_advanced/1_did_issues_nft";

describe(
  "didIssuesNft",
  () => {
    it("DID Issues NFT", async () => {
      await setup(didIssuesNft)
    });
  }
);

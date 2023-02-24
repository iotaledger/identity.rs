import { setup } from '../../support/setup';
import { nftOwnsDid } from "../../../examples/dist/web/1_advanced/2_nft_owns_did";

describe(
  "nftOwnsDid",
  () => {
    it("NFT owns DID", async () => {
      await setup(nftOwnsDid)
    });
  }
);

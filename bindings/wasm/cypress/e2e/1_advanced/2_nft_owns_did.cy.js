import { nftOwnsDid } from "../../../examples/dist/web/1_advanced/2_nft_owns_did";
import { setup } from "../../support/setup";

describe(
    "nftOwnsDid",
    () => {
        it("NFT owns DID", async () => {
            await setup(nftOwnsDid);
        });
    },
);

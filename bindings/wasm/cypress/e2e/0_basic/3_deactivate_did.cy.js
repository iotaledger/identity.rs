import { setup } from '../../support/setup';
import { deactivateIdentity } from "../../../examples/dist/web/0_basic/3_deactivate_did";

describe(
  "deactivateIdentity",
  () => {
    it("Deactivate Identity", async () => {
      await setup(deactivateIdentity)
    });
  }
);

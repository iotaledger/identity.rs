import { setup } from '../../support/setup';
import { updateIdentity } from "../../../examples/dist/web/0_basic/1_update_did";

describe(
  "updateIdentity",
  () => {
    it("Update Identity", async () => {
      await setup(updateIdentity)
    });
  }
);

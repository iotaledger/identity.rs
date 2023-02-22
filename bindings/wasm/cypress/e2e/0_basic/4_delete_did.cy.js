import { setup } from '../../support/setup';
import { deleteIdentity } from "../../../examples/dist/web/0_basic/4_delete_did";

describe(
  "deleteIdentity",
  () => {
    it("Delete Identity", async () => {
      await setup(deleteIdentity)
    });
  }
);

import { setup } from '../../support/setup';
import { createIdentity } from "../../../examples/dist/web/0_basic/0_create_did";

describe(
  "createIdentity",
  () => {
    it("Create Identity", async () => {
      await setup(createIdentity);
    });
  }
);

import { setup } from '../../support/setup';
import { createVC } from "../../../examples/dist/web/0_basic/5_create_vc";

describe(
  "createVC",
  () => {
    it("Create Credential", async () => {
      await setup(createVC)
    });
  }
);

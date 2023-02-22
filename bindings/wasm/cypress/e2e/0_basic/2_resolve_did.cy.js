import { setup } from '../../support/setup';
import { resolveIdentity } from "../../../examples/dist/web/0_basic/2_resolve_did";

describe(
  "resolveIdentity",
  () => {
    it("Resolve Identity", async () => {
      await setup(resolveIdentity)
    });
  }
);

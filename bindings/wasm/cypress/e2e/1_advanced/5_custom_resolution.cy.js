import { setup } from '../../support/setup';
import { customResolution } from "../../../examples/dist/web/1_advanced/5_custom_resolution";

describe(
  "customResolution",
  () => {
    it("Custom Resolution", async () => {
      await setup(customResolution)
    });
  }
);

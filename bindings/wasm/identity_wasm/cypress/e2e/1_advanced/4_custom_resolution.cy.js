import { customResolution } from "../../../examples/dist/web/1_advanced/4_custom_resolution";
import { setup } from "../../support/setup";

describe(
    "customResolution",
    () => {
        it("Custom Resolution", async () => {
            await setup(customResolution);
        });
    },
);

import { updateIdentity } from "../../../examples/dist/web/0_basic/1_update_did";
import { setup } from "../../support/setup";

describe(
    "updateIdentity",
    () => {
        it("Update Identity", async () => {
            await setup(updateIdentity);
        });
    },
);

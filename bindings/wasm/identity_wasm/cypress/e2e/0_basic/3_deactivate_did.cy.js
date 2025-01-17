import { deactivateIdentity } from "../../../examples/dist/web/0_basic/3_deactivate_did";
import { setup } from "../../support/setup";

describe(
    "deactivateIdentity",
    () => {
        it("Deactivate Identity", async () => {
            await setup(deactivateIdentity);
        });
    },
);

import { createVC } from "../../../examples/dist/web/0_basic/5_create_vc";
import { setup } from "../../support/setup";

describe(
    "createVC",
    () => {
        it("Create Credential", async () => {
            await setup(createVC);
        });
    },
);

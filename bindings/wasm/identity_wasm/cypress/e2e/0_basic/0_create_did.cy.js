import { createIdentity } from "../../../examples/dist/web/0_basic/0_create_did";
import { setup } from "../../support/setup";

describe(
    "createIdentity",
    () => {
        it("Create Identity", async () => {
            await setup(createIdentity);
        });
    },
);

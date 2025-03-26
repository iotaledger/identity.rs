import { createVP } from "../../../examples/dist/web/0_basic/6_create_vp";
import { setup } from "../../support/setup";

describe(
    "createVP",
    () => {
        it("Create Presentation", async () => {
            await setup(createVP);
        });
    },
);

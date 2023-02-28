import { didControlsDid } from "../../../examples/dist/web/1_advanced/0_did_controls_did";
import { setup } from "../../support/setup";

describe(
    "didControlsDid",
    () => {
        it("DID Controls DID", async () => {
            await setup(didControlsDid);
        });
    },
);

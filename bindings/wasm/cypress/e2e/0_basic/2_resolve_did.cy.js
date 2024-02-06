import { resolveIdentity } from "../../../examples/dist/web/0_basic/2_resolve_did";
import { setup } from "../../support/setup";

describe(
    "resolveIdentity",
    () => {
        it("Resolve Identity", async () => {
            await setup(resolveIdentity);
        });
    },
);

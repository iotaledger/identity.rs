import { deleteIdentity } from "../../../examples/dist/web/0_basic/4_delete_did";
import { setup } from "../../support/setup";

describe(
    "deleteIdentity",
    () => {
        it("Delete Identity", async () => {
            await setup(deleteIdentity);
        });
    },
);

import { statusList2021 } from "../../../examples/dist/web/1_advanced/7_status_list_2021";
import { setup } from "../../support/setup";

describe(
    "statusList2021",
    () => {
        it("Status List 2021", async () => {
            await setup(statusList2021);
        });
    },
);

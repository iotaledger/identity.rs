import { sdJwt } from "../../../examples/dist/web/1_advanced/6_sd_jwt";
import { setup } from "../../support/setup";

describe(
    "domainLinkage",
    () => {
        it("Domain Linkage", async () => {
            await setup(sdJwt);
        });
    },
);

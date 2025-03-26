import { domainLinkage } from "../../../examples/dist/web/1_advanced/5_domain_linkage";
import { setup } from "../../support/setup";

describe(
    "domainLinkage",
    () => {
        it("Domain Linkage", async () => {
            await setup(domainLinkage);
        });
    },
);

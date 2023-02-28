import { keyExchange } from "../../../examples/dist/web/1_advanced/4_key_exchange";
import { setup } from "../../support/setup";

describe(
    "keyExchange",
    () => {
        it("Key Exchange", async () => {
            await setup(keyExchange);
        });
    },
);

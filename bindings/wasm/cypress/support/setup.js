import * as client from "@iota/client-wasm/web";
import * as identity from "../../web";

export const setup = async (func) => {
    await client
        .init("../../../node_modules/@iota/client-wasm/web/wasm/client_wasm_bg.wasm")
        .then(async () => await identity.init("../../../web/identity_wasm_bg.wasm"))
        .then(func);
};

import init from "@iota/sdk-wasm/web";
import * as identity from "../../web";

export const setup = async (func) => {
    await init("../../../node_modules/@iota/sdk-wasm/web/wasm/iota_sdk_wasm_bg.wasm")
        .then(async () => await identity.init("../../../web/identity_wasm_bg.wasm"))
        .then(func);
};

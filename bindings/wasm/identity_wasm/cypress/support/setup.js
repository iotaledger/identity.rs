import * as identity from "../../web";

export const setup = async (func) => {
    await identity.init("../../../web/identity_wasm_bg.wasm")
        .then(func);
};

import * as id from "../../web/identity_wasm.js";
import { logToScreen } from "./utils.js";

logToScreen("Initialization started...");

id.init("../../web/identity_wasm_bg.wasm")
  .then(() => {
    logToScreen("Initialization success!");
  })
  .catch((err) => {
    logToScreen(err)
  });

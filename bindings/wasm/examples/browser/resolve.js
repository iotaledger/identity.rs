import { logToScreen } from "./utils.js";
import * as id from "../../web/identity_wasm.js";

export async function resolveIdentity() {
  logToScreen("Identity resolution started...");

  const mainNet = id.Network.mainnet();

  const config = id.Config.fromNetwork(mainNet);

  const client = id.Client.fromConfig(config);

  const inputId = document.querySelector("#resolve-did-input").value;

  try{
    const res = await client.resolve(inputId)
    logToScreen("<pre>" + JSON.stringify(res, null, 4) + "</pre>") ;
    logToScreen("Identity resolution done!")
  }
  catch(err) {
    logToScreen(err)
  }
}

//run the createIdentity function on button click
document
  .querySelector("#resolve-did-btn")
  .addEventListener("click", resolveIdentity);

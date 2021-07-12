import { logObjectToScreen, logToScreen } from "../utils.js";
import {resolveIdentity} from "../resolve.js"

export async function handleResolveDid() {
    logToScreen("Identity resolution started...");

    try {
        //get the DID string from the input field
        const inputDid = document.querySelector("#resolve-did-input").value;

        //resolve the DID
        const res = await resolveIdentity(inputDid);

        //log the result
        logObjectToScreen(res);
        logToScreen("Identity resolution done!");
    } catch (err) {
        logToScreen(err);
    }
}

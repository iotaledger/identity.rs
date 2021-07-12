import { createIdentity } from "../create_did.js";
import {
    logExplorerUrlToScreen,
    logObjectToScreen,
    logToScreen,
} from "../utils.js";

export async function handleCreateIdentity() {
    logToScreen("Identity creation started...");
    logToScreen("This might take a few seconds to complete proof of work!");

    const res = await createIdentity();

    logToScreen("Identity creation done!");
    logObjectToScreen(res.doc);
    logExplorerUrlToScreen(res.explorerUrl);
}

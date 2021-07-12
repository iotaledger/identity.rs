import { manipulateIdentity } from "../mainpulate_did.js";
import {
    logExplorerUrlToScreen,
    logObjectToScreen,
    logToScreen,
} from "../utils.js";

export async function handleManipulateDid() {
    try {
        logToScreen(
            "Identity creation and mainpulation started... <br/> This might take a few seconds to complete proof of work!"
        );

        const res = await manipulateIdentity();

        logObjectToScreen(res.doc);
        logExplorerUrlToScreen(res.explorerUrl);
    } catch (err) {
        logToScreen(err);
    }
}

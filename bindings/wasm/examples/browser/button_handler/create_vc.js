import { createVC } from "../create_vc.js";
import {
    logExplorerUrlToScreen,
    logObjectToScreen,
    logToScreen,
} from "../utils.js";

export async function handleCreateVC() {
    try{
        logToScreen("Verifiable Credential creation started... <br/> This might take a few seconds to complete proof of work!");

        const res = await createVC();
    
        logToScreen("Verifiable Credential creation done!");
    
        logToScreen("Holder (Alice):")
        logObjectToScreen(res.alice)
    
        logToScreen("Issuer:")
        logObjectToScreen(res.issuer)
    
        logToScreen("VC:")
        logObjectToScreen(res.signedVc)
    
        logToScreen("Check credential result:")
        logObjectToScreen(res.checkResult)
    } catch(err){
        logToScreen(err)
    }

}

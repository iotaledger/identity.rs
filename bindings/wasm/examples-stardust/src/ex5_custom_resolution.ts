import {StardustDocument, StardustDID, MixedResolver} from '../../node';
import {createIdentity} from "./ex0_create_did";

export async function customResolution() {
        // Creates a new wallet and identity (see "ex0_create_did" example).
        const {didClient, did} = await createIdentity();
        const resolveDid = async function name(did_input: string) {
            const parsedDid: StardustDID = StardustDID.parse(did_input);
            const resolved = await didClient.resolveDid(parsedDid);
            return resolved;
        };

        let handlerMap = new Map(); 
        handlerMap.set("iota", resolveDid);

        const resolver = new MixedResolver(handlerMap); 

        console.log("before resolving"); 
        const output = await resolver.resolve(did.toString());
        console.log("The resolved DID document:", JSON.stringify(output, null, 2));

        console.log("after resolving");

        console.log("calling handler outside of the resolver");
        const docFromHandler = await resolveDid(did.toString());
        console.log("output from handler:", JSON.stringify(docFromHandler, null, 2)); 

}
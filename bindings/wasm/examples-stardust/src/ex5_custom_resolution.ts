import {StardustDocument, StardustDID, MixedResolver, ResolutionHandlers} from '../../node';
import {createIdentity} from "./ex0_create_did";

export async function customResolution() {
        // Creates a new wallet and identity (see "ex0_create_did" example).
        const {didClient, did} = await createIdentity();
        const resolveDid = async function name(did_input: string) {
            const parsedDid: StardustDID = StardustDID.parse(did_input);
            const resolved = await didClient.resolveDid(parsedDid);
            return resolved;
        };

        let handlerMap: ResolutionHandlers = new Map(); 
        handlerMap.set("iota", resolveDid);

        
        const resolverWithClient = new MixedResolver({
            client: didClient,
        }); 

        console.log("before resolving with resolverWithClient"); 
        const outputResolverWithClient = await resolverWithClient.resolve(did.toString());
        console.log("The resolved DID document from resolverWithClient:", JSON.stringify(outputResolverWithClient, null, 2));

        //@ts-ignore
        const idOutput = outputResolverWithClient.id(); 

        console.log("id is:", idOutput.toString()); 
        
        console.log("output type:", outputResolverWithClient.constructor.name); 
        if (outputResolverWithClient instanceof StardustDocument) {
            console.log("Got a stardust document"); 
        }

        console.log("after resolving with resolverWithClient");

        console.log("calling client.resolveDid outside the resolver");
        const outputFromClient = await didClient.resolveDid(did);
        console.log("outputFromClient: ", JSON.stringify(outputFromClient, null, 2)); 
        console.log("after resolving from the client directly"); 

        
        const resolverWithHandler = new MixedResolver({
            client: undefined, 
            handlers: handlerMap
        }); 

        console.log("before resolving with resolverWithHandler"); 
        const outputFromResolverWithHandler = await resolverWithHandler.resolve(did.toString()); 
        console.log("output from resolverWithHandler: ", JSON.stringify(outputFromResolverWithHandler, null, 2));
        console.log("after resolving with resolverWithHandler"); 

        console.log("calling handler outside of the resolver");
        const docFromHandler = await resolveDid(did.toString());
        console.log("output from handler:", JSON.stringify(docFromHandler, null, 2)); 

}
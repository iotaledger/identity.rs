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

        const resolver = new MixedResolver(); 

        resolver.attachHandler("stardust", resolveDid);

        const output = await resolver.resolve(did.toString());
        console.log("Resolved DID document:", JSON.stringify(output, null, 2));

}
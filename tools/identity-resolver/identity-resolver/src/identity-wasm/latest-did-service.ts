import { latestDid } from "../stores/stores";
import { resolveIdentity } from "./resolve";

export async function storeLatestDidDocument(did: string){

  let document = await resolveIdentity(did)

  latestDid.update(did=> {return document})
}
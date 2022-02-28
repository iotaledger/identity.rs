import { Stronghold } from '../../dist'

const strongholdPath = "./example-strong.hodl";
const password = "my-password";
export const stronghold = new Stronghold(strongholdPath, password, true);
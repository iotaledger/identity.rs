import { Stronghold } from '../../dist'

const strongholdPath = "./example-strong.hodl";
const password = "my-password";
export const stronghold = async () => await Stronghold.build(strongholdPath, password, true);
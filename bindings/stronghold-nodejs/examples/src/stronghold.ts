import { Stronghold } from '../../dist'
import * as os from "os";
import * as path from "path";

export async function stronghold(): Promise<Stronghold> {
    const random = Math.round(Math.random() * 100_000_000)
    const strongholdPath = path.join(os.tmpdir(), "test_strongholds", `${random}.stronghodl`);
    const password = "my-password";
    return await Stronghold.build(strongholdPath, password, true);
}
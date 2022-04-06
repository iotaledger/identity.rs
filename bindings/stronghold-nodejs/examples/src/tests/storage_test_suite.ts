import { stronghold } from '../stronghold';
import { StorageTestSuite } from '../../../../wasm/node/identity_wasm.js';
// import { StorageTestSuite } from '@iota/identity-wasm/node';

// Only verifies that no uncaught exceptions are thrown, including syntax errors etc.
describe("Test Stronghold Node.js examples", function () {
    it("Storage Test Suite", async () => {
        await StorageTestSuite.didCreateTest(await stronghold());
        await StorageTestSuite.keyGenerateTest(await stronghold());
        await StorageTestSuite.keyDeleteTest(await stronghold());
        await StorageTestSuite.keyInsertTest(await stronghold());
        await StorageTestSuite.didListTest(await stronghold());
        await StorageTestSuite.keySignEd25519Test(await stronghold());
        // TODO: Deliberately exclude didPurge test because key deletion
        // is not implemented properly in stronghold. Should be activated with #757.
        // In that case, we can just call the storage_test_suite Wasm account example.
        // await StorageTestSuite.didPurgeTest(await stronghold());
    });
})

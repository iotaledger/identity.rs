import strongholdConnector from '../index.js'
import { DID, KeyPair, KeyType, Network, KeyLocation, MethodType, Generation } from "../../../wasm/node/identity_wasm.js";
//import * as identity from "../../../wasm/node/identity_wasm.js";

const stronghold = await strongholdConnector.NapiStronghold.new("./test.hodl", "secret", true);

let key_pair = new KeyPair(KeyType.Ed25519);
let wasmDID = new DID(key_pair, Network.mainnet().toString());
let napiDID = strongholdConnector.JsDid.fromJsonValue(wasmDID.toJSON());
let keyLocation = new KeyLocation(MethodType.Ed25519VerificationKey2018(), "#example-service", new Generation());
let napiKeyLocation = strongholdConnector.JsKeyLocation.fromBuffer(keyLocation.asBytes());
let publick_key = await stronghold.keyNew(napiDID, napiKeyLocation);
console.log(publick_key);


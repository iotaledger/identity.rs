"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (_) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
exports.__esModule = true;
var index_js_1 = require("../index.js");
var identity_wasm_js_1 = require("../../wasm/node/identity_wasm.js");
var Stronghold = /** @class */ (function () {
    function Stronghold(snapshot, password, dropsave) {
        this.napiStronghold = index_js_1.NapiStronghold.create(snapshot, password, dropsave);
    }
    Stronghold.prototype.setPassword = function (encryptionKey) {
        return __awaiter(this, void 0, void 0, function () {
            return __generator(this, function (_a) {
                return [2 /*return*/, this.napiStronghold.setPassword(Array.from(encryptionKey))];
            });
        });
    };
    Stronghold.prototype.flushChanges = function () {
        return __awaiter(this, void 0, void 0, function () {
            return __generator(this, function (_a) {
                return [2 /*return*/, this.napiStronghold.flushChanges()];
            });
        });
    };
    Stronghold.prototype.keyNew = function (did, keyLocation) {
        return __awaiter(this, void 0, void 0, function () {
            var napiDID, napiKeyLocation;
            return __generator(this, function (_a) {
                napiDID = index_js_1.NapiDID.fromJSON(did.toJSON());
                napiKeyLocation = index_js_1.NapiKeyLocation.fromJSON(keyLocation.toJSON());
                return [2 /*return*/, this.napiStronghold.keyNew(napiDID, napiKeyLocation)];
            });
        });
    };
    Stronghold.prototype.keyInsert = function (did, keyLocation, privateKey) {
        return __awaiter(this, void 0, void 0, function () {
            var napiDID, napiKeyLocation;
            return __generator(this, function (_a) {
                napiDID = index_js_1.NapiDID.fromJSON(did.toJSON());
                napiKeyLocation = index_js_1.NapiKeyLocation.fromJSON(keyLocation.toJSON());
                return [2 /*return*/, this.napiStronghold.keyInsert(napiDID, napiKeyLocation, privateKey)];
            });
        });
    };
    Stronghold.prototype.keyExists = function (did, keyLocation) {
        return __awaiter(this, void 0, void 0, function () {
            var napiDID, napiKeyLocation;
            return __generator(this, function (_a) {
                napiDID = index_js_1.NapiDID.fromJSON(did.toJSON());
                napiKeyLocation = index_js_1.NapiKeyLocation.fromJSON(keyLocation.toJSON());
                return [2 /*return*/, this.napiStronghold.keyExists(napiDID, napiKeyLocation)];
            });
        });
    };
    Stronghold.prototype.keyGet = function (did, keyLocation) {
        return __awaiter(this, void 0, void 0, function () {
            var napiDID, napiKeyLocation;
            return __generator(this, function (_a) {
                napiDID = index_js_1.NapiDID.fromJSON(did.toJSON());
                napiKeyLocation = index_js_1.NapiKeyLocation.fromJSON(keyLocation.toJSON());
                return [2 /*return*/, this.napiStronghold.keyGet(napiDID, napiKeyLocation)];
            });
        });
    };
    Stronghold.prototype.keyDel = function (did, keyLocation) {
        return __awaiter(this, void 0, void 0, function () {
            var napiDID, napiKeyLocation;
            return __generator(this, function (_a) {
                napiDID = index_js_1.NapiDID.fromJSON(did.toJSON());
                napiKeyLocation = index_js_1.NapiKeyLocation.fromJSON(keyLocation.toJSON());
                return [2 /*return*/, this.napiStronghold.keyDel(napiDID, napiKeyLocation)];
            });
        });
    };
    Stronghold.prototype.keySign = function (did, keyLocation, data) {
        return __awaiter(this, void 0, void 0, function () {
            var napiDID, napiKeyLocation, napiSignature;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        napiDID = index_js_1.NapiDID.fromJSON(did.toJSON());
                        napiKeyLocation = index_js_1.NapiKeyLocation.fromJSON(keyLocation.toJSON());
                        return [4 /*yield*/, this.napiStronghold.keySign(napiDID, napiKeyLocation, Array.from(data))];
                    case 1:
                        napiSignature = _a.sent();
                        return [2 /*return*/, identity_wasm_js_1.Signature.fromJSON(napiSignature.toJSON())];
                }
            });
        });
    };
    Stronghold.prototype.chainState = function (did) {
        return __awaiter(this, void 0, void 0, function () {
            var napiDID, napiChainState;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        napiDID = index_js_1.NapiDID.fromJSON(did.toJSON());
                        return [4 /*yield*/, this.napiStronghold.chainState(napiDID)];
                    case 1:
                        napiChainState = _a.sent();
                        return [2 /*return*/, identity_wasm_js_1.ChainState.fromJSON(napiChainState.toJSON())];
                }
            });
        });
    };
    Stronghold.prototype.setChainState = function (did, chainState) {
        return __awaiter(this, void 0, void 0, function () {
            var napiDID, napiChainState;
            return __generator(this, function (_a) {
                napiDID = index_js_1.NapiDID.fromJSON(did.toJSON());
                napiChainState = index_js_1.NapiChainState.fromJSON(chainState.toJSON());
                return [2 /*return*/, this.napiStronghold.setChainState(napiDID, napiChainState)];
            });
        });
    };
    Stronghold.prototype.state = function (did) {
        return __awaiter(this, void 0, void 0, function () {
            var napiDID, napiIdentityState;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        napiDID = index_js_1.NapiDID.fromJSON(did.toJSON());
                        return [4 /*yield*/, this.napiStronghold.state(napiDID)];
                    case 1:
                        napiIdentityState = _a.sent();
                        return [2 /*return*/, identity_wasm_js_1.IdentityState.fromJSON(napiIdentityState.toJSON())];
                }
            });
        });
    };
    Stronghold.prototype.setState = function (did, identityState) {
        return __awaiter(this, void 0, void 0, function () {
            var napiDID, napiIdentityState;
            return __generator(this, function (_a) {
                napiDID = index_js_1.NapiDID.fromJSON(did.toJSON());
                napiIdentityState = index_js_1.NapiIdentityState.fromJSON(identityState.toJSON());
                return [2 /*return*/, this.napiStronghold.setState(napiDID, napiIdentityState)];
            });
        });
    };
    Stronghold.prototype.purge = function (did) {
        return __awaiter(this, void 0, void 0, function () {
            var napiDID;
            return __generator(this, function (_a) {
                napiDID = index_js_1.NapiDID.fromJSON(did.toJSON());
                return [2 /*return*/, this.napiStronghold.purge(napiDID)];
            });
        });
    };
    return Stronghold;
}());
exports.Stronghold = Stronghold;

'use strict';

Object.defineProperty(exports, '__esModule', { value: true });

var require$$0 = require('fs');
var require$$1 = require('path');
var node = require('@iota/identity-wasm/node');

function _interopDefaultLegacy (e) { return e && typeof e === 'object' && 'default' in e ? e : { 'default': e }; }

var require$$0__default = /*#__PURE__*/_interopDefaultLegacy(require$$0);
var require$$1__default = /*#__PURE__*/_interopDefaultLegacy(require$$1);

/*! *****************************************************************************
Copyright (c) Microsoft Corporation.

Permission to use, copy, modify, and/or distribute this software for any
purpose with or without fee is hereby granted.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
PERFORMANCE OF THIS SOFTWARE.
***************************************************************************** */

function __awaiter(thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
}

function __generator(thisArg, body) {
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
}

const { existsSync, readFileSync } = require$$0__default["default"];
const { join } = require$$1__default["default"];

const { platform, arch } = process;

let nativeBinding = null;
let localFileExisted = false;
let loadError = null;

function isMusl() {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      return readFileSync('/usr/bin/ldd', 'utf8').includes('musl')
    } catch (e) {
      return true
    }
  } else {
    const { glibcVersionRuntime } = process.report.getReport().header;
    return !glibcVersionRuntime
  }
}

switch (platform) {
  case 'android':
    switch (arch) {
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'stronghold-nodejs.android-arm64.node'));
        try {
          if (localFileExisted) {
            nativeBinding = require('./stronghold-nodejs.android-arm64.node');
          } else {
            nativeBinding = require('stronghold-nodejs-android-arm64');
          }
        } catch (e) {
          loadError = e;
        }
        break
      case 'arm':
        localFileExisted = existsSync(join(__dirname, 'stronghold-nodejs.android-arm-eabi.node'));
        try {
          if (localFileExisted) {
            nativeBinding = require('./stronghold-nodejs.android-arm-eabi.node');
          } else {
            nativeBinding = require('stronghold-nodejs-android-arm-eabi');
          }
        } catch (e) {
          loadError = e;
        }
        break
      default:
        throw new Error(`Unsupported architecture on Android ${arch}`)
    }
    break
  case 'win32':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(
          join(__dirname, 'stronghold-nodejs.win32-x64-msvc.node')
        );
        try {
          if (localFileExisted) {
            nativeBinding = require('./stronghold-nodejs.win32-x64-msvc.node');
          } else {
            nativeBinding = require('stronghold-nodejs-win32-x64-msvc');
          }
        } catch (e) {
          loadError = e;
        }
        break
      case 'ia32':
        localFileExisted = existsSync(
          join(__dirname, 'stronghold-nodejs.win32-ia32-msvc.node')
        );
        try {
          if (localFileExisted) {
            nativeBinding = require('./stronghold-nodejs.win32-ia32-msvc.node');
          } else {
            nativeBinding = require('stronghold-nodejs-win32-ia32-msvc');
          }
        } catch (e) {
          loadError = e;
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'stronghold-nodejs.win32-arm64-msvc.node')
        );
        try {
          if (localFileExisted) {
            nativeBinding = require('./stronghold-nodejs.win32-arm64-msvc.node');
          } else {
            nativeBinding = require('stronghold-nodejs-win32-arm64-msvc');
          }
        } catch (e) {
          loadError = e;
        }
        break
      default:
        throw new Error(`Unsupported architecture on Windows: ${arch}`)
    }
    break
  case 'darwin':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'stronghold-nodejs.darwin-x64.node'));
        try {
          if (localFileExisted) {
            nativeBinding = require('./stronghold-nodejs.darwin-x64.node');
          } else {
            nativeBinding = require('stronghold-nodejs-darwin-x64');
          }
        } catch (e) {
          loadError = e;
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'stronghold-nodejs.darwin-arm64.node')
        );
        try {
          if (localFileExisted) {
            nativeBinding = require('./stronghold-nodejs.darwin-arm64.node');
          } else {
            nativeBinding = require('stronghold-nodejs-darwin-arm64');
          }
        } catch (e) {
          loadError = e;
        }
        break
      default:
        throw new Error(`Unsupported architecture on macOS: ${arch}`)
    }
    break
  case 'freebsd':
    if (arch !== 'x64') {
      throw new Error(`Unsupported architecture on FreeBSD: ${arch}`)
    }
    localFileExisted = existsSync(join(__dirname, 'stronghold-nodejs.freebsd-x64.node'));
    try {
      if (localFileExisted) {
        nativeBinding = require('./stronghold-nodejs.freebsd-x64.node');
      } else {
        nativeBinding = require('stronghold-nodejs-freebsd-x64');
      }
    } catch (e) {
      loadError = e;
    }
    break
  case 'linux':
    switch (arch) {
      case 'x64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'stronghold-nodejs.linux-x64-musl.node')
          );
          try {
            if (localFileExisted) {
              nativeBinding = require('./stronghold-nodejs.linux-x64-musl.node');
            } else {
              nativeBinding = require('stronghold-nodejs-linux-x64-musl');
            }
          } catch (e) {
            loadError = e;
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'stronghold-nodejs.linux-x64-gnu.node')
          );
          try {
            if (localFileExisted) {
              nativeBinding = require('./stronghold-nodejs.linux-x64-gnu.node');
            } else {
              nativeBinding = require('stronghold-nodejs-linux-x64-gnu');
            }
          } catch (e) {
            loadError = e;
          }
        }
        break
      case 'arm64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'stronghold-nodejs.linux-arm64-musl.node')
          );
          try {
            if (localFileExisted) {
              nativeBinding = require('./stronghold-nodejs.linux-arm64-musl.node');
            } else {
              nativeBinding = require('stronghold-nodejs-linux-arm64-musl');
            }
          } catch (e) {
            loadError = e;
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'stronghold-nodejs.linux-arm64-gnu.node')
          );
          try {
            if (localFileExisted) {
              nativeBinding = require('./stronghold-nodejs.linux-arm64-gnu.node');
            } else {
              nativeBinding = require('stronghold-nodejs-linux-arm64-gnu');
            }
          } catch (e) {
            loadError = e;
          }
        }
        break
      case 'arm':
        localFileExisted = existsSync(
          join(__dirname, 'stronghold-nodejs.linux-arm-gnueabihf.node')
        );
        try {
          if (localFileExisted) {
            nativeBinding = require('./stronghold-nodejs.linux-arm-gnueabihf.node');
          } else {
            nativeBinding = require('stronghold-nodejs-linux-arm-gnueabihf');
          }
        } catch (e) {
          loadError = e;
        }
        break
      default:
        throw new Error(`Unsupported architecture on Linux: ${arch}`)
    }
    break
  default:
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

if (!nativeBinding) {
  if (loadError) {
    throw loadError
  }
  throw new Error(`Failed to load native binding`)
}

const { NapiChainState, NapiIdentityState, NapiStronghold, NapiKeyLocation, NapiSignature, NapiDID } = nativeBinding;

var NapiChainState_1 = NapiChainState;
var NapiIdentityState_1 = NapiIdentityState;
var NapiStronghold_1 = NapiStronghold;
var NapiKeyLocation_1 = NapiKeyLocation;
var NapiSignature_1 = NapiSignature;
var NapiDID_1 = NapiDID;

var Stronghold = /** @class */ (function () {
    function Stronghold() {
    }
    Stronghold.prototype.init = function (snapshot, password, dropsave) {
        return __awaiter(this, void 0, void 0, function () {
            var _a;
            return __generator(this, function (_b) {
                switch (_b.label) {
                    case 0:
                        _a = this;
                        return [4 /*yield*/, NapiStronghold_1["new"](snapshot, password, dropsave)];
                    case 1:
                        _a.napiStronghold = _b.sent();
                        return [2 /*return*/];
                }
            });
        });
    };
    Stronghold.build = function (snapshot, password, dropsave) {
        return __awaiter(this, void 0, void 0, function () {
            var stronghold;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        stronghold = new Stronghold();
                        return [4 /*yield*/, stronghold.init(snapshot, password, dropsave)];
                    case 1:
                        _a.sent();
                        return [2 /*return*/, stronghold];
                }
            });
        });
    };
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
                napiDID = NapiDID_1.fromJSON(did.toJSON());
                napiKeyLocation = NapiKeyLocation_1.fromJSON(keyLocation.toJSON());
                return [2 /*return*/, this.napiStronghold.keyNew(napiDID, napiKeyLocation)];
            });
        });
    };
    Stronghold.prototype.keyInsert = function (did, keyLocation, privateKey) {
        return __awaiter(this, void 0, void 0, function () {
            var napiDID, napiKeyLocation;
            return __generator(this, function (_a) {
                napiDID = NapiDID_1.fromJSON(did.toJSON());
                napiKeyLocation = NapiKeyLocation_1.fromJSON(keyLocation.toJSON());
                return [2 /*return*/, this.napiStronghold.keyInsert(napiDID, napiKeyLocation, privateKey)];
            });
        });
    };
    Stronghold.prototype.keyExists = function (did, keyLocation) {
        return __awaiter(this, void 0, void 0, function () {
            var napiDID, napiKeyLocation;
            return __generator(this, function (_a) {
                napiDID = NapiDID_1.fromJSON(did.toJSON());
                napiKeyLocation = NapiKeyLocation_1.fromJSON(keyLocation.toJSON());
                return [2 /*return*/, this.napiStronghold.keyExists(napiDID, napiKeyLocation)];
            });
        });
    };
    Stronghold.prototype.keyGet = function (did, keyLocation) {
        return __awaiter(this, void 0, void 0, function () {
            var napiDID, napiKeyLocation;
            return __generator(this, function (_a) {
                napiDID = NapiDID_1.fromJSON(did.toJSON());
                napiKeyLocation = NapiKeyLocation_1.fromJSON(keyLocation.toJSON());
                return [2 /*return*/, this.napiStronghold.keyGet(napiDID, napiKeyLocation)];
            });
        });
    };
    Stronghold.prototype.keyDel = function (did, keyLocation) {
        return __awaiter(this, void 0, void 0, function () {
            var napiDID, napiKeyLocation;
            return __generator(this, function (_a) {
                napiDID = NapiDID_1.fromJSON(did.toJSON());
                napiKeyLocation = NapiKeyLocation_1.fromJSON(keyLocation.toJSON());
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
                        napiDID = NapiDID_1.fromJSON(did.toJSON());
                        napiKeyLocation = NapiKeyLocation_1.fromJSON(keyLocation.toJSON());
                        return [4 /*yield*/, this.napiStronghold.keySign(napiDID, napiKeyLocation, Array.from(data))];
                    case 1:
                        napiSignature = _a.sent();
                        return [2 /*return*/, node.Signature.fromJSON(napiSignature.toJSON())];
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
                        napiDID = NapiDID_1.fromJSON(did.toJSON());
                        return [4 /*yield*/, this.napiStronghold.chainState(napiDID)];
                    case 1:
                        napiChainState = _a.sent();
                        return [2 /*return*/, node.ChainState.fromJSON(napiChainState.toJSON())];
                }
            });
        });
    };
    Stronghold.prototype.setChainState = function (did, chainState) {
        return __awaiter(this, void 0, void 0, function () {
            var napiDID, napiChainState;
            return __generator(this, function (_a) {
                napiDID = NapiDID_1.fromJSON(did.toJSON());
                napiChainState = NapiChainState_1.fromJSON(chainState.toJSON());
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
                        napiDID = NapiDID_1.fromJSON(did.toJSON());
                        return [4 /*yield*/, this.napiStronghold.state(napiDID)];
                    case 1:
                        napiIdentityState = _a.sent();
                        return [2 /*return*/, node.IdentityState.fromJSON(napiIdentityState.toJSON())];
                }
            });
        });
    };
    Stronghold.prototype.setState = function (did, identityState) {
        return __awaiter(this, void 0, void 0, function () {
            var napiDID, napiIdentityState;
            return __generator(this, function (_a) {
                napiDID = NapiDID_1.fromJSON(did.toJSON());
                napiIdentityState = NapiIdentityState_1.fromJSON(identityState.toJSON());
                return [2 /*return*/, this.napiStronghold.setState(napiDID, napiIdentityState)];
            });
        });
    };
    Stronghold.prototype.purge = function (did) {
        return __awaiter(this, void 0, void 0, function () {
            var napiDID;
            return __generator(this, function (_a) {
                napiDID = NapiDID_1.fromJSON(did.toJSON());
                return [2 /*return*/, this.napiStronghold.purge(napiDID)];
            });
        });
    };
    return Stronghold;
}());

exports.NapiChainState = NapiChainState_1;
exports.NapiDID = NapiDID_1;
exports.NapiIdentityState = NapiIdentityState_1;
exports.NapiKeyLocation = NapiKeyLocation_1;
exports.NapiSignature = NapiSignature_1;
exports.NapiStronghold = NapiStronghold_1;
exports.Stronghold = Stronghold;

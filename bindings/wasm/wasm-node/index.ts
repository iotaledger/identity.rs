import * as identity from './identity_wasm';
for (var prop in identity) {
  exports[prop] = identity[prop];
}
export * from './stardust_identity_client';
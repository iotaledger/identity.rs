// export WasmError derive
#[cfg(feature = "derive")]
pub use wasm_error_derive as derive;

/// Convenience trait to enable generics over the derived structs, such as with [IntoWasmError]
pub trait WasmError: serde::Serialize {}

pub trait IntoWasmError<T>
where
  T: WasmError,
{
  fn into_wasm_error(self) -> T;
}

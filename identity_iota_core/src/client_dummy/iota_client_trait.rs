// content might be moved to `identity_sui_name_tbd` project after it's compilable to wasm
// moving or not, depending on if we can compile the actual Rust Iota SDK client to wasm

// pub type KinesisClientResult<T> = Result<T, anyhow::Error>;

#[async_trait::async_trait(?Send)]
pub trait IotaClientTrait {
  type Error;

  async fn get_chain_identifier(&self) -> Result<String, Self::Error>;
}

use examples::API_ENDPOINT;
use identity_iota::prelude::IotaIdentityClientExt;
use iota_client::api::verify_semantic;
use iota_client::api::PreparedTransactionData;
use iota_client::api::SignedTransactionData;
use iota_client::block::address::Address;
use iota_client::block::input::UtxoInput;
use iota_client::block::output::unlock_condition::GovernorAddressUnlockCondition;
use iota_client::block::output::unlock_condition::StateControllerAddressUnlockCondition;
use iota_client::block::output::AliasId;
use iota_client::block::output::AliasOutput;
use iota_client::block::output::AliasOutputBuilder;
use iota_client::block::output::OutputId;
use iota_client::block::output::UnlockCondition;
use iota_client::block::payload::Payload;
use iota_client::block::payload::TransactionPayload;
use iota_client::block::semantic::ConflictReason;
use iota_client::block::unlock::Unlock;
use iota_client::block::unlock::Unlocks;
use iota_client::block::Block;
use iota_client::secret::stronghold::StrongholdSecretManager;
use iota_client::secret::SecretManage;
use iota_client::secret::SecretManager;
use iota_client::Client;

// The controller of the DID.
struct Controller {
  secret_manager: SecretManager,
  address: Address,
}

// The type representing the plugin that funds TXs.
struct Plugin {
  secret_manager: SecretManager,
  client: Client,
  address: Address,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let (controller, plugin) = setup().await.unwrap();
  let network_hrp = plugin.client.network_name().await.unwrap();

  println!(
    "Go and get some funds onto this address: {}\nPress Enter when done...",
    plugin.address.to_bech32(network_hrp.as_ref())
  );
  let stdin = std::io::stdin();
  let mut input = String::new();
  stdin.read_line(&mut input).unwrap();

  println!("Continuing...");

  let token_supply = plugin.client.get_token_supply().await.unwrap();

  let (output_id, alias_output) = create_alias(&plugin.client, &plugin.secret_manager, controller.address.clone())
    .await
    .unwrap();

  // Modify output locally.
  let (input, output) = controller_prepare_update(output_id.clone(), alias_output, token_supply)
    .await
    .unwrap();

  // Send input to plugin.
  let prepared_tx = plugin.plugin_prepare_tx(input, output).await.unwrap();

  // Skipped: Controller verifies the included output is correct.

  // Sign input locally and produce an Unlock.
  let controller_unlock = controller.controller_sign_tx(&prepared_tx).await.unwrap();

  // Send Unlock to plugin.
  let signed_tx = plugin.plugin_finish_tx(controller_unlock, prepared_tx).await.unwrap();

  // Plugin publishes.
  plugin.plugin_publish(signed_tx).await.unwrap();

  let alias_id = AliasId::from(&output_id);
  println!("published alias with id\n{alias_id}");

  Ok(())
}

async fn setup() -> Result<(Controller, Plugin), Box<dyn std::error::Error>> {
  let mut controller_secret_manager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password("secure_password")
      .build("./controller.stronghold")
      .unwrap(),
  );

  let mut plugin_secret_manager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password("another_secure_password")
      .build("./plugin.stronghold")
      .unwrap(),
  );

  let plugin_client = Client::builder()
    .with_primary_node(API_ENDPOINT, None)
    .unwrap()
    .finish()
    .unwrap();

  let plugin_address: Address = examples::get_address(&plugin_client, &mut plugin_secret_manager)
    .await
    .unwrap();
  let controller_address: Address = examples::get_address(&plugin_client, &mut controller_secret_manager)
    .await
    .unwrap();

  let controller = Controller {
    secret_manager: controller_secret_manager,
    address: controller_address,
  };

  let plugin = Plugin {
    secret_manager: plugin_secret_manager,
    client: plugin_client,
    address: plugin_address,
  };

  Ok((controller, plugin))
}

async fn create_alias(
  client: &Client,
  secret_manager: &SecretManager,
  state_controller: Address,
) -> Result<(OutputId, AliasOutput), Box<dyn std::error::Error>> {
  let rent_structure = client.get_rent_structure().await.unwrap();

  let alias_output = AliasOutputBuilder::new_with_minimum_storage_deposit(rent_structure, AliasId::null())
    .unwrap()
    .with_state_index(0)
    .with_foundry_counter(0)
    .with_state_metadata(vec![0, 1, 2, 3])
    .add_unlock_condition(UnlockCondition::StateControllerAddress(
      StateControllerAddressUnlockCondition::new(state_controller),
    ))
    .add_unlock_condition(UnlockCondition::GovernorAddress(GovernorAddressUnlockCondition::new(
      state_controller,
    )))
    .finish(client.get_token_supply().await.unwrap())
    .unwrap();

  let block: Block = client
    .block()
    .with_secret_manager(secret_manager)
    .with_outputs(vec![alias_output.clone().into()])
    .unwrap()
    .finish()
    .await
    .unwrap();

  let _ = client.retry_until_included(&block.id(), None, None).await.unwrap();

  let output_id = utils::unpack_output_id_from_block(&block).unwrap();

  Ok((output_id, alias_output))
}

async fn controller_prepare_update(
  output_id: OutputId,
  alias_output: AliasOutput,
  token_supply: u64,
) -> Result<(UtxoInput, AliasOutput), Box<dyn std::error::Error>> {
  // This is the input we want to consume.
  let input = UtxoInput::from(output_id);
  // These are the modification we want to make to the existing alias output.
  let output: AliasOutput = AliasOutputBuilder::from(&alias_output)
    .with_state_metadata(vec![5, 6, 7, 8])
    .finish(token_supply)
    .unwrap();

  Ok((input, output))
}

impl Plugin {
  async fn plugin_prepare_tx(
    &self,
    alias_input: UtxoInput,
    alias_output: AliasOutput,
  ) -> Result<PreparedTransactionData, Box<dyn std::error::Error>> {
    // Calculate the storage deposit for the alias output.
    let rent_structure = self.client.get_rent_structure().await.unwrap();
    let alias_output: AliasOutput = AliasOutputBuilder::from(&alias_output)
      .with_minimum_storage_deposit(rent_structure)
      .finish(self.client.get_token_supply().await.unwrap())
      .unwrap();

    // Find one or more inputs to fund the alias output.
    let network_hrp = self.client.network_name().await.unwrap();
    let addresses = vec![self.address.to_bech32(network_hrp.as_ref())];
    let mut inputs = self.client.find_inputs(addresses, alias_output.amount()).await.unwrap();

    // Include the alias_input in the inputs for the TX.
    inputs.push(alias_input);

    // Prepare the transaction.
    let mut transaction_builder = self.client.block();
    for input in inputs {
      transaction_builder = transaction_builder.with_input(input).unwrap();
    }

    let prepared_transaction = transaction_builder
      .with_outputs(vec![alias_output.into()])
      .unwrap()
      .prepare_transaction()
      .await
      .unwrap();

    Ok(prepared_transaction)
  }

  async fn plugin_finish_tx(
    &self,
    controller_unlock: Unlock,
    prepared_tx: PreparedTransactionData,
  ) -> Result<SignedTransactionData, Box<dyn std::error::Error>> {
    let essence_hash = prepared_tx.essence.hash();

    let mut unlocks = vec![];

    // We sign everything except the alias output of the controller which is the last output.
    for input in prepared_tx.inputs_data.iter().take(prepared_tx.inputs_data.len() - 1) {
      unlocks.push(
        self
          .secret_manager
          .signature_unlock(input, &essence_hash, &prepared_tx.remainder)
          .await
          .unwrap(),
      );
    }

    // Append the unlock of the alias output that we received from the controller.
    unlocks.push(controller_unlock);

    let unlocks: Unlocks = Unlocks::new(unlocks).unwrap();

    let signed_transaction = TransactionPayload::new(prepared_tx.essence.clone(), unlocks).unwrap();

    let signed_transaction_data = SignedTransactionData {
      transaction_payload: signed_transaction,
      inputs_data: prepared_tx.inputs_data,
    };

    Ok(signed_transaction_data)
  }

  async fn plugin_publish(
    &self,
    signed_transaction_payload: SignedTransactionData,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let client = &self.client;
    let current_time = client.get_time_checked().await.unwrap();

    let conflict = verify_semantic(
      &signed_transaction_payload.inputs_data,
      &signed_transaction_payload.transaction_payload,
      current_time,
    )
    .unwrap();

    if conflict != ConflictReason::None {
      panic!("{conflict:?}");
    }

    // Sends the offline signed transaction online.
    let block = client
      .block()
      .finish_block(Some(Payload::Transaction(Box::new(
        signed_transaction_payload.transaction_payload,
      ))))
      .await
      .unwrap();

    let _ = client.retry_until_included(&block.id(), None, None).await.unwrap();

    Ok(())
  }
}

impl Controller {
  async fn controller_sign_tx(
    &self,
    prepared_transaction_data: &PreparedTransactionData,
  ) -> Result<Unlock, Box<dyn std::error::Error>> {
    let essence_hash = prepared_transaction_data.essence.hash();

    // We define the last input to be the alias output.
    // Can be changed, we just need to somehow agree on it.
    let last_input_idx = prepared_transaction_data.inputs_data.len() - 1;
    let input = prepared_transaction_data
      .inputs_data
      .get(last_input_idx)
      .expect("handle error");

    // Sign the alias input and return the `Unlock`.
    return self
      .secret_manager
      .signature_unlock(input, &essence_hash, &prepared_transaction_data.remainder)
      .await
      .map_err(Into::into);
  }
}

mod utils {
  use iota_client::block::output::Output;
  use iota_client::block::output::OutputId;
  use iota_client::block::payload::transaction::TransactionEssence;
  use iota_client::block::payload::Payload;
  use iota_client::block::Block;

  pub fn unpack_output_id_from_block(block: &Block) -> Result<OutputId, Box<dyn std::error::Error>> {
    if let Some(Payload::Transaction(tx_payload)) = block.payload() {
      let TransactionEssence::Regular(regular) = tx_payload.essence();

      for (index, output) in regular.outputs().iter().enumerate() {
        if let Output::Alias(_alias_output) = output {
          return OutputId::new(tx_payload.id(), index.try_into().unwrap()).map_err(Into::into);
        }
      }
    }

    Err("did not find alias id".into())
  }
}

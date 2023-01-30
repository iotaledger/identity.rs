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
use iota_client::block::output::BasicOutputBuilder;
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

  // Create an alias funded by the plugin and controlled by the controller for demonstration purposes,
  // so we can model the "update alias output in delegated funding" scenario.
  let (output_id, alias_output) = create_alias(&plugin.client, &plugin.secret_manager, controller.address.clone())
    .await
    .unwrap();

  // Modify output locally. This is where we would update the contained DID document.
  let (alias_input, alias_output) = controller
    .update_alias(output_id.clone(), alias_output, token_supply)
    .await
    .unwrap();

  // Send input to plugin.
  // Construct the TX from the alias output and a suitable Basic Output that funds the Alias' storage deposit.
  let prepared_tx = plugin.prepare_tx(alias_input, alias_output).await.unwrap();

  // Skipped: Controller verifies the included output is correct.

  // Sign input locally and produce an Unlock.
  let controller_unlock = controller.sign_tx(&output_id, &prepared_tx).await.unwrap();

  // Send Unlock to plugin.
  let signed_tx = plugin
    .sign_tx(&output_id, controller_unlock, prepared_tx)
    .await
    .unwrap();

  // Plugin publishes.
  let output_id = plugin.publish(signed_tx).await.unwrap();

  let alias_id = AliasId::from(&output_id);
  println!("Published alias with id\n{alias_id}");

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

  let output_id = utils::unpack_alias_output_id_from_block(&block).unwrap();

  Ok((output_id, alias_output))
}

impl Plugin {
  async fn prepare_tx(
    &self,
    alias_input: UtxoInput,
    alias_output: AliasOutput,
  ) -> Result<PreparedTransactionData, Box<dyn std::error::Error>> {
    let token_supply = self.client.get_token_supply().await.unwrap();

    // Calculate the storage deposit for the alias output.
    let rent_structure = self.client.get_rent_structure().await.unwrap();
    let alias_output: AliasOutput = AliasOutputBuilder::from(&alias_output)
      .with_minimum_storage_deposit(rent_structure)
      .finish(token_supply)
      .unwrap();

    // Find an input to fund the alias output.
    let network_hrp = self.client.network_name().await.unwrap();
    let address = self.address.to_bech32(network_hrp.as_ref());
    let (basic_input, amount) = utils::find_funding_output(&self.client, address, alias_output.amount()).await;

    // Build the Basic Output that the remaining funds will be put into explicitly,
    // so that the plugin retains control over them.
    // Without this, the automatic input selection transferred the funds to the controller.
    let remainder_output = BasicOutputBuilder::new_with_amount(amount)
      .unwrap()
      .add_unlock_condition(UnlockCondition::Address(self.address.into()))
      .finish(token_supply)
      .unwrap();

    // Prepare the transaction.
    let transaction_builder = self
      .client
      .block()
      .with_input(alias_input)
      .unwrap()
      .with_input(basic_input)
      .unwrap()
      .with_outputs(vec![alias_output.into(), remainder_output.into()])
      .unwrap();

    let prepared_transaction = transaction_builder.prepare_transaction().await.unwrap();

    Ok(prepared_transaction)
  }

  /// Signs all inputs in the given transaction except for the alias input that was signed by the controller.
  async fn sign_tx(
    &self,
    alias_output_id: &OutputId,
    controller_unlock: Unlock,
    prepared_tx: PreparedTransactionData,
  ) -> Result<SignedTransactionData, Box<dyn std::error::Error>> {
    let essence_hash = prepared_tx.essence.hash();

    let mut unlocks = Vec::with_capacity(prepared_tx.inputs_data.len());

    // We sign everything except the alias output of the controller which is the last output.
    for input in prepared_tx.inputs_data.iter() {
      if input.output_id() == alias_output_id {
        // We don't sign the alias input ourselves but use the received unlock from the controller instead.
        unlocks.push(controller_unlock.clone());
      } else {
        // Otherwise we sign ourselves.
        unlocks.push(
          self
            .secret_manager
            .signature_unlock(input, &essence_hash, &prepared_tx.remainder)
            .await
            .unwrap(),
        );
      }
    }

    let unlocks: Unlocks = Unlocks::new(unlocks).unwrap();

    let signed_transaction = TransactionPayload::new(prepared_tx.essence.clone(), unlocks).unwrap();

    let signed_transaction_data = SignedTransactionData {
      transaction_payload: signed_transaction,
      inputs_data: prepared_tx.inputs_data,
    };

    Ok(signed_transaction_data)
  }

  /// Publishes and returns the output id of the published alias output.
  async fn publish(
    &self,
    signed_transaction_payload: SignedTransactionData,
  ) -> Result<OutputId, Box<dyn std::error::Error>> {
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

    utils::unpack_alias_output_id_from_block(&block)
  }
}

impl Controller {
  async fn sign_tx(
    &self,
    output_id: &OutputId,
    prepared_transaction_data: &PreparedTransactionData,
  ) -> Result<Unlock, Box<dyn std::error::Error>> {
    let essence_hash = prepared_transaction_data.essence.hash();

    // Find the alias input that we need to sign.
    let input_signing_data = prepared_transaction_data
      .inputs_data
      .iter()
      .find(|input| input.output_id() == output_id)
      .expect("should be present");

    // Sign the alias input and return the `Unlock`.
    return self
      .secret_manager
      .signature_unlock(input_signing_data, &essence_hash, &prepared_transaction_data.remainder)
      .await
      .map_err(Into::into);
  }

  async fn update_alias(
    &self,
    output_id: OutputId,
    alias_output: AliasOutput,
    token_supply: u64,
  ) -> Result<(UtxoInput, AliasOutput), Box<dyn std::error::Error>> {
    // This is the input we want to consume.
    let input = UtxoInput::from(output_id);
    // These are the modification we want to make to the existing alias output.
    let output: AliasOutput = AliasOutputBuilder::from(&alias_output)
      .with_alias_id(AliasId::from(&output_id))
      .with_state_index(alias_output.state_index() + 1)
      .with_state_metadata(vec![5, 6, 7, 8])
      .finish(token_supply)
      .unwrap();

    Ok((input, output))
  }
}

mod utils {
  use std::str::FromStr;

  use iota_client::api::ClientBlockBuilder;
  use iota_client::block::input::UtxoInput;
  use iota_client::block::output::Output;
  use iota_client::block::output::OutputId;
  use iota_client::block::payload::transaction::TransactionEssence;
  use iota_client::block::payload::transaction::TransactionId;
  use iota_client::block::payload::Payload;
  use iota_client::block::Block;
  use iota_client::node_api::indexer::query_parameters::QueryParameter;
  use iota_client::Client;

  pub fn unpack_alias_output_id_from_block(block: &Block) -> Result<OutputId, Box<dyn std::error::Error>> {
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

  // Adapted from Client::find_inputs.
  pub async fn find_funding_output(client: &Client, address: String, amount: u64) -> (UtxoInput, u64) {
    let basic_output_ids = client
      .basic_output_ids(vec![
        QueryParameter::Address(address),
        QueryParameter::HasExpiration(false),
        QueryParameter::HasTimelock(false),
        QueryParameter::HasStorageDepositReturn(false),
      ])
      .await
      .unwrap();

    let outputs = client.get_outputs(basic_output_ids).await.unwrap();

    let current_time = client.get_time_checked().await.unwrap();
    let token_supply = client.get_token_supply().await.unwrap();
    let mut basic_outputs = Vec::new();

    for output_resp in outputs {
      let (amount, _) = ClientBlockBuilder::get_output_amount_and_address(
        &Output::try_from_dto(&output_resp.output, token_supply).unwrap(),
        None,
        current_time,
      )
      .unwrap();
      basic_outputs.push((
        UtxoInput::new(
          TransactionId::from_str(&output_resp.metadata.transaction_id).unwrap(),
          output_resp.metadata.output_index,
        )
        .unwrap(),
        amount,
      ));
    }
    basic_outputs.sort_by(|l, r| r.1.cmp(&l.1));

    if amount <= basic_outputs[0].1 {
      return basic_outputs.into_iter().next().unwrap();
    } else {
      unimplemented!("we assume there exists one output with enough funds for now")
    }
  }
}

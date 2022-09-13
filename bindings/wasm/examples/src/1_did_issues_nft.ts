// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { IotaDocument, IotaIdentityClient, MethodScope, KeyPair, KeyType, IotaVerificationMethod, IotaDID } from '../../node';
import { AddressTypes, IAliasOutput, IRent, TransactionHelper, ALIAS_ADDRESS_TYPE, IAliasAddress, ISSUER_FEATURE_TYPE, IStateControllerAddressUnlockCondition, STATE_CONTROLLER_ADDRESS_UNLOCK_CONDITION_TYPE, NFT_OUTPUT_TYPE, METADATA_FEATURE_TYPE, PayloadTypes, TRANSACTION_PAYLOAD_TYPE, ITransactionPayload, TRANSACTION_ESSENCE_TYPE, INftOutput, ADDRESS_UNLOCK_CONDITION_TYPE } from '@iota/iota.js';
import { API_ENDPOINT, createDid } from './util';
import { Client, INftOutputBuilderOptions, MnemonicSecretManager } from '@cycraig/iota-client-wasm/node';
import { Bip39 } from '@iota/crypto.js';
import { Converter } from '@iota/util.js';

/** Demonstrates how an identity can issue and own NFTs,
and how observers can verify the issuer of the NFT.

For this example, we consider the case where a manufacturer issues
a digital product passport (DPP) as an NFT. */
export async function didIssuesNft() {
  // ==============================================
  // Create the manufacturer's DID and the DPP NFT.
  // ==============================================

  // Create a new client to interact with the IOTA ledger.
  const client = new Client({
    primaryNode: API_ENDPOINT,
    localPow: true,
  });
  const didClient = new IotaIdentityClient(client);

  // Generate a random mnemonic for our wallet.
  const secretManager: MnemonicSecretManager = {
    Mnemonic: Bip39.randomMnemonic()
  };

  // Create a new DID for the manufacturer. (see "ex0_create_did" example).
  var { did: manufacturerDid } = await createDid(client, secretManager);

  // Get the current byte costs.
  const rentStructure: IRent = await didClient.getRentStructure();

  // Get the Bech32 human-readable part (HRP) of the network.
  const networkName: string = await didClient.getNetworkHrp();

  const manufacturerAliasAddress: AddressTypes = {
    type: ALIAS_ADDRESS_TYPE,
    aliasId: manufacturerDid.toAliasId()
  }

  // Create a Digital Product Passport NFT issued by the manufacturer.
  let productPassportNft: INftOutput = {
    type: NFT_OUTPUT_TYPE,
    amount: "0",
    nftId: "0x0000000000000000000000000000000000000000000000000000000000000000",
    immutableFeatures: [
      {
        // Set the manufacturer as the immutable issuer.
        type: ISSUER_FEATURE_TYPE,
        address: manufacturerAliasAddress,
      },
      {
        // A proper DPP would hold its metadata here.
        type: METADATA_FEATURE_TYPE,
        data: Converter.utf8ToHex("Digital Product Passport Metadata", true)
      }
    ],
    unlockConditions: [
      {
        // The NFT will initially be owned by the manufacturer.
        type: ADDRESS_UNLOCK_CONDITION_TYPE,
        address: manufacturerAliasAddress
      }
    ]
  };

  productPassportNft.amount = TransactionHelper.getStorageDeposit(productPassportNft, rentStructure).toString();

  // Publish the NFT.
  const [blockId, block] = await client.buildAndPostBlock(secretManager, { outputs: [productPassportNft] });
  await client.retryUntilIncluded(blockId);

  // ========================================================
  // Resolve the Digital Product Passport NFT and its issuer.
  // ========================================================

  // Extract the identifier of the NFT from the published block.

  // Non-null assertion is safe because we published a block with a payload.
  let nftId: string = nft_output_id(block.payload!);

  console.log("nftId ", nftId);

  // let nft_id: NftId = NftId::from(get_nft_output_id(
  //   block
  //     .payload()
  //     .ok_or_else(|| anyhow::anyhow!("expected block to contain a payload"))?,
  // )?);

  // // Fetch the NFT Output.
  // let nft_output_id: OutputId = client.nft_output_id(nft_id).await?;
  // let output_response: OutputResponse = client.get_output(&nft_output_id).await?;
  // let output: Output = Output::try_from(&output_response.output)?;

  // // Extract the issuer of the NFT.
  // let nft_output: NftOutput = if let Output::Nft(nft_output) = output {
  //   nft_output
  // } else {
  //   anyhow::bail!("expected NFT output")
  // };

  // let issuer_address: Address = if let Some(Feature::Issuer(issuer)) = nft_output.immutable_features().iter().next() {
  //   *issuer.address()
  // } else {
  //   anyhow::bail!("expected an issuer feature")
  // };

  // let manufacturer_alias_id: AliasId = if let Address::Alias(alias_address) = issuer_address {
  //   *alias_address.alias_id()
  // } else {
  //   anyhow::bail!("expected an Alias Address")
  // };

  // // Reconstruct the manufacturer's DID from the Alias Id.
  // let network: NetworkName = client.network_name().await?;
  // let manufacturer_did: IotaDID = IotaDID::new(&*manufacturer_alias_id, &network);

  // // Resolve the issuer of the NFT.
  // let manufacturer_document: IotaDocument = client.resolve_did(&manufacturer_did).await?;

  // println!("The issuer of the Digital Product Passport NFT is: {manufacturer_document:#}");
}

function nft_output_id(payload: PayloadTypes): string {
  if (payload.type === TRANSACTION_PAYLOAD_TYPE) {
    const txPayload: ITransactionPayload = payload;
    const txHash = Converter.bytesToHex(TransactionHelper.getTransactionPayloadHash(txPayload), true);

    if (txPayload.essence.type === TRANSACTION_ESSENCE_TYPE) {
      const outputs = txPayload.essence.outputs;
      for (let index in txPayload.essence.outputs) {
        if (outputs[index].type === NFT_OUTPUT_TYPE) {
          return TransactionHelper.outputIdFromTransactionData(txHash, parseInt(index));
        }
      }
      throw new Error("no NFT output in transaction essence");
    } else {
      throw new Error("expected transaction essence");
    }
  } else {
    throw new Error("expected transaction payload");
  }
}

import { selectedNetwork } from "../stores/stores";
import { IotaJsClient } from "./iota-js-client";
import { Network } from "./networks";

export function createClient() {
  let network: Network;
  selectedNetwork.subscribe((value) => (network = value));

  if (network === Network.Mainnet) {
    return new IotaJsClient(Network.Mainnet.toString());
  }
  //ToDo testnet!
}

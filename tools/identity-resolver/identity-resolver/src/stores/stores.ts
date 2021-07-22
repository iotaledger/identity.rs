import { writable } from "svelte/store";
import { Network } from "../iota-client/networks";

export const debugMode = writable(false);
export const selectedNetwork = writable(Network.Mainnet);
export const Messages = writable([]);
export const latestDid = writable({})

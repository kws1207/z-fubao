import type { ClusterUrl } from "@solana/kit";
import { testnet } from "@solana/kit";
import { createContext } from "react";

export type ChainContext = Readonly<{
  chain: `solana:${string}`;
  displayName: string;
  setChain?(chain: `solana:${string}`): void;
  solanaExplorerClusterName: "devnet" | "mainnet-beta" | "testnet";
  solanaRpcSubscriptionsUrl: ClusterUrl;
  solanaRpcUrl: ClusterUrl;
}>;

export const DEFAULT_CHAIN_CONFIG = Object.freeze({
  chain: "solana:testnet",
  displayName: "Testnet",
  solanaExplorerClusterName: "testnet",
  solanaRpcSubscriptionsUrl: testnet("wss://api.testnet.solana.com"),
  solanaRpcUrl: testnet("https://api.testnet.solana.com"),
});

export const ChainContext = createContext<ChainContext>(DEFAULT_CHAIN_CONFIG);

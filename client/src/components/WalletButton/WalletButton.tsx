"use client";

import { Button, DropdownMenu } from "@radix-ui/themes";
import { useWallet } from "@solana/wallet-adapter-react";
import { useWalletModal } from "@solana/wallet-adapter-react-ui";

import { useBitcoinWallet } from "@/contexts/BitcoinWalletProvider";
import useStore from "@/stores/store";
import { MODAL_NAMES } from "@/utils/constant";

export function WalletButton() {
  const openModalByName = useStore((state) => state.openModalByName);
  const { connected: solanaWalletConnected, publicKey } = useWallet();
  const { setVisible: setShowSolanaModal } = useWalletModal();
  const { connected: bitcoinWalletConnected } = useBitcoinWallet();

  const handleConnectWallet = () => {
    if (!solanaWalletConnected) {
      setShowSolanaModal(true);
    } else if (solanaWalletConnected && !bitcoinWalletConnected) {
      openModalByName(MODAL_NAMES.ADD_NEW_WALLET);
    }
  };

  return (
    <DropdownMenu.Root>
      <DropdownMenu.Trigger>
        <Button onClick={handleConnectWallet}>
          {solanaWalletConnected ? (
            <>{publicKey?.toBase58().slice(0, 8)}...</>
          ) : (
            "Connect Wallet"
          )}
          <DropdownMenu.TriggerIcon />
        </Button>
      </DropdownMenu.Trigger>
      {solanaWalletConnected && (
        <DropdownMenu.Content>
          {bitcoinWalletConnected ? (
            <DropdownMenu.Item>Disconnect</DropdownMenu.Item>
          ) : (
            <DropdownMenu.Item
              onClick={() => openModalByName(MODAL_NAMES.ADD_NEW_WALLET)}
            >
              Connect Bitcoin Wallet
            </DropdownMenu.Item>
          )}
        </DropdownMenu.Content>
      )}
    </DropdownMenu.Root>
  );
}

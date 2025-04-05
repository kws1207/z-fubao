import {
  Box,
  Button,
  Flex,
  Select,
  Text,
  TextField,
  Separator,
  Card,
} from "@radix-ui/themes";
import { useContext, useState, useEffect } from "react";
import { FeaturePanel } from "./FeaturePanel";
import { SelectedWalletAccountContext } from "../context/SelectedWalletAccountContext";

const fetchBTCPrice = async (): Promise<number> => {
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve(84000 + Math.random() * 2000);
    }, 500);
  });
};

export function LockAndMintPanel() {
  const [selectedWalletAccount] = useContext(SelectedWalletAccountContext);
  const [btcAmount, setBtcAmount] = useState("");
  const [usdAmount, setUsdAmount] = useState("");
  const [btcPrice, setBtcPrice] = useState<number | null>(null);
  const [isProcessing, setIsProcessing] = useState(false);
  const LTV = 0.7;

  useEffect(() => {
    const getPrice = async () => {
      const price = await fetchBTCPrice();
      setBtcPrice(price);
    };

    getPrice();

    const interval = setInterval(getPrice, 30000);
    return () => clearInterval(interval);
  }, []);

  useEffect(() => {
    if (btcAmount && btcPrice) {
      const valueInUSD = parseFloat(btcAmount) * btcPrice;
      const mintableUSD = valueInUSD * LTV;
      setUsdAmount(mintableUSD.toFixed(2));
    } else {
      setUsdAmount("");
    }
  }, [btcAmount, btcPrice]);

  const handleLockAndMint = async () => {
    if (!selectedWalletAccount) {
      alert("Please connect your wallet first");
      return;
    }

    setIsProcessing(true);
    try {
      console.log(
        `Locking ${btcAmount} zBTC as collateral and minting ${usdAmount} zUSD`
      );

      await new Promise((resolve) => setTimeout(resolve, 1500));

      setBtcAmount("");
      setUsdAmount("");
      alert(
        `Successfully locked ${btcAmount} zBTC and minted ${usdAmount} zUSD`
      );
    } catch (error: unknown) {
      console.error("Operation failed:", error);
      const errorMessage =
        error instanceof Error ? error.message : "Unknown error";
      alert(`Operation failed: ${errorMessage}`);
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <FeaturePanel label="Lock & Mint">
      <Card style={{ width: "100%" }}>
        <Flex direction="row" gap="6" align="center">
          <Box style={{ flex: 1 }}>
            <Flex direction="column" gap="2">
              <Text as="label" size="2" weight="bold">
                Lock zBTC as Collateral
              </Text>
              <Flex gap="2" align="center">
                <Select.Root value="zBTC" disabled>
                  <Select.Trigger style={{ width: "80px" }} />
                  <Select.Content>
                    <Select.Item value="zBTC">zBTC</Select.Item>
                  </Select.Content>
                </Select.Root>
                <TextField.Root
                  style={{ width: "100%" }}
                  placeholder="0.0"
                  type="number"
                  value={btcAmount}
                  onChange={(e: React.SyntheticEvent<HTMLInputElement>) =>
                    setBtcAmount(e.currentTarget.value)
                  }
                />
              </Flex>
              {btcPrice && (
                <Text size="1" color="gray">
                  1 BTC = ${btcPrice.toFixed(2)} USD
                </Text>
              )}
            </Flex>
          </Box>

          <Separator orientation="vertical" size="3" />

          <Box style={{ flex: 1 }}>
            <Flex direction="column" gap="2">
              <Text as="label" size="2" weight="bold">
                Mint zUSD (70% LTV)
              </Text>
              <Flex gap="2" align="center">
                <Select.Root value="zUSD" disabled>
                  <Select.Trigger style={{ width: "80px" }} />
                  <Select.Content>
                    <Select.Item value="zUSD">zUSD</Select.Item>
                  </Select.Content>
                </Select.Root>
                <TextField.Root
                  style={{ width: "100%" }}
                  placeholder="0.0"
                  type="number"
                  value={usdAmount}
                  readOnly
                />
              </Flex>
              <Text size="1" color="gray">
                LTV Ratio: 70%
              </Text>
            </Flex>
          </Box>
        </Flex>

        <Box style={{ marginTop: "16px" }}>
          <Button
            color="indigo"
            size="3"
            style={{ width: "100%" }}
            onClick={handleLockAndMint}
            disabled={
              !btcAmount ||
              parseFloat(btcAmount) <= 0 ||
              !selectedWalletAccount ||
              isProcessing
            }
          >
            {isProcessing
              ? "Processing..."
              : selectedWalletAccount
              ? "Lock & Mint"
              : "Connect Wallet to Lock & Mint"}
          </Button>
        </Box>
      </Card>
    </FeaturePanel>
  );
}

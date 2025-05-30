import {
  Box,
  Button,
  Flex,
  Text,
  TextField,
  Separator,
  Card,
} from "@radix-ui/themes";
import { useState, useEffect } from "react";
import { FeaturePanel } from "./FeaturePanel";
import { tokenIcons } from "../config";
import { useWallet } from "@solana/wallet-adapter-react";

const fetchBTCPrice = async (): Promise<number> => {
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve(84636);
    }, 500);
  });
};

export function LockAndMintPanel() {
  const { publicKey } = useWallet();
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
    if (!publicKey) {
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
    <FeaturePanel>
      <Card
        style={{
          maxWidth: "750px",
          width: "100%",
          background: "rgba(25, 25, 28, 0.8)",
          border: "1px solid rgba(99, 102, 241, 0.3)",
          borderRadius: "20px",
          padding: "32px",
          boxShadow:
            "0 10px 25px -5px rgba(0, 0, 0, 0.2), 0 8px 10px -6px rgba(0, 0, 0, 0.1)",
          transition: "transform 0.2s, box-shadow 0.2s",
        }}
        onMouseEnter={(e) => {
          e.currentTarget.style.transform = "translateY(-5px)";
          e.currentTarget.style.boxShadow =
            "0 20px 25px -5px rgba(0, 0, 0, 0.3), 0 10px 10px -6px rgba(0, 0, 0, 0.2)";
        }}
        onMouseLeave={(e) => {
          e.currentTarget.style.transform = "none";
          e.currentTarget.style.boxShadow =
            "0 10px 25px -5px rgba(0, 0, 0, 0.2), 0 8px 10px -6px rgba(0, 0, 0, 0.1)";
        }}
      >
        <Flex direction="row" gap="8" align="center">
          <Box style={{ flex: 1 }}>
            <Flex direction="column" gap="3">
              <Text
                as="label"
                size="3"
                weight="bold"
                style={{ color: "var(--indigo-11)" }}
              >
                Lock zBTC as Collateral
              </Text>
              <Flex gap="2" align="center">
                <Box
                  style={{
                    width: "160px",
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                    padding: "10px 12px",
                    background: "rgba(59, 130, 246, 0.1)",
                    borderRadius: "8px",
                    border: "1px solid rgba(59, 130, 246, 0.2)",
                  }}
                >
                  {tokenIcons.zBTC?.endsWith(".svg") ? (
                    <img
                      src={tokenIcons.zBTC}
                      alt="zBTC"
                      style={{
                        width: "20px",
                        height: "20px",
                        marginRight: "8px",
                      }}
                    />
                  ) : (
                    <Text
                      size="3"
                      weight="medium"
                      style={{ marginRight: "8px" }}
                    >
                      {tokenIcons.zBTC || "?"}
                    </Text>
                  )}
                  <Text size="3" weight="medium">
                    zBTC
                  </Text>
                </Box>
                <TextField.Root
                  style={{ width: "100%", fontSize: "16px" }}
                  placeholder="0.0"
                  type="number"
                  value={btcAmount}
                  onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
                    setBtcAmount(e.target.value)
                  }
                  size="3"
                />
              </Flex>
              {btcPrice && (
                <Text size="2" style={{ color: "var(--gray-9)" }}>
                  1 BTC = ${btcPrice.toFixed(2)} USD
                </Text>
              )}
            </Flex>
          </Box>

          <Separator
            orientation="vertical"
            size="4"
            style={{
              background:
                "linear-gradient(to bottom, rgba(99, 102, 241, 0.1), rgba(147, 51, 234, 0.1))",
              height: "120px",
            }}
          />

          <Box style={{ flex: 1 }}>
            <Flex direction="column" gap="3">
              <Text
                as="label"
                size="3"
                weight="bold"
                style={{ color: "var(--indigo-11)" }}
              >
                Mint zUSD (70% LTV)
              </Text>
              <Flex gap="2" align="center">
                <Box
                  style={{
                    width: "160px",
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                    padding: "10px 12px",
                    background: "rgba(16, 185, 129, 0.1)",
                    borderRadius: "8px",
                    border: "1px solid rgba(16, 185, 129, 0.2)",
                  }}
                >
                  {tokenIcons.zUSD?.endsWith(".svg") ? (
                    <img
                      src={tokenIcons.zUSD}
                      alt="zUSD"
                      style={{
                        width: "20px",
                        height: "20px",
                        marginRight: "8px",
                      }}
                    />
                  ) : (
                    <Text
                      size="3"
                      weight="medium"
                      style={{ marginRight: "8px" }}
                    >
                      {tokenIcons.zUSD || "?"}
                    </Text>
                  )}
                  <Text size="3" weight="medium">
                    zUSD
                  </Text>
                </Box>
                <TextField.Root
                  style={{ width: "100%", fontSize: "16px", height: "45px" }}
                  placeholder="0.0"
                  type="number"
                  value={usdAmount}
                  readOnly
                  size="3"
                />
              </Flex>
              <Text size="2" style={{ color: "var(--gray-9)" }}>
                LTV Ratio: 70%
              </Text>
            </Flex>
          </Box>
        </Flex>

        <Box style={{ marginTop: "24px" }}>
          <Button
            size="4"
            style={{
              width: "100%",
              background: "var(--indigo-9)",
              color: "white",
              fontWeight: "bold",
              cursor: isProcessing ? "not-allowed" : "pointer",
              opacity: isProcessing ? 0.7 : 1,
            }}
            onClick={handleLockAndMint}
            disabled={isProcessing || !btcAmount || parseFloat(btcAmount) <= 0}
          >
            {isProcessing ? "Processing..." : "Lock & Mint"}
          </Button>
          {!publicKey && (
            <Text
              size="2"
              style={{ color: "var(--amber-9)", marginTop: "8px" }}
            >
              Please connect your wallet to lock and mint.
            </Text>
          )}
        </Box>
      </Card>
    </FeaturePanel>
  );
}

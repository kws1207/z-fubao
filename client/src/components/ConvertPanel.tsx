import { ArrowDownIcon } from "@radix-ui/react-icons";
import {
  Box,
  Button,
  Flex,
  IconButton,
  Select,
  Text,
  TextField,
} from "@radix-ui/themes";
import { useContext, useState, useEffect, useMemo } from "react";
import { FeaturePanel } from "./FeaturePanel";
import { SelectedWalletAccountContext } from "../context/SelectedWalletAccountContext";

type TokenRates = {
  [key: string]: number;
};

type Mode = "btc" | "zusd";

const modeRatesMap: Record<Mode, TokenRates> = {
  btc: {
    "BTC-zBTC": 1,
    "zBTC-BTC": 1,
  },
  zusd: {
    "zUSD-szUSD": 1,
    "szUSD-zUSD": 1,
  },
};

export function StakePanel({ mode, label }: { mode: Mode; label: string }) {
  const [selectedWalletAccount] = useContext(SelectedWalletAccountContext);

  const rates = modeRatesMap[mode];

  const { uniqueTokens, tokenPairs } = useMemo(() => {
    const tokens = new Set<string>();
    const pairs: Record<string, string[]> = {};

    Object.keys(rates).forEach((pair) => {
      const [from, to] = pair.split("-");
      tokens.add(from);
      tokens.add(to);

      if (!pairs[from]) pairs[from] = [];
      pairs[from].push(to);
    });

    return {
      uniqueTokens: Array.from(tokens),
      tokenPairs: pairs,
    };
  }, [rates]);

  const [fromToken, setFromToken] = useState(uniqueTokens[0] || "");
  const [toToken, setToToken] = useState("");
  const [amount, setAmount] = useState("");
  const [isSwapping, setIsSwapping] = useState(false);

  useEffect(() => {
    if (fromToken && !toToken) {
      const validPairs = tokenPairs[fromToken] || [];
      if (validPairs.length > 0) {
        setToToken(validPairs[0]);
      }
    }
  }, [fromToken, toToken, tokenPairs]);

  const handleFromTokenChange = (token: string) => {
    setFromToken(token);
    const validPairs = tokenPairs[token] || [];
    if (validPairs.length > 0) {
      setToToken(validPairs[0]);
    } else {
      setToToken("");
    }
  };

  const handleToTokenChange = (token: string) => {
    setToToken(token);

    const validFromTokens = Object.keys(tokenPairs).filter((from) =>
      tokenPairs[from]?.includes(token)
    );

    if (!validFromTokens.includes(fromToken)) {
      if (validFromTokens.length > 0) {
        setFromToken(validFromTokens[0]);
      }
    }
  };

  const getRate = () => {
    const key = `${fromToken}-${toToken}`;
    return rates[key] || 0;
  };

  const getEstimatedAmount = () => {
    if (!amount) return "0";
    const rate = getRate();
    return (parseFloat(amount) * rate).toFixed(6);
  };

  const handleSwap = async () => {
    if (!selectedWalletAccount) {
      alert("Please connect your wallet first");
      return;
    }

    setIsSwapping(true);
    try {
      console.log(
        `Swapping ${amount} ${fromToken} for approximately ${getEstimatedAmount()} ${toToken}`
      );

      await new Promise((resolve) => setTimeout(resolve, 1000));

      setAmount("");
      alert(
        `Successfully swapped ${amount} ${fromToken} to ${getEstimatedAmount()} ${toToken}`
      );
    } catch (error: unknown) {
      console.error("Swap failed:", error);
      const errorMessage =
        error instanceof Error ? error.message : "Unknown error";
      alert(`Swap failed: ${errorMessage}`);
    } finally {
      setIsSwapping(false);
    }
  };

  const switchTokens = () => {
    const reversePairExists = Object.keys(rates).includes(
      `${toToken}-${fromToken}`
    );

    if (reversePairExists) {
      const oldFrom = fromToken;
      setFromToken(toToken);
      setToToken(oldFrom);
    } else {
      alert(`Trading pair ${toToken}-${fromToken} is not available`);
    }
  };

  const isStake =
    (mode === "btc" && fromToken === "BTC") ||
    (mode === "zusd" && fromToken === "zUSD");
  const operationString = isStake ? "Stake" : "Unstake";

  return (
    <FeaturePanel label={`${operationString} ${label}`}>
      <Box style={{ maxWidth: "400px", width: "100%" }}>
        <Flex direction="column" gap="3">
          <Box>
            <Text as="label" size="2" weight="bold">
              From
            </Text>
            <Flex gap="2">
              <Select.Root
                value={fromToken}
                onValueChange={handleFromTokenChange}
              >
                <Select.Trigger style={{ width: "100px" }} />
                <Select.Content>
                  {uniqueTokens.map((token) => (
                    <Select.Item key={token} value={token}>
                      {token}
                    </Select.Item>
                  ))}
                </Select.Content>
              </Select.Root>
              <TextField.Root
                style={{ width: "100%" }}
                placeholder="0.0"
                type="number"
                value={amount}
                onChange={(e: React.SyntheticEvent<HTMLInputElement>) =>
                  setAmount(e.currentTarget.value)
                }
              />
            </Flex>
          </Box>

          <Flex justify="center" align="center">
            <IconButton variant="soft" color="gray" onClick={switchTokens}>
              <ArrowDownIcon />
            </IconButton>
          </Flex>

          <Box>
            <Text as="label" size="2" weight="bold">
              To
            </Text>
            <Flex gap="2">
              <Select.Root value={toToken} onValueChange={handleToTokenChange}>
                <Select.Trigger style={{ width: "100px" }} />
                <Select.Content>
                  {uniqueTokens.map((token) => (
                    <Select.Item key={token} value={token}>
                      {token}
                    </Select.Item>
                  ))}
                </Select.Content>
              </Select.Root>
              <TextField.Root
                style={{ width: "100%" }}
                placeholder="0.0"
                type="number"
                value={getEstimatedAmount()}
                readOnly
              />
            </Flex>
          </Box>

          <Box style={{ marginTop: "8px" }}>
            <Text size="1" color="gray">
              Rate: 1 {fromToken} = {getRate()} {toToken}
            </Text>
          </Box>

          <Button
            color="indigo"
            size="3"
            onClick={handleSwap}
            disabled={
              !amount || !selectedWalletAccount || isSwapping || !toToken
            }
          >
            {isSwapping
              ? `${operationString}...`
              : selectedWalletAccount
              ? operationString
              : `Connect Wallet to ${operationString}`}
          </Button>
        </Flex>
      </Box>
    </FeaturePanel>
  );
}

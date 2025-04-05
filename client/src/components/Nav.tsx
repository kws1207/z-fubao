import {
  Badge,
  Box,
  Button,
  DropdownMenu,
  Flex,
  Heading,
} from "@radix-ui/themes";
import { useContext } from "react";
import { Routes, Route, Link } from "react-router-dom";
import { ChainContext } from "../context/ChainContext";
import { ConnectWalletMenu } from "./ConnectWalletMenu";
import { SignInMenu } from "./SignInMenu";
import Mint from "../routes/mint";
import Earn from "../routes/earn";
import Deposit from "../routes/deposit";

export function Nav() {
  const {
    displayName: currentChainName,
    chain,
    setChain,
  } = useContext(ChainContext);
  const currentChainBadge = (
    <Badge color="gray" style={{ verticalAlign: "middle" }}>
      {currentChainName}
    </Badge>
  );
  return (
    <Box
      style={{
        backgroundColor: "var(--gray-1)",
        borderBottom: "1px solid var(--gray-a6)",
        zIndex: 1,
      }}
      position="sticky"
      p="3"
      top="0"
    >
      <Flex gap="4" justify="between" align="center">
        <Box flexGrow="1">
          <Heading as="h1" size={{ initial: "4", xs: "6" }} truncate>
            Solana React App{" "}
            {setChain ? (
              <DropdownMenu.Root>
                <DropdownMenu.Trigger>{currentChainBadge}</DropdownMenu.Trigger>
                <DropdownMenu.Content>
                  <DropdownMenu.RadioGroup
                    onValueChange={(value) => {
                      setChain(value as "solana:${string}");
                    }}
                    value={chain}
                  >
                    <DropdownMenu.RadioItem value="solana:testnet">
                      Testnet
                    </DropdownMenu.RadioItem>
                  </DropdownMenu.RadioGroup>
                </DropdownMenu.Content>
              </DropdownMenu.Root>
            ) : (
              currentChainBadge
            )}
          </Heading>
        </Box>
        <Flex gap="2">
          <Button asChild variant="soft">
            <Link to="/">Home</Link>
          </Button>
          <Button asChild variant="soft">
            <Link to="/deposit">Deposit</Link>
          </Button>
          <Button asChild variant="soft">
            <Link to="/mint">Mint</Link>
          </Button>
          <Button asChild variant="soft">
            <Link to="/earn">Earn</Link>
          </Button>
        </Flex>
        <ConnectWalletMenu>Connect Wallet</ConnectWalletMenu>
        <SignInMenu>Sign In</SignInMenu>
      </Flex>
    </Box>
  );
}

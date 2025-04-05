import {
  Badge,
  Box,
  DropdownMenu,
  Flex,
  Heading,
  Avatar,
} from "@radix-ui/themes";
import { useContext } from "react";
import { Link } from "react-router-dom";
import { ChainContext } from "../context/ChainContext";
import { ConnectWalletMenu } from "./ConnectWalletMenu";
import { SignInMenu } from "./SignInMenu";

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
        borderBottom: "1px solid var(--gray-a4)",
        zIndex: 1,
        padding: "12px 24px",
      }}
      position="sticky"
      top="0"
    >
      <Flex gap="4" justify="between" align="center">
        <Flex align="center" gap="2">
          <Avatar
            fallback="Z"
            color="indigo"
            radius="full"
            size="3"
            style={{ marginRight: "4px" }}
          />
          <Heading as="h1" size={{ initial: "4", xs: "5" }} truncate>
            Z-Fubao{" "}
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
                    <DropdownMenu.RadioItem value="solana:devnet">
                      Devnet
                    </DropdownMenu.RadioItem>
                  </DropdownMenu.RadioGroup>
                </DropdownMenu.Content>
              </DropdownMenu.Root>
            ) : (
              currentChainBadge
            )}
          </Heading>
        </Flex>

        <Flex gap="6" align="center">
          <Flex gap="5" display={{ initial: "none", md: "flex" }}>
            <Link
              to="/"
              style={{
                color: "var(--gray-12)",
                textDecoration: "none",
                fontWeight: 500,
              }}
            >
              Home
            </Link>
            <Link
              to="/deposit"
              style={{
                color: "var(--gray-12)",
                textDecoration: "none",
                fontWeight: 500,
              }}
            >
              Deposit
            </Link>
            <Link
              to="/mint"
              style={{
                color: "var(--gray-12)",
                textDecoration: "none",
                fontWeight: 500,
              }}
            >
              Mint
            </Link>
            <Link
              to="/earn"
              style={{
                color: "var(--gray-12)",
                textDecoration: "none",
                fontWeight: 500,
              }}
            >
              Earn
            </Link>
            <Link
              to="/dashboard"
              style={{
                color: "var(--gray-12)",
                textDecoration: "none",
                fontWeight: 500,
              }}
            >
              Dashboard
            </Link>
          </Flex>

          <Flex gap="2">
            <ConnectWalletMenu>Connect Wallet</ConnectWalletMenu>
            <SignInMenu>Sign In</SignInMenu>
          </Flex>
        </Flex>
      </Flex>
    </Box>
  );
}

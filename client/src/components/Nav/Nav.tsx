"use client";

import { Badge, Box, Flex, Heading } from "@radix-ui/themes";
import Image from "next/image";
import Link from "next/link";

import { WalletButton } from "@/components/WalletButton/WalletButton";

export function Nav() {
  return (
    <Box
      style={{
        backgroundColor: "var(--gray-1)",
        borderBottom: "1px solid var(--gray-a4)",
        zIndex: 1,
        padding: "12px 24px",
        height: "80px",
      }}
      position="sticky"
      top="0"
    >
      <Flex gap="4" justify="between" align="center" height="100%">
        <Flex align="center" gap="2">
          <Image
            src="/zFuBao_logo.png"
            alt="zFuBao Logo"
            width={144}
            height={38}
            style={{ marginRight: "8px" }}
          />
          <Heading as="h1" size={{ initial: "4", xs: "5" }} truncate>
            <Badge color="gray" style={{ verticalAlign: "middle" }}>
              Devnet
            </Badge>
          </Heading>
        </Flex>

        <Flex gap="6" align="center">
          <Flex gap="5" display={{ initial: "none", md: "flex" }}>
            <Link
              href="/"
              style={{
                color: "var(--gray-12)",
                textDecoration: "none",
                fontWeight: 500,
              }}
            >
              Home
            </Link>
            <Link
              href="/deposit"
              style={{
                color: "var(--gray-12)",
                textDecoration: "none",
                fontWeight: 500,
              }}
            >
              Deposit
            </Link>
            <Link
              href="/mint"
              style={{
                color: "var(--gray-12)",
                textDecoration: "none",
                fontWeight: 500,
              }}
            >
              Mint
            </Link>
            <Link
              href="/earn"
              style={{
                color: "var(--gray-12)",
                textDecoration: "none",
                fontWeight: 500,
              }}
            >
              Earn
            </Link>
            <Link
              href="/dashboard"
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
            <WalletButton />
          </Flex>
        </Flex>
      </Flex>
    </Box>
  );
}

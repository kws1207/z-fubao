import { Box, Container, Flex, Text } from "@radix-ui/themes";
import { Link } from "react-router-dom";

export function Footer() {
  return (
    <Box
      style={{
        background: "var(--gray-2)",
        borderTop: "1px solid var(--gray-3)",
        paddingTop: "40px",
        paddingBottom: "40px",
      }}
    >
      <Container size="3">
        <Flex justify="between" align="center" wrap="wrap" gap="4">
          <Text style={{ color: "var(--gray-11)" }}>
            © 2025 z-fubao. All rights reserved.
          </Text>
          <Flex gap="4">
            <Link
              to="https://github.com/kws1207/z-fubao"
              style={{
                color: "var(--indigo-11)",
                textDecoration: "none",
                fontWeight: 500,
              }}
            >
              GitHub
            </Link>
            <Link
              to="/"
              style={{
                color: "var(--purple-11)",
                textDecoration: "none",
                fontWeight: 500,
              }}
            >
              Discord
            </Link>
            <Link
              to="/"
              style={{
                color: "var(--blue-11)",
                textDecoration: "none",
                fontWeight: 500,
              }}
            >
              Twitter
            </Link>
          </Flex>
        </Flex>
      </Container>
    </Box>
  );
}

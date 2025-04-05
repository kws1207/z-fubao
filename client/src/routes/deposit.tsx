import { StakePanel } from "../components/ConvertPanel";
import { Box, Container, Flex, Heading, Section, Text } from "@radix-ui/themes";
import { Footer } from "../components/Footer";

function Deposit() {
  return (
    <Box
      style={{
        background: "linear-gradient(135deg, var(--indigo-1), var(--purple-1))",
        color: "var(--gray-12)",
        minHeight: "calc(100vh - 60px)",
        display: "flex",
        flexDirection: "column",
        justifyContent: "space-between",
      }}
    >
      <Section
        size="3"
        style={{
          background:
            "linear-gradient(135deg, var(--indigo-2), var(--purple-2))",
          paddingTop: "40px",
          paddingBottom: "40px",
        }}
      >
        <Container mx="auto" size="3">
          <Heading
            as="h1"
            size="8"
            style={{
              background:
                "linear-gradient(90deg, var(--indigo-11), var(--purple-11))",
              WebkitBackgroundClip: "text",
              WebkitTextFillColor: "transparent",
            }}
          >
            Deposit
          </Heading>
        </Container>
      </Section>

      <Section
        size="3"
        style={{
          paddingTop: "40px",
          paddingBottom: "60px",
        }}
      >
        <Container mx="auto" size="3">
          <Flex direction="column" gap="4" mb="6">
            <Heading
              size="5"
              style={{
                color: "var(--indigo-10)",
              }}
            >
              Native BTC Deposits
            </Heading>
            <Text
              size="3"
              style={{
                color: "var(--gray-11)",
              }}
            >
              Deposit your native BTC to get zBTC tokens in return. These tokens
              can be used as collateral to mint zUSD or for staking.
            </Text>
          </Flex>
          <StakePanel mode="btc" label="Native BTC" />
        </Container>
      </Section>
      <Footer />
    </Box>
  );
}

export default Deposit;

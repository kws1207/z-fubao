import {
  Box,
  Button,
  Container,
  Flex,
  Grid,
  Heading,
  Text,
  Card,
  Section,
  IconButton,
  Progress,
} from "@radix-ui/themes";
import { InfoCircledIcon, PlusIcon } from "@radix-ui/react-icons";
import { Link } from "react-router-dom";
import { useWallets } from "@wallet-standard/react";

interface AssetCardProps {
  label: string;
  value: string;
  change?: string;
  isPositive?: boolean;
}

function AssetCard({
  label,
  value,
  change,
  isPositive = true,
}: AssetCardProps) {
  const [wallet] = useWallets();
  const account = wallet?.accounts?.[0];

  const displayValue = account ? value : "-";
  const displayChange = account && change ? change : "-";

  return (
    <Card
      size="3"
      style={{
        minWidth: "180px",
        background:
          "linear-gradient(135deg, rgba(99, 102, 241, 0.1) 0%, rgba(79, 70, 229, 0.2) 100%)",
        border: "1px solid rgba(99, 102, 241, 0.15)",
        boxShadow:
          "0 10px 25px -5px rgba(0, 0, 0, 0.1), 0 8px 10px -6px rgba(0, 0, 0, 0.05)",
        transition: "transform 0.2s, box-shadow 0.2s",
        overflow: "hidden",
        position: "relative",
      }}
      onMouseEnter={(e) => {
        e.currentTarget.style.transform = "translateY(-5px)";
        e.currentTarget.style.boxShadow =
          "0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)";
      }}
      onMouseLeave={(e) => {
        e.currentTarget.style.transform = "none";
        e.currentTarget.style.boxShadow =
          "0 10px 25px -5px rgba(0, 0, 0, 0.1), 0 8px 10px -6px rgba(0, 0, 0, 0.05)";
      }}
    >
      <Box
        style={{
          position: "absolute",
          top: 0,
          left: 0,
          right: 0,
          height: "4px",
          background:
            "linear-gradient(90deg, var(--indigo-9) 0%, var(--purple-9) 100%)",
        }}
      />
      <Flex direction="column" gap="2" style={{ padding: "8px 0" }}>
        <Flex align="center" gap="1">
          <Text
            size="2"
            weight="bold"
            style={{
              color: "var(--indigo-11)",
              textTransform: "uppercase",
              letterSpacing: "0.05em",
            }}
          >
            {label}
          </Text>
          <IconButton
            size="1"
            variant="ghost"
            style={{ color: "var(--indigo-9)" }}
          >
            <InfoCircledIcon />
          </IconButton>
        </Flex>
        <Heading
          size="8"
          weight="bold"
          style={{
            background:
              "linear-gradient(90deg, var(--indigo-9), var(--purple-9))",
            WebkitBackgroundClip: "text",
            WebkitTextFillColor: "transparent",
            marginBottom: "4px",
          }}
        >
          {displayValue}
        </Heading>
        {change && (
          <Text
            size="2"
            style={{
              color: isPositive ? "var(--jade-11)" : "var(--tomato-11)",
              fontWeight: "medium",
            }}
          >
            {account ? (isPositive ? "+" : "") : ""}
            {displayChange}
          </Text>
        )}
      </Flex>
    </Card>
  );
}

interface VaultCardProps {
  title: string;
  percentage: number;
  apy: string;
  amount: string;
  color: string;
  icon?: React.ReactNode;
}

function VaultCard({ title, percentage, apy, amount, color }: VaultCardProps) {
  const [wallet] = useWallets();
  const account = wallet?.accounts?.[0];

  const displayPercentage = account ? percentage : 0;
  const displayApy = account ? apy : "-";
  const displayAmount = account ? amount : "-";

  return (
    <Card
      style={{
        background: "rgba(255, 255, 255, 0.7)",
        border: `1px solid ${color}`,
        borderRadius: "16px",
        padding: "24px",
        boxShadow:
          "0 10px 25px -5px rgba(0, 0, 0, 0.05), 0 8px 10px -6px rgba(0, 0, 0, 0.02)",
        transition: "transform 0.2s, box-shadow 0.2s",
      }}
      onMouseEnter={(e) => {
        e.currentTarget.style.transform = "translateY(-5px)";
        e.currentTarget.style.boxShadow =
          "0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -6px rgba(0, 0, 0, 0.04)";
      }}
      onMouseLeave={(e) => {
        e.currentTarget.style.transform = "none";
        e.currentTarget.style.boxShadow =
          "0 10px 25px -5px rgba(0, 0, 0, 0.05), 0 8px 10px -6px rgba(0, 0, 0, 0.02)";
      }}
    >
      <Flex direction="column" gap="3">
        <Heading
          size="4"
          style={{
            color: color,
          }}
        >
          {title}
        </Heading>
        <Box>
          <Flex justify="between" mb="1">
            <Text size="2" style={{ color: "var(--gray-11)" }}>
              Allocation
            </Text>
            <Text size="2" weight="bold" style={{ color: color }}>
              {account ? `${displayPercentage}%` : "-"}
            </Text>
          </Flex>
          <Progress value={displayPercentage} />
        </Box>
        <Grid columns="2" gap="3" mt="2">
          <Box>
            <Text size="2" style={{ color: "var(--gray-11)" }}>
              APY
            </Text>
            <Text size="3" weight="bold" style={{ color: color }}>
              {displayApy}
            </Text>
          </Box>
          <Box>
            <Text size="2" style={{ color: "var(--gray-11)" }}>
              Amount
            </Text>
            <Text size="3" weight="bold" style={{ color: "var(--gray-12)" }}>
              {displayAmount}
            </Text>
          </Box>
        </Grid>
      </Flex>
    </Card>
  );
}

function Dashboard() {
  const [wallet] = useWallets();
  const account = wallet?.accounts?.[0];

  return (
    <Box
      style={{
        background: "linear-gradient(135deg, var(--indigo-1), var(--purple-1))",
        color: "var(--gray-12)",
        minHeight: "100vh",
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
          <Flex justify="between" align="center">
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
              Dashboard
            </Heading>
            <Button
              size="3"
              style={{
                background:
                  "linear-gradient(45deg, var(--jade-9), var(--mint-9))",
                borderRadius: "20px",
                color: "white",
                boxShadow: "0 4px 14px rgba(0, 160, 120, 0.4)",
                transition: "transform 0.2s, box-shadow 0.2s",
              }}
              onMouseEnter={(e) => {
                e.currentTarget.style.transform = "translateY(-2px)";
                e.currentTarget.style.boxShadow =
                  "0 6px 20px rgba(0, 160, 120, 0.5)";
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.transform = "none";
                e.currentTarget.style.boxShadow =
                  "0 4px 14px rgba(0, 160, 120, 0.4)";
              }}
            >
              <PlusIcon style={{ marginRight: "4px" }} />
              Charge USDC
            </Button>
          </Flex>
        </Container>
      </Section>

      <Section
        size="3"
        style={{
          paddingTop: "40px",
          paddingBottom: "40px",
        }}
      >
        <Container mx="auto" size="3">
          <Heading
            size="5"
            style={{
              marginBottom: "24px",
              color: "var(--indigo-10)",
            }}
          >
            My Assets
          </Heading>
          <Grid columns={{ initial: "1", sm: "3" }} gap="6">
            <AssetCard
              label="zBTC Balance"
              value="1.25 BTC"
              change="0.05 BTC (4.2%)"
              isPositive={true}
            />
            <AssetCard
              label="zUSD Balance"
              value="$15,750"
              change="$750 (5.0%)"
              isPositive={true}
            />
            <AssetCard
              label="szUSD Balance"
              value="$8,320"
              change="$320 (4.0%)"
              isPositive={true}
            />
          </Grid>
        </Container>
      </Section>

      <Section
        size="3"
        style={{
          background:
            "linear-gradient(180deg, var(--indigo-1) 0%, var(--gray-1) 100%)",
          paddingTop: "40px",
          paddingBottom: "40px",
          borderTop: "1px solid rgba(79, 70, 229, 0.1)",
          borderBottom: "1px solid rgba(79, 70, 229, 0.1)",
        }}
      >
        <Container mx="auto" size="3">
          <Heading
            size="5"
            style={{
              marginBottom: "24px",
              color: "var(--indigo-10)",
            }}
          >
            Accumulated Yield
          </Heading>
          <Grid columns={{ initial: "1", sm: "2" }} gap="6">
            <Card
              size="3"
              style={{
                background:
                  "linear-gradient(135deg, rgba(99, 102, 241, 0.1) 0%, rgba(79, 70, 229, 0.2) 100%)",
                border: "1px solid rgba(99, 102, 241, 0.15)",
                boxShadow:
                  "0 10px 25px -5px rgba(0, 0, 0, 0.1), 0 8px 10px -6px rgba(0, 0, 0, 0.05)",
              }}
            >
              <Flex direction="column" gap="3">
                <Heading
                  size="4"
                  style={{
                    color: "var(--indigo-11)",
                  }}
                >
                  Total Yield
                </Heading>
                <Heading
                  size="8"
                  style={{
                    background:
                      "linear-gradient(90deg, var(--jade-9), var(--mint-9))",
                    WebkitBackgroundClip: "text",
                    WebkitTextFillColor: "transparent",
                  }}
                >
                  {account ? "$1,245.65" : "-"}
                </Heading>
                <Text style={{ color: "var(--gray-11)" }}>
                  Since January 2025
                </Text>
              </Flex>
            </Card>
            <Card
              size="3"
              style={{
                background:
                  "linear-gradient(135deg, rgba(99, 102, 241, 0.1) 0%, rgba(79, 70, 229, 0.2) 100%)",
                border: "1px solid rgba(99, 102, 241, 0.15)",
                boxShadow:
                  "0 10px 25px -5px rgba(0, 0, 0, 0.1), 0 8px 10px -6px rgba(0, 0, 0, 0.05)",
              }}
            >
              <Flex direction="column" gap="3">
                <Heading
                  size="4"
                  style={{
                    color: "var(--indigo-11)",
                  }}
                >
                  Current APY
                </Heading>
                <Heading
                  size="8"
                  style={{
                    background:
                      "linear-gradient(90deg, var(--indigo-9), var(--purple-9))",
                    WebkitBackgroundClip: "text",
                    WebkitTextFillColor: "transparent",
                  }}
                >
                  {account ? "15.2%" : "-"}
                </Heading>
                <Text style={{ color: "var(--jade-11)" }}>
                  {account ? "+2.3% from last month" : "-"}
                </Text>
              </Flex>
            </Card>
          </Grid>
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
          <Heading
            size="5"
            style={{
              marginBottom: "24px",
              color: "var(--indigo-10)",
            }}
          >
            Staked zUSD Management
          </Heading>
          <Text
            size="3"
            style={{
              color: "var(--gray-11)",
              marginBottom: "24px",
            }}
          >
            Your staked zUSD is distributed across multiple vaults to optimize
            yield and minimize risk.
          </Text>
          <Grid columns={{ initial: "1", sm: "3" }} gap="6">
            <VaultCard
              title="Drift Vault"
              percentage={45}
              apy="16.8%"
              amount="$3,741"
              color="rgba(147, 51, 234, 0.9)"
            />
            <VaultCard
              title="Kamino Vault"
              percentage={35}
              apy="14.5%"
              amount="$2,912"
              color="rgba(59, 130, 246, 0.9)"
            />
            <VaultCard
              title="zUSD Pool"
              percentage={20}
              apy="12.3%"
              amount="$1,664"
              color="rgba(236, 72, 153, 0.9)"
            />
          </Grid>
        </Container>
      </Section>

      <Section
        size="3"
        style={{
          background:
            "linear-gradient(135deg, var(--indigo-2), var(--purple-2))",
          paddingTop: "40px",
          paddingBottom: "60px",
          borderTop: "1px solid rgba(99, 102, 241, 0.2)",
        }}
      >
        <Container mx="auto" size="3">
          <Flex direction="column" align="center" gap="4">
            <Heading
              size="6"
              style={{
                background:
                  "linear-gradient(90deg, var(--indigo-11), var(--purple-11))",
                WebkitBackgroundClip: "text",
                WebkitTextFillColor: "transparent",
                textAlign: "center",
              }}
            >
              Manage Your Assets
            </Heading>
            <Flex gap="4" mt="4" wrap="wrap" justify="center">
              <Button
                size="3"
                style={{
                  background:
                    "linear-gradient(45deg, var(--indigo-9), var(--purple-9))",
                  borderRadius: "20px",
                  color: "white",
                  boxShadow: "0 4px 14px rgba(79, 70, 229, 0.4)",
                  transition: "transform 0.2s, box-shadow 0.2s",
                }}
                onMouseEnter={(e) => {
                  e.currentTarget.style.transform = "translateY(-2px)";
                  e.currentTarget.style.boxShadow =
                    "0 6px 20px rgba(79, 70, 229, 0.5)";
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.transform = "none";
                  e.currentTarget.style.boxShadow =
                    "0 4px 14px rgba(79, 70, 229, 0.4)";
                }}
              >
                <Link
                  to="/mint"
                  style={{ color: "inherit", textDecoration: "none" }}
                >
                  Mint New Assets
                </Link>
              </Button>
              <Button
                size="3"
                style={{
                  background:
                    "linear-gradient(45deg, var(--jade-9), var(--mint-9))",
                  borderRadius: "20px",
                  color: "white",
                  boxShadow: "0 4px 14px rgba(0, 160, 120, 0.4)",
                  transition: "transform 0.2s, box-shadow 0.2s",
                }}
                onMouseEnter={(e) => {
                  e.currentTarget.style.transform = "translateY(-2px)";
                  e.currentTarget.style.boxShadow =
                    "0 6px 20px rgba(0, 160, 120, 0.5)";
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.transform = "none";
                  e.currentTarget.style.boxShadow =
                    "0 4px 14px rgba(0, 160, 120, 0.4)";
                }}
              >
                <PlusIcon style={{ marginRight: "4px" }} />
                Charge USDC
              </Button>
              <Button
                size="3"
                style={{
                  background:
                    "linear-gradient(45deg, var(--amber-9), var(--orange-9))",
                  borderRadius: "20px",
                  color: "white",
                  boxShadow: "0 4px 14px rgba(230, 160, 40, 0.4)",
                  transition: "transform 0.2s, box-shadow 0.2s",
                }}
                onMouseEnter={(e) => {
                  e.currentTarget.style.transform = "translateY(-2px)";
                  e.currentTarget.style.boxShadow =
                    "0 6px 20px rgba(230, 160, 40, 0.5)";
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.transform = "none";
                  e.currentTarget.style.boxShadow =
                    "0 4px 14px rgba(230, 160, 40, 0.4)";
                }}
              >
                <Link
                  to="/stake"
                  style={{ color: "inherit", textDecoration: "none" }}
                >
                  Manage Staking
                </Link>
              </Button>
            </Flex>
          </Flex>
        </Container>
      </Section>
    </Box>
  );
}

export default Dashboard;

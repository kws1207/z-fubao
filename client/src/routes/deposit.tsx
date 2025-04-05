import { StakePanel } from "../components/ConvertPanel";
import { Container } from "@radix-ui/themes";
function Deposit() {
  return (
    <Container
      mx="auto"
      style={{
        padding: "20px",
        display: "flex",
        justifyContent: "center",
      }}
    >
      <StakePanel mode="btc" label="Native BTC" />
    </Container>
  );
}

export default Deposit;

import { StakePanel } from "../components/ConvertPanel";
import { Container } from "@radix-ui/themes";

function Earn() {
  return (
    <Container
      mx="auto"
      style={{
        padding: "20px",
        display: "flex",
        justifyContent: "center",
      }}
    >
      <StakePanel mode="zusd" label="zUSD" />
    </Container>
  );
}

export default Earn;

import { LockAndMintPanel } from "../components/LockAndMintPanel";
import { Container } from "@radix-ui/themes";
function Mint() {
  return (
    <Container
      mx="auto"
      style={{
        padding: "20px",
        display: "flex",
        justifyContent: "center",
      }}
    >
      <LockAndMintPanel />
    </Container>
  );
}

export default Mint;

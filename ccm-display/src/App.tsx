import { useState } from "react";
import MappingsList from "./components/MappingsList";
import { Link, Spacer, Text } from "@chakra-ui/react";
import "./App.css";

function App() {
  const [count, setCount] = useState(0);

  return (
    <div className="App">
      <Text fontSize="2xl" fontWeight={500}>
        Clippy C2Rust Mapping
      </Text>
      <Text>
        Code snippets that map C to Rust code using the{" "}
        <Link color="teal.500" href="https://c2rust.com/">
          C2Rust
        </Link>{" "}
        tool.
      </Text>

      <Spacer height={10} />

      <MappingsList />
    </div>
  );
}

export default App;

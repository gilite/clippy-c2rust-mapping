import { useState } from "react";
import Code from "./Code";
import { Spacer, Text } from "@chakra-ui/react";

function CRustCard({
  cContent,
  rustContent,
}: {
  cContent: string;
  rustContent: string;
}) {
  return (
    <div style={{ display: "flex", width: "100%" }}>
      <div>
        <Text>C</Text>
        <Code language="c" content={cContent} />
      </div>
      
      <Spacer width={2} />

      <div>
        <Text>Rust</Text>
        <Code language="rust" content={rustContent} />
      </div>
    </div>
  );
}

export default CRustCard;
